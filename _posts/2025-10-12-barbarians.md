---
layout: post
title: "What is the Scope of Systems Research?"

---
{{ page.title }}
================

<p class="meta">The Barbarian F.C. of systems research would be pretty cool.</p>

Lots of folks online have been talking about [Barbarians at the Gate: How AI is Upending Systems Research](https://arxiv.org/abs/2510.06189) by Cheng, Liu, Pan, et al this week. Maybe unsurprisingly, given the fact that I work in AI for my day job, and both consume and produce systems research, I found it super interesting.

Perhaps the most interesting discussion, however, isn't about AI at all. It's about the scope of systems research. What systems research *is*, or *aught to be*.

The paper's core argument is well captured in the abstract:

> We argue that systems research, long focused on designing and evaluating new performance-oriented algorithms, is particularly well-suited for AI-driven solution discovery. This is because system performance problems naturally admit reliable verifiers: solutions are typically implemented in real systems or simulators, and verification reduces to running these software artifacts against predefined workloads and measuring performance.

I, 100%, enthusiastically, agree with this point. I think it's a point that generalizes way beyond systems research to the entire software industry: AI is going to be most effective in problem spaces where there are what the authors call *reliable verifiers*. Where, in effect, we can do automated hill climbing towards a low-ambiguity solution or Pareto frontier of solutions. Much of the next decade is going to be defined by finding better techniques to build these *reliable verifiers* where none existed before. If you're a software engineer, what I'm saying here is *testing is going to be the most important thing*.

A bit later, the paper highlights two of the challenges with building these verifiers for systems builders (and software developers generally):

> Prevent overfitting. Evaluating against narrow workloads lead to algorithm failures like overfitting, where the solutions either hard-code behaviors or overfit to specific traces.

> Prevent reward hacking. Reward hacking occurs when solutions exploit evaluator loopholes rather than solving the intended problem.

These two pitfalls aren't new news to anybody who's built AI systems. They also shouldn't be new news to folks who read or write systems research. Overfitting to traces like benchmarks is a systemic problem in the systems world, and has been for decades. Benchmarks like TPC-C, which so poorly represent their target workloads as to be mostly meaningless, are ruthlessly overfit by both academic and commercial systems. Reward hacking is more of a meta problem, with excessive energy spent reward hacking the whole publishing system. A problem pre-AI, and a problem that the scalability of AI will no doubt exacerbate.

Then, right towards the end, the paper hits what I think is the most important point:

> As [AI optimization] tools begin to solve an increasing number of problems autonomously, researchers will have more time to focus on higher-leverage activities—selecting which problems to pursue and formulating them precisely.

In one sense, this is an exciting vision, because it says that researchers can spend more time on things that are much more important than mere optimization. *[S]electing which problems to pursue and formulating them precisely* is exactly the activity that, in my mind, separates good systems research from weak systems research. Altogether too much energy is spent on *number go up* research without sufficient context on whether the number matters. Hill climbing is only useful if we're climbing the right hill, or learning something useful about hills generally.

On the other hand, it's missing what I think is a major problem. Much of academic systems work already seems bottlenecked on *selecting which problems to pursue*. This appears to be two reasons. The short-term reason is disconnection from the problems that the customers of systems (in industry and the wider world) face. The longer-term, and more important, reason is that coming up with a vision for the future is just much harder than hill climbing. It takes more experience, more insight, and more vision to choose problems than to optimize on them. It takes more taste to reject noise, and avoid following dead ends, than to follow the trend.

Which leads systems to a tough spot. More bottlenecked than ever on the most difficult things to do. In some sense, this is a great problem to have, because it opens the doors for higher quality with less effort. But it also opens the doors for higher volumes of meaningless hill climbing and less insight (much of which we're already seeing play out in more directly AI-related research). Conference organizers, program committees, funding bodies, and lab leaders will all be part of setting the tone for the next decade. If that goes well, we could be in for the best decade of systems research ever. If it goes badly, we could be in for 100x more papers and 10x less insight.

I'll let the authors have the last word:

> If anything, [AI] will expand the research community by enabling individuals who may not be expert problem solvers to contribute meaningfully. The result will be a faster rate of progress—solving more problems, faster and better.
