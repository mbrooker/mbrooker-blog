---
layout: post
title: "Invariants: A Better Debugger?"
---

{{ page.title }}
================

<p class="meta">🎵Some things never change🎵</p>

*Like many of my blog posts, this started out as a long email to a colleague. I expanded it here because I thought folks might find it interesting.*

I don't tend to use debuggers. I'm not against them. I've seen folks do amazing things with `gdb`, and envy their skills. I just don't tend to reach for a debugger very often.

I'm also not a huge fan of `printf` debugging. It can be useful, it's easy to implement, and works well in both one-box and distributed systems. But very quickly the reams of output become overwhelming, and slow me down rather than helping me reason about things.

My go-to approach when faced with bugs is testing. More specifically, testing *invariants*. Most specifically, writing unit tests that assert those invariants after a system or algorithm takes each step.

*Invariants*, like *assertions*, are things that must be true during or after the execution of a piece of code. "This array is sorted", "the first item is the smallest", "items in the *deleted* state must have a *deleted time*", that kind of thing. Invariants are broader than assertions: they can assert properties of a piece of data, properties of an entire data structure, or even properties of collections of data structures spread across multiple machines.

**Some History**

Way back in undergrad at UCT, I was trying to implement [Guibas and Stolfi's algorithm for Delaunay triangulation](https://dl.acm.org/doi/abs/10.1145/282918.282923) for a class project<sup>[1](#foot1)</sup>. My implementation very nearly worked, but there was one example where it just gave the wrong answer. I spent days banging my head on the problem with printfs and debuggers and just wasn't making progress. The TA was no help. I asked a CS graduate student I knew that lived nearby, and his approach just blew my mind.

He sat me down with a piece of paper, and went through the algorithm step-by-step, asking *what is true about the data structure at this step?* after each one. Together, we came up with a set of step-by-step invariants, and some global invariants that must hold true after every run of the algorithm. Within minutes of getting back to my desk and writing the tests for the invariants, I had found my bug: a `>` which should have been `>=`.

Over the years, I've kept coming back to this approach. It's turned out to be useful when writing dense algorithmic code, when capturing business logic, and when implementing distributed systems. It's also one of of the things that [attracted me strongly to TLA+ a decade ago](https://cacm.acm.org/magazines/2015/4/184701-how-amazon-web-services-uses-formal-methods/fulltext). The way TLA+ thinks about correctness is all based on global invariants.

**Example: Paxos**

Paxos is famously difficult to reason about. I'm not going to pretend that I've ever found it easy, but I believe people struggle more than needed because they don't pay enough attention to Section 2.2 of [Paxos Made Simple](https://lamport.azurewebsites.net/pubs/paxos-simple.pdf). In it, Lamport goes step-by-step through the development of a set of invariants for implementing consensus. Starting with some incorrect ones:

> P1. An acceptor must accept the first proposal that it receives.

and layering on requirements before settling on the right invariant. Notably, during this development, the invariants switch from something that can be easily asserted on a single node, to larger properties of the whole system that could only really be asserted by some omniscient higher power.

In a real system, or even in an integration test, its very hard to implement this kind of omniscience. In a model checker, like TLA+'s TLC, it's trivial, but that doesn't help all that much for the real implementations. Omnisciently asserting global invariants is one of the most powerful abilities granted by deterministic simulation testing (such as with [turmoil](https://github.com/tokio-rs/turmoil)). In the simulator, you can stop time, check invariants, then tick forward deterministically. And by *you*, I mean *a test*. Tests, unlike debuggers, are easily automatable and repeatable. They're also able to check things that humans could never keep straight in their heads.

Like Paxos's key invariant:

> P2c . For any v and n, if a proposal with value v and number n is issued,
then there is a set S consisting of a majority of acceptors such that
either (a) no acceptor in S has accepted any proposal numbered less
than n, or (b) v is the value of the highest-numbered proposal among
all proposals numbered less than n accepted by the acceptors in S.

Multipaxos, Raft, and pretty much every other distributed protocol has invariant like these. Reasoning about them and testing them automatically is, in my mind, an under-appreciated superpower.

**Example: HNSW**

[Hierarchical Navigable Small World Graphs](https://arxiv.org/abs/1603.09320) are a popular data structure for performing approximate K Nearest Neighbor searches on large sets of high dimensionality vectors. HNSW isn't at it's core, too conceptually difficult ([this is a good introduction](https://www.pinecone.io/learn/series/faiss/hnsw/)), but is also way harder to reason about than most of the algorithms we come across day to day. The large size of the vectors, big data, and complexity of graph connectivity makes it difficult to reason about HNSW in a debugger.

But many implementation bugs can be shaken out by thinking about the invariants. What are the things that must be true about the data structure after inserting a new element? What are the things that must be true of the set of entry points passed down into each layer of the search?

For example:

* The *entry point* must be present in the highest populated layer.
* Each layer is a subset of the previous layer<sup>[2](#foot2)</sup>. Nodes don't just disappear as you go down the layers.
* Every node is accessible from the highest appearance of the entry point.
* All nodes appear in layer 0.
* Each layer is approximately *e* times larger than the one above it.

Some of these invariants are rather expensive to test for, such as the last one which requires *O(N log N)* work. They aren't practical to assert on each step in a production implementation, but are very practical to test for in a set of unit tests. My experience has been that reasoning about, listing, and then testing for invariants like this is a much better way to test data structures like this than testing through the interfaces.

**Example: Systems Design**

When working on AWS Lambda's container loading system (which I've [blogged about before](https://brooker.co.za/blog/2023/05/23/snapshot-loading.html), and we describe in our paper [On-demand Container Loading in AWS Lambda](https://www.usenix.org/conference/atc23/presentation/brooker)), we needed to make sure that chunks weren't lost during garbage collection. Highly-concurrent large-scale distributed systems of this kind can be extremely difficult to reason about, and so we needed to start our thinking with a set of invariants. As we say in the paper:

> Past experience with distributed garbage collection has taught us that the problem is both complex (because the tree of chunk references is changing dynamically) and uniquely risky (because it is the one place in our system where we delete customer data).

Despite this complexity, the system invariants turn out to be relatively simple:

* All new chunks are written into a root in the *active* state.
* All read chunks are under roots in either then *active* or *retired* state.
* Roots move monotonically through the *active*, *retired*, *expired*, and *deleted* states. They never move backwards through this state chain.
* Chunks can only be deleted if they are referenced only by *expired* roots.
* A root can only move to *deleted* once all its chunks have been deleted.

Provided the system preserves these invariants, no data can be lost, even in the face of arbitrary concurrency and scale. This handful of invariants is much easier to reason about than the full system implementation, and writing it down allowed us to come up with a clear formal argument for why these invariants are sufficient.

**The Lesson**

Invariants are a powerful tool for reasoning about algorithms, data structures, and distributed systems. It's worth thinking through a set of invariants for any complex system or algorithm you design or implement. It's also worth building your implementation in such a way that even global invariants can be easily tested in a deterministic and repeatable way. 

**Footnotes**

1. <a name="foot1"></a> This is a very cool algorithm, and actually not too complicated. But you can tell by the fact that the abstract contains the phrase *separation of the geometrical and topological aspects of the problem* that it's also not the most straightforward thing to reason about.
2. <a name="foot2"></a> Advanced implementations may choose to reduce their memory or storage footprint by relaxing these invariants.