---
layout: post
title: "Balls Into Bins In Distributed Systems"








related_posts:
  - "/2024/03/25/needles.html"
  - "/2012/01/17/two-random.html"
  - "/2023/02/07/hot-keys.html"
dissimilar_posts:
  - "/2015/05/24/sodium-carbonate.html"
---
{{ page.title }}
================

<p class="meta">Throwing things can be fun.</p>

If you've come across the [Balls Into Bins](https://en.wikipedia.org/wiki/Balls_into_bins) problem, you probably heard about in context of hash tables. When you hash things into a hash table (especially with [separate chaining](https://en.wikipedia.org/wiki/Hash_table#Separate_chaining)) it's really useful to be able to ask "If I throw 𝑀 balls into 𝑁 bins, what is the distribution of balls in bins?" You can see how this is fundamental to hash tables: the amortized complexity argument for hash tables depends on their being some *load factor* (i.e. 𝑀/𝑁) for which most bins contain a small number of items. Once this stops being true, lookup and insertion time on hash tables starts to get ugly. So from that perspective it's already clearly an important problem.

### Load Balancing and Work Allocation
Hash tables aren't the only place that the Balls Into Bins problem is interesting. It comes up often in distributed systems, too. For one example, think about a load balancer (in this case a distributor of independent requests) sending load to some number of backends. Requests (𝑀) are balls, and the backends are bins (𝑁) and typically there are multiple requests going to each backend (𝑀 > 𝑁). If we know how to solve for the number of balls in each bin, we can understand the limits of random load balancing, or whether we need a stateful load balancing algorithm like *least connections*. This is an important question to ask, because sharing consistent state limits scalability, and sharing eventually-consistent state can even [make load balancing decisions worse](//brooker.co.za/blog/2012/01/17/two-random.html). Load balancing is much easier if it can be done statelessly.

