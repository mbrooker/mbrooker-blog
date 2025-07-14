---
layout: post
title: "Is there a CAP theorem for Durability?"









related_posts:
  - "/2024/07/25/cap-again.html"
  - "/2014/07/16/pacelc.html"
  - "/2018/02/25/availability-liveness.html"
dissimilar_posts:
  - "/2020/07/28/fish.html"
---
{{ page.title }}
================

<p class="meta">Expanding the taxonomy of distributed systems.</p>

The CAP theorem considers only two of the axes of tradeoffs in distributed systems design. There are many others, including operability, security, latency, integrity, efficiency, and durability. I was recently talking over a beer or two with a colleague about whether there is a CAP theorem for durability (DAP theorem?). These are my thoughts.

**What is durability?**

To have a meaningful conversation, we need to talk about what durability is. Its typically given a few meanings:

 * Persistence of information to stable storage, to tolerate loss of in-memory (volatile) state. This is the *D* in ACID.
 * Loss of the data stored in a database. This is typically measured using population statistics. [Annualized failure rate (AFR)](https://en.wikipedia.org/wiki/Annualized_failure_rate), and Mean Time to Data Loss (MTTDL) are typical, easy to understand, (but flawed<sup>[1](#foot1)</sup>) metrics.
 * Loss of recently committed transactions or recently-written data.

On single-node systems, these topics are deeply connected. Persistence to stable storage is required to keep data around across crashes. RAID and backups<sup>[2](#foot2)</sup> are widely used to protect against permanent loss of the single system. Traditionally, non-zero [RPO](https://en.wikipedia.org/wiki/Recovery_point_objective) is tolerated on node failure.

Distributed systems can be different. Instead of having a single gold-plated node with its own great durability properties, distributed databases spread the risk out over multiple machines. That unlinks the topics of persistence to stable storage and loss of data, where systems can be tolerant to some number of node reboots without any stable storage.

For the rest of this post I'll define durability as "the ability to tolerate t node failures without losing data". It's a flawed but hopefully useful definition.

**What has this all got to do with CAP?**

The limits on consistency are well-known. CAP is one boundary in one system model and set of definitions, another (possibly more useful) one is from the all-time classic [Consensus in the Presence of Partial Synchrony](http://groups.csail.mit.edu/tds/papers/Lynch/jacm88.pdf):

> For fail-stop or omission faults we show that t-resilient consensus is possible iff N &ge; 2t + 1

What that means is that you can build systems that keep on going<sup>[3](#foot3)</sup> even if *t* things fail, as long as at least *t + 1* things don't fail. It also means you can keep going in the majority side of a network partition, if one exists. In the Gilbert and Lynch *total availability* sense, that means the system is not available<sup>[4](#foot4)</sup>. In the common sense, the system is still available for everybody on the majority side of the partition.

There's a similar definition possible for durability: "*For fail-stop or omission faults, t-resilient durability is possible iff N &ge; t + 1*".

The next step in developing the DAP theorem is to define *failed*. We quickly descend into rule-lawyering.

Alternative 1: Nodes that we can't talk to count as failed. If we're in a system of *N = t + 1* nodes, and we can't talk to *k* other nodes, we can accept writes. That's because, in this state, we've already got *k* failures, so we only need to tolerate another *t - k*. That's not a very helpful definition.

Alternative 2: Nodes that we can't talk to don't count as failed. If we're in a system of *N = t + 1* nodes, we can only accept writes if we can talk to another *t* nodes.

In alternative 1, you can stay common-sense available on both sides of a partition. In alternative 2, any partition causes unavailability in both senses. Neither is a very useful definition, and our DAP theorem doesn't seem useful at all.

**Towards a useful rule**

Abadi's [PACELC](http://cs-www.cs.yale.edu/homes/dna/papers/abadi-pacelc.pdf) could be a better fit for durability. Let's revisit Abadi's definition:

>  if there is a partition (P), how does the system trade off availability and consistency (A and C); else (E), when the system is running normally in the absence of partitions, how does the system trade off latency (L) and consistency (C)?

Replacing C with our definition of D (the ability to tolerate t node failures without losing data), and defining A as the common-sense version of availability (*at least some clients are able to make writes*), we get PADELD.

>  if there is a partition (P), how does the system trade off availability and durability (A and D); else (E), when the system is running normally in the absence of partitions, how does the system trade off latency (L) and durability (D)?

That actually does seem to be helpful, in the sense that it could be used to have a real conversation about real systems. In the happy *E* case, the tradeoff between latency and durability could be between synchronous and asynchronous replication, or it could be between different write quorum sizes. Asynchronous replication reduces latency because fewer steps are required, or particularly expensive steps (like cross-WAN replication) are skipped. Smaller write quorums (for example, writing to 2 of 3 replicas) also reduces latency, especially outlier latency, because writes can be acked while replication is still proceeding to slower replicas. In both cases, a failure is unlikely to lead to complete data loss, but rather some non-zero RPO, where recent writes are more likely to be lost than colder data.

In the partition *P* case, the tradeoff is between availability and durability. The concerns here are the same as in the *E* case, and the implementation flavors will be very similar. The partition case is meaningfully distinct, because systems may either change behavior based on failure detection (choosing to lower durability during partitions), or may best-effort replicate but give up after some latency target has been breached.

 * PD/ED is a pure synchronous replication pattern, where writes are rejected if they can't be offered full durability.
 * PA/ED could be a system with either modal or latency-target based behavior that generally chooses durability, but may fall back to availability if that can't be achieved.
 * PA/EL is a pure asynchronous or quorum replication system, which offers a non-zero RPO for *t* failures at all times.
 * PD/EL appears to be meaningless.

*PADELD* may actually be a useful taxonomy of durability behaviors. Durability, at least if we only consider RPO and *t-resiliency*, is also a less subtle topic than consistency, so it may even be a more useful tool than PACELC in its own right.

**Footnotes:**

 1. <a name="foot1"></a> Greenan et al's [Mean time to meaningless](http://web.eecs.utk.edu/~plank/plank/papers/Hot-Storage-2010.pdf) does a good job of explaining why these metrics aren't ideal descriptions of true storage system behavior. They propose a different metric, NoMDL, which captures some of the missing subtlety but may be significantly more difficult to understand.
 2. <a name="foot2"></a> Although backups are a kind of distributed replica.
 3. <a name="foot3"></a> By *keep on going* I mean *keep on doing the consensus thing*.
 4. <a name="foot4"></a> The *total availability* definition from [Brewer's conjecture and the feasibility of consistent, available, partition-tolerant web services](http://dl.acm.org/citation.cfm?id=564601&CFID=716755369&CFTOKEN=66839118) is "For a distributed system to be continuously available, every request received by a non-failing node in the system must result in a response." A more common sense definition is something like "For a distributed system to be continuously available, some requests received by the system must result in a response within some goal latency". Gilbert and Lynch's definition leads to a more beautiful CAP theorem, but probably a less helpful one.