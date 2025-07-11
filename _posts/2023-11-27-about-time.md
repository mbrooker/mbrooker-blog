---
layout: post
title: "It's About Time!"


related_posts:
  - "/2024/12/03/aurora-dsql"
  - "/2024/12/05/inside-dsql-writes"
  - "/2023/10/18/optimism"
---{{ page.title }}
================

<p class="meta">What's the time? Time to get a watch.</p>


<script>
  MathJax = {
    tex: {inlineMath: [['$', '$'], ['\\(', '\\)']]}
  };
</script>
<script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>

My friend Al Vermeulen used to say *time is for the amusement of humans*<sup>[1](#foot1)</sup>. Al's sentiment is still the common one among distributed systems builders: real wall-clock physical time is great for human-consumption (like log timestamps and UI presentation), but shouldn't be relied on by computer for things like actually affect the operation of the system. This remains a solid starting point, the right default position, but the picture has always been more subtle. Recently, the availability of ever-better time synchronization has made it even more subtle. This post will attempt to unravel some of that subtlety.

Today is a good day to talk about time, because last week [AWS announced](https://aws.amazon.com/about-aws/whats-new/2023/11/amazon-time-sync-service-microsecond-accurate-time/) ([more details here](https://aws.amazon.com/blogs/compute/its-about-time-microsecond-accurate-clocks-on-amazon-ec2-instances/)) microsecond-accurate time synchronization in EC2, improving on what was [already very good](https://aws.amazon.com/blogs/mt/manage-amazon-ec2-instance-clock-accuracy-using-amazon-time-sync-service-and-amazon-cloudwatch-part-1/). All this means is that if you have an EC2 instance<sup>[2](#foot2)</sup> you can expect its clock to by accurate to within microseconds of the *physical time*. It turns out that having microsecond-level time accuracy makes some *distributed systems stuff* much easier than it was in the past.

In hopes of understanding the controversy over using real time in systems, let's descend level-by-level into how we might entangle physical time more deeply in our system designs.

**Level 0: Observability, and the Amusement of Humans**

> ... reality, the name we give to the common experience<sup>[3](#foot3)</sup>

When we try understand how a system works, or why it's not working, the first task is to establish *causality*. Thing A caused thing B. Here in our weird little universe, we need thing A to have *happened before* thing B for A to have caused B. Time is useful for this. 

*Prosecutor*: Why, Mr Load Balancer, did you stop sending traffic to Mrs Server?  
*Mr LB*: Simply, sir, because she stopped processing my traffic!  
*Mrs Server, from the gallery*: Liar! Liar! I only stopped processing because you stopped sending!  

If we can't trust the order of our logs (or other events), finding causality is difficult. If our logs are accurately timestamped the task becomes much easier. If we can expect our logs to be timestamped so accurately that a A having a lower timestamp than B implies that A happened before B, then our ordering task becomes trivial.

We'll get back to talking about clock error later, but for now the important point is that sufficiently accurate clocks make observing systems significantly easier, because they make establishing causality significantly easier. This is a big deal. If we get nothing else out of good clocks, just the observability benefits are probably worth it.

**Level 1: A Little Smarter about Wasted Work**

> He's worth no more;  
> They say he parted well, and paid his score,  
> And so, God be with him! Here comes newer comfort.<sup>[7](#foot7)</sup>

Have you ever worked on something, then once you got it done you were told it wasn't needed anymore? Distributed systems feel like that all the time. Clients give us work, then time out, or wander off, and the work still gets done. One solution to this problem is to give each piece of work a Time To Live (TTL), where each item of work is marked with an expiry time. "If you're still working on this after twelve thirty, don't bother finishing it because I won't be waiting anymore". TTLs have traditionally been implemented using relative time (*in 10 seconds*, or in steps as with [IP](https://datatracker.ietf.org/doc/html/rfc791)) rather than absolute time (*until 09:54:10 UTC*) because comparing absolute times across machines is risky. The downside of the relative approach is that everybody needs to measure the time taken and remember to decrease the TTL, which adds complexity. High quality clocks fix the drift problem, and allow us to use absolute time TTLs.

Cache TTLs can also be based on absolute time, and the ability to accurately compare absolute time across machines allows caches to more easily implement patterns like *bounded staleness*. 

Here on Level 1 clock quality matters more than Level 0, because the operational properties of the system (and therefore its availability and cost) depend on clock correctness. So we're starting to step away from the amusement of humans to make assumptions about clocks that actually affect the client-observable running of the system.

**Level 2: Rates and Leases**

> Gambling's wrong and so is cheating, so is forging phony I.O.U.s.  
> Let's let Lady Luck decide what type of torture's justified,  
> I'm pit boss here on level two!<sup>[8](#foot8)</sup>

[Leases](https://dl.acm.org/doi/10.1145/74851.74870) are a nearly ubiquitous, go-to, mutual exclusion mechanism in distributed systems. The core idea is simple: have a client *lease* the right to exclude other clients for a period of time, and allow them to periodically renew their lease to keep excluding others. Leases, unlike more naive locks, allow the system to recover if a client fails while holding onto exclusivity: the lease isn't renewed, it times out, and other clients are allowed to play. It's this fault tolerance property that makes leases so popular.

Did you notice those words *a period of time*? Leases make a very specific assumption: that the lease provider's clock moves at about the same speed as the lease holder's clock. They don't have to have the same absolute value, but they do need to mostly agree on how long a second is. If the lease holder's clock is running fast, that's mostly OK because they'll just renew too often. If the lease provider's clock is moving fast, they might allow another client to take the lease while the first one still thinks they're holding it. That's less OK.

Robust lease implementations fix this problem with a *safety time* ($\Delta_{safety}$). Instead of allowing the lease provider to immediately give the lease to somebody else when it expires ($T \langle expiry \rangle$), they need to wait an extra amount of time (until $T \langle expiry \rangle + \Delta_{safety}$) before handing out the lease to somebody else, while the lease holder tries to ensure that they renew comfortably before $T \langle expiry \rangle$.

Robust lease implementations also need to ensure that lease holders don't keep assuming they hold the lease beyond $T \langle expiry \rangle$. This sounds trivial, but in a world of pauses from GC and IO and multithreading and whatnot it's harder than it looks. Being able to reason about the expiry time with absolute time may make this simpler.

Whatever the implementation, leases fundamentally make assumptions about clock rate. Historically, clock rates have been more reliable than clock absolute values, but still aren't entirely foolproof. Better clocks make leases more reliable.

**Level 3: Getting Real about Time**

> I am the very model of a modern Major-General,  
> I've information vegetable, animal, and mineral,  
> I know the kings of England, and I quote the fights historical  
> From Marathon to Waterloo, in order categorical;<sup>[9](#foot9)</sup>

When a client asks a database for *consistent* data, they're typically asking something very specific: make sure the answer reflects all the facts that were known *before I started this request* (or, even more specifically, at some point between this request was started and when it completed). They might also be asking for an *isolated snapshot* of the facts, but they can't ask for facts that haven't come along yet. Just the facts so far, please.

In other words, they're asking the database to pick a time $T \langle now \rangle$ such that $T \langle request start \rangle \leq T \langle now \rangle \leq T \langle request end \rangle$ and all facts that were committed before $T \langle now \rangle$ are visible. They might also be asking that facts committed after $T \langle now \rangle$ are not visible, but that's more a matter of isolation than of consistency.

In a single-system database, this is trivial. In a sharded database, the isolation part is a little tricky but the per-key consistency part is easy. Replication, when we have multiple copies of any individual fact in the database, is when things get tricky. What we want is for a client to be able to go to any replica independently, and not require any coordination between replicas when these reads occur, because this allows us to scale reads horizontally.

There are many, many, variants on solutions to this problem. High-quality absolute time gives us a rather simple one: the client picks its $T \langle request start \rangle$, then goes to a replica and says "wait until you're sure you've seen all the writes before $T \langle request start \rangle$, then do this read for me". This complicates writes somewhat (writes need to be totally ordered in an order consistent with physical time), but makes consistent reads easy.

We're starting to form a picture of a tradeoff now. Relying on physical time allows distributed systems to avoid coordination in some cases where it would have otherwise been necessary. However, if that time is wrong, the result will also likely be wrong.

**Level 4: Consistent Snapshots**

> Life is not about significant details, illuminated in a flash, fixed forever.<sup>[10](#foot10)</sup>

Just like we can use absolute time to get consistent reads, we can use absolute time to take consistent snapshots. Classic algorithms like [Chandy-Lamport](https://www.microsoft.com/en-us/research/publication/distributed-snapshots-determining-global-states-distributed-system/) have to deal with the fact that distributed systems can't easily tell everybody to do something at the same time (e.g. "write down everything you know and send it to me"). With high-quality absolute time we can. "At 12:00:00 exactly, write down everything you know and send it to me". With a perfect clock, this is trivial.

Even excellent clocks, however, aren't perfect. Even with only tens of microseconds of time error, things can change during the uncertainty interval and make the snapshot inconsistent. This is where having a bound on clock error (such as what you can get with [clock-bound](https://github.com/aws/clock-bound)) becomes useful: it provides a bounded window of time when a snapshot can be captured along with a window of changes that are relatively easy to fix with a full view of the system. The smaller the window, the less post-repair work is needed.

**Level 5: Ordering Updates**

> Effective leadership is putting first things first.<sup>[11](#foot11)</sup>

Last Writer Wins (LWW) is a very popular, and effective, way to avoid coordination in a multi-writer distributed database. It provides a simple rule for dealing with conflicts: the one with the higher timestamp overwrites the one with the lower timestamp. LWW has two big advantages. First, it doesn't require coordination, and therefore allows for low latency, high availability, and high scalability. The second is that it's really super simple. CRDTs (and other generalizations of monotonicity) have the same first advantage, but not typically the second<sup>[14](#foot14)</sup>. They are seldom *super simple*.

LWW also has two disadvantages. First, the semantics of "clobber this write with that one" aren't great, making it difficult to make internally consistent changes to complex databases (ACID's *C*) or data structures. Second, the definition of *last* may not always match what the clients expect. In fact, they may do write *A* then write *B* and see *A* take precedence over *B* just because it landed on a server with a slightly faster clock. High quality clocks help us solve this second problem. For example, if the clock error is less than the client round-trip time, then the client can never observe this kind of anomaly. They can still happen, but the client can never prove they happened.

Using physical clocks to order writes is, for good reasons, controversial. In fact, most experienced distributed system builders would consider it a sin. But high quality clocks allow us to avoid one of the major downsides of LWW, and make its attractive properties even more attractive in the right applications. However, it's important to note that many of the commonly-cited downsides of using physical clocks to order writes don't have much to do with clocks at all, and instead have to do with coordination avoidance (especially accepting an unbounded amount of change on both sides of a partition). Great clocks don't fix those problems, because they aren't fundamentally caused by time. Kyle Kingsbury's [work on Riak data loss](https://aphyr.com/posts/285-call-me-maybe-riak) from a decade ago is a perfect illustration of the problem (and a problem that dates back to Riak's roots in [Dynamo](https://www.allthingsdistributed.com/files/amazon-dynamo-sosp2007.pdf)).

If you're thinking about ordering writes or doing consistent snapshots using physical time, it's worth checking out hybrid approaches (like [Hybrid Logical Clocks](http://muratbuffalo.blogspot.com/2014/07/hybrid-logical-clocks.html) or [physiological time order](https://people.csail.mit.edu/devadas/pubs/tardis.pdf)) that offer properties that degrade more gracefully in the face of time error.

**When Things Go Wrong**

> They're funny things, Accidents. You never have them till you're having them.<sup>[6](#foot6)</sup>

So far, I've been talking about time as though programs can know what the current time is. This is obviously impossible.

First, even assuming access to a perfect clock, they can only know what the current time *was*. The moment we execute the next instruction, that time is outdated. Variable CPU clocks, cache misses, OS schedulers, runtime schedulers, GC pauses, bus contention, interrupts, and all sorts of other things conspire against us to make it difficult to know how long ago *was* was. The best we can generally do on general-purpose computers is to use any measure of time as a sort of lower bound of the current time<sup>[12](#foot12)</sup>.

But clocks aren't perfect. Every oscillator has some amount of jitter and some amount of drift (or, rather, a complex spectrum of error). We can correct much, but not all, of this error. Thus our current time might also be a time from the future, even by the time we get to use it. In EC2 this error is very low, but it still exists.

To avoid getting too confused, and riffing off Lamport<sup>[4](#foot4)</sup>, we can establish some notation. Let's say $T \langle A \rangle$ is the time that event $A$ happens. But $T \langle A \rangle$ is a secret to us: instead we can only know that it lies somewhere between $T \langle A \rangle_{low}$ and $T \langle A \rangle_{high}$ (the open source project [clockbound](https://github.com/aws/clock-bound) provides just this API). Alternatively, we can say that we can know $T \langle A \rangle + \epsilon$ where $\epsilon$ is chosen from some asymmetrical error distribution. Improving clock quality is both about driving $E[\epsilon]$ to zero, and about putting tight bounds on the range of $\epsilon$. 

If our bounds are accurate enough we can say that $T \langle A \rangle_{high} < T \langle B \rangle_{low}$ implies that $A$ *happens before* $B$. We can write this as $A \rightarrow B$. The full statement is then $T \langle A \rangle_{high} < T \langle B \rangle_{low} \Rightarrow A \rightarrow B$ <sup>[5](#foot5)</sup>, and $A \rightarrow B \Rightarrow T \langle A \rangle_{high} < T \langle B \rangle_{low}$ (here, we're using $\Rightarrow$ to mean *implies*).

There's something qualitative and important that happens when the error on $T \langle A \rangle$ (aka $\epsilon$) is smaller than the amount of time it would take event $A$ to cause anything to happen (e.g. smaller than one network latency): that means that we can be sure that events that are timestamped before $T \langle A \rangle$ *cannot have been caused by A*. This is a rather magical property.

I'm suspicious of any distributed system design that uses time without talking about the range of errors on the time estimate (i.e. any design that assumes $\epsilon == 0$ or even $\epsilon \geq 0$).

**Paradiso**

> But already my desire and my will  
> were being turned like a wheel, all at one speed<sup>[13](#foot13)</sup>

If you're still with me, brave and intrepid to have made it this far, I'd like to offer a tool for thinking about how to use physical time in your distributed systems: start by thinking about what can go wrong.

*What if the clock I use for my log timestamps are wrong?* Operators and customers will likely be confused. This is unlikely to have any first-order effects on the operations of your system, but could make it more difficult to operate and increase downtime in that way.

*What if the clock I use to do reads is wrong?* Perhaps your design, like [DynamoDB's transaction design](https://www.usenix.org/system/files/atc23-idziorek.pdf) would retain serializability but lose linearizability and see a lower transaction rate. Keeping some properties in the face of clock error is where approaches like [Hybrid Logical Clocks](http://muratbuffalo.blogspot.com/2014/07/hybrid-logical-clocks.html) come in super handy.

And so on. If you can come up with a good explanation for what will happen when time is wrong, and you're OK with that happening with some probability, then you should feel OK using physical time. If arbitrarily bad things happen when time is wrong, you're probably going to have a bad time. If you don't consider it all, then you may consider yourself lost.

**Footnotes**

1. <a name="foot1"></a> I'm sure he still does, but likely not as often now he's retired.
2. <a name="foot2"></a> Of the right type, in the right region (for now), with all the configuration set up right (for now).
3. <a name="foot3"></a> From Tom Stoppard's *Rosencrantz and Guildenstern are Dead*. Endlessly quotable.
4. <a name="foot4"></a> In [Time, Clocks and the Ordering of Events in a Distributed System](https://www.microsoft.com/en-us/research/publication/time-clocks-ordering-events-distributed-system/). You should read this paper, today. In fact, stop here and read it now. Yes, I know you read it before and know the key points, but there's a lot of smart stuff going on here that you may not remember.
5. <a name="foot5"></a> Compare this to Lamport's *clock condition* on page 2 of Time, Clocks.
6. <a name="foot6"></a> A. A. Milne, of course.
7. <a name="foot7"></a> Shakespeare, from Macbeth. This line is followed with the greatest stage direction of all "Enter Macduff, with Macbeth's head."
8. <a name="foot8"></a> From the delightful Futurama episode "Hell is Other Robots", credited to Ken Keeler and Eric Kaplan.
9. <a name="foot9"></a> *For my military knowledge, though I'm plucky and adventury, Has only been brought down to the beginning of the century.*
10. <a name="foot10"></a> From Sontag's *On Photography*. "One can't possess reality, one can possess images" is nearly as fitting.
11. <a name="foot11"></a> From Stephen Covey, I think from the book of the same name. You thought you'd make it this far without Self Help, but alas.
12. <a name="foot12"></a> Dedicated hardware can do much better. Back in graduate school I shared my office with Stephan Sandenbergh, who was building extremely high-quality clocks aimed at building coherent radar systems, many orders of magnitude better than what I'm talking about here. No doubt the state-of-the-art has continued to advance since then.
13. <a name="foot13"></a> How did I get this far into a level-by-level descent without Dante? But, of course, preferring the spheres of heaven to the circles of hell.
14. <a name="foot14"></a> Some folks pointed out to me that LWW *is* technically a CRDT, which I guess, is fair but not particularly useful.