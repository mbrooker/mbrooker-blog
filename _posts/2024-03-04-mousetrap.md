---
layout: post
title: "The Builder's Guide to Better Mousetraps"






related_posts:
  - "/2020/10/19/big-changes.html"
  - "/2022/01/31/deployments.html"
  - "/2024/06/04/scale.html"
---
{{ page.title }}
================

<p class="meta">A little rubric for making a tough decision.</p>

*Some people who ask me for advice at work get very long responses. Sometimes, those responses aren't specific to my particular workplace, and so I share them here. In the past, I've written about [writing](https://brooker.co.za/blog/2022/11/08/writing.html), [writing for an audience](https://brooker.co.za/blog/2023/09/21/audience.html), [heuristics](https://brooker.co.za/blog/2022/12/15/thumb.html), [getting big things done](https://brooker.co.za/blog/2020/10/19/big-changes.html), and [how to spend your time](https://brooker.co.za/blog/2024/02/06/time.html). This is another of those emails.*

So, you're thinking of building a new thing. It's going to be a lot like that other thing that already exists. In fact, it seems so similar that lots of folks are asking you why you're building a new thing rather than using that existing thing, or maybe adapting that existing thing to your needs. Those people have the right general instincts&mdash;rebuilding a thing that already exists and works is seldom a good bet&mdash;and you have other important things to do. On the other hand, you seem convinced that your thing will be better in important ways. You also point to a long history of innovation where folks had similar doubts. That, too, is correct. After all, Newcomen and Savery's steam engines weren't much better than what they replaced (and ended up being a pretty big deal).

I tend to be biased towards innovation. Towards building. I think most advice for technical leaders over-emphasizes the short-term risks of innovating too much, and under-emphasizes the long-term risks of innovating too little. However, both sides have good points, and we owe it to ourselves and our businesses to think carefully about the decision. Because of my bias, I force myself to deeply question my motivations when making decisions like this.

Its worth mentioning that this thinking is related to, but distinct from, the classic *build vs buy* decision. The thing you want, the thing you really need, doesn't seem available to buy.

Here are some questions that are worth asking yourself as you make this decision (then [write down your answers](https://brooker.co.za/blog/2022/11/08/writing.html)):

* *Is the juice worth the squeeze?* If this works out just like you plan, and your new thing is just as good as you think its going to be, will the benefit of that outweigh the cost? Don't even think about risk yet&mdash;just the optimistic position of perfect delivery. What's the cost and what's the benefit? Answering this question well is going to require you to think like a business owner. What are the dollar and cent costs of doing this? What are the opportunity costs (i.e. people or other resources that could be doing something else)? What are the direct business benefits of doing this (reduced costs, increased revenue, increased capacity, etc)? What are the indirect benefits (improved growth, etc)?

* *What could I be doing instead?* This is the *opportunity cost* of building. In any organization with constrained resources, you need to be thoughtful about how you spend those resources. For any single human, resources are necessarily constrained, and so you need to [be thoughtful about how you spend your time](https://brooker.co.za/blog/2024/02/06/time.html). What will doing this distract you or your organization from doing? If you do this instead of that, will you be in a better place long-term?

* *Do I want to own this?* Building something comes with long-term operational ownership. Often, the majority of the cost of building something is in long-term operations and maintenance. Do you continue to accrue benefits by being able to tailor the way you run the system to your needs, or does owning it become pure drag the moment initial building is finished? How much of a distraction is owning it? This is the long-term version of the first question: to get the juice, you've gotta keep squeezing. Forever.

* *Am I solving a simpler problem?* One way to do much better than existing solutions is to solve an easier problem than they do. NoSQL is a great example. Instead of solving the hard problems of performance and scale for joins and transactions, the first generation of NoSQL databases simply didn't solve those problems. For some workloads, this lead to transformatively better scalability and performance. [Firecracker](https://www.usenix.org/conference/nsdi20/presentation/agache) is another example. Much of Firecracker's innovation is around problems it *doesn't* solve, which enabled us to use it to solve a set of hard problems that other solutions couldn't<sup>[1](#foot1)</sup>. Specialization is a powerful way to improve performance and reduce costs.

* *Is my problem different?* Sometimes, two things that look very similar but solve different problems. Screwdrivers and chisels look similar, but we don't use them for the same thing. How is the task you're doing different from the task the existing thing does? How does that change the properties of the solution, and how much?

* *Is my scale different?* A [$20 bottle capper](https://www.amazon.com/FastRack-Absorbing-Standard-Homebrew-Bottles/dp/B001D6KGTK/), and a [bottling plant](https://www.comacgroup.com/bottling-plants/) do the same thing, at different scales. One is the right solution for a home brewer, the other is the right solution for a commercial brewery. If either use was trying to use the wrong tool, they'd have a bad time, waste a lot of money, and probably not get the job done at all. Differences in scale, if large enough, can lead to qualitative difference in solution complexity. For example, when we were doing [Physalia](https://www.usenix.org/conference/nsdi20/presentation/brooker) we had a much bigger scale than alternatives (like Zookeeper) in the number of clusters, but a much smaller scale per cluster (and, to the previous question, we were also trying to solve a blast radius problem that most other solutions just weren't thinking about at all).

* *Do I understand what's difficult?* I see two common failure modes when folks make decisions like this, diametrically opposed. In one, folks stand outside a field or problem space they don't know yet, and don't see the difficulties. They rush in, and go nowhere, because the mud is deep and soft and getting stuck is easy. In the other, folks see only the difficulties and never take bold bets. The only remedy, really, is to learn as much as is feasible about the space ahead of making the decision, and be clearly able to articulate why your simplifying assumptions or constraints mean you'll make faster progress than others<sup>[2](#foot2)</sup>. Reading a couple books and papers to get a flavor of the problems in the field is often a good idea.

* *What's the technical risk?* If you execute poorly on this, for whatever reason, what's the risk to your customers, your business, and your team? 

* *When will we know?* There are places, like cryptography and distributed replication, where it's sometimes hard to even know if you've been successful until its too late. Innovation in those spaces is riskier. There are other places where it's pretty easy to know as you go along whether you're succeeding. What are your milestones? At each milestone, how will you know whether you're actually getting close to a solution?

* *Do I have a technological advantage?* In other words, is there some technology or tool available to you other solutions can't, or don't, use. The exponential curve of compute and storage scaling has unlocked a vast amount of this kind of innovation over the last decade, both in computing itself and in other areas. Examples include today's AI and ML techniques, modern statistical techniques, in-memory databases, LED light bulbs, diode-based laser engravers, and many many more<sup>[3](#foot3)</sup>. Tons of stuff built against the cloud, too.

The decision you're trying to make here isn't really a [one way door](https://www.youtube.com/watch?v=rxsdOQa_QkM). But the decision to build here is expensive, risky, and comes with a significant chance it'll distract you and your team from more important things. On the other hand, it could mean that you get much better performance or cost or flexibility. This is the kind of decision its easy to be wrong about. So I'd suggest you answer my questions, and write down your answers. Then sit down with some smart people you trust and see if they believe your conclusions.

**Footnotes**

1. <a name="foot1"></a> Couldn't solve *at the time*. One of the really satisfying things for me about Firecracker is how much it's turned out to be a catalyst for innovation in this space.
2. <a name="foot2"></a> If your answer is *we're smarter* or *we'll work harder* or variants of that, then you need to deeply examine your assumptions. Maybe you are, maybe you will. Or maybe you're not, or won't, or can't sustainably.
3. <a name="foot3"></a> There are also some interesting, thought-provoking counter-examples. For example, I would have expected the rise of SSDs (and the 1000x step change in random IO latency that came with it) to lead to a new generation of database engines dominating, built from the ground up to take advantage of the hardware. Seemingly against the odds, PostgreSQL (and MySQL and others to a lesser extent) is as dominant as ever, despite being built for the steampunk age of spinning media.