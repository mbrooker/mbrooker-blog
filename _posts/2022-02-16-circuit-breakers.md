---
layout: post
title: "Will circuit breakers solve my problems?"



related_posts:
  - "/2022/02/28/retries"
  - "/2018/02/25/availability-liveness"
  - "/2021/05/24/metastable"
---{{ page.title }}
================

<p class="meta">Maybe, but you need to know what problem you're trying to solve first.</p>

A couple of weeks ago, I started a tiny storm on Twitter by posting this image, and claiming that retries (mostly) make things worse in real-world distributed systems.

![Retry loop, showing how retries make overload conditions worse](https://mbrooker-blog-images.s3.amazonaws.com/retry_loop.png)

The bottom line is that retries are often triggered by overload conditions, permanent or transient, and tend to make those conditions worse by increasing traffic. Many people replied saying that I'm ignoring the obvious effective solution to this problem: circuit breakers.

**What is a circuit breaker?**

Way down in your basement, or in a closet, or wherever your local government decrees it to be, there's a box full of electrical circuit breakers. These circuit breakers have one job<sup>[1](#foot1)</sup>: turn off during overload before something else melts, burns, or flashes. They're pretty great from a "staying alive" perspective. Reasoning by analogy, folks<sup>[2](#foot2)</sup> developed the concept of circuit breakers for distributed systems. They goal of circuit breakers is usually defined something like this (from the [Azure docs](https://docs.microsoft.com/en-us/azure/architecture/patterns/circuit-breaker)):

> A circuit breaker acts as a proxy for operations that might fail. The proxy should monitor the number of recent failures that have occurred, and use this information to decide whether to allow the operation to proceed, or simply return an exception immediately.

or this (from [Martin Fowler](https://martinfowler.com/bliki/CircuitBreaker.html)):

> The basic idea behind the circuit breaker is very simple. You wrap a protected function call in a circuit breaker object, which monitors for failures. Once the failures reach a certain threshold, the circuit breaker trips, and all further calls to the circuit breaker return with an error, without the protected call being made at all.

So far, so sensible. But why? What is the goal?

Martin, again:

> It's common for software systems to make remote calls to software running in different processes, probably on different machines across a network. One of the big differences between in-memory calls and remote calls is that remote calls can fail, or hang without a response until some timeout limit is reached. What's worse if you have many callers on a unresponsive supplier, then you can run out of critical resources leading to cascading failures across multiple systems.

and they do this in a way that's better than just short timeouts. Microsoft again:

> Note that setting a shorter timeout might help to resolve this problem, but the timeout shouldn't be so short that the operation fails most of the time, even if the request to the service would eventually succeed.

When people talk about circuit breakers, they're typically considering two potential benefits. One, as Martin points out, is that failing early can prevent you from wasting work or resources on something that's doomed. Doing that may allow work that requires the same resources, but isn't dependent on the same downstream dependency, to continue to succeed. The second benefit is allowing a kind of progressive degradation in service. Maybe you can present your website without some optional feature, if the service backing that optional feature doesn't work<sup>[3](#foot3)</sup>. Again, sensible.

**The Problem with Circuit Breakers**

The problem with circuit breakers is that they don't take into account the fundamental properties of real distributed systems. Let's consider the architecture of a toy distributed NoSQL database:

    ┌────────────────────────────────────────┐
    │          Load Balancer/Router          │
    └────────────────────────────────────────┘
                         │                    
          ┌──────────────┼──────────────┐     
          │              │              │     
          ▼              ▼              ▼     
    ┌──────────┐   ┌──────────┐   ┌──────────┐
    │          │   │          │   │          │
    │ Storage  │   │ Storage  │   │ Storage  │
    │  (A-H)   │   │  (I-R)   │   │  (S-Z)   │
    │          │   │          │   │          │
    └──────────┘   └──────────┘   └──────────┘

There's a router layer, and some shards of storage. When a request comes in for a key starting with B, it goes the the A-H shard. Requests for keys starting with T go to the S-Z shard, and so on. Real systems tend to be more complex and more sophisticated than this, but the top level architecture of scale-out databases almost always looks a little bit like this.

How might this system fail? Clearly, the router layer could fail taking the whole thing down. But that seems less likely because its simple, probably stateless, easily horizontally scalable, etc. More likely is that one of the storage shards gets overloaded. Say *AaronCon* is in town, and everybody is trying to sign up. The A-H shard will get a lot of load, while the others might get little. Calls for A-H may start failing, while calls for other keys keep working.

That presents the circuit breaker with a problem. Is this database *down*? Have failures reached a threshold?

If you say *yes, it's down*, then you've made service worse for Jane and Tracy. If you say *no, it's not down*, then you may as well not have the breaker at all. Breakers that don't trip aren't very useful<sup>[4](#foot4)</sup>.

The same issue is true of cell-based architectures, where a circuit breaker tripping on the failure of one cell may make the whole system look like its down, defeating the purpose of cells entirely. Cell-based architectures are similar to the sharded architectures, just sharded for availability and blast-radius instead of scale.

**Can We Fix Them?**

Maybe. The problem here is that for circuit breakers to do the right thing in cell-based and sharded systems they need to predict something very specific: is *this call for these parameters* likely to work? Inferring that from other calls with other parameters may not be possible. Clients simply don't know enough (and, mostly, shouldn't know enough) about the inner workings of the systems they are calling to make that decision. Typically, three solutions to this problem are proposed:

 - Tight coupling. If the client does know how internal data sharding works in the service, it can see which shards of the service are down, and make a good decision. The tradeoff here, obviously, is that this layering violation makes change hard. Nobody wants to be unable to change their service without changing every client. On the other hand, this approach may work well if you can guess well enough, like having circuit breakers per upstream customer.
 - Server information. On overload, the service can say things like "I'm overloaded for requests that start with A", and the client can flip the corresponding mini circuit breaker. I've seen real-world systems that work this way, but the complexity cost may be high.
 - Statistical inference magic/AI magic/ML magic. Could work. Hard to get right. Have fun writing the postmortem when the arriving traffic looks nothing like the training set.

**Bottom Line**

Modern distributed systems are designed to partially fail, continuing to provide service to some clients even if they can't please everybody. Circuit breakers are designed to turn partial failures into complete failures. One mechanism will likely defeat the other. Make sure you think that through before deploying circuit breakers.

**Footnotes**

 1. <a name="foot1"></a> Ok, ok, modern circuit breakers have multiple jobs including detecting ground and arc faults, and industrial circuit breakers can do fancy things like detect high-impedance faults.
 2. <a name="foot2"></a> Commonly attributed to Michael Nygard in [Release It!](https://www.amazon.com/Release-Production-Ready-Software-Pragmatic-Programmers/dp/0978739213), but it's not clear that's the actual origin, and I don't have my copy of the book to hand to check if he credits somebody else. It's a good book, worth reading.
 3. <a name="foot3"></a> Fans of the paper [Harvest, Yield, and Scalable Tolerant Systems](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.24.3690&rep=rep1&type=pdf) might call this a reduction in Harvest. That's a good paper, as long as you skip the confusing section about CAP.
 4. <a name="foot4"></a> And [can even be actively harmful](https://www.nbcbayarea.com/news/local/federal-pacific-circuit-breakers-investigation-finds-decades-of-danger/1930189/) by catching fire themselves. Useless complexity is bad.