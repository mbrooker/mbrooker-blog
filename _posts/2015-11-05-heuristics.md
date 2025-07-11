---
layout: post
title: "Heuristic Traps for Systems Operators"


related_posts:
  - "/2014/06/29/rasmussen"
  - "/2022/11/08/writing"
  - "/2024/03/25/needles"
---{{ page.title }}
================

<p class="meta">What can we learn from avalanche safety?</p>

Powder magazine's new feature [The Human Factor 2.0](http://features.powder.com/human-factor-2.0/chapter-1) is a fantastic read. It's a good disaster story, like the New York Times' [Snow Fall: The Avalanche At Tunnel Creek](http://www.nytimes.com/projects/2012/snow-fall/#/?part=tunnel-creek), but looks deeply at a very interesting topic: the way that we make risk decisions. I think there are interesting lessons there for operators and designers of computer systems.

> “That’s the brutal thing,” Donovan said. “It’s hard to get experience without exposing yourself to risk.”

The consequences of making bad decisions in back-country skiing can be life and death. As designers, builders and operators of systems of computers our bad decisions are generally less dramatic. But they are real. Data loss, customer disappointment, and business failure have all come as the results of making poor risk decisions. One way to mitigate those risks is experience, and it's product: intuition. Unfortunately, as David Page's article reminds us, intuition isn't always the route to safety.

> The difficulty, he explained, especially when it comes to continuing education for veteran mountain guides and other professionals, is breaking through a culture of expertise that is based on savoir-faire - in other words, on some deep combination of knowledge and instinct derived from experience - especially if that experience may happen to include years of imperfect decisions and sheer luck. “When we talk about human behavior, people feel a bit attacked in their certainty, in their habits"

The problem with intuition is exactly that it is not the product of conscious thought. Intuition is [Daniel Kahneman's](http://www.amazon.com/Thinking-Fast-Slow-Daniel-Kahneman/dp/0374533555) System 1: effortless, implicit, and based on heuristics rather than reason. It's well documented that these heuristics serve us very well in many situations, especially when speed of movement is important above all else, but they don't lead to us making good decisions in the long term.

Ian McCammon's [Evidence of heuristic traps in recreational avalanche accidents](http://avalanche-academy.com/uploads/resources/Traps%20Reprint.pdf) is great evidence of this effect in practice. He presents four heuristics that appear to correlate with snow sports accidents:

 - The familiarity heuristic, or taking increased risks in familiar places or situations.
 - The social proof heuristic, or taking increased risks because others are doing it.
 - The commitment heuristic, or taking increased risks because we want to appear consistent with our words, keep our promises, or feel committed to the situation. Here, we're *stepped in so far*.
 - The scarcity heuristic, or taking increased risks to take advantage of limited resource or opportunities.

McCammon presents evidence that the *social proof*, *commitment* and *scarcity* heuristics do correlate with avalanche deaths. Even more interesting is the effect of experience and training. The *familiarity* and *social proof* heuristics correlate most strongly in those with advanced training in avalanche safety. The strength of the *familiarity* heuristic's effect is remarkable: advanced training appears to lead to clearly better decisions in unfamiliar situations, but equal or worse quality decisions in familiar situations.

This all applies to the decisions that systems operators make every day. In systems operations, much like in avalanche safety, there is a strong familiarity heuristic. In my experience, operators don't tend to reflect on the safety of operations they are familiar with as much as unfamiliar operations. This is logical, of course, because if we stopped to think through every action we'd be immobile. Still, it's critical to reevaluate the safety of familiar operations periodically, especially if conditions change.

> First, most accidents happen on slopes that are familiar to the victims. While it’s likely that people tend to recreate more often on slopes they are familiar with, the high percentage of accidents on familiar slopes suggests that familiarity alone does not correspond to a substantially lower incidence of triggering an avalanche.

The social proof heuristic (and its buddy, appeal to authority) are also common to systems operators. Things that multiple people are doing are seen as safer, despite evidence to the contrary. Like the familiarity heuristic, this one makes some sense on the surface. Successful completion of tasks by others *is* evidence of their safety. What is irrational is overweighting this evidence.

> All of this suggests that the social proof heuristic may have some marginal value in reducing risk, but in view of the large number of accidents that occur when social proof cues are present it cannot be considered in any way reliable.

Finally, the commitment heuristic is our noble wish to be true to our word, to keep our promises and to get things done before its dark out. Committing to get work done is a very important social force - it's what operators are paid for, after all - but can lead to poor risk decisions. This is where I see very interesting overlaps with [Jens Rasmussen's model of risk management](http://brooker.co.za/blog/2014/06/29/rasmussen.html). The commitment heuristic aligns well with what Rasmussen describes as the "gradient towards least effort* in systems operations.

There is great value in looking at the way that people in unrelated fields makes risk decisions and exercise heuristics, because it allows us to use our *system 2* (in Kahneman's terminology) to train our *system 1*, and recognize where it might be leading us astray.