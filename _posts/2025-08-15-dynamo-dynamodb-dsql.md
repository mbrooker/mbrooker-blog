---
layout: post
title: "Dynamo, DynamoDB, and Aurora DSQL"

related_posts:
  - "/2022/07/12/dynamodb.html"
  - "/2024/12/04/inside-dsql.html"
  - "/2024/12/03/aurora-dsql.html"
dissimilar_posts:
  - "/2015/04/11/zero-one.html"
---
{{ page.title }}
================

<script>
  MathJax = {
    tex: {inlineMath: [['$', '$'], ['\\(', '\\)']]}
  };
</script>
<script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>


<p class="meta">Names are hard, ok?</p>

People often ask me about the architectural relationship between [Amazon Dynamo](https://www.allthingsdistributed.com/files/amazon-dynamo-sosp2007.pdf) (as described in the classic 2007 SOSP paper), [Amazon DynamoDB](https://aws.amazon.com/dynamodb/) (the serverless distributed NoSQL database from AWS), and [Aurora DSQL](https://aws.amazon.com/rds/aurora/dsql/) (the serverless distributed SQL database from AWS). There's a ton to say on the topic, but I'll start off on comparing how the systems achieve a few key properties.

The key references for this post are:

* For Dynamo, [Dynamo: Amazon’s Highly Available Key-value Store](https://www.allthingsdistributed.com/files/amazon-dynamo-sosp2007.pdf) from SOSP'07.
* For DynamoDB, [Amazon DynamoDB: A Scalable, Predictably Performant, and Fully Managed NoSQL Database Service](https://www.usenix.org/conference/atc22/presentation/elhemali) from ATC'22, [Distributed Transactions at Scale in Amazon DynamoDB](https://www.usenix.org/conference/atc23/presentation/idziorek) from ATC'23, and [Lessons learned from 10 years of DynamoDB](https://www.amazon.science/blog/lessons-learned-from-10-years-of-dynamodb) from the Amazon Science blog.
* For DSQL, [my blog series on DSQL](https://brooker.co.za/blog/2024/12/03/aurora-dsql.html).

*Durability*

The databases we're looking at offer different levels of durability, but all three are designed not to lose data when a single host fails. Dynamo does this by taking advantage of its *consistent hashing* approach, replicating the data across multiple hosts in order in the hash ring:

> To achieve high availability and durability, Dynamo replicates its data on multiple hosts. Each data item is replicated at N hosts. ... Each key, k, is assigned to a coordinator node[]. The coordinator is in charge of the replication of the data items that fall within its range. In addition to locally storing each key within its range, the coordinator replicates these keys at the N-1 clockwise successor nodes in the ring.

Like Dynamo, DynamoDB assigns a node in a hash ring to each individual item. But that's where the similarities stop. Instead of replicating across multiple nodes in the ring, in DynamoDB each node consists of a replica group with multiple servers in multiple AZs using Paxos to replicate the data. Instead of appearing the ring *N* times, each item appears once, and takes advantage of fault-tolerant nodes rather than spreading over multiple nodes.

>  Upon receiving a write request, the leader of the replication group for the key being written generates a write-ahead log record and sends it to its peer (replicas). A write is acknowledged to the application once a quorum of peers persists the log record to their local write-ahead logs.

DynamoDB's approach to durability has several advantages over Dynamo's. First, because durability is based on replica sets and replica sets have much lower cardinality than keys, it's much easier for the system to find and react to cases where there aren't enough copies of a key and respond appropriately<sup>[2](#foot2)</sup>. Second, it doesn't require a drop in durability during scale up or scale down: with Dynamo scaling changes the set of replicas for a key, with DynamoDB scaling is done by splitting or merging replica sets with no decrease in the number of copies.

> Once the adjudicator has decided we can commit our transaction, we write it to the Journal for replication. (from [DSQL Vignette: Transactions and Durability](https://brooker.co.za/blog/2024/12/05/inside-dsql-writes.html))

DSQL is different from the other two. Like DynamoDB, it uses a Paxos variant to replicate a log of changes. Unlike DynamoDB, this is done with an additional component (the Journal), independent from the storage nodes. This brings the same benefits as DynamoDB, but additionally allows independent scaling of reads and writes, and cross-shard atomic commitment of changes. DSQL also (primarily) uses a range-based primary key sharding scheme, as opposed to Dynamo and DynamoDB's hash-based schemes. The trade-offs between these choices are worth their own blog.

*Consistency*

Dynamo offers only eventual consistency to clients.

> Dynamo provides eventual consistency, which allows for updates to be propagated to all replicas asynchronously.

It does, however, spend some effort ensuring that replicas converge, and the paper somewhat confusingly also refers to this as consistency. 

> To maintain consistency among its replicas, Dynamo uses a consistency protocol similar to those used in quorum systems. This protocol has two key configurable values: R and W. R is the minimum number of nodes that must participate in a successful read operation. W is the minimum number of nodes that must participate in a successful write operation. Setting R and W such that R + W > N yields a quorum-like system.

I'll admit that I'm a little confused by the way `R + W > N` is treated here, because it doesn't seem to align with the way the rest of the paper talks about consistency, and offers a path to stronger consistency as some Dynamo-inspired designs have achieved.

DynamoDB, by constrast, offers strongly consistent writes, and a choice of strongly consistent and eventually consistent reads. The choice approach is rather simple:

> Only the leader replica can serve write and strongly consistent read requests.

and

> Any replica of the replication group can serve eventually consistent reads.

This is a nice model, because it allows applications that can tolerate eventually consistent reads to opt in for reduced cost and latency, while keeping all writes strongly consistent (and avoiding all the complexity Dynamo has with vector clocks and object versioning, which come from accepting weak writes). It also offers strong consistency without application developers needing to understanding things like quorum (which, let's be honest, most don't).

DSQL, on the other hand, uses a combination of physical time and multi-version concurrency control to offer strong consistency for all reads and writes, even in long-running interactive transactions.

> To do that, we start every transaction by picking a transaction start time, $\tau_{start}$. We use EC2's [precision time infrastructure](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/set-time.html) which provides an accurate clock with strong error bounds. Then, for each read that the QP does to storage, it asks storage to do that read *as of* $\tau_{start}$. (from [DSQL Vignette: Reads and Compute](https://brooker.co.za/blog/2024/12/04/inside-dsql.html))

DSQL's approach has two benefits over DynamoDB's: strongly consistent reads can go to any storage replica, and strong consistency can be maintained even for interactive transactions, while never blocking writers. The cost of this is additional complexity, and the dependency on physical time. DSQL could offer weakly consistent reads with slightly lowered latency (by omitting the $\tau_{start}$ check and simply reading the latest version of a key, for example), but currently doesn't.

*Programming Model*

Dynamo is a simple key-value store, that doesn't offer transactions of any kind:

> Dynamo does not provide any isolation guarantees and permits only single key updates.

DynamoDB offers [single-shot](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/transaction-apis.html) serializable ACID transactions, with a single transaction consisting of multiple reads and writes. DSQL has the richest programming model, offering interactive transactions, full SQL support, and a rich type system. 

*Availability and Latency*

The Dynamo paper makes a number of claims about the trade-offs between consistency, availability, and latency that have not stood the test of time. I'm not trying to *call out* the paper authors (several are personal friends of mine, and many are long-time colleagues), but point out that we've learned a lot about building distributed databases in 20 years. Cloud infrastructure has also advanced considerably.

> Experience at Amazon has shown that data stores that provide ACID guarantees tend to have poor availability.

This was true in the mid 2000s, but many ACID systems offer excellent availability today. That includes DynamoDB, DSQL, and others like Aurora Postgres. DynamoDB and DSQL can tolerate the failure of hosts, or an entire availability zone, without losing consistency, durability, or availability.

> From the very early replicated database works, it is well known that when dealing with the possibility of network failures, strong consistency and high data availability cannot be achieved simultaneously.

Here, the Dynamo paper is citing [Bernstein and Goodman](https://dl.acm.org/doi/10.1145/1994.2207) (from 1984) and [Lindsay et al](https://www.scribd.com/document/767274926/Notes-on-Distributed-Databases)<sup>[1](#foot1)</sup> (from 1979) to highlight the inherent trade-offs between availability and consistency. These results aren't in any way wrong, but ([as I've argued before](https://brooker.co.za/blog/2024/07/25/cap-again.html)), they aren't as practically important as the Dynamo paper implies they are. Strongly consistent systems offer excellent availability in the face of failures of many kinds ([including entire region failures](https://brooker.co.za/blog/2024/12/06/inside-dsql-cap.html)).

Dynamo also allows applications to pick different trade-offs for performance, losing durability, consistency, or availability in the process.

> The main advantage of Dynamo is that its client applications can tune the values of N, R and W to achieve their desired levels of performance, availability and durability.

This made complete sense in the mid-2000s. But better ways of thinking about replication and failure correlation, vastly improved system performance (thanks SSDs!), and much better datacenter networks have made this kinds of tunability uninteresting. It's notable that both DynamoDB and DSQL offer significantly lower latencies than Dynamo while making none of the associated trade-offs discussed in the paper.

*Conclusion*

The Amazon Dynamo paper is a classic. You should read it if you haven't. But time has marched on, we've learned a ton, we've got better hardware and better ideas, and much of what the Dynamo paper says doesn't make sense in the real world anymore. That's a good thing!

*Footnotes*

1. <a name="foot1"></a> *et al* is doing some heavy lifting here, with other authors including Pat Selinger, Jim Gray, and Franco Putzolu. 
2. <a name="foot2"></a> See the discussion of *read repair* in the Dynamo paper, and think about what happens with infrequently-read keys.