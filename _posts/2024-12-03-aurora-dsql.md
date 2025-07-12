---
layout: post
title: "DSQL Vignette: Aurora DSQL, and A Personal Story"








related_posts:
  - "/2024/12/04/inside-dsql.html"
  - "/2024/07/29/aurora-serverless.html"
  - "/2022/07/12/dynamodb.html"
dissimilar_posts:
  - "/2015/05/24/sodium-carbonate.html"
---
{{ page.title }}
================

<p class="meta">It's happening.</p>

In this morning's re:Invent keynote, Matt Garman announced Aurora DSQL. We're all excited, and some extremely excited, to have this preview release in customers' hands. Over the next few days, I'm going to be writing a few posts about what DSQL is, how it works, and how to make the best use of it. This post is going to look at the product itself, and a little bit of a personal story. 

The official [AWS documentation for Aurora DSQL](https://aws.amazon.com/rds/aurora/dsql/features/) is a great place to start to understand what DSQL is and how to use it.

*What is Aurora DSQL?*

Aurora DSQL is a new serverless SQL database, optimized for transaction processing, and designed for the cloud. DSQL is designed to scale up and down to serve workloads of nearly any size, from your hobby project to your largest enterprise application. All the SQL stuff you expect is there: transactions, schemas, indexes, joins, and so on, all with strong consistency and isolation<sup>[5](#foot5)</sup>.

DSQL offers active-active multi-writer capabilities in multiple availability zones (AZs) in a single region, or across multiple regions. Reads and writes, even in read-write transactions, are fast and local, requiring no cross-region communication (or cross-AZ communication in single region setups). Transaction commit goes across regions (for multi-region setups) or AZs (for single-regions setups), ensuring that your transactions are durable, isolated, and atomic.

DSQL is PostgreSQL compatible, offering a subset of PostgreSQL's (huge) SQL feature set. You can connect with your favorite PostgreSQL client (even the `psql` cli), use your favorite ORMs and frameworks, etc. We'll be adding more PostgreSQL-compatible features over time, making it easy to bring your existing code to DSQL.

DSQL is serverless. Here, we mean that you create a cluster in the AWS console (or API or CLI), and that cluster will include an endpoint. You connect your PostgreSQL client to that endpoint. That's all you have to do: management, scalability, patching, fault tolerance, durability, etc are all built right in. You never have to worry about infrastructure.

As we launch Aurora DSQL, we're talking a lot about multi-region active-active, but that's not the only thing its for. We built DSQL to be a great choice for single-region applications of all sizes - from a few requests per day to thousands a second and beyond.

*A Personal Story*

In 2020 I was working on serverless compute at AWS, spending most of my time with the great AWS Lambda team<sup>[1](#foot1)</sup>. As always, I spent a lot of time talking to customers, and realized that I was hearing two consistent things from serverless and container customers:

* Existing relational database offerings weren't a great fit for the fast-moving scalable world of serverless and containers. These customers loved relational databases and SQL, for all the reasons folks have loved relational for forty years, but felt a lot of friction between the needs of serverless compute and existing relational products. [Amazon RDS Proxy](https://aws.amazon.com/rds/proxy/) helped with some of this friction, but it wasn't going away.

* Large, highly-regulated, AWS customers with global businesses were building applications across multiple AWS regions, but running into a tricky architectural trade-off. They could pick multi-region active-active (with DynamoDB Global Tables, for example), but lose out on SQL, ACID, and strong cross-region consistency. Or they could choose active-standby (with Aurora Global Database, for example), but lose the peace of mind of having their application actively running in multiple places, and the ability to serve strongly consistent data to customers from their closest region. These customers wanted both things.

At the same time, a few pieces of technology were coming together. One was a set of new virtualization capabilities, including Caspian (which can dynamically and securely scale the resources allocated to a virtual machine up and down), Firecracker<sup>[3](#foot3)</sup> (a lightweight VMM for fast-scaling applications), and the VM snapshotting technology we were using to build [Lambda Snapstart](https://docs.aws.amazon.com/lambda/latest/dg/snapstart.html). We used Caspian to build Aurora Serverless V2<sup>[2](#foot2)</sup>, bringing a vertical auto scaling to Aurora's full feature set. 

The second was EC2 [time sync](https://aws.amazon.com/blogs/compute/its-about-time-microsecond-accurate-clocks-on-amazon-ec2-instances/), which brings microsecond-accurate time to EC2 instances around the globe. High-quality physical time is [hugely useful for all kinds of distributed system problems](https://brooker.co.za/blog/2023/11/27/about-time.html). Most interestingly, it unlocks ways to avoid coordination within distributed systems, offering better scalability and better performance. The new horizontal sharding capability for Aurora Postgres, [Aurora Limitless Database](https://aws.amazon.com/blogs/aws/amazon-aurora-postgresql-limitless-database-is-now-generally-available/), uses these clocks to make cross-shard transactions more efficient.

The third was Journal, the distributed transaction log we'd used to build critical parts of multiple AWS services (such as [MemoryDB](https://aws.amazon.com/memorydb/), the Valkey compatible durable in-memory database<sup>[4](#foot4)</sup>). Having a reliable, proven, primitive that offers atomicity, durability, and replication between both availability zones and regions simplifies a lot of things about building a database system (after all, *A*tomicity and *D*urability are half of ACID).

The fourth was AWS's strong [formal methods and automated reasoning tool set](https://aws.amazon.com/blogs/security/an-unexpected-discovery-automated-reasoning-often-makes-systems-more-efficient-and-easier-to-maintain/). Formal methods allow us to explore the space of design and implementation choices quickly, and also helps us build reliable and dependable distributed system implementations<sup>[6](#foot6)</sup>. Distributed databases, and especially fast distributed transactions, are a famously hard design problem, with tons of interesting trade-offs, lots of subtle traps, and a need for a strong correctness argument. Formal methods allowed us to move faster and think bigger about what we wanted to build.

Finally, AWS has been building big cloud systems for a long time ([S3 is turning 19 next year!](https://aws.amazon.com/blogs/aws/celebrate-amazon-s3s-17th-birthday-at-aws-pi-day-2023/), can you believe it?), and we have a ton of experience. Along with that experience is an incredible pool of talented engineers, scientists, and leaders who know how to build and operate things. If there's one thing that's AWS's real secret sauce, it's that our engineers and leaders are close to the day-to-day operation of our services<sup>[7](#foot7)</sup>, bringing a constant flow of real-world lessons of how to improve our existing services and build better new ones.

The combination of all of these things made it the right time to think big about building a new distributed relational database. We knew we wanted to solve some really hard problems on behalf of our customers, and we were starting to see how to solve them. 

So, in 2021 I started spending a lot more time with the databases teams at AWS, including the incredibly talented teams behind Aurora and QLDB. We built a team to go do something audacious: build a new distributed database system, with SQL and ACID, global active-active, scalability both up and down (with independent scaling of compute, reads, writes, and storage), PostgreSQL compatibility, and a serverless operational model. I'm proud of the incredibly talented group of people that built this, and can't wait to see how our customers use it.

*One Big Thing*

There are a lot of interesting benefits to the approach we've taken with DSQL, but there's one I'm particularly excited about (the same one Matt highlighted in the keynote): the way that latency scales with the number of statements in a transaction. For cross-region active-active, latency is all about round-trip times. Even if you're 20ms away from the quorum of regions, making a round trip (such as to a lock server) on every statement really hurts latency. In DSQL local in-region reads are as fast as 1.2ms, so 20ms on top of that would really hurt.

From the beginning, we took avoiding this as a key design goal for our transaction protocol, and have achieved our goals. In Aurora DSQL, you only incur additional cross-region latency on `COMMIT`, not for each individual `SELECT`, `UPDATE`, or `INSERT` in your transaction (from any of the endpoints in an active-active setup). That's important, because even in the relatively simple world of OLTP, having 10s or even 100s of statements in a transaction is common. It's only when you `COMMIT` (and then only when you `COMMIT` a read-write transaction) that you incur cross-region latency. Read-only transactions, and read-only autocommit `SELECT`s are always in-region and fast (and strongly consistent and isolated).

In designing DSQL, we wanted to make sure that developers can take advantage of the full power of transactions, and the full power of SQL. Later this week I'll share more about how that works under the covers. The goal was to simplify the work of developers and architects, and make it easier to build reliable, scalable, systems in the cloud.

*A Few Other Things*

In Aurora DSQL, we've chosen to offer strong consistency and *snapshot isolation*. Having observed teams at Amazon build systems for over twenty years, we've found that application programmers find dealing with eventual consistency difficult, and exposing eventual consistency by default leads to application bugs. Eventual consistency absolutely does have its place in distributed systems<sup>[8](#foot8)</sup>, but strong consistency is a good default. We've designed DSQL for strongly consistent in-region (and in-AZ) reads, giving many applications strong consistency with few trade-offs.

We've also picked snapshot isolation by default. We believe that snapshot isolation<sup>[9](#foot9)</sup> is, in distributed databases, a sweet spot that offers both a high level of isolation and few performance surprises. Again, our goal here is to simplify the lives of operators and application programmers. Higher isolation levels push a lot of performance tuning complexity onto the application programmer, and lower levels tend to be hard to reason about. As we talk more about DSQL's architecture, we'll get into how we built for snapshot isolation from the ground up.

Picking a serverless operational model, and PostgreSQL compatibility, was also based on our goal of simplifying the work of operators and builders. Tons of folks know (and love) Postgres already, and we didn't want them to have to learn something new. For many applications, moving to Aurora DSQL is as simple as changing a few connection-time lines. Other applications may need larger changes, but we'll be working to reduce and simplify that work over time.

*Footnotes*

1. <a name="foot1"></a> Maybe my favorite thing we built during that time was [container support for Lambda](https://docs.aws.amazon.com/lambda/latest/dg/images-create.html), which we ended up publishing [a paper about](https://www.usenix.org/conference/atc23/presentation/brooker) at ATC'23. 
2. <a name="foot2"></a> To learn more about how Aurora Serverless V2 works, check out our paper from VLDB'24 [Resource management in Aurora Serverless ](https://www.amazon.science/publications/resource-management-in-aurora-serverless), or [my blog post about it](https://brooker.co.za/blog/2024/07/29/aurora-serverless.html), or [Peter DeSantis's Monday Night Live](https://youtu.be/pJG6nmR7XxI?si=akCeo-MEB35WPnHI&t=919) keynote from reInvent2023.
3. <a name="foot3"></a> You can learn more about Firecracker by checking out our paper [Firecracker: Lightweight Virtualization for Serverless Applications](https://www.usenix.org/conference/nsdi20/presentation/agache) from NSDI'20, or checking out the [Firecracker source code](https://github.com/firecracker-microvm/firecracker) on Github.
4. <a name="foot4"></a> You can learn more about MemoryDB from the team's great SIGMOD'24 paper [Amazon MemoryDB: A fast and durable memory-first cloud database](https://www.amazon.science/publications/amazon-memorydb-a-fast-and-durable-memory-first-cloud-database), or my [blog post about their paper](https://brooker.co.za/blog/2024/04/25/memorydb.html).
5. <a name="foot5"></a> By *strong consistency* we mean *linearizability*, and by *strong isolation* we mean *snapshot isolation*. In a future post I'll look at these in detail, and explain why I think they're the best choices for a distributed database system.
6. <a name="foot6"></a> For a great example, check out the S3 team's SOSP'21 paper [Using lightweight formal methods to validate a key-value storage node in Amazon S3](https://www.amazon.science/publications/using-lightweight-formal-methods-to-validate-a-key-value-storage-node-in-amazon-s3).
7. <a name="foot7"></a> I've [written before](https://brooker.co.za/blog/2019/04/03/learning.html) about how going on call is the best way I know to learn how to build large-scale systems.
8. <a name="foot8"></a> If you're interested in the history of eventual consistency, especially at Amazon, check out Werner Vogels' article [Eventually Consistent](https://dl.acm.org/doi/10.1145/1435417.1435432) from the January 2009 edition of CACM.
9. <a name="foot9"></a> One day I'll tell the snapshot-isolation-related story of how I made a fool of myself in front of Betty O'Neil. More to the point: if you'd like to learn more about snapshot isolation, [Martin Kleppmann's Hermitage Repo](https://github.com/ept/hermitage/blob/master/postgres.md) has great SQL examples showing the differences between isolation levels. Aurora DSQL's snapshot level is equivalent to PostgreSQL's `REPEATABLE READ` level on these tests.