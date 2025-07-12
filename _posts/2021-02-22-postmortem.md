---
layout: post
title: "Incident Response Isn't Enough"




related_posts:
  - "/2016/01/03/correlation"
  - "/2019/06/20/redundancy"
  - "/2018/02/25/availability-liveness"
---
{{ page.title }}
================

<p class="meta">Single points of failure become invisible.</p>

Postmortems, COEs, incident reports. Whatever your organization calls them, when done right they are a popular and effective way of formalizing the process of digging into system failures, and driving change. The success of this approach has lead some to believe that postmortems are the *best*, or even *only*, way to improve the long-term availability of systems. Unfortunately, that isn't true. A good availability program requires deep insight into the design of the system.

To understand why, let's build a house, then a small community.

![A house, with four things it needs to be a working home](https://mbrooker-blog-images.s3.amazonaws.com/avail_slide_1.png)

Our house has four walls, a roof, and a few things it needs to be a habitable home. We've got a well for water, a field of corn for food, a wood pile for heat, and a septic tank. If any one of these things is not working, let's say that the house is *unavailable*. Our goal is to build many houses, and make sure they are unavailable for as little of the time as possible.

When we want to build a second house, we're faced with a choice. The simple approach is just to stamp out a second copy of the entire house, with it's own field, wood, well, and tank. That approach is great: the failure of the two houses is completely independent, and availability is very easy to reason about.

![Two houses, with full redundancy](https://mbrooker-blog-images.s3.amazonaws.com/avail_slide_2.png)

As we scale this approach up, however, we're met with the economic pressure to share components. This makes a lot of sense: wells are expensive to drill, and don't break down often, so sharing one between many houses could save the home owners a lot of money. Not only does sharing a well reduce construction costs, but thanks to the averaging effect of adding the demand of multiple houses together, reduces the peak-to-average ratio of water demand. That improves ongoing economics, too.

![Five houses, sharing a well](https://mbrooker-blog-images.s3.amazonaws.com/avail_slide_3.png)

In exchange for the improved economics, we've bought ourselves a potential problem. The failure of the well will cause all the houses in our community to become *unavailable*. The well has high *blast radius*. Mitigating that is well-trodden technical ground, but there's a second-order organizational and cultural effect worth paying attention to.

Every week, our community's maintenance folks get together and talk about problems that occurred during the week. Dead corn, full tanks, empty woodpiles, etc. They're great people with good intentions, so for each of these issues they carefully draw up plans to prevent recurrence of the issue, and invest the right amount in following up on those issues. They invest in the most urgent issues, and talk a lot about the most common issues. The community grows, and the number of issues grows. The system of reacting to them scales nicely.

Everything is great until the well breaks. The community is without water, and everybody is mad at the maintenance staff. They'd hardly done any maintenance on the well all year! It wasn't being improved! They spent all their attention elsewhere! Why?

The problem here is simple. With 100 houses in the community, there were 100 fields, 100 tanks, 100 piles, and one well. The well was only responsible for 1 in every 301 issues, just 0.33%. So, naturally, the frequency-based maintenance plan spent just 0.33% of the maintenance effort on it. Over time, with so little maintenance, it got a little creaky, but was still only a tiny part of the overall set of problems.

![Plot showing how the percentage of action items related to the well drops with scale](https://mbrooker-blog-images.s3.amazonaws.com/avail_slide_4.png)

This is one major problem with driving any availability program only from postmortems. It feels like a data-driven approach, but tends to be biased in exactly the ways we don't want a data-driven approach to be biased. As a start, the frequency measurement needs to be weighted based on impact. That doesn't solve the problem. The people making decisions are human, and humans are bad at making decisions. One way we're bad at decisions is called the [Availability Heuristic](https://en.wikipedia.org/wiki/Availability_heuristic): We tend to place more importance on things we can remember easily. Like those empty wood piles we talk about every week, and not the well issue from two years ago. Fixing this requires that an availability program takes *risk* into account, not only in how we measure, but also in how often we talk about issues.

It's very easy to forget about your single point of failure. After all, there's just one.