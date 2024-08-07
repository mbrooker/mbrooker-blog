---
layout: post
title: The power of two random choices
---

{{ page.title }}
================

<p class="meta">Using less information to make better decisions.</p>

In many large-scale web services, multiple layers of stateless and stateful services are seperated by load balancers. Load balancing can be done with dedicated hardware, with dedicated software load balancers, using DNS trickery or through a load-balancing mechanism in the client. In large systems, the resources and constraints at each layer can vary widely. Some layers are stateless, and easily scale horizontally to many machines. Other layers may be more constrained, either due to the need to access state or contention for some other shared resource.

Centralized load balancing solutions can distribute load over a fleet of machines very well. They track the amount of load they are sending to each machine (usually based on a simple measurement like connection count). Because they are centralized, load balancers typically don't need to worry about load sent from other sources. They have complete control over the distribution of load.

Despite this advantage, dedicated load balancers are often undesirable. They add cost, latency, complexity, and may introduce a single point of failure. Handing the task of load balancing to each upstream client is also possible, but introduces the challenge of fairly balancing the load from multiple places. In large systems with large numbers of clients and fairly homogeneous calls, a purely random system like DNS round robin can work very well. In smaller systems, systems where each downstream service can only handle a small number or concurrent requests, and systems where requests are heterogeneous it's often desirable to load balance better than random.

Perfect distributed load balancing could be done, at least in the happy case, by distributing information about system load across all the clients. The overhead of constantly sharing the exact load information between different sources can be high, so it's tempting to have each source work off a cached copy. This data can periodically be refreshed from downstream, or from other clients.

It turns out that's not a great idea.

In [The Power of Two Random Choices: A Survey of Techniques and Results](http://www.eecs.harvard.edu/~michaelm/postscripts/handbook2001.pdf), Mitzenmacher et. al. survey some research very relevant to this problem. The entire survey is good reading, but one of the most interesting results is about the effects of delayed data (like the cached load results mentioned above) on load balancing. The results are fairly logical in retrospect, but probably don't match most engineers' first expectations.

Using stale data for load balancing leads to a herd behavior, where requests will herd toward a previously quiet host for much longer than it takes to make that host very busy indeed. The next refresh of the cached load data will put the server high up the load list, and it will become quiet again. Then busy again as the next herd sees that it's quiet. Busy. Quiet. Busy. Quiet. And so on.

One possible solution would be to give up on load balancing entirely, and just pick a host at random. Depending on the load factor, that can be a good approach. With many typical loads, though, picking a random host degrades latency and reduces throughput by wasting resources on servers which end up unlucky and quiet.

The approach taken by the studies surveyed by Mitzenmacher is to try two hosts, and pick the one with the least load. This can be done directly (by querying the hosts) but also works surprisingly well on cached load data.

To illustrate how well this works, I ran a simulation of a very simplistic system. Every second one request arrives at a system with 10 servers. Every 8 seconds the oldest request (if any) is removed from the queue on the servers. Load balancing is done with a cached copy of the server queue lengths, updated every N seconds. I considered four approaches: chose a random server, chose the best server, best of two random choices and best of three random choices.

![](https://s3.amazonaws.com/mbrooker-blog-images/mbrooker_best_of_two_result.png)

As you can expect, the *pick the best* approach worked best when perfect undelayed information was available. In the same case, the random pick approach worked poorly, leading to the worst queue times of any of the approaches. As the age of the data increases, the *pick the best* approach becomes worse and worse because of herding and soon overtakes the random approach as the worst one.

*Best of 3* starts in second place, and puts in a good performance. It gains first place, only to cede it to *best of 2* as the delay increases. Clearly, the behavior for *best of k* will approach the behavior of *best* as k approaches the number of servers. *Best of 2* remains the strong leader all the way to the end of this simulation. Given these parameters it would start losing to the random approach around a refresh interval of 70. It is the clear leader for intervals over the range from 10 to 70, which is an impressive performance for such a simple approach.

*Best of 2* is good because it combines the best of both worlds: it uses real information about load to pick a host (unlike random), but rejects herd behavior much more strongly than the other two approaches.

Take a look at [The Power of Two Random Choices](http://www.eecs.harvard.edu/~michaelm/postscripts/handbook2001.pdf) for a much stronger mathematical argument, and some more surprising places this algorithm works really well.
