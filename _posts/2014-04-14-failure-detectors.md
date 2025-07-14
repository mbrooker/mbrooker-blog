---
layout: post
title: "Failure Detectors, and Non-Blocking Atomic Commit"









related_posts:
  - "/2014/01/12/ben-or.html"
  - "/2020/10/05/consensus.html"
  - "/2014/05/10/lynch-pub.html"
dissimilar_posts:
  - "/2020/07/28/fish.html"
---
{{ page.title }}
================

<p class="meta">Non-blocking atomic commit is harder than uniform consensus. Why would that be?</p>

Many of the most interesting results in distributed systems have come from looking at problems known to be impossible under one set of constraints, and finding how little those constraints can be relaxed before the problem becomes possible. One great example is how adding a [random Oracle](http://brooker.co.za/blog/2014/01/12/ben-or.html) to the asynchronous system model used by [FLP](http://cs-www.cs.yale.edu/homes/arvind/cs425/doc/fischer.pdf) makes consensus possible. That result is very interesting, but not as practically important as the idea of failure detectors.

The theoretical importance of detecting failures in the asynchronous model dates back to work in the 1980s from [Dolev, Dwork and Stockmeyer](http://groups.csail.mit.edu/tds/papers/Stockmeyer/DolevDS83-focs.pdf) and [Dwork, Lynch and Stockmeyer](http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.13.3423). The latter of these papers is very interesting, because it describes what can be argued is the first practical consensus algorithm before the publication of Viewstamped Replication and Paxos. More on that another time. A great, detailed, description and characterization of failure detectors can be found in [Unreliable Failure Detectors for Reliable Distributed Systems](http://www.cs.utexas.edu/~lorenzo/corsi/cs380d/papers/p225-chandra.pdf) by Chandra and Toueg. They also introduced the concept of _unreliable failure detectors_:

> In this paper, we propose an alternative approach to circumvent such impossibility results, and to broaden the applicability of the asynchronous model of computation. Since impossibility results for asynchronous systems stem from the inherent difficulty of determining whether a process has actually crashed or is only "very slow," we propose to augment the asynchronous model of computation with a model of an external failure detection mechanism that can make mistakes. In particular, we model the concept of _unreliable failure detectors_ for systems with _crash failures_.

The failure detectors that Chandra and Toueg describe are distributed, rather than global, failure detectors. Each process uses local state to keep a list of other processes that it suspects have failed, and adds and removes processes from this list based on communication with other processes. A local failure detector like that can make two kinds of mistakes: putting processes that haven't failed onto the list, and not putting processes that have failed onto the list. Remember, that like in all real distributed systems, there is no central oracle that can tell a node whether its list is correct. These two kinds of mistakes lead to the definition of two properties. From [Chandra and Toueg](http://www.cs.utexas.edu/~lorenzo/corsi/cs380d/papers/p225-chandra.pdf):

> _completeness_ requires that a failure detector eventually suspects every process that actually crashes, while _accuracy_ restricts the mistakes that a failure detector can make.⋄

Let's start with the best combination of these properties, a failure detector where every correct process eventually permanently suspects every crashed process of failing (_strong completeness_), and never suspects a non-crashed process (_strong accuracy_). This failure detector, **_P_**, can be seen as the ideal asynchronous failure detector. It doesn't make mistakes, and it does the best it can at detection while remaining asynchronous. At the other end of the scale is ◇**_W_**. With this failure detector, every crashed process is eventually permanently suspected by some crashed process, and eventually some correct process is not suspected by any correct process.  ◇**_W_**, unlike **_P_**, can make lots and lots of mistakes, for arbitrarily long amounts of time.

Before going further, it's worth introducing one piece of notation. Even informal writing about failure detectors tends to make heavy use of the ◇ operator from [temporal logic](http://en.wikipedia.org/wiki/Temporal_logic). Don't be put off by the notation, ◇F simply means _F is eventually true_. There is some state in the future where F is true. To better understand that, let's compare the failure detectors ◇**_W_** and **_W_**. Both of these meet the weak completeness condition:

> _Weak Completeness._ Eventually every process that crashes is permanently suspected by some correct process.

**_W_** meets the weak accuracy condition:

> _Weak Accuracy_. Some correct process is never suspected.

While ◇**_W_** only meets the strictly weaker eventual weak accuracy condition.

> _Eventual Weak Accuracy_. There is a time after which some correct process is never suspected by any correct process.

Comparing those two makes the difference more obvious. ◇**_W_** is allowed to make mistakes early on (before _a time_) what **_W_** isn't allowed to make.

The existence of these classes of failure detectors allows meaningful comparisons to be made about the difficulty of different distributed problems, much like [complexity classes](http://en.wikipedia.org/wiki/Complexity_class) allow us to compare the difficulty of computational problems. For example, [it is known](http://www.cs.utexas.edu/~lorenzo/corsi/cs380d/papers/p685-chandra.pdf) that consensus can be solved using ◇**_W_** if only a minority of processes fail. The problem known as non-blocking atomic commit (NB-AC), on the other hand, [cannot be solved](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.27.6456&rep=rep1&type=pdf) with ◇**_W_** if there is a single failure. In a very meaningful sense, NB-AC is _harder than_ consensus. When I first learned about that result, I found it surprising: my assumption had been that uniform consensus was equivalent to the hardest problems in distributed systems.

First, let's define the NB-AC and consensus problems. They have a lot in common, both being non-blocking agreement problems. Both consensus and NB-AC attempt to get a multiple processes to agree on a single value without blocking in the presence of failures. [Two-phase commit](http://en.wikipedia.org/wiki/2PC) is, like NB-AC and consensus, an agreement protocol, but it is a blocking one. The presence of a single failure will cause 2PC to block forever.

[Guerraoui](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.27.6456&rep=rep1&type=pdf) defines consensus with three conditions:

> _Agreement_: No two correct participants decide different values

> _Uniform-Validity:_ If a participant decides _v_, then _v_ must have been proposed by some participant

> _Termination:_ Every correct process eventually decides

Uniform consensus expands the _agreement_ condition to a stronger one, called _uniform agreement_:

> _Uniform-Agreement_: No two participants (correct or not) decide different values.

Consensus is, therefore, about _deciding_. NB-AC, on the other hand, is about _accepting_ or voting on whether to _commit_ a transaction. [Guerraoui](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.27.6456&rep=rep1&type=pdf) defines it with four conditions:

> _Uniform-Agreement_: No two participants AC-decide different values.

> _Uniform-Validity:_ If a participant AC-decides _commit_, then all participants voted ''yes''.

> _Termination:_ Every correct process eventually AC-decides

> _NonTriviality_: If all participants vote _yes_, and there is no failure, then every correct participant eventually AC-decides _commit_.

Notice how similar this appears to be to the uniform consensus problem. Guerraoui describes how it is the last one of these conditions, _NonTriviality_, which has the effect of requiring that a solution to NB-AC has precise knowledge about failures. To meet the _Termination_ condition, eventually each process needs to _commit_ or _abort_. Eventual strong accuracy doesn't provide the knowledge required to make that decision, because it admits a time *t* where a process is simply delayed but it's vote is ignored (violating the _uniform validity_ or _NonTriviality_ conditions depending on the vote). Weak accuracy doesn't provide the right knowledge either, because it allows an incorrect abort (and hence violation of _NonTriviality_) based on incomplete knowledge of the failed set.

If you only have unreliable failure detectors, uniform consensus is [no harder than consensus](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.27.6456&rep=rep1&type=pdf), though reliable failure detectors (like **_P_**) [make consensus easier](http://infoscience.epfl.ch/record/88273/files/CBS04.pdf?version=1) than uniform consensus. Therefore, the addition of the _uniform agreement_ requirement doesn't explain why consensus can be solved with ◇**_W_** and NB-AC can't. Instead, it's that seemingly harmless _NonTriviality_ condition that makes NB-AC harder. That's a great example of how intuition is often a poor guide in distributed systems problems: seemingly similar problems, with very similar definitions, may end up with completely different difficulties.