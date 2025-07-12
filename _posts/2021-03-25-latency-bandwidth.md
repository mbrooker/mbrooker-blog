---
layout: post
title: "What You Can Learn From Old Hard Drive Adverts"








related_posts:
  - "/2012/02/11/latency-lags-bandwidth.html"
  - "/2014/07/04/iostat-pct.html"
  - "/2013/07/14/io-performance.html"
dissimilar_posts:
  - "/2015/05/24/sodium-carbonate.html"
---
{{ page.title }}
================

<p class="meta">The single most important trend in systems.</p>

Adverts for old computer hardware, especially hard drives, are a fun staple of computer forums and the nerdier side of the internet<sup>[1](#foot1)</sup>. For example, a couple days ago, Glenn Lockwood tweeted out this old ad:

<blockquote class="twitter-tweet" data-dnt="true"><p lang="en" dir="ltr">At least this isn’t an ad for a HAMR drive. $10k in today’s dollars. <a href="https://t.co/2h2g3Gnguw">pic.twitter.com/2h2g3Gnguw</a></p>&mdash; Glenn K. Lockwood (@glennklockwood) <a href="https://twitter.com/glennklockwood/status/1374770622748708864?ref_src=twsrc%5Etfw">March 24, 2021</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script> 

Apparently from the early '80s, these drives offered seek times of 70ms, access speeds of about 900kB/s, and capacities up to 10MB. Laughable, right? But these same ads hide a really important trend that's informed system design more than any other. To understand what's going on, let's compare this creaky old 10MB drive to a modern competitor. Most consumers don't buy magnetic drives anymore, so we'll throw in an SSD for good measure.

| | XCOMP 10MB&nbsp;&nbsp;&nbsp; | Modern HDD&nbsp;&nbsp; | Change | Modern SSD&nbsp; | Change |
| ----------- | ----------- |
| Capacity | 10MB | 18TiB | 1.8 million times&nbsp;&nbsp; | 2 TiB | 200,000x |
| Latency  | 70ms | 5ms | 14x | 50μs | 1400x |
| Throughput | 900kB/s | 220MB/s | 250x | 3000MB/s | 3300x |
| IOPS/GiB (QD1) | 1400 | 0.01 | 0.00007x | 10 | 0.007x |

Or there abouts<sup>[2](#foot2)</sup>. Starting with the magnetic disk, we've made HUGE gains in storage size, big gains in throughput, modest gains in latency, and a seen a massive drop in random IO per unit of storage. What may be surprising to you is that SSDs, despite being much faster in every department, have seen pretty much the same overall trend. 

This is not, by any stretch, a new observation. 15 years ago the great Jim Gray said "Disk is Tape". David Patterson (you know, Turing award winner, RISC co-inventor, etc) wrote a great paper back in 2004 titled [Latency Lags Bandwidth](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.115.7415&rep=rep1&type=pdf) that made the same observation. He wrote:

> I am struck by a consistent theme across many technologies: bandwidth improves much more quickly than latency.

and 

> In the time that bandwidth doubles, latency improves by no more than a factor of 1.2 to 1.4.

That may not sound like a huge amount, but remember that we're talking about exponential growth here, and exponential growth is a wicked thing that breaks our minds. Multiplying Patterson's trend out, by the time bandwidth improves 1000x, latency improves only 6-30x. That's about what we're seeing on the table above: a 250x improvement in bandwidth, and a 14x improvement in latency. Latency lags bandwidth. Bandwidth lags capacity.

One way to look at this is how long it would take to read the whole drive with a serial stream of 4kB random reads. The 1980s drive would take about 3 minutes. The SSD would take around 8 hours. The modern hard drive would take about 10 months. It's not a surprise to anybody that small random IOs are slow, but maybe not how slow. It's a problem that's getting exponentially worse.

**So what?**

Every stateful system we build brings with it some tradeoff between latency, bandwidth, and storage costs. For example, RAID5-style 4+1 erasure coding allows a system to survive the loss of one disk. 2-replication can do the same thing, with 1.6x the storage cost and 2/5ths the IOPS cost. Log-structured databases, filesytems and file formats all make bets about storage cost, bandwidth cost, and random access cost. The changing ratio between the hardware capabilities require that systems are re-designed over time to meet the capabilities of new hardware: yesterday's software and approaches just aren't efficient on today's systems.

The other important thing is parallelism. I pulled a bit of a slight-of-hand up there by using QD1. That's a queue depth of one. Send an IO, wait for it to complete, send the next one. Real storage devices can do better when you give them multiple IOs at a time. Hard drives do better with scheduling trickery, handling "nearby" IOs first. Operating systems have done IO scheduling for this purpose forever, and for the last couple decades drives have been smart enough to do it themselves. SSDs, on the other hand, [have real internal parallelism](https://brooker.co.za/blog/2014/07/04/iostat-pct.html) because they aren't constrained by the bounds of physical heads. Offering lots of IOs to an SSD at once can improve performance by as much as 50x. Back in the 80's, IO parallelism didn't matter. It's a huge deal now.

There are two conclusions here for the working systems designer. First, pay attention to hardware trends. Stay curious, and update your internal constants from time to time. Exponential growth may mean that your mental model of hardware performance is completely wrong, even if it's only a couple years out of date. Second, system designs rot. The real-world tradeoffs change, for this reasons as well as many others. The data structures and storage strategies in your favorite textbook likely haven't stood the test of time. The POSIX IO API definitely hasn't.

**Footnotes**

 1. <a name="foot1"></a> See, for example, [this Reddit thread](https://www.reddit.com/r/interestingasfuck/comments/ay225x/this_xcomp_hard_disk_advertisement_from_1981_how/), [unraid forums](https://forums.unraid.net/topic/7377-10-mb-xcomp-hard-drive-339800/), [this site](http://mag.metamythic.com/old-hard-disk-drive-adverts/) and so on. They're everywhere.
 2. <a name="foot2"></a> I extracted these numbers from my head, but I think they're more-or-less representative of modern mainstream NVMe and enterprise magnetic drives.