---
layout: post
title: "Metastability and Distributed Systems"
---

{{ page.title }}
================

<p class="meta">What if computer science had different parents?</p>

There's no more time-honored way to get things working again, from toasters to global-scale distributed systems, than turning them off and on again. The reasons that works so well are varied, but one reason is especially important for the developers and operators of distributed systems: metastability.

I'll let the authors of [Metastable Failures in Distributed Systems](https://sigops.org/s/conferences/hotos/2021/papers/hotos21-s11-bronson.pdf) define what that means:

> Metastable failures occur in open systems with an uncontrolled source of load where a trigger causes the system to enter a bad state that persists even when the trigger is removed.

What they're identifying here is a kind of *stable down* state, where the system is stable but not doing useful work, even though it's only being offered a load that it successfully handled sometime in the past. 

One classic version of this problem involves queues. A system is ticking along nicely, and something happens. Could be a short failure, a spike of load, a deployment, or one of many other things. This causes queues to back up in the system, causing an increase in latency. That increased latency causes clients to time out before the system responds to them. Clients continue to send work, and the system continues to complete that work. Throughput is great. None of the work is useful, though, because clients aren't waiting for the results, so goodput is zero. The system is mostly stable in this state, and without an external kick, could continue going along that way indefinitely. Up, but down. Working, but broken.

In [Metastable Failures in Distributed Systems](https://sigops.org/s/conferences/hotos/2021/papers/hotos21-s11-bronson.pdf), Bronson et al correctly observe that these types of failure modes are well-known<sup>[1](#foot1)</sup> to the builders of large-scale systems:

> By reviewing experiences from a decade of operating hyperscale distributed systems, we identify a class of failures that can disrupt them, even when there are no hardware failures, configuration errors, or software bugs. These metastable failures have caused widespread outages at large internet companies, lasting from minutes to hours. Paradoxically, the root cause of these failures is often features that improve the efficiency or reliability of the system.

The paper identifies a list of other triggers for these types of metastable failures, including retries, caching, slow error handling paths and emergent properties of load-balancing algorithms. That's a good list, but just scratching the surface of all of the possible causes of these 'down but up' states in distributed systems. 

**Is there a cure?**

The disease is a serious one, but perhaps with the right techniques we can build systems that don't have these metastable states. Bronson et al propose approaching that in several ways:

> We consider the root cause of a metastable failure to be the sustaining feedback loop, rather than the trigger. There are many triggers that can lead to the same failure state, so addressing the sustaining effect is much more likely to prevent future outages.

This isn't a controversial point, but is an important one: focusing on just fixing the triggering causes of issues causes us to fail to prevent similar issues with slightly different causes in future.

The rest of their proposed solutions are more debatable. Changing policy during overload introduces modal behavior that can be hard to reason about (and [modes are bad](https://aws.amazon.com/builders-library/avoiding-fallback-in-distributed-systems/)). Prioritization and [fairness](https://aws.amazon.com/builders-library/fairness-in-multi-tenant-systems/) are good if you can get them, but many systems can't, either because their workloads are complex interdependent graphs without clear priority order, or because the priority order is unpalatable to the business. Fast error paths and outlier hygiene are good, in an *eat your broccoli* kind of way.

The other two they cover that really resonate with me are *organizational incentives* and *autoscaling*. Autoscaling, again, is a *good if you can get it* kind of thing, but most applications can get it by building on top of modern cloud systems. Maybe even get it for free by building on serverless<sup>[2](#foot2)</sup>. On organizational incentives:

> Optimizations that apply only to the common case exacerbate feedback loops because they lead to the system being operated at a larger multiple of the threshold between stable and vulnerable states.

Yes, precisely. This is a very important dynamic to understand, and design an organization around defeating<sup>[4](#foot4)</sup>. One great example of this behavior is retries. If you're only looking at your day-to-day error rate metric, you can be lead to believe that adding more retries makes systems better because it makes the error rate go down. However, the same change can make systems more vulnerable, by converting small outages into sudden (and metastable) periods of internal retry storms. Your weekly loop where you look at your metrics and think about how to improve things may be making things worse.

**Where do we go?**

Knowing this problem exists, and having some tactics to fix certain versions of it, is useful. Even more useful would be to design systems that are fundamentally stable.

> Can you predict the next one of these metastable failures, rather than explain the last one?

The paper lays out a couple of strategies here. The most useful one is a *characteristic metric* that gives insight into the state of the feedback loop that's holding the system down. This is the start of a line of thinking that treats large-scale distributed systems as control systems, and allows us to start applying the mathematical techniques of control theory and [dynamical systems theory](https://en.wikipedia.org/wiki/Dynamical_system).

I believe that many of the difficulties we have in this area come from where computing grew up. Algorithms, data structures, discrete math, finite state machines, and the other core parts of the CS curriculum are only one possible intellectual and theoretical foundation for computing. It's interesting to think about what would be different in the way we teach CS, and the way we design and build systems, if we had instead chosen the mathematics of control systems and dynamical systems as the foundation. Some things would likely be harder. Others, like avoiding building metastable distributed systems, would likely be significantly easier.

In lieu of a time-travel-based rearrangement of the fundamentals of computing, I'm excited to see more attention being paid to this problem, and to possible solutions. We've made a lot of progress in this space over the last few decades, but there's a lot more research and work to be done.

Overall, [Metastable Failures in Distributed Systems](https://sigops.org/s/conferences/hotos/2021/papers/hotos21-s11-bronson.pdf) is an important part of a conversation that doesn't get nearly the attention it deserves in the academic or industrial literature. If I have any criticism, it's that the paper overstates its case for novelty. These kinds of issues are well known in the world of control systems, in [health care](https://qualitysafety.bmj.com/content/14/2/130), in operations research, and other fields. The organizational insights echo those of folks like Jens Rasmussen<sup>[3](#foot3)</sup>. But it's a HotOS paper, and this sure is a hot topic, so I won't hold the lack of a rigorous investigation of the background against the authors.

If you build, operate, or research large-scale distributed systems, you should read this paper. There's also a good summary on [Aleksey Charapko's blog](http://charap.co/metastable-failures-in-distributed-systems/).

**Footnotes**

 1. <a name="foot1"></a> For example, I wrote about part of this problem in [Some risks of coordinating only sometimes](https://brooker.co.za/blog/2019/05/01/emergent.html), and [talked about it at HPTS'19](http://www.hpts.ws/papers/2019/brooker.pdf), although framed the issue as a bistability rather than metastability. Part of the thinking in that talk came from my own experience, and discussions of the topic in books like [designing distributed control systems](https://www.amazon.com/Designing-Distributed-Control-Systems-Language/dp/1118694155/). It's a topic we've spent a lot of energy on at AWS over the last decade, although typically using different words.
 2. <a name="foot2"></a> Of course I'm heavily biased, but the big advantage of serverless is that most applications are small relative to the larger serverless systems they run on, and so have a lot of headroom to deal with sudden changes in efficiency. In practice, I think that building on higher-level abstractions is going to be the best way for *most* people to avoid problems like those described in the paper, most of the time.
 3. <a name="foot3"></a> Specifically the discussion of the "error margin" in [Risk Management in a Dynamic Society](https://lewebpedagogique.com/audevillemain/files/2014/12/maint-Rasmus-1997.pdf), and how economic and labor forces push systems closer to the boundary of acceptable performance. 
 4. <a name="foot4"></a> An organization and an economy. As we saw with supply-side shortages of things like masks early in the Covid pandemic, real-world systems are optimized for little excess capacity too, and optimized for the happy case.

