---
layout: post
title: Some Patterns of Engineering Design Meetings








related_posts:
  - "/2015/01/25/patterns.html"
  - "/2020/10/19/big-changes.html"
  - "/2024/04/17/formal.html"
dissimilar_posts:
  - "/2015/05/24/sodium-carbonate.html"
---
{{ page.title }}
================

<p class="meta">On discussing designs.</p>

I spend a lot of time in engineering design discussions and meetings, talking about how we are going to introduce new features, solve problems, and increase capacity. For me, a good design meeting is the highlight of any day - sharing ideas, comparing options and considering alternatives can be extremely rewarding. Good design meetings, unfortunately, aren't the rule. Frequently, meetings don't go anywhere. Earlier in my career, I thought that there is a direct correlation between the complexity of the problem to be solved and the productivity of the design meetings. It felt natural that solving harder problems would be more difficult.

Recently, though, I've been thinking about some of the things that go right and wrong in design meetings, and have come to the conclusion that there is very little correlation between problem complexity and meeting productivity. Instead, the problem typically seems to be that we are starting in the wrong place. To explain what I mean, let me start with the simplest design process in a single-person isolated world.

 1. Identify the current state of the system, *where you are*.
 2. Identify the goal state of the system, *where you are going*.
 3. Decide how you are going to get from the current state of the system to the goal state, *how do you get there*.

![](https://s3.amazonaws.com/mbrooker-blog-images/design_base.png)

Naively, it appears as though group design meeting should focus on step three (*how do we get there?*). The group considers a variety of different paths from *where we are* to *where we are going*, compares their merits, and chooses one of the alternatives. The focus is always on the design, which is what most engineers find most interesting. 

![](https://s3.amazonaws.com/mbrooker-blog-images/design_two.png)

Unfortunately, it is this naive belief - the belief that design discussions should focus on design - that derails most design discussions. The most common manifestation of this problem is when different stakeholders enter a design meeting with different goals in mind. Imagine trying to discuss driving directions with somebody who before agreeing on where you are going. "No", you'll say, "we have to turn *left* on 5th Avenue". "Absolutely not! *Right* on 5th!". "Left!". "Right!".

![](https://s3.amazonaws.com/mbrooker-blog-images/design_goal.png)

There are three forms of this problem that I have observed. One is the simple disagreement, where participants don't have the same goals in mind. All participants have concrete goals, but they aren't the same. The second form is when one or more of the participants haven't really thought about where they want to go. The destination isn't concrete in their head, so they can't meaningfully contribute to the discussion about the route. The third is a mismatch of models. In this case, the participants may agree on the goal, but don't agree on how they see the goal. I want to spend time outdoors, and my colleague wants to play a game of skill. We'd both be happy with a round of golf, but often we can't see that because we don't realize that we are looking at the problem from different angles.

Agreeing on goals can always take some time, but it's almost always time well invested. It's much more wasteful of time when you start discussion the design before you agree on goals. In that case, it becomes very unlikely that the discussion will find consensus, and even less likely that the consensus will have any real value. I've found that starting with goals and requirements is critical to a successful design meeting. This isn't an argument for being inflexible - goals should be allowed to evolve if the discussion of the solution shows them to be incorrect or inadequate - but an argument for explicit consensus.

The other end of the design arrow is just as important to agree on. Before meaningfully discuss directions not only do you need to agree on where you are going, but you need to agree on where you are now. At first glance, it appears obvious that everybody should agree on where we are now. The current state of the world appears to be a simple and concrete thing.

![](https://s3.amazonaws.com/mbrooker-blog-images/design_current.png)

Unfortunately that's not the case. At best, everybody brings a different perspective on the current state of the world to the discussion. Different experience, different levels of exposure to various pieces, and different job roles all contribute to different perspectives. Beyond this, however, is a deeper problem with common understanding. Every system designed by humans is designed to fit not into the real world, but to fit into a mental model of the real world. We don't have the capability to include all possible factors into a design, so we build a simplified mental model to fit the design into. Models necessarily differ between people. The most common forms of differences in mental model are disagreements about model fidelity, how important a factor needs to be before being included in the model, and factor importance, how important any given factor is.

I don't believe that it is possible, or even desirable, to completely match the models of different participants in design meetings. On the other hand, it is critical to spend some time understanding the differences between these models. They will always differ in detail, but need to be the same general shape for productive discussion.

So far, I've mostly written about these patterns of disagreement in context of meeting productivity. The much more serious issue is the issue of meeting outcomes. When we discuss design without agreeing on the current and goal states of the system, we run a high risk of making very bad design decisions. The outcome of a good design meeting is often a compromise, mixing various aspects of each proposal or idea into a coherent whole.

![](https://s3.amazonaws.com/mbrooker-blog-images/design_compromise.png)

The outcome of a bad design meeting is a false compromise, where various aspects of each proposal are mixed to make a design that doesn't match any model of the current world, and doesn't achieve anybodies goals. This is the most common cause of bad design I have seen in my career: mismatched goals and mismatched models leading to a non-solution. The best way, in my opinion, to avoid this mistake is to make the steps of finding consensus on *where we are* and *where we are going* explicit and upfront, and not being ashamed to loop back to them during the discussion of *how are we going to get there?*.

![](https://s3.amazonaws.com/mbrooker-blog-images/design_bad_compromise.png)