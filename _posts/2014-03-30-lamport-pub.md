---
layout: post
title: "The Essential Leslie Lamport"






related_posts:
  - "/2014/05/10/lynch-pub.html"
  - "/2014/09/21/liskov-pub.html"
  - "/2014/03/08/model-checking.html"
---
{{ page.title }}
================

<p class="meta">Some of my favourite Leslie Lamport publications.</p>

After it was announced that Leslie Lamport had won [the 2013 A.M. Turing award](http://amturing.acm.org/award_winners/lamport_1205376.cfm), the link to his [list of publications](http://research.microsoft.com/en-us/um/people/lamport/pubs/pubs.html) found popularity on most of the tech-related sites I visit. It's an excellent page, with a long (and growing) list of Lamport's publications, and witty comments by the author on each one. The whole list is worth a read, but can feel overwhelming, so I thought I'd try distill it down it some papers that I feel are really worth reading, if you read nothing else on that page. The criteria are: I like these papers for some reason. I'd probably make a different list if I wrote this post again next week.

> The algorithm is quite simple. It is based upon one commonly used in bakeries, in which a customer receives a number upon entering the store. The holder of the lowest number is the next one served.

In [A New Solution of Dijkstra's Concurrent Programming Problem](http://research.microsoft.com/en-us/um/people/lamport/pubs/bakery.pdf) Lamport describes the mutual exclusion problem formally posed by [Dijkstra](http://dl.acm.org/citation.cfm?id=365617), and presents a solution to it. The *bakery algorithm* is remarkable. Unlike the earlier solutions, which depended on shared memory locations with fairly restrictive behaviors, the bakery algorithm works without any underlying mutual exclusion. Lamport's invention, or discovery, of this algorithm seems to kicked off a cascade of other solutions. [Szymanski's](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.113.2277&rep=rep1&type=pdf), which seems to have been the first to offer both strong fairness and use of a fixed number of shared variables of a bounded size, is particularly interesting.

*Why this is worth reading*: The bakery algorithm, while not very relevant to today's concurrent software due to changes in memory models, is very simple, very beautiful, and solves a complex problem in an innovative way. It's simply a beautiful piece of computer science.

> In a distributed system, it is sometimes impossible to say that one of two events occurred first. The relation *"happened before"* is therefore only a partial ordering of the events in the system. We have found that problems often arise because people are not fully aware of this fact and its implications.

> Being able to totally order the events can be very useful in implementing a distributed system. In fact, the reason for implementing a correct system of logical clocks is to obtain such a total ordering.

[Time, Clocks and the Ordering of Events in a Distributed System](http://research.microsoft.com/en-us/um/people/lamport/pubs/time-clocks.pdf) is Lamport's most widely cited paper by a large margin. Google Scholar claims over 8000 citations, [CiteSeer](http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.142.3682&rank=1) finds 2335. His next most cited paper, [The Byzantine Generals Problem](http://research.microsoft.com/en-us/um/people/lamport/pubs/byz.pdf), has fewer than a quarter of as many citations. Citeseer's view of [co-citations](http://citeseerx.ist.psu.edu/viewdoc/similar?doi=10.1.1.142.3682&type=cc) of this paper is interesting. The top two are Fischer, Lynch and Paterson's [Impossibility of Distributed Consensus with One Faulty Process](http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.13.6760) which contains the famous *FLP impossibility* result, and Lamport's own The Byzantine Generals problem. Browsing through the papers that cite both of these, it's disturbingly easy to find cases where they are included more for name recognition than any real relevance to the subject at hand.

*Why this is worth reading*: Aside from the sad realities of citation practices, this is legitimately a fascinating, important and highly influential paper. The paper points to [The Maintenance of Duplicate Databases](https://tools.ietf.org/html/rfc677) as the first published description of logical clocks in distributed systems, and both extends and clarifies the idea. Time, Clocks... presents a way to extract a deterministic total ordering (note, not *the* total ordering) from a partial ordering of events. It also introduces the idea of replicated state machines, which Lamport later expanded on in [The Implementation of Reliable Distributed Multiprocess Systems](http://research.microsoft.com/en-us/um/people/lamport/pubs/implementation.pdf). It deserves to be recognized for both of these contributions, and is well worth reading because it presents the foundation of these ideas in a way that's easy to understand. Don't be ashamed if you skip over the proof in the Appendix, but it's worth your time if you'd like to get a deeper understanding of why logical clocks work.

> We cannot ensure that the states of all processes and channels will be recorded at the same instant because there is no global clock; however, we require that the recorded process and channel states form a “meaningful” global system state. 

The algorithm described in [Distributed Snapshots: Determining Global States of a Distributed System](http://research.microsoft.com/en-us/um/people/lamport/pubs/chandy.pdf), written by Lamport with [K. Mani Chandy](http://infospheres.caltech.edu/people/mani), does something that is apparently impossible: it creates a consistent global snapshot of a distributed system without requiring global synchronization. It does this by ensuring that a snapshot is taken of the local state of each process in the system, along with every message in flight to that process at the time that local snapshot was taken. The [wikipedia page](http://en.wikipedia.org/wiki/Snapshot_algorithm#Working) has a nice summary of how it works.

*Why this is worth reading*: the algorithm presented in this paper, like many great algorithms, does something that seems really difficult in a way that, in retrospect, appears to be simple or even trivial. The paper also does a nice job of explaining the system model, and breaking down each step of the algorithm in clear terms. It's worth reading because it's a fascinating piece of computer science, and because of the way it presents its ideas. 

> Designing a concurrent program is a difficult task; no formalism can make it easy.

> when designing a concurrent program, we cannot restrict our attention to what is true before and after its execution; we must also consider what happens *during* its execution.

Lamport has written a great deal on temporal logic, including the deservedly heavily-cited [The Temporal Logic of Actions](http://research.microsoft.com/en-us/um/people/lamport/pubs/lamport-actions.pdf), but none of his other papers on the subject are (in my opinion) as well written as the earlier [What Good Is Temporal Logic?](http://research.microsoft.com/en-us/um/people/lamport/pubs/what-good.pdf) This paper came fairly early on in the development of TLA, and isn't a complete picture of the idea, but what is there is presented in a way that is both precise and compelling. The paper compares the approach to Milner's work on the Calculus of Communicating Systems, explains the philosophy of temporal logic, explores the value of specifications, and presents the idea of refinement mappings. One point of interest is the conclusion about the expense of mechanical verification, something that has been substantially improved since 1983 more by hardware power than by theory.

*Why this is worth reading*: it is excellent scientific writing, and covers a lot of ground in not much text.

> as commerce flourished, priests began wandering in and out of the Chamber while the Synod was in progress

[The Part-Time Parliament](http://research.microsoft.com/en-us/um/people/lamport/pubs/lamport-paxos.pdf) is important because of what it contains, and when it was published. It describes, it a fun but impractical way, the Paxos algorithm for distributed consensus. Originally submitted in 1989 (or 1990, depending on who you believe), it came at around the same time as Oki and Liskov's work on [viewstamped replication](http://dl.acm.org/citation.cfm?id=62549) and is one of the earliest solutions for a problem of both practical and theoretical importance. [Paxos Made Simple](http://research.microsoft.com/en-us/um/people/lamport/pubs/paxos-simple.pdf), [Cheap Paxos](http://research.microsoft.com/en-us/um/people/lamport/pubs/web-dsn-submission.pdf), [Fast Paxos](http://research.microsoft.com/research/pubs/view.aspx?type=Technical%20Report&id=966) and [Vertical Paxos and Primary-Backup Replication](http://research.microsoft.com/en-us/um/people/lamport/pubs/vertical-paxos.pdf) all contain more easily understandable descriptions of various variants of Paxos.

*Why this is worth reading*: it's fun, has a great sense of humor, and no sense of self-importance to go with its real importance.

*Don't read this if*: you're trying to understand or implement Paxos.