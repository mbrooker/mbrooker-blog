---
layout: post
title: Latency lags bandwidth









related_posts:
  - "/2021/03/25/latency-bandwidth.html"
  - "/2025/12/15/database-for-ssd.html"
  - "/2021/08/05/utilization.html"
dissimilar_posts:
  - "/2015/05/24/sodium-carbonate.html"
---
{{ page.title }}
================

<p class="meta">On the growing gap between capacity, bandwidth and latency.</p>

One of the key engineering challenges for most high-performance storage systems is minimizing the number of disk seeks that are required to access stored data. Sometimes of this requires smart techniques like reordering, but the majority of the win comes from caching - storing as much data as you can in memory, or at least away from slow disk platters. Database systems do the same, as do modern operating systems - they constantly cache reads and buffer writes in memory in an attempt to hide disk latency. The classic [rule of thumb](ftp://ftp.research.microsoft.com/pub/tr/tr-99-100.pdf) is that it's worth caching pages in memory which are likely to be accessed again within five minutes. This five minute rule has proven to be amazingly constant over [several decades](ftp://ftp.research.microsoft.com/pub/tr/tr-97-33.pdf) and many orders of magnitude of computer speed, capacity and development.

Another way of stating the same thing is that page sizes have stayed approximately constant, disk access latencies have stayed approximately constant, and the ratio between the cost of a byte of RAM and a byte of disk have stayed approximately constant. We spend an ever increasing amount of our cheap RAM on hiding the crappy latency of our cheap storage.

Caching has been very successful. So successful, in fact, that it has effectively hidden from all but the biggest applications the ever-growing split between capacity, bandwidth and latency in our storage systems.

From [Latency Lags Bandwidth](http://dl.acm.org/citation.cfm?id=1022596), a 2003 paper by David Patterson:

![](https://s3.amazonaws.com/mbrooker-blog-images/mbrooker_patterson_llb.png)

For every decade that Patterson measured for the paper, disks got on average 50 times bigger, 12 times faster at doing bulk transfers, and only 2.4 times faster at seeking. That paper came out in 2003, but there isn't much indication that the picture has changed substantially since then. The other way of looking at that is even more disturbing: the time to read a complete disk with random IO is increasing by a factor of 22 every decade or 36% a year.

The problem with the success of caching at hiding latency is that the cliff is getting steeper: the ratio between the speed of a cache hit and a cache miss is changing. Many applications nominally use disk, but may not be able to afford cache misses at all.

There are two solutions to this: simply give up on disks for online data (RAM is the new disk), or expend bandwidth to reduce apparent latency. Both of these options are already widely seen in production systems. [MongoDB](http://www.mongodb.org), [Redis](http://redis.io/) and [VoltDB](http://www.voltdb.com/) are good examples of the first and [BDB-JE](http://www.oracle.com/technetwork/database/berkeleydb/learnmore/bdb-je-architecture-whitepaper-366830.pdf) is a good example of the second. Neither of these approaches is ideal, however. Storing data in RAM requires very careful attention to durability, while bandwidth intensive methods have to deal with the very real and widening gap between capacity and bandwidth and the tradeoff between write and read speeds.

This sets the stage for the explosive rise of SSDs, hybrid disks and hybrid storage systems, which is very exciting. Unfortunately, they only change the constants and not the fundamental arithmetic. Latency is going to keep lagging, and our data is going to keep getting further away.