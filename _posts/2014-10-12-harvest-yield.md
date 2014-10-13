---
layout: post
title: "Harvest and Yield: A Natural Cure for Tradeoff Confusion?"
---

{{ page.title }}
================

<p class="meta">Are harvest and yield the concepts we are looking for?</p>

As I wrote about in my [post on PACELC](http://brooker.co.za/blog/2014/07/16/pacelc.html), I don't think the CAP theorem is the right way for teachers to present distributed systems tradeoffs. I also don't think it's ideal for working practitioners, despite its wide use. I prefer Abadi's [PACELC](http://cs-www.cs.yale.edu/homes/dna/papers/abadi-pacelc.pdf), but there are legitimate criticisms of that one too. One criticism is that it's poorly formalized, which makes it hard to make formal statements about. Another is the PC/EL is an awkward edge case. There are more. Fox and Brewer's *harvest* and *yield* model, from [Harvest, Yield, and Scalable Tolerant Systems](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.33.411&rep=rep1&type=pdf), is often held up as a useful alternative. 

Before talking about the usefulness of the model, though, I feel like I need to give a warning about the paper. Specifically, this from the first page:

> CA without P: Databases that provide distributed transactional semantics can only do so in the absence of a network partition separating server peers.

This is an awkward statement, because it supports the (in my opinion) mistaken belief that you can pick CA. You certainly can't in with linearizability (as in [Gilbert and Lynch's](http://lpd.epfl.ch/sgilbert/pubs/BrewersConjecture-SigAct.pdf) proof) [or with serializability](http://www.bailis.org/blog/linearizability-versus-serializability/#fn:hardness). If you're allowed to pick CA, either your definition of C is weaker than either of those, your definition of A doesn't require minority partitions to make progress or you're in denial about network partitions (which [do exist](http://aphyr.com/posts/288-the-network-is-reliable)). I don't follow Fox and Brewer's thinking about *partition tolerance* in this paper, and it sounds like fuzzy thinking to me.

On to the model itself, which concerns itself with:

> large applications whose output behavior tolerates *graceful degradation*

The idea of *graceful degradation* is that a partial response may be more useful to the client than no response, so you can directly trade off between the compeleness of the response and the availability of the system.

> We assume that clients make queries to servers, in which case there are at least two metrics for correct behavior: *yield*, which is the probability of completing a request, and *harvest*, which measures the fraction of the data reflected in the response, i.e. the completeness of the answer to the query.

*Yield* is the availability metric that most practitioners end up working with, and it's worth noting that its different from CAP's *A*. The authors don't define it formally, but treat it as a long-term probability rather than the specific probability during failure attempts. That's a good common-sense definition, and one that fits well with the way that most practitioners think about availability.

> In the presence of faults there is typically a tradeoff between providing no answer (reducing yield) and providing an imperfect answer (maintaining yield, but reducing harvest).

The paper then talks about some examples where degradation may be acceptable, including search, aggregate data analysis and image serving. It also hints at a broader definition of *harvest* than incomplete results, which would include stale results and lower-resolution or lower-precision results. I really like this concept, and I like the legitimization of returning partial results in failure cases. This paper hits it's stride, and proves the usefulness of the *harvest* concept by talking about how it can inform system design:

> The actual benefit is the ability to provision each subsystem’s state management separately, providing strong consistency or persistent state only for the subsystems that need it, not for the entire application.

The concept of *orthogonal mechanisms* is an interesting one, but I don't think it's very well developed. It appears to be a restatement of a very old architectural principal, justified using the statistics of system failures. Still, it's well worth reading.

Overall, *harvest* and *yield* are powerful concepts, and paper contains multiple good ideas. Unfortunately, its formal thinking about these concepts is very limited, which I find makes them less useful. Harvest and yield provide a good way to think about the way that systems degrade, and should be more widely used, but I don't see *H* and *Y* taking over from *C* and *A* in most practitioner's vocabularies.









