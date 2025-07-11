---
layout: post
title: "The Operations Gradient: Improving Safety in Complex Systems"



related_posts:
  - "/2015/11/05/heuristics"
  - "/2016/01/03/correlation"
  - "/2021/02/22/postmortem"
---{{ page.title }}
================

<p class="meta">Can we improve the safety of complex systems by listening to operators more?</p>

This week, I watched [an excellent lecture](https://www.youtube.com/watch?v=PGLYEDpNu60&feature=youtu.be) by [Richard Cook](http://www.ctlab.org/Cook.cfm). He goes in some detail about why failures happen, through the lens of Rasmussen's model of system safety. If you build or maintain any kind of complex system, don't miss this lecture.

> What is surprising is not that there are so many accidents, it's that there are so few.

The model that takes up most of the lecture is best expressed in Rasmussen's [Risk Management in a Dynamic Society: A Modelling Problem](http://www.sciencedirect.com/science/article/pii/S0925753597000520), a classic paper that deserves more attention among engineers. The core of the insight of the model from Rasmussen's paper comes from Figure 3:

![Rasmussen, 1997, Figure 3](https://s3.amazonaws.com/mbrooker-blog-images/rasmussen-figure3.png)

Rasmussen describes the process of developing systems as an *adaptive search* within a boundary defined by a set of economic constraints (it's not economically viable to run the system beyond this boundary), engineering effort constraints (there are not enough actors to push the system beyond this boundary), and safety constraints (the system has failed beyond this boundary). The traditional balance between engineering effort and economic return plays out in pushing the operating point of the system away from two of these boundaries. From the paper:

> During the adaptive search the actors have ample opportunity to identify an *effort gradient* and management will normally supply an effective *cost gradient*. 

The combination of optimizing for these two gradients tends to push the operating point towards the safety boundary (or *boundary of acceptable performance*). A conscious push for safety (or *availability*, *durability* and other safety-related properties) forces the operating point away from this boundary. One danger of this is that the position of the safety boundary is not always obvious, and it's also not a single clean line. From the paper:

> in systems designed according to a defence-in-depth strategy, the defenses are likely to degenerate systematically through time, when pressure towards cost-effectiveness is dominating.

This is a key point, because defence-in-depth is frequently seen as a very good thing. It's danger in this model is that it turns the safety boundary into a gradient, and significant degeneration in safety can happen before there is any accident. In response to this, we estimate an error margin, we put up an organizational "*perceived boundary of acceptable performance*", and we put systems in place to monitor that boundary. That's a good idea, but doesn't solve the problem. In Cook's talk he says "*repeated experience with successful operations leads us to believe that the margin is too conservative, that we have space in there that we could use*". We want to use that space, because that allows us to optimize both for economics (getting further from our economic failure boundary) and engineering effort (getting further from our effort boundary). The response to this is organizational pressure to shrink the margin.

On the other hand, growing the margin, or at least the perceived margin, doesn't necessarily increase safety:

> ... drivers tend to try to keep their arousal at a desired, constant level and, consequently, go faster if conditions become too undemanding. ... traffic safety is hard to improve beyond a certain limit.

Rasmussen cites [Taylor](http://www.tandfonline.com/doi/abs/10.1080/00140138108924870?journalCode=terg20#preview) to provide evidence of *risk homeostasis* in traffic safety. This effect suggests that people (and organizations) will push the limits of safety to a certain perceived level, and increasing perceived safety will encourage risky behavior. While Rasmussen cites some data for this, and it has been suggested by many others, it is hard to reconcile some of these claims with the evidence that real-world traffic safety has been improved significantly since these claims were made. Whether risk compensation behavior exists, and the extent to which is contributes to a *risk homeostasis* effect, appears to be an area of active research. In skiing, for example, there [does not appear to be a significant risk compensation effect](http://journals.lww.com/epidem/Fulltext/2012/11000/Does_Risk_Compensation_Undo_the_Protection_of_Ski.35.aspx) with helmet use. Still, this effect may be a significant one, and should be considered before attempting to widen the perceived safety margin of a system.

Another way to move the operating point away from the safety boundary is to shift it in a big discontinuity after accidents occur. Using accidents as a driver away from the safety boundary is difficult for three reasons. The biggest one is that it requires accidents to occur, and that no feedback is provided between accidents. This *wait until you crash before turning the corner* approach can have very high costs, especially in safety and life critical systems. Another difficulty is the effect of the [availability heuristic](http://en.wikipedia.org/wiki/Availability_heuristic), our natural tendency to discount evidence that is difficult to recall. The longer it has been since an accident occurred, the smaller the push it provides away from the safety boundary. The third difficulty is that investigating accidents is really hard, and knowing which direction to move the operating point relies on good investigations. Simple *it was human error* conclusions are unlikely to move the operating point in the right direction.

Is all lost? No, but to make progress we may need to change the way that we think about measuring the safety of our systems. [Cook and Nemeth](http://www.ctlab.org/documents/Cook%20and%20Nemeth-Observations%20of%20the%20Usefulness%20of%20Error.pdf) make a distinction between those at the *sharp end* (operators) and those at the *blunt end* (management).

> Those who are closest to the blunt (management) end are most remote from sharp end operations and are concerned with maintaining the organization, and threats to the organization are minimized by casting adverse events as anomalies.

Instead of treating crossings of the safety boundary as anomalies, we should incorporate more feedback from the *sharp end* into the process that chooses the system operating point. This sharp-end gradient, mostly supplied by operators of complex systems, can provide a valuable third gradient (along with the gradients towards least effort and towards efficiency). The advantage of this approach is that it is continuous, in that it doesn't rely on big accidents and investigations, and adaptive, it constantly measures the local position of the safety boundary and provides a slope away from it. Getting this right requires constant attention from operators, and requires a conscious decision to include operators in the decision making process.