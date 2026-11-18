---
layout: post
title: "What does it mean to understand what we built?"

---
{{ page.title }}
================

<p class="meta">Let's try come to an understanding.</p>

Today, a couple of people sent me the [No AI Slop Manifesto](https://www.slopmanifesto.com/). I found it interesting, in part, because it touches on something that seems super important right now: *understanding*.

> Through this work, we have come to value:
> ...
> Understanding over speed

I fully agree with that as a pithy statement, but I think we are working with different definitions of the word *understanding*. For example:

> We do our own thinking when working with a model. We explore resources, code, and prior art to truly understand the problem before the work starts.

That's an example of a symptom of something that I think makes this document substantially less useful and forward-looking than it could have been. Specifically, it privileges *code*. That's natural for programmers, of course, because program text has been somewhat privileged in our field's thinking for something like 60 years.

I believe that the goal of understanding code is, longer-term, holding us back from understanding what matters. I'm excited about the possibilities of what we can understand better when we are freed from understanding code, and can focus on other things.

What are those other things? The properties of the as-built system. Precise *properties* like "a SELECT never shows results from uncommitted transactions". And fuzzy properties like "makes the user's life easier". Those are already non-local properties that come from the code as a whole, not from any individual line. In fact, trying to understand how they are synthesized out of individual lines is a mammoth task! One that we've spent six decades coming up with abstractions to handle.

Then there are the really emergent properties. The dynamic behaviors that only come from the system all working together. Like [metastable failures](https://brooker.co.za/blog/2021/05/24/metastable.html). As with the [Lorenz Attractor](https://en.wikipedia.org/wiki/Lorenz_system) or [Julia Set](https://en.wikipedia.org/wiki/Julia_set) it's generally intractable to extract an understanding of the behavior of these systems from the code. We need different, better, more powerful tools.

Privileging code is, I believe, getting in the way of us really understanding what those tools should look like. The ones that help us really help understand the behavior of a system overall. Software engineering hasn't developed its [Bode plots](https://en.wikipedia.org/wiki/Bode_plot) or [Nyquist stability criterion](https://en.wikipedia.org/wiki/Nyquist_stability_criterion) yet. Or many equivalents to the kinds of tools that other fields of engineering have developed to understand complex systems as built.

We're still looking at each screw, each grain of aggregate, and each PCB trace, and trying to understand the whole by thinking super hard about it. It's time to look up. And AI gives us an incredible tool to do just that.

I like the manifesto. I just they didn't entirely think through what it means to be *for understanding* rather than *against not understanding*.