---
layout: post
title: "Why do we need distributed systems?"




related_posts:
  - "/2024/06/04/scale"
  - "/2021/01/22/cloud-scale"
  - "/2021/04/14/redundancy"
---
{{ page.title }}
================

<p class="meta">Building distributed systems is hard. It's expensive. It's complex. But we do it anyway.</p>

I grew up reading John Carmack's .plan file. His stories about the development of Doom, Quake and the rest were a formative experience for me, and a big reason I was interested in computers beyond just gaming<sup>[1](#foot1)</sup>. I was a little bit disappointed to see this tweet:

<blockquote class="twitter-tweet" data-dnt="true"><p lang="en" dir="ltr">My formative memory of Python was when the Quake Live team used it for the back end work, and we wound up having serious performance problems with a few million users. My bias is that a lot (not all!) of complex “scalable” systems can be done with a simple, single C++ server.</p>&mdash; John Carmack (@ID_AA_Carmack) <a href="https://twitter.com/ID_AA_Carmack/status/1210997702152069120?ref_src=twsrc%5Etfw">December 28, 2019</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script> 

This isn't an isolated opinion, but I don't think it's a particularly good one. To be fair, there are a lot of good reasons not to build distributed systems. Complexity is one: distributed systems are legitimately harder to build, and significantly harder to understand and operate. Efficiency is another. As McSherry et al point out in [Scalability! But at what COST?](https://www.usenix.org/system/files/conference/hotos15/hotos15-paper-mcsherry.pdf), single-system designs can have great performance and efficiency. Modern computers are huge and fast.

I was not so much disappointed in John, as in our success at building distributed systems tools that make this untrue. Distributed computing could be much easier, and needs to be much easier. We need to get to a point, with services, tooling and technology, that monolithic systems aren't a good default. To understand why, let me answer the question in the post's title.

**Distributed systems offer better availability**

The availability of a monolithic system is limited to the availability of the piece of hardware it runs on. Modern hardware is pretty great, and combined with a good datacenter and good management practices servers can be expected to fail with an annual failure rate (AFR) in the single-digit percentages. That's OK, but not great in two ways. First, if you run a lot of systems fixing these servers stacks up to an awful lot of toil. The toil is unavoidable, because if we're building a monolithic system we need to store the system state on the one server, and so creating a new server takes work (and lost state, and understanding what the lost state means to your users). The second way they get you is with time-to-recovery (TTR): unless you're super disciplined in keeping and testing backups, your rebuild process and all the rest, it's been a couple years since you last made a new one of these things. It's going to take time.

Distributed systems incur cost and complexity because they continuously avoid getting into this state. Dedicated state stores, replication, consensus and all the rest add up to avoiding any one server being a single point of failure, but also hide the long TTR that comes with fixing systems. Modern ops practices, like infrastructure as code, immutable infrastructure, containers, and serverless reduce the TTR and toil even more.

Distributed systems can also be placed nearer the users that need them. It doesn't really matter if a system is available or not if clients can't get to it, and [network partitions happen](https://dl.acm.org/doi/10.1145/2643130). Despite the restrictions of the CAP theorem and friends, this extra degree of flexibility allows distributed systems to do much better than monolithic systems.

**Distributed systems offer better durability**

Like availability, the durability of single storage devices is pretty great these days. The Backblaze folks release [some pretty great stats](https://www.backblaze.com/blog/backblaze-hard-drive-stats-q1-2019/) that show that they see about 1.6% of their drives fail in any given year. This has been the case since [at least the late 2000s](https://dl.acm.org/doi/10.5555/1267903.1267905). If you put your customer's data on a single disk, you're highly likely to still have it at the end of the year.

For this blog, "highly likely" is good enough. For almost all meaningful businesses, it simply isn't. Monolithic systems then have two choices. One is RAID. Keep the state on multiple disks, and replace them as they fail. RAID is a good thing, but only protects against a few drive failures. Not floods, fires, or explosions. Or correlated drive failure<sup>[2](#foot2)</sup>. The other option is backups. Again, a good thing with a big downside. Backups require you to choose two things: how often you run them (and therefore how much data you lose when you need them), and how long they take to restore. For the stuff on my laptop, a daily backup and multi-hour restore is plenty. For business-critical data, not so much.

Distributed storage systems continuously make multiple copies of a piece of data, allowing a great deal of flexibility around cost, time-to-recovery, durability, and other factors. They can also be built to be extremely tolerant to correlated failures, and avoid correlation outright.

**Distributed systems offer better scalability**

As with availability and durability, distributing a system over many machines gives a lot of flexibility about how to scale it. Stateless systems are relatively easy to scale, and basic techniques like HTTP load balancers are great for an awful lot of use-cases. Stateful systems are harder to scale, both because you need to decide how to spread the state around, and because you need to figure out how to send users to the right place to get the state. These two problems are at the heart of a high percentage of the distributed systems literature, and more is published on them every single day.

The good news is that many good solutions to these problems are already available. They are available as services (as in the cloud), and available as software (open source and otherwise). You don't need to figure this out yourself, and shouldn't try (unless you are really sure you want to).

**Distributed systems offer better efficiency**

Workloads are very seldom constant. Computers like to do things on the hour, or every day, or every minute. Humans, thanks to our particular foibles like sleeping and hanging out with our kids, tend to want to do things during the day, or on the holidays, or during the work week. Other humans like to do things in the evening, or late at night. This all means that the load on most systems varies, both randomly and *seasonally*. If you're running each thing on it's own box you can't take advantage of that<sup>[3](#foot3)</sup>. Big distributed systems, like the cloud, can. They also give you tools (like automatic scaling) to take advantage of it economically.

When you count all the factors that go into their cost, most computers aren't that much more expensive to keep busy than they are to keep idle. That means it makes a lot of economic sense to keep computers as busy as possible. Monolithic systems find it hard to do that.

**No magic**

Unfortunately, none of this stuff comes for free. Actually building (and, critically, operating) distributed systems that do better than monolithic systems on all these properties is difficult. The reality is seldom as attractive as the theory would predict.

As an industry, we've made a fantastic amount of progress in making great distributed systems available over the last decade. But, as Carmack's tweet shows, we've still got a lot to do. Despite all the theoretical advantages it's still reasonable for technically savvy people to see monolithic systems as simpler and better. This is a big part of why I'm excited about serverless: it's the start of a big opportunity to make all the magic of distributed systems even more widely and simply available.

If we get this right, we can change the default. More availability, more durability, more efficiency, more scale, less toil. It's going to be an interesting decade.

## Footnotes

 1. <a name="foot1"></a> Along with hacking on [gorillas.bas](https://github.com/GorillaStack/gorillas/blob/master/gorillas.bas).
 1. <a name="foot2"></a> Which is a real thing. In [Disk failures in the real world: what does an MTTF of 1,000,000 hours mean to you?](https://www.usenix.org/legacy/events/fast07/tech/schroeder/schroeder.pdf) Schroeder and Gibson report that *Time between replacement, a proxy for time between failure, is not well modeled by an exponential distribution and exhibits significant levels of correlation, including autocorrelation and long-range dependence.* This situation hasn't improved since 2007.
 1. <a name="foot3"></a> I guess you can [search for primes](https://www.mersenne.org/), or mine Ethereum, or something else. Unfortunately, these activities are seldom economically interesting.