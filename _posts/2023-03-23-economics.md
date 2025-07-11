---
layout: post
title: "Surprising Scalability of Multitenancy"


related_posts:
  - "/2024/06/04/scale"
  - "/2022/11/29/snapstart"
  - "/2020/01/02/why-distributed"
---{{ page.title }}
================

<p class="meta"></p>


<script>
  MathJax = {
    tex: {inlineMath: [['$', '$'], ['\\(', '\\)']]}
  };
</script>
<script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>

When most folks talk about the economics of cloud systems, their focus is on automatically scaling for long-term seasonality: changes on the order of days (*fewer people buy things at night*), weeks (*fewer people visit the resort on weekdays*), seasons, and holidays. Scaling for this kind of seasonality is useful and important, but there's another factor that can be even more important and is often overlooked: short-term peak-to-average. Roughly speaking, the cost of a system scales with its (short-term<sup>[1](#foot1)</sup>) peak traffic, but for most applications the value the system generates scales with the (long-term) average traffic.

The gap between "paying for peak" and "earning on average" is critical to understand how the economics of large-scale cloud systems differ from traditional single-tenant systems.

Why is it important? 

It's important because multi-tenancy (i.e. running a lot of different workloads on the same system) very effectively reduces the peak-to-average ratio that the overall system sees. This is highly beneficial for two reasons. The first-order reason is that it improves the economics of the underlying system, by bringing costs (proportional to *peak*) closer to value (proportional to *average*). The second-order benefit, and the one that is most directly beneficial to cloud customers, is that it allows individual workloads to have higher peaks without breaking the economics of the system.

Most people would call that *scalability*.

**Example 1: S3**

Earlier this month, Andy Warfield from the S3 team did a [really fun talk at OSDI'23](https://www.youtube.com/watch?v=sc3J4McebHE&t=1282s) about his experiences working on S3. There's a lot of gold in his talk, but there's one point he made that I think is super important, and worth diving deeper into: heat management and multi-tenancy. Here's the start of the relevant bit on heat<sup>[2](#foot2)</sup> management:

<iframe width="560" height="315" src="https://www.youtube-nocookie.com/embed/sc3J4McebHE?start=1282" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

Andy makes a lot of interesting point here, but the key one has got to do with the difference between the *per object* heat distribution, the *per aggregate* heat distribution, and the *system-wide* heat distribution.

> Scale allows us to deliver performance for customers that would otherwise be prohibitive to build.

Here, Andy is talking about that second-order benefit. By spreading customers workloads over large numbers of storage devices, S3 is able to support individual workloads with peak-to-average ratios that would be prohibitively expensive in any other architecture. Importantly, this happens without increasing the peak-to-average of the overall system, and so comes without additional cost to customers or the operator.

**Example 2: Lambda**

Much like S3, Lambda's scalability is directly linked to multi-tenancy. In our [Firecracker paper](https://www.usenix.org/system/files/nsdi20-paper-agache.pdf), we explain why:

> Each slot can exist in one of three states: initializing, busy,
and idle... Slots use different amounts
of resources in each state. When they are idle they consume
memory, keeping the function state available. When they are
initializing and busy, they use memory but also resources like
CPU time, caches, network and memory bandwidth and any
other resources in the system. Memory makes up roughly
40% of the capital cost of typical modern server designs, so
idle slots should cost 40% of the cost of busy slots. Achieving
this requires that resources (like CPU) are both soft-allocated
and oversubscribed, so can be sold to other slots while one is
idle.

This last point is the key one: we can't dynamically scale how much memory or CPU an EC2 metal instance has, but we can plan to sell memory or CPU that is currently not being used by one customer to another customer. This is, fundamentally, a statistical bet. There is always some non-zero probability that demand will exceed supply on any given instance, and so we manage it very carefully.

> We set some compliance goal X (e.g., 99.99%), so that functions
are able to get all the resources they need with no contention
X% of the time. Efficiency is then directly proportional to the
ratio between the Xth percentile of resource use, and mean
resource use. Intuitively, the mean represents revenue, and the
Xth percentile represents cost. Multi-tenancy is a powerful
tool for reducing this ratio, which naturally drops approximately with $\sqrt{N}$ when running $N$ uncorrelated workloads on
a worker.

And, of course, for this to work we need to be sure that the different workloads don't all spike up at the same time, which requires that workloads are not correlated. Again, this is not only an economic effect, but also a scalability one. By working this way, Lambda can absorb both long and short spikes in load for any single workload very economically, allowing it to offer scalability that is difficult to match with traditional infrastructure.

> Keeping these workloads uncorrelated requires that
they are unrelated: multiple workloads from the same application, and to an extent from the same customer or industry,
behave as a single workload for these purposes.

This last point is very important, because it illustrates the difference between our real-world setting and idealized models.

**Poisson Processes and the Real World**

What I said above is true for Poisson processes, but not nearly as powerful as what we see in the real world, which is interesting because Poisson processes are widely used to model the economics and scalability of systems. To understand why, we need to think a little bit about the sum of two Poisson processes. Say we have two customers of the system, one being a Poisson process with a mean arrival rate of 1 tps ($\lambda_1 = 1$), and one with a mean arrival rate of 4 tps ($\lambda_2 = 4$). The sum of the two is a Poisson process with an arrival rate of 5 tps ($\lambda_t = \lambda_1 + \lambda_2 = 1 + 4$)<sup>[3](#foot3)</sup>. This keeps going: no matter how many Poisson customers you add, you keep having a Poisson arrival process.

That's still good, because as you scale the system to handle the higher-rate process, the $c$ in the *M/M/c* system goes up, and [utilization increases](https://brooker.co.za/blog/2020/08/06/erlang.html).

But many real-world processes, like the ones that Andy talks about, are not Poisson. They behave much worse, with much higher spikes and much more autocorrelation than Poisson processes would exhibit. This isn't some kind of numerical anomaly, but rather a simple observation about the world: traffic changes with time and use. I don't use my computer's hard drive once every $\frac{1}{\lambda}$ seconds, exponentially distributed. I use it a lot, then not really at all. I don't use my car that way, or my toaster. And humans don't use the cloud that way either. One use leads to another, and one user and another being on are not independent.

But, if you can mix a lot of different workloads, with different needs and patterns, you can hide the patterns of each. That's the fundamental economic effect of multi-tenancy, and the thing that a lot of people overlook when thinking about the economics of the cloud)<sup>[4](#foot4)</sup>.

**Footnotes**

1. <a name="foot1"></a> Where the definition of *short-term* depends on how quickly the system can scale up and down without incurring costs. Running on Lambda, short-term may be seconds. If you're building datacenters, it may be months or years.
2. <a name="foot2"></a> To be clear, Andy's talking about logical workload heat here (a *hot* workload is one doing a lot of IO at a given moment), not physical *temperature*.
3. <a name="foot3"></a> [This StackOverflow question covers it well](https://math.stackexchange.com/questions/4446957/prove-sum-of-two-independent-poisson-processes-is-another-poisson-process), and there's a more formal treatment in [Stochastic Models in Operations Research](https://www.amazon.com/Stochastic-Models-Operations-Research-Vol/dp/0486432599).
4. <a name="foot4"></a> One way to think about this is that by summing over multiple non-stationary, autocorrelated, seasonal, and otherwise poorly behaved workloads we're restoring the *poisson-ness* of the overall workload. That's not too far from the truth, because the Poisson process is the result of summing a large number of independent arrival processes. It's not quite true, but directionally OK in my mind. On the other hand, some sensible people don't support doing math based on vibes, and might object.