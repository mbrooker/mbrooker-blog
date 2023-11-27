---
layout: post
title: "It's About Time!"
---

{{ page.title }}
================

<p class="meta">Time to get a watch.</p>

<script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
<script>
  MathJax = {
    tex: {inlineMath: [['$', '$'], ['\\(', '\\)']]}
  };
</script>
<script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>

My friend Al Vermeulen used to<sup>[1](#foot1)</sup> say *time is for the amusement of humans*. Al's sentiment is still the common one among distributed systems builders: real wall-clock physical time is great for human-consumed purposes like log timestamps and UI presentation, but shouldn't be relied on by computer for things like figuring out the order of changes to a piece of data. This remains a great starting point, the right default position, but the picture has always been more subtle. Recently, the availability of ever-better time synchronization has made it even more subtle. This post will attempt to unravel some of that subtlety.

Today is a good day to talk about time, because last week [AWS announced](https://aws.amazon.com/about-aws/whats-new/2023/11/amazon-time-sync-service-microsecond-accurate-time/) microsecond-accurate time synchronization in EC2, building on what was [already very good](https://aws.amazon.com/blogs/mt/manage-amazon-ec2-instance-clock-accuracy-using-amazon-time-sync-service-and-amazon-cloudwatch-part-1/) time synchronization in EC2. All this means is that if you have an EC2 instance<sup>[2](#foot2)</sup> you can expect its clock to by correct to within microseconds of the *physical time*. It turns out that having microsecond-level time accuracy makes some *distributed systems stuff* much easier than it was in the past.

The idea of using physical time in distributed systems is rather controversial. Let's descend into that controversy, level by level.

**Level 0: Observability, and the Amusement of Humans**

*... reality, the name we give to the common experience<sup>[3](#foot3)</sup>*

When we try understand how a system works, or why it's not working, the first task is to establish *causality*. Thing A caused thing B. Here in our weird little universe, we need thing A to have *happened before* thing B for A to have caused B. Time, of course, is useful for this. 

*Prosecutor*: Why, Mr Load Balancer, did you stop sending traffic to Mrs Server?
*Mr LB*: Simply, sir, because she stopped processing my traffic!
*Mrs Server, from the gallery*: Liar! Liar! I only stopped processing because you stopped sending!

If we can't trust the order of our logs (or other events), finding the truth behind the ordering of events is difficult. If we can expect our logs to be accurately timestamped the task becomes much easier. If we can expect our logs to be timestamped so accurately that a A having a lower timestamp than B implies that A happened before B, then our ordering task becomes trivial.

Riffing off Lamport<sup>[4](#foot4)</sup>, we can establish some notation. If you want, you can ignore the notation. I'll use it on and off to be more precise about the things I'm saying, but if you don't need the precision then ignore it. Let's say $T \langle A \rangle$ is the time that event $A$ happens, then if our clock is accurate enough we can say that $T \langle A \rangle < T \langle B \rangle$ implies that $B$ *happens before* $A$. We can write this as $A \rightarrow B$. The full statement is then $T \langle A \rangle < T \langle B \rangle \Rightarrow A \rightarrow B$ <sup>[5](#foot5)</sup>, and $A \rightarrow B \Rightarrow T \langle A \rangle < T \langle B \rangle$ (here, we're using $\Rightangle$ to mean *implies*).

Perhaps most critically, if the error on our clocks is faster than messages can travel between components, then the timestamps on our logs become entirely reliable. In the distributed setting that's typically one network latency (or half an RTT), but not always. 

**Level 1: A Little Smarter about Wasted Work**

*He's worth no more;
They say he parted well, and paid his score,
And so, God be with him! Here comes newer comfort.<sup>[7](#foot7)</sup>*

Have you ever worked on something, then once you got it done you were told it wasn't needed anymore? Our systems feel like that all the time. Clients give us work, then time out, or wander off, and the work still needs to be done. One solution to this problem is to give each piece of work a Time To Live (TTL), where each item of work is marked with an expiry time. "If you're still working on this after twelve thirty, don't bother finishing it because I won't be waiting anymore". TTLs have traditionally been implemented using relative time (*in 10 seconds*) rather than absolute time (*until 09:54:10 UTC*) because comparing absolute times across machines is risky. The downside of the relative approach is that everybody needs to measure the time taken and remember to decrease the TTL, which adds complexity. High quality clocks fix the drift problem, and allow us to use absolute time TTLs.

Cache TTLs can also be based on absolute time, and the ability to accurately compare absolute time across machines allows caches to more easily implement patterns like *bounded staleness*. 

Here on Level 1 clock quality matters more than Level 0, because the operational properties of the system (and therefore its availability and cost) depend on clock correctness. So we're starting to step away from the amusement of humans to make assumptions about clocks that actually affect the client-observable running of the system.

**Level 2: Rates and Leases**

*Gambling's wrong and so is cheating, so is forging phony I.O.U.s. Let's let Lady Luck decide what type of torture's justified, I'm pit boss here on level two!<sup>[8](#foot8)</sup>*

[Leases](https://dl.acm.org/doi/10.1145/74851.74870) are the nearly ubiquitous, go-to, time-based mutual exclusion mechanism in distributed systems. The core idea is simple: have a client *lease* the right to exclude other clients for a period of time, and allow them to periodically renew their lease to keep excluding others. Leases, unlike more naive locks, allow the system to recover if a client fails while holding onto exclusivity: the lease isn't renewed, it times out, and other clients are allowed to play. It's this fault tolerance property that makes leases so popular.

Did you notice those words *a period of time*? Leases make a very specific assumption: that the lease provider's clock moves at about the same speed as the lease holder's clock. They don't have to have the same absolute value, but they do need to mostly agree on how long a second is. If the lease holder's clock is running fast, that's mostly OK because they'll just renew too often. If the lease provider's clock is moving fast, they might allow another client to take the lease while the first one still thinks they're holding it. That's less OK.

Robust lease implementations fix this problem with a *safety time* ($\Delta_{safety}$). Instead of allowing the lease provider to immediately when the lease expires ($\T \langle expiry \rangle$), they need to wait an extra amount of time (until $\T \langle expiry \rangle + \Delta_{safety}$)


**When Things Go Wrong**

*They're funny things, Accidents. You never have them till you're having them.<sup>[6](#foot6)</sup>*





**Footnotes**

1. <a name="foot1"></a> I'm sure he still does, but likely not as often now he's retired.
2. <a name="foot2"></a> Of the right type, in the right region (for now), with all the configuration set up right (for now).
3. <a name="foot3"></a> From Tom Stoppard's *Rosencrantz and Guildenstern are Dead*. Endlessly quotable.
4. <a name="foot4"></a> In [Time, Clocks and the Ordering of Events in a Distributed System](https://www.microsoft.com/en-us/research/publication/time-clocks-ordering-events-distributed-system/). You should read this paper, today. In fact, stop here and read it now. Yes, I know you read it before and know the key points, but there's a lot of smart stuff going on here that you may not remember.
5. <a name="foot5"></a> Compare this to Lamport's *clock condition* on page 2 of Time, Clocks.
6. <a name="foot6"></a> A. A. Milne, of course.
7. <a name="foot7"></a> Shakespeare, from Macbeth. This line is followed with the greatest stage direction of all "Enter Macduff, with Macbeth's head."
8. <a name="foot8"></a> From the delightful Futurama episode "Hell is Other Robots", credited to Ken Keeler and Eric Kaplan.
