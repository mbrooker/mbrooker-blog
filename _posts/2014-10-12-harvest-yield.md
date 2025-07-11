---
layout: post
title: "Harvest and Yield: Not A Natural Cure for Tradeoff Confusion"



related_posts:
  - "/2018/02/25/availability-liveness"
  - "/2024/07/25/cap-again"
  - "/2015/09/26/cap-durability"
---{{ page.title }}
================

<p class="meta">Comments on a 15 year old paper.</p>

As I wrote about in my [post on PACELC](http://brooker.co.za/blog/2014/07/16/pacelc.html), I don't think the CAP theorem is the right way for teachers to present distributed systems tradeoffs. I also don't think it's ideal for working practitioners, despite its wide use. I prefer Abadi's [PACELC](http://cs-www.cs.yale.edu/homes/dna/papers/abadi-pacelc.pdf), but there are legitimate criticisms of that one too. One criticism is that it's poorly formalized, which makes it hard to apply to precise statements. Another is the PC/EL is an awkward edge case. There are more. Fox and Brewer's *harvest* and *yield* model, from [Harvest, Yield, and Scalable Tolerant Systems](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.33.411&rep=rep1&type=pdf), is a [widely promoted](http://codahale.com/you-cant-sacrifice-partition-tolerance/) alternative. 

While I like the concepts of *harvest* and *yield*, I find it hard to recommend the paper. Both [Eric Brewer](http://www.cs.berkeley.edu/~brewer/) and [Armando Fox](http://www.eecs.berkeley.edu/Faculty/Homepages/fox.html) have made big contributions to the field, and I like many of their papers. I just don't like this one.

I'll start with what I dislike most about it. From the first page:

> Partition-resilience means that the system as whole can survive a partition between data replicas.

> CA without P: Databases that provide distributed transactional semantics can only do so in the absence of a network partition separating server peers.

I find these statements awkward, and feel like they support the mistaken belief that CA is a valid option. You certainly can't pick CA where your C is linearizability (as in [Gilbert and Lynch's](http://lpd.epfl.ch/sgilbert/pubs/BrewersConjecture-SigAct.pdf) proof) [or serializability](http://www.bailis.org/blog/linearizability-versus-serializability/#fn:hardness). If you're allowed to pick CA, either your definition of C is weaker than either of those, your definition of A doesn't require minority partitions to make progress, or you're in denial about network partitions (which [do exist](http://aphyr.com/posts/288-the-network-is-reliable)). Compare the definition of CP:

> CP without A: In the event of a partition, further transactions to an ACID database may be blocked until the partition heals, to avoid the risk of introducing merge conflicts (and thus inconsistency).

Is that saying that CA exists, but just introduces inconsistencies that CP doesn't? Overall, I don't follow Fox and Brewer's thinking about *partition tolerance* in this paper.

On to the model itself, which concerns itself with:

> large applications whose output behavior tolerates *graceful degradation*

The idea of *graceful degradation* is that a partial response may be more useful to the client than no response, so you can directly trade off between the completeness of the response and the availability of the system. Many real-world systems can tolerate partial responses, especially if you can provide some bounds on the definition of partial. Using probabilistic data structures like [Bloom filters](http://en.wikipedia.org/wiki/Bloom_filter) and the [count-min sketch](http://www.cse.unsw.edu.au/~cs9314/07s1/lectures/Lin_CS9314_References/cm-latin.pdf) is a widely accepted technique for scaling systems, and it makes sense to apply the same ideas to availability.

> We assume that clients make queries to servers, in which case there are at least two metrics for correct behavior: *yield*, which is the probability of completing a request, and *harvest*, which measures the fraction of the data reflected in the response, i.e. the completeness of the answer to the query.

*Yield* is the availability metric that most practitioners end up working with, and it's worth noting that its different from CAP's *A*. The authors don't define it formally, but treat it as a long-term probability of response rather than the probability of a response conditioned on there being a failure. That's a good common-sense definition, and one that fits well with the way that most practitioners think about availability.

> In the presence of faults there is typically a tradeoff between providing no answer (reducing yield) and providing an imperfect answer (maintaining yield, but reducing harvest).

That's a very powerful idea, and one worth sharing. 

> Even when the 100%-harvest answer is useful to the client, it may still be preferable to trade response time for harvest when client-to-server bandwidth is limited, for example, by intelligent degradation to low-bandwidth formats.

Another good idea, and one that has been [widely used](http://www.opera.com/mobile/mini). As good an ideas as it is, though, the paper is conflating at least three separate ideas: cases of shrinking data to conserve bandwidth, responding with cached data to conserve latency, and responding with a partial response to conserve availability. A more precise definition of *harvest* would be very useful, as would definitions of different availability, latency and bandwidth tradeoffs.

> The actual benefit is the ability to provision each subsystem’s state management separately, providing strong consistency or persistent state only for the subsystems that need it, not for the entire application.

That's yet another good idea, as is the concept of *orthogonal mechanisms* from section 5. Again, the problem is that the idea isn't fully developed, and has some significant edge cases.

I really like the concepts of *harvest* and *yield*, and many of the other ideas in this paper. I just find the whole thing hard to recommend as a unit. It feels like a bag full of unmarked tools. A sharp scalpel. A rusty hammer. A glass bottle of [FOOF](http://pipeline.corante.com/archives/2010/02/23/things_i_wont_work_with_dioxygen_difluoride.php). A nice microscope. There's a lot in there to like, but sticking your hand in and rummaging around may do more harm than good.

In any case, *harvest* and *yield* isn't really a CAP replacement. The CAP theorem is one boundary of the space of possible designs, a fence that can't be crossed. Fox and Brewer's ideas are more about the shape of the landscape inside the fence. That's useful knowledge, but it's really in a different category from CAP.