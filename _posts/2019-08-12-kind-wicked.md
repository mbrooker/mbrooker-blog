---
layout: post
title: "Kindness, Wickedness and Safety"






related_posts:
  - "/2014/06/29/rasmussen.html"
  - "/2019/06/17/chernobyl.html"
  - "/2021/02/22/postmortem.html"
---
{{ page.title }}
================

<p class="meta">We must build kind systems.</p>

David Epstein's book [Range: Why Generalists Triumph in a Specialized World](https://www.amazon.com/Range-Generalists-Triumph-Specialized-World/dp/0735214484) turned me on to the idea of Kind and Wicked learning environments, and I've found the idea to be very useful in framing all kinds of problems.<sup>[1](#foot1)</sup> The idea comes from [The Two Settings of Kind and Wicked Learning Environments](https://pdfs.semanticscholar.org/5c5d/33b858eaf38f6a14b3f042202f1f44e04326.pdf). The abstract gets right to the point:

> Inference involves two settings: In the first, information is acquired (learning); in the second, it is applied (predictions or choices). Kind  learning environments involve  close matches between the informational elements in the two settings and are a necessary condition for accurate  inferences. Wicked learning environments involve mismatches.

The authors go on to describe the two environments in terms of the information that we can learn from *L* (for learning), and information that we use when we actually have to make predictions *T* (for target). They break environments down into *kind* or *wicked* depending on how *L* relates to *T*. In kind environments, *L* and *T* are closely related: if you learn a rule from *L* it applies at least approximately to *T*. In wicked environments, *L* is a subset or superset of *T*, or the sets intersect only partially, or are completely unrelated.

Simplifying this a bit more, in kind environments we can learn the right lessons from experience, in wicked environment we learn the wrong lessons (or at least incomplete lessons).

From the paper again:

> If  kind,  we  have  the  necessary conditions for accurate inference. Therefore, any errors must be attributed to the person (e.g., inappropriate  information  aggregation).  If  wicked,  we  can  identify how error results from task features, although these can also be affected by human actions. In short, our  framework  facilitates  pinpointing  the  sources  of  errors (task structure and/or person).

This has interesting implications for thinking about safety, and the role of operators (and builders) in ensuring safety. In kind environments, operator mistakes can be seen as *human error*, where the human learned the wrong lesson or did the wrong thing. In wicked environments, humans will always make errors, because there are risks that are not captured by their experience.

Going back to [Anatoly Dyatlov's question to the IAEA](//brooker.co.za/blog/2019/06/17/chernobyl.html):

> How and why should the operators have compensated for design errors they did not know about?

Dyatlov is saying that operating Chernobyl was a wicked environment. Operators applying their best knowledge and experience, even flawlessly, weren't able to make the right inferences about the safety of the system.

Back to the paper:

> Since kind environments are a necessary condition for accurate judgments, our framework suggests deliberately creating kind environments.

I found reading this to be something of a revelation. When building safe systems, we need to make those systems *kind*. We need to deliberately create kind evironments. If we build them so they are *wicked*, then we set our operators, tooling and automation up for failure.

Some parts of our field are inherently wicked. In large and complex systems the set of circumstances we learn from is always incomplete, because the system has so many states that there's no way to have seen them all before. In security, there's an active attacker who's trying very hard to make the environment wicked.

The role of the designer and builder of systems is to make the environment as kind as possible. Extract as much wickedness as possible, and try not to add any.

## Footnotes

 1. <a name="foot1"></a> The book is worth reading. It contains a lot of interesting ideas, but like all popular science books also contains a lot of extrapolation beyond what the research supports. If you're pressed for time, the [EconTalk episode](http://www.econtalk.org/david-epstein-on-mastery-specialization-and-range/) about the book covers a lot of the material.