---
layout: post
title: "What Does a Database for SSDs Look Like?"

related_posts:
  - "/2024/04/25/memorydb.html"
  - "/2025/11/02/thinking-dsql.html"
  - "/2022/10/04/commitment.html"
dissimilar_posts:
  - "/2020/07/28/fish.html"
---
{{ page.title }}
================

<script>
  MathJax = {
    tex: {inlineMath: [['\\(', '\\)']]}
  };
</script>
<script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>

<p class="meta">Maybe not what you think.</p>

Over on X, [Ben Dicken asked](https://x.com/BenjDicken/status/2000197741478384029):

> What does a relational database designed *specifically* for local SSDs look like?
> Postgres, MySQL, SQLite and many others were invented in the 90s and 00s, the era of spinning disks. A local NVMe SSD has ~1000x improvement in both throughput and latency.
> Design decisions like write-ahead logs, large page sizes, and buffering table writes in bulk were built around disks where I/O was SLOW, and where sequential I/O was order(s)-of-magnitude faster than random.
> If we had to throw these databases away and begin from scratch in 2025, what would change and what would remain?

How might we tackle this question quantitatively for the modern transaction-orientated database?

**Approach One: The Five Minute Rule**

Perhaps my single favorite systems paper, [The 5 Minute Rule...](https://dsf.berkeley.edu/cs286/papers/fiveminute-tr1986.pdf) by Jim Gray and Franco Putzolu gives us a very simple way to answer one of the most important questions in systems: how big should caches be? The five minute rule is that, back in 1986, if you expected to read a page again within five minutes you should keep in in RAM. If not, you should keep it on disk<sup>[1](#foot1)</sup>. Let's update the numbers for 2025, assuming that pages are around 32kB<sup>[2](#foot2)</sup> (this becomes important later).

 The EC2 `i8g.48xlarge` [delivers about 1.8 million](https://docs.aws.amazon.com/ec2/latest/instancetypes/so.html) read iops of this size, at a price of around $0.004576 per second, or \\(1 \times 10^{-9}\\) dollars per transfer (assuming we're allocating about 40% of the instance price to storage). It also has enough RAM for about 50 million pages of this size, costing around \\(3 \times 10^{-11}\\) to storage a page for one second.

 So, on this instance type, we should size our RAM cache to store pages for about 30 seconds. Not too different from Gray and Putzolu's result 40 years ago!

 That's answer number one: the database should have a cache sized so that the hot set contains pages expected to be accessed in the next 30 seconds, for optimal cost. For optimal latency, however, the cache may want to be considerably bigger.

 **Approach Two: The Throughput/IOPS Breakeven Point**

The next question is what size accesses we want to send to our storage devices to take best advantage of their performance. In the days of spinning media, the answer to this was surprisingly big: a 100MB/s disk could generally do around 100 seeks a second, so if your transfers were less than around 1MB you were walking away from throughput. Give or take a factor of 2. What does it look like for modern SSDs?

SSDs are much faster on both throughput and iops. They're less sensitive than spinning drives to workload patterns, but read/write ratios and the fullness of the drives still matter. Absent benchmarking on the actual hardware with the real workload, my [rule of thumb](https://brooker.co.za/blog/2022/12/15/thumb.html) is that SSDs are throughput limited for transfers bigger than 32kB, and iops limited for transfers smaller than 32kB.

So that's answer number two: we want our transfers to disk not to be much smaller than 32kB on average, or we're walking away from throughput.

**Approach Three: Durability and Replication**

Building reads on local SSDs is great: tons of throughput, tons of iops. Writes on local SSDs, on the other hand, have the distinct problem of only being durable on the local box, which is unacceptable for most workloads. Modern hardware is very reliable, but thinking through the business risks of losing data on failover isn't very fun at all, so let's assume that our modern database is going to replicate off-box, making at least one more synchronous copy. Ideally in a different availability zone.

That `i8g.48xlarge` we were using for our comparison earlier has 100Gb/s (or around 12GB/s) of network bandwidth. That puts a cap on how much write throughput we can have for a single-leader database. Cross-AZ latency in EC2 varies from a couple hundred microseconds to a millisecond or two, which puts a minimum on our commit latency. 

That gives us answer number three: we want to incur cross-AZ latency only at commit time, and not during writes.

Which is where we run into one of my favorite topics: isolation. The *I* in *ACID*. A modern database design will avoid read-time coordination using multiversioning, but to offer isolation stronger than `READ COMMITTED` will need to coordinate either on each write or at commit time. It can do that like, say, Aurora Postgres does, having a single leader at a time running in a single AZ. This means great latency for clients in that zone, and higher latency for clients in different AZs. Given that most applications are hosted in multiple AZs, this can add up for latency-sensitive applications which makes a lot of round trips to the database. The alternative approach is the one Aurora DSQL takes, doing the cross-AZ round trip only at `COMMIT` time, saving round-trips.

Here's me talking about the shape of that trade-off at re:Invent this year:

<iframe width="560" height="315" src="https://www.youtube-nocookie.com/embed/SNnUpYvBfow?si=hRTXS5kyHtyXW7zB&amp;start=3260" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

There's no clear answer here, because there are real trade-offs between the two approaches. But do make sure to ask your database vendor whether those impressive latency benchmarks are running where you application actually runs. In the spirit of the original question, though, the incredible bandwidth and latency availability in modern datacenter networks is as transformative as SSDs in database designs. Or should be.

While we're incurring the latency cost of synchronous replication, we may as well get [strongly consistent](https://brooker.co.za/blog/2025/11/18/consistency.html) scale-out reads for free. In DSQL, we do this using high-quality hardware clocks that [you can use too](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/set-time.html). Another nice win from modern hardware. There are other approaches too.

That's answer number four for me: The modern database uses high-quality clocks and knowledge of actual application architectures to optimize for real-world performance (like latency in multiple availability zones or regions) without compromising on strong consistency.

**Approach Four: What about that WAL?**

> Design decisions like write-ahead logs, large page sizes, and buffering table writes in bulk were built around disks where I/O was SLOW, and where sequential I/O was order(s)-of-magnitude faster than random.

WALs, and related low-level logging details, are critical for database systems that care deeply about durability on a single system. But the modern database isn't like that: it doesn't depend on commit-to-disk on a single system for its durability story. Commit-to-disk on a single system is both unnecessary (because we can replicate across storage on multiple systems) and inadequate (because we don't want to lose writes even if a single system fails).

That's answer number five: the modern database commits transactions to a distributed log, which provides multi-machine multi-AZ durability, and might provide other services like atomicity. Recovery is a replay from the distributed log, on any one of a number of peer replicas. 

**What About Data Structures?**

B-Trees versus LSM-trees vs B-Tree variants versus LSM variants versus other data structures are trade-offs that have a lot to do with access patterns and workload patterns. Picking a winner would be a whole series of blog posts, so I'm going to chicken out and say *its complicated*.

**Conclusion**

> If we had to throw these databases away and begin from scratch in 2025, what would change and what would remain?

I'd keep the relational model, atomicity, isolation (but would probably pick `SNAPSHOT` as a default), strong consistency, SQL, interactive transactions, and the other core design decisions of relational databases. But I'd move durability, read and write scale, and high availability into being distributed rather than single system concerns. I think that helps with performance and cost, while making these properties easier to achieve. I'd mostly toss out local durability and recovery, and all the huge history of optimizations and data structures around that<sup>[3](#foot3)</sup>, in favor of getting better properties in the distributed setting. I'd pay more attention to internal strong isolation (in the security sense) between clients and workloads. I'd size caches for a [working set](https://denninginstitute.com/pjd/PUBS/WSModel_1968.pdf) of between 30 seconds and 5 minutes of accesses. I'd optimize for read transfers around that 32kB sweet spot from local SSD, and the around 8kB sweet spot for networks.

Probably more stuff too, but this is long enough as-is.

Other topics worth covering include avoiding copies on IO, co-design with virtualization (e.g. [see our Aurora Serverless paper](https://www.amazon.science/publications/resource-management-in-aurora-serverless)), trade-offs of batching, how the relative performance of different isolation levels changes, what promises to give clients, encryption and authorization of data at rest and in motion, dealing with very hot single items, new workloads like vector, verifiable replication journals, handing off changes to analytics systems, access control, multi-tenancy, forking and merging, and even locales.

*Footnotes*

1. <a name="foot1"></a> The reasoning is slightly smarter, thinking about the *marginal* page and *marginal* cost of memory, but this simplification works for our purposes here.
2. <a name="foot2"></a> Yes, I know that pages are typically 4kB or 2MB, but bear with me here.
3. <a name="foot3"></a> Sorry [ARIES](https://web.stanford.edu/class/cs345d-01/rl/aries.pdf).