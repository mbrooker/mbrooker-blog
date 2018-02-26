---
layout: post
title: "Availability and availability"
---

{{ page.title }}
================

<p class="meta">Translating math into engineering.</p>

It's well known that the term Availability in the CAP theorem (as formally defined by [Gilbert and Lynch](https://dl.acm.org/citation.cfm?id=564601)) means something different from the term *availability* that's commonly used by the designers, builders and operators of distributed systems. Gilbert and Lynch define availability for the CAP theorem as:

> every request received by a non-failing node in the system must result in a response.

That's cool, and useful for the mathematical analysis that's needed to prove the CAP theorem. Most builders and users of distributed systems, on the other hand, define *availability* as the percentage of requests that their clients see as successful, or something close to that. The terms, like 'clients' and 'successful' and 'see', are pretty fuzzy. Not much good for analysis, but more useful for capturing what people care about.

This isn't a new observation. You can find a whole lot of writing about it online. Some of that writing is pretty great. What I don't see addressed as often is how to translate one into the other, using the CAP (or PACELC or whatever) reasoning about Availability to help us think about *availability*. In reality, are Available systems more available than Consistent systems?

This post isn't a complete answer to that question, but does include some of the things worth thinking about in that space.

### Harvest and Yield

Before I dive into this topic, it's worth talking about Harvest and Yield, from a paper by [Fox and Brewer](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.33.411&rep=rep1&type=pdf). The paper itself has some flaws (as I've [blogged about before](http://brooker.co.za/blog/2014/10/12/harvest-yield.html)), but the underlying concept is very useful. The core is about graceful degradation, and how it's useful for systems to return partial or stale answers when they aren't able to answer authoritatively.

The paper makes its case well, but whether its conclusions are practically useful depend on what promises you make to your clients. If the direct clients of your service are people, then you're likely to be able to get away with graceful degradation. If your clients are computers, they're likely expecting a complete, authoritative, response. That's mostly because when people program computers they don't think through all of the edge cases introduced by other computers leaving out some information. This isn't a hard-and-fast rule. Sometimes computers can tolerate partial responses, and sometimes humans can't.

In other words, Harvest and Yield is a partial answer, useful when you can use it.

### Taking Availability to Clients

How does CAP's big-A availability translate to clients? The most useful simple answer is that if you've decided you want to Consistent system, then clients on the minority side of a network partition get nothing, and clients on the majority side don't have any problems. Once the partition heals (or moves around), those minority clients might be able to make progress again. If you've chosen an Available system, everybody is able to make progress all the time.

The reality is fuzzier than this simple answer.

The first reason is that, in real systems, there isn't typically a binary choice between A and C. Part of the reason for that is that the definition of Consistency in CAP is also different from the common sense one clients probably care about, and it's possible to give some clients meaningfully consistent experiences without losing A. The details of that are for another day<sup>[1](#foot1)</sup>.

Lets assume that you've chosen a common-sense definition of consistency that requires real strong Consistency properties. Then you run into a second problem: many meaningful workloads from clients don't only read or write a single atomic piece of data. Some workloads are nice a clean and translate into a single database transaction on a single database. Some are messier, and require pulling data from many different shards of different databases, or from other services. Over time, many of the nice clean workloads turn into ugly messy workloads.

There's also a third problem. The vast majority of the patterns that are used for building real-world large-scale systems do different amounts of work in the happy case and the failure case. Master election, for example, is a very commonly-used pattern. Paxos implementations typically optimize away one round most of the time. Raft is explicitly modal.

Clients on the majority side of a partition are theoretically able to continue, but only if they are on the same majority side of all the data and services they depend on. They're also likely to be some cost to continuing, requiring the system to detect the problem, and shift from happy mode into failure handling mode. Depending on the design of the system, this can take a significant period of time.

### Failure Detection and Remediation

The first step to surviving a network partition (or any other failure), is figuring out what happened. Sometimes, what happened is a nice clean host failure that everybody can agree on. The real world is uglier: host failures may be partial, network failures may show up as latency or congestion rather than failure, and systems could be cycling between up and down.

Whether you've chosen an Available (A) system or a Consistent (C) system, your system needs to be able to identify failures. How quickly you can do that, and how the system behaves in the mean time, is fundamental to *availability*.

There are many ways to detect failures: timeouts, direct health pings, latency thresholds, error rate thresholds, TCP connection state (a special case of latency threshold), and even hardware magic like physical-layer connection state. None of those are instantaneous, and most will eat some requests while deciding to fail over. If that happens often, *availability* will be decreased.

Failure remediation is where the distributed systems protocol literature shines. [Paxos Made Simple](https://lamport.azurewebsites.net/pubs/paxos-simple.pdf) or [Viewstamped Replication](http://www.pmg.csail.mit.edu/papers/vr.pdf) or [Chord](https://pdos.csail.mit.edu/papers/chord:sigcomm01/chord_sigcomm.pdf) or one of hundreds of other papers provide answers to that problem to fit all kinds of different situations. I'm not going to go into that topic, but even if you nail the implementation of failure handling, you've still not solved your client's problem.

When a failure is fixed, who needs to learn about the new location of the data and how quickly they can learn about it? While clients are trying to talk to the old, broken primary or trying to talk to the other side of a network partition, they aren't going to be making progress. Again, whether you've chosen A or C, *availability* suffers. Available systems do have a bit of an easier time of this than Consistent systems. They might be able to fail over more aggressively. They also don't have to solve the age-old "oops, I just flipped into the side of the partition away from my clients" problem.

### Where do failures happen?

Network partitions do happen. From the perspective of the client of a Consistent system, the system is down if they are partitioned away from the majority of the nodes in the system. From the perspective of the client of an Available system, the system is down if they are partitioned away from all the nodes in the system.

Whether that's a useful distinction or not depends on where the clients are relative to the larger system. If the system is in a single datacenter in a single location, and the clients are spread around the global Internet, it's not much more likely that they'll be able to contact less than half of the nodes than none of the nodes. On the other hand, if the clients are in the same datacenter as the system, then the probabilities are going to be different. More generally, if nodes are spread around the network about the same way as clients, A and C are going to be practically different.

### Conclusion

In practice, CAP Available doesn't mean 'highly available to clients'. In practice, picking an Available design over a Consistent one means that it's going to be more available to some clients in a fairly limited set of circumstances. That may very well be worth it, but it's in no way a panacea for availability.

### Footnotes

 1. <a name="foot1"></a> Although, do check out [Bailis and Ghodsi](https://dl.acm.org/citation.cfm?doid=2460276.2462076) for a very readable introduction to the landscape of consistency.
 