A related problem is push-based work allocation. Here, there is some co-ordinator handing out work items to a fleet of workers, and trying to have those workers do approximately equal amounts of work. One way that systems end up with this pattern is if they are using [shuffle sharding](https://aws.amazon.com/blogs/architecture/shuffle-sharding-massive-and-magical-fault-isolation/) or consistent hashing to distribute work items (or records). These hashing-based methods can be great for scaling, and so are widely used across all kinds of large-scale systems. Just as with load balancing, its interesting to be able to understand how well requests are distributed.

Traditionally, papers about this problem have been most concerned about the expectation of the maximum number of balls in a bin ("how bad can it get?"), but other statistics like the expectation of the mean and expectation of the median can be interesting when planning and designing for load. It's also interesting to understand the variance of the maximum, and the size of the right tail on the distribution of the maximum. If the maximum can get really high, but will do so infrequently, then load testing can be difficult.

### Closed-Form Analysis
Gaston Gonnet's [Expected Length of the Longest Probe Sequence in Hash Code Searching](https://cs.uwaterloo.ca/research/tr/1978/CS-78-46.pdf), was one of the first papers to tackle analyzing the problem, in context of separate-chaining hash tables<sup>[2](#foot2)</sup>. Michael Mitzenmacher's PhD thesis ([the power of two choices in randomized load balancing](https://www.eecs.harvard.edu/~michaelm/postscripts/mythesis.pdf)) simplifies Gonnet's analysis and finds, for the 𝑀=𝑁 case, the maximum number of balls is<sup>[1](#foot1)</sup> 𝝝(log 𝑁/log log 𝑁). That's not a curve you'll come across often, so this is what it looks like:

![](https://s3.amazonaws.com/mbrooker-blog-images/logn_loglogn.png)

In other words, it grows, but grows pretty slowly. Most real-world cases are probably going to have 𝑀>𝑁, and many will have 𝑀≫𝑁. To understand those cases, we can turn to one of my favorite papers in this area, [Balls Into Bins: A Simple and Tight Analysis](https://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.43.4186) by Raab and Steger. They provide a great overview of the problem, a useful survey of the literature, and a table of bounds on the maximum number of balls in any bin, depending on the relationship between 𝑀 and 𝑁. The proofs are interesting, and somewhat enlightening, but not necessary to understand to find the paper useful.

While this kind of analysis is very useful, when I've needed to solve these problems in the past, I haven't tended to use the results directly. Instead, I've used them to sanity check the output of simulations. This is where *engineering practice* diverges a bit from *computer science theory* (although it does it in a pretty theoretically rigorous way).

### Simulating the Problem
There are a couple of limitations on the usefulness of the closed-form analysis of this problem. One problem is that it's fairly difficult to understand clearly (at least for me), and quite complex to communicate. The bigger problem, though, is that it's quite inflexible: extending the analysis to include cases like balls of different sizes (as requests are in the real world) and balls coming out of bins (requests completing) is difficult, and difficult to code review unless you are lucky enough to work with a team that's very mathematically sophisticated. The good news, though, is that this problem is exceptionally easy to simulate.

When I think about doing these kinds of simulations, I don't generally think about using specialized simulation tools or frameworks (although you could certainly do that). Instead, I generally think about just writing a few tens of lines of Python or R which directly try the thing that I want an answer for many times, and then output data in a form that's easy to plot. Computer simulation is a broad and subtle topic, but this kind of thing (throw balls into bins, count, repeat) avoids many of the subtleties because you can avoid floating point (its just counting) and because you can avoid being too concerned about the exact values.

Knowing the closed-form analysis makes it easy to sanity-check the simulation. According to Gonnet, the 𝑀=𝑁 case should approach log𝑁/loglog𝑁 (1+𝑜(1)), and we can plot that curve (choosing a value of 𝑜(1) to minimize the difference) alongside the simulation results to see if the simulation matches the theory. The results look pretty good.

![](https://s3.amazonaws.com/mbrooker-blog-images/bb_sim_vs_model.png)

Gonnet's paper also contains a table of example values, which compare very well to our simulated and modelled numbers. That all increases our confidence that the simulation is telling us sensible things.

You can also extend this basic counting to be closer to real-world load-balancing. Follow a Poisson process (a fancy way of saying "use exponentially distributed random numbers to decide how long to wait") to add random balls into bins over time, and follow your completion time distribution (probably exponential too) to pull them out of the bins. Every so often, sample the size of the biggest bin. Next, output those samples for analysis. If you have real arrival-time data, and completion-time distributions, you can use those to avoid making *any* statistical assumptions. Which is nice.

When you've got the basic simulation, it's easy to add in things like different-sized requests, or bursts of traffic, or the effects of scaling up and down the backends.

### Some Basic Results
For small M and N, the constant factors are a big problem. With 𝑀=𝑁=100, I get an expected maximum of around 4.2. In other words, we can expect the busiest backend to be over 4x busier than the average. That means that you either need to significantly over-scale all your backends, or put up with the problems that come with hotspotting. This problem also gets worse (although very very slowly, going back to the closed-form) with scale.

In the closer-to-reality case with 𝑀=1000 and 𝑁=100, the gap shrinks. The expected maximum comes to as 18.8, compared to a mean (aka 𝑀/𝑁) of 10. That still means that the hottest backend gets 80% more traffic, but the gap is closing. By 𝑀=10000 and 𝑁=100, the gap has closed to 25%, which starts to be close to the realm of acceptable. Up to 𝑀=100,000 and the gap is closed to 8%. In most distributed systems contexts, 8% is probably within the variation in performance due to other factors.

Still, the conclusion of all of this is that random load balancing (and random shuffle-sharding, and random consistent hashing) distributed load rather poorly when 𝑀 is small. Load-sensitive load-balancing, either stateful or stateless with an algorithm [like best-of-two](//brooker.co.za/blog/2012/01/17/two-random.html) is still very much interesting and relevant. The world would be simpler and more convenient if that wasn't the case, but it is.

### Footnotes:

 1. <a name="foot1"></a> That's a Big Theta, if you're not familiar with it [wikipedia has a good explanation](https://en.wikipedia.org/wiki/Big_O_notation#Family_of_Bachmann%E2%80%93Landau_notations) of what it means. If you don't feel like reading that, and replace it with a big O in your head, that's close enough in this case.
 1. <a name="foot2"></a> That paper also contains some great analysis on the numerical properties of different hash-table probing strategies versus seperate chaining. If you like algorithm analysis, the conclusion section is particularly interesting.