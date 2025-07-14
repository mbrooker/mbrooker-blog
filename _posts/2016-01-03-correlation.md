---
layout: post
title: "Why Must Systems Be Operated?"









related_posts:
  - "/2021/05/24/metastable.html"
  - "/2014/06/29/rasmussen.html"
  - "/2019/05/01/emergent.html"
dissimilar_posts:
  - "/2020/07/28/fish.html"
---
{{ page.title }}
================

<p class="meta">Latent Failures and the Safety Margin of Systems</p>

Mirrored RAID<sup>[1](#foot1)</sup> is a classic way of increasing storage durability. It's also a classic example of a system that's robust against independent failures, but fragile against dependent failure. [Patterson et al's 1988](http://www.eecs.berkeley.edu/Pubs/TechRpts/1987/CSD-87-391.pdf) paper, which popularized mirroring, even covered the problem:

> As mentioned above we make the same assumptions that disk manufacturers make -- that the failures are exponential and independent. (An earthquake or power surge is a situation where an array of disks might not fail independently.)

A 2-way striped RAID can be in three possible states: a state with no failures, a state with one failure, or a state with two failures. The system moves between the first and second states, and the second and third states, when a failure happens. It can return from the second state to the first by repair. In the third state, data is lost, and returning becomes an exercise in *disaster recovery* (like restoring a backup). The classic Markov model looks like this, with the failure rate λ and repair rate μ:

![](https://s3.amazonaws.com/mbrooker-blog-images/markov_2stage.png)

This model clearly displays its naive thinking: it assumes that the failure rate of 2 disks is double the failure rate of a single disk<sup>[2](#foot2)</sup>. All experienced system operators know that's not true in practice. A second disk failure seems more likely to happen soon after a first. This happens for three reasons.

 1. Failures with the same cause. These failures, like Patterson's earthquakes and power surges, affect both drives at the same time. A roof falling in on a server can move its RAID from state 1 to state 3 pretty quickly. Operator mistakes are also a common (and maybe dominant) source of these kinds of failures.
 2. Failures triggered by the first failure. When the first drive fails, it triggers a failure of the second drive. In a RAID, the second drive is going to be put under high load as the system attempts to get back to two good copies. This extra load increases the probability of the second drive failing.
 3. Latent failures. These cases start with the system believing (and the system operator believing) that the system is in stage one. When a failure occurs, it very quickly learns that the second *good* copy isn't so good<sup>[3](#foot3)</sup>.

![](https://s3.amazonaws.com/mbrooker-blog-images/markov_2stage_corr.png)

The third case, latent failures, may be the most interesting to system designers. They are a great example of the fact that systems<sup>[4](#foot4)</sup> often don't know how far they are from failure. In the simple RAID case, a storage system with a latent failure *believes* that it's in the first state, but actually is in the second state. This problem isn't, by any means, isolated to RAID.

Another good example of the same problem is a system with a load balancer and some webservers behind it. The load balancer runs health checks on the servers, and only sends load to the servers that it believes are healthy. This system, like mirrored RAID, is susceptible to having outages caused by failures with the same cause (flood, earthquake, etc), failures triggered by the first failure (overload), and latent failures. The last two are vastly more common than the first: the servers fail one-by-one over time, and the system stays up until it either dies of overload, or the last server fails.

In both the load-balancer and RAID cases, a *black box* monitoring of the system is not sufficient. Black box monitoring, including external monitors, canaries, and so on, only tell the system which side of an *externally visible failure boundary* a system is on. Many kinds of systems, including nearly every kind that includes some redundancy, can move towards this boundary through multiple failures without crossing it. Black-box monitoring misses these internal state transitions. Catching them can significantly improve the actual, real-world, durability and availability of a system.

![](https://s3.amazonaws.com/mbrooker-blog-images/failure_state_space.png)

Presented that way, it seems obvious. However, I think there's something worth paying real attention to hear: complex systems, the kind we tend to build when we want to build failure-tolerant systems, have a property that simple systems don't. Simple systems, like a teacup, are either working or they aren't. There is no reason to invest in maintenance (beyond the occasional cleaning) until a failure happens. Complex systems are different. They need to be constantly maintained to allow them to achieve their optimum safety characteristics.

This requires deep understanding of the behavior of the system, and involves complexities that are often missed in planning and management activities. If planning for, and allocating resources to, maintenance activities is done without this knowledge (or, worse, only considering external failure rates) then its bound to under-allocate resources to the real problems.

That doesn't mean that all maintenance must, or should, be done by humans. It's possible, and necessary at scale, to automate many of the tasks needed to keep systems far from the failure boundary. You've just got to realize that your automation is now part of the system, and the same conclusions apply.

**Footnotes:**

 1. <a name="foot1"></a> Also known as RAID 1. Despite nearly a decade working on computer storage, my brain refuses to store the bit of which of RAID 0 and RAID 1 is mirroring, and which is striping.
 2. <a name="foot2"></a> And a whole lot more. [Hafner and Rao](http://domino.watson.ibm.com/library/CyberDig.nsf/papers/BD559022A190D41C85257212006CEC11/$File/rj10391.pdf) is a good place to start for a more complete picture of RAID reliability.
 3. <a name="foot3"></a> In storage systems the most common cause of these kinds of issues are *latent sector errors*. [Understanding latent sector errors and how to protect against them](https://www.usenix.org/legacy/event/fast10/tech/full_papers/schroeder.pdf) is a good place to start with the theory, and [An Analysis of Latent Sector Errors in Disk Drives](http://research.cs.wisc.edu/wind/Publications/latent-sigmetrics07.pdf) present some (possibly dated) data on their frequency.
 4. <a name="foot4"></a> Here, the system includes its operators, both human and automated.