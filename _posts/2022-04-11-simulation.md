---
layout: post
title: "Simple Simulations for System Builders"





related_posts:
  - "/2022/06/02/formal/"
  - "/2018/06/20/littles-law/"
  - "/2024/04/17/formal/"
---
{{ page.title }}
================

<p class="meta">Even the most basic numerical methods can lead to surprising insights.</p>

It's no secret that I'm a big fan of formal methods. I use [P](https://github.com/p-org/P) and [TLA+](https://lamport.azurewebsites.net/tla/tla.html) often. I like these tools because they provide clear ways to communicate about even the trickiest protocols, and allow us to use computers to reason about the systems we're designing before we build them<sup>[1](#foot1)</sup>. These tools are typically focused on safety (*Nothing bad happens*) and liveness (*Something good happens (eventually)*)<sup>[2](#foot2)</sup>. Safety and liveness are crucial properties of systems, but far from being all the properties we care about. As system designers we typically care about many other things that aren't strictly safety or liveness properties. For example:

 - What latency can customers expect, on average and in outlier cases?
 - What will it cost us to run this service?
 - How do those costs scale with different usage patterns, and dimensions of load (data size, throughput, transaction rates, etc)?
 - What type of hardware do we need for this service, and how much?
 - How sensitive is the design to network latency or packet loss?
 - How do availability and durability scale with the number of replicas?
 - How will the system behave under overload?

 The formal tools we typically use don't do a great job of answering these questions. There are many ways to answer them, of course, from closed-form analysis<sup>[3](#foot3)</sup> to prototyping. One of my favorite approaches is one I call *simple simulation*: writing small simulators that simulate the behavior of simple models, where the code can be easily read, reviewed, and understood by people who aren't experts on simulation or numerical methods.

 **A Quick Example**

 If you hang around with skiers or snowboarders, you'll have heard a lot of talk over the last couple of winters about how crowded resorts have become, and how much time they now spend waiting to ride the ski lift<sup>[4](#foot4)</sup>. Resort operators say that visits have been up only quite modestly, but skiers are seeing much longer waits. Is somebody lying? Or could we see significant increases in wait times with only modest increases in traffic?

 To help explore this question, I wrote a small [example simulator in Python](https://github.com/mbrooker/simulator_example) which you can check out.

 It starts off by building a model of each skier, who can be in one of three states: skiing down the hill, queuing, or riding up on the lift:

          +-------------------------------------------+       
          |                                           |       
          v                                           +       
    +-------------+      +-------------+      +-------------+
    |   Waiting   |----->| Riding Lift |----->|   Skiing    |
    +-------------+      +-------------+      +-------------+

Then, it models the chair fairly explicitly, pulling folks from the queue and delivering them to the top of the mountain after a delay. Each skier, lift, and slope creates some events, which the simulation simply reacts to in virtual time order. The whole thing comes out to about 170 lines, with loads of comments.

That's simple enough, but can we learn anything from it?

It turns out that, despite the extreme simplicity of the model, the results are interesting and run a little bit counter to our intuition. From example, here's the result showing the percentage of time each skier spends skiing, versus the number of virtual skiers in our simulation:

![](https://mbrooker-blog-images.s3.amazonaws.com/ski_percent_time.png)

I suspect that most people's intuition would have this as a fairly linear relationship, and the pronounced *knee* in the curve would be a surprise. I don't know what the realities are of ski resort attendance, but these simulations do suggest that its plausible that small increases in attendance could lead to long wait times.

As another example, my [post on Serial, Parallel and Quorum Latencies](https://brooker.co.za/blog/2021/10/20/simulation.html) is powered by a simple simulator.

It's exactly these kinds of small insights that bring me back to building small simulators over and over.

**How do I get started?**

Start simply. You can use any programming language you like (I tend to reach for Python first), don't need to learn any frameworks or libraries (although there are some good ones), and often don't have to write more than a few tens of lines of code. The coding side, in other words, is relatively easy.

The hard part is *modeling*. Simply, coming up with an abstract model of your system and its actors, and choosing what to include and what to exclude. What's important and what's irrelevant. What's the big picture, and what's detail. The success of simulations of all sizes depends on making good choices here. 

Think about the ski lift example. I modeled skier speed variations, and lift speed variations, and the periodic arrival of chairs. I didn't model weather, or fatigue, or lunch time, or any one of many other factors that could change the result. Are those important? Maybe! But to answer our core question ("is it plausible that small increases in visits could lead to long increases in waiting?") it didn't seem like we needed to include them.

Then, when you have the model, convert it to code. I like to do this as literally and straightforwardly as possible. It's very attractive to build in some abstraction that simplifies the code at the cost of obscuring the model. I avoid that as much as possible: being able to correlate the model and the code seems important to helping other people understand the assumptions. Our goal is to make the model and its assumptions obvious, not obscured.

Finally, explore and test. Play with the parameters and see what happens. Compare your intuition to the results. Look at the data coming out of the simulation. Try simple cases and check if they match. Validate against real systems if you can. How much effort to spend here depends a lot on how much is riding on the simulation being exact, but at least some validation is always warranted.

**But what about....**

Simple simulations aren't the last word in computational or numerical methods. You can write simulations that are arbitrarily sophisticated, very carefully validated, and exquisitely crafted. Depending on what you're trying to do that may be worth the effort. But I've seen a lot of people avoid reaching for simulation at all under the assumption that they have to be sophisticated. Often, you don't. In the majority of cases I've seen, the results are robust, validation is fairly simple, and simplicity beats sophistication. Don't let the depth of the field dissuade you from getting started.

**Footnotes**

 1. <a name="foot1"></a> I was also one of the authors on [How Amazon Web Services Uses Formal Methods](https://cacm.acm.org/magazines/2015/4/184701-how-amazon-web-services-uses-formal-methods/fulltext) which appeared in CACM back in 2015. Also check out the introduction/framing Leslie Lamport wrote in the same issue: [Who Builds a House Without Drawing Blueprints?](https://cacm.acm.org/magazines/2015/4/184705-who-builds-a-house-without-drawing-blueprints/fulltext)
 2. <a name="foot2"></a> These succinct descriptions of safety and liveness come from [Defining Liveness](https://www.cs.cornell.edu/fbs/publications/DefLiveness.pdf) by Alpern and Schneider, which is well worth reading if you're interested in going deeper on what liveness means.
 3. <a name="foot3"></a> For example, modelling the durability of replicated and erasure-coded storage systems can be done fairly easily in closed-form (see, for example [Notes on Reliability Models for Non-MDS Erasure Codes](https://dominoweb.draco.res.ibm.com/reports/rj10391.pdf)). The benefit is that the models are nice and clean and can be thrown in a spreadsheet. The downside is that they get complex quickly when you try include things like non-MDS erasure codes, correlated failure, and so on. The messy realities of life complicate modelling.
 4. <a name="foot4"></a> Often this increase in traffic has been blamed on lower pass or ticket prices, which seems reasonable to believe. On the other hand, the same folks often complain about how expensive skiing has become. Clearly, the sport is both too cheap and too expensive, a real challenge for resort operators!