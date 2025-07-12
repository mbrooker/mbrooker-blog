---
layout: post
title: "The Essential Barbara Liskov"




related_posts:
  - "/2014/03/30/lamport-pub"
  - "/2014/05/10/lynch-pub"
  - "/2020/10/05/consensus"
---
{{ page.title }}
================

<p class="meta">Some of my favorite Barbara Liskov publications.</p>

Barbara Liskov is one of the greats of computer science. Over a research career nearing 45 years, she's had a resounding impact on multiple different fields, and received an impressive list of honors and awards, including the [2009 Turing Award](http://amturing.acm.org/award_winners/liskov_1108679.cfm). In the same spirit as [The Essential Leslie Lamport](http://brooker.co.za/blog/2014/03/30/lamport-pub.html) and [The Essential Nancy Lynch](http://brooker.co.za/blog/2014/05/10/lynch-pub.html), I thought I'd write about some of my favorite Liskov papers. These are just papers I like or found particularly interesting, and I'm likely to have missed some you like.

> What does it mean for one type to be a subtype of another? We argue that this is a semantic question having to do with the behavior of the objects of the two types: the objects of the subtype ought to behave the same as those of the supertype as for as anyone or any program using the supertype objects can tell.

[Data abstraction and hierarchy](http://dl.acm.org/citation.cfm?doid=62139.62141), [A behavioral notion of subtyping](http://dl.acm.org/citation.cfm?doid=197320.197383) and [Behavioral subtyping using invariants and constraints](http://reports-archive.adm.cs.cmu.edu/anon/1999/CMU-CS-99-156.ps), are why most working programmers would recognize Liskov's name. The *Liskov Substitution Principle*, widely known as the *L* in SOLID, is a widely-followed rule about the relationship between the behavior of its supertypes and subtypes.

I'll readily admit that types are an area of computer science that I'm not very familiar with, but I found these papers easy to follow and very applicable. For the working programmers, there's not much material there that isn't covered in the [wiki page](http://en.wikipedia.org/wiki/Liskov_substitution_principle), but it's worth reading to see how Liskov lays out the arguments for the principle. If you're interested in the history and thinking behind rules, these are great papers to read.

> Availability is achieved through replication.

> Transaction processing depends on forcing information to backups so that a majority of cohorts know about particular events.

[Viewstamped Replication: A New Primary Copy Method to Support Highly Available Distributed Systems](http://pmg.csail.mit.edu/papers/vr.pdf) deserves to be recognized as one of the most influential papers in distributed systems. Viewstamped replication predates Lamport's Paxos, but solves the same problem in a very similar (though [distinct](http://www.cs.cornell.edu/fbs/publications/viveLaDifference.pdf)) way. The viewstamped replication paper remains both readable and relevant, although some of the descriptions and formalisms used show the paper's age. If you only have time to read one paper on Viewstamped Replication, the recent [Viewstamped Replication Revisited](http://pmg.csail.mit.edu/papers/vr-revisited.pdf) is probably a better bet, because it provides a clearer description of the protocol and the design decisions it makes.

I've [written before](http://brooker.co.za/blog/2014/05/19/vr.html) about viewstamped replication, and why I think it should be more widely recognized for the contributions it made to consensus, and the idea of [state machine replication](https://www.cs.cornell.edu/fbs/publications/SMSurvey.pdf). It would be great to see more knowledge about VR among distributed systems practitioners.

> Unlike other multistamp (or vector clock) schemes, our scheme is based on time rather than on logical clocks: each entry in a multistamp contains a timestamp representing the clock time at some server in the system.

Version vectors (or vector clocks or multistamps) are a very widely used scheme for versioning data in distributed systems, but keeping them short in the face of scaling or reconfigurations and scaling is a real challenge. [Lazy consistency using loosely synchronized clocks](http://dl.acm.org/citation.cfm?id=259425) paper presents an one approach, using loosely synchronized physical clocks. An interesting aspect of it is that it breaks from the orthodoxy that using clocks for ordering is bad (which it is, unless you are careful with your safety properties). If you find reading this worthwhile, I'd also recommend [Dynamic Version Vector Maintenance](ftp://ftp.cs.ucla.edu/tech-report/1997-reports/970022.ps.Z) and [Flexible update propagation for weakly consistent replication](http://dl.acm.org/citation.cfm?id=266711).

> One reason why Byzantine-fault-tolerant algorithms will be important in the future is that they can allow systems to continue to work correctly even when there are software errors. Not all errors are survivable; our approach cannot mask a software error that occurs at all replicas. However, it can mask errors that occur independently at different replicas, including non-deterministic software errors, which are the most problematic and persistent errors since they are the hardest to detect.

Byzantine faults [happen all the time](http://www.rvs.uni-bielefeld.de/publications/DriscollMurphyv19.pdf) in the real world. Handling them, and making safe progress in distributed systems despite them, is a huge challenge. Just getting the theory right is tricky. Getting to a practical implementation of Byzantine fault tolerance is even harder. That's what makes [Practical Byzantine fault tolerance](http://pmg.csail.mit.edu/papers/osdi99.pdf) so important. Castro and Liskov describe an practically implementable Byzantine fault tolerant system, that works in a realistic system model. The key contribution here, over precursors like [Rampart](https://www.cs.unc.edu/~reiter/papers/1994/CCS.pdf), is safety in real-world system models, especially removing assumptions about synchronicity.

On a similar topic, [Byzantine clients rendered harmless](http://pmg.csail.mit.edu/papers/rodrigo_tr05.pdf) is also worth reading. Many approaches to Byzantine-fault-tolerant replication make both safety and liveness assumptions about the behavior of clients. Strengthening the protocol against client behavior is very important to practical systems.