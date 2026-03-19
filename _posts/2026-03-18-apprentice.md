---
layout: post
title: "Music To Build Agents By"

related_posts:
  - "/2026/01/12/agent-box.html"
  - "/2025/09/18/firecracker.html"
  - "/2018/06/20/littles-law.html"
dissimilar_posts:
  - "/2013/01/06/volatile.html"
---
{{ page.title }}
================

<p class="meta">I don't have this problem, because I don't use a mouse.</p>

Press play, then start reading:

<iframe width="560" height="315" src="https://www.youtube-nocookie.com/embed/40xhyXscddY?si=P4Vm3Ol3IhwNKlUj&amp;start=540" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

Want to learn how to think about agent policy? Start with Goethe's [Der Zauberlehrling](https://oxfordsong.org/song/der-zauberlehrling).

> So come along, you old broomstick!
> Dress yourself in rotten rags!
> You've long been a servant;
> Obey my orders now!

When I talk to customers and teams around me about agents and agent policy, and the work we're doing on [AgentCore Policy](https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/policy.html) (now [GA](https://aws.amazon.com/about-aws/whats-new/2026/03/policy-amazon-bedrock-agentcore-generally-available/)) and [Strands Steering](https://strandsagents.com/docs/user-guide/concepts/plugins/steering/), I hear a lot of folks worried about adversarial agents, about prompt injection, and about hallucinations. That's not unreasonable, because all those things exist, and are worth paying attention to. But the most common problem is a more basic one, more Fantasia than James Bond. 

AI agents are persistent problem solvers. You ask them to solve a problem, and they'll go to work solving the problem.

> Look, he's running down to the bank;
> In truth! He's already reached the river,
> And back he comes as quick as lightning
> And swiftly pours it all out. 

That's exactly what makes agents powerful. If we knew how to solve the problem as a fixed workflow, we probably wouldn't bother with an agent. Workflows are faster, cheaper, and simpler. We build agents because they're persistent, because they handle edge cases, because they can adapt to changing circumstances and work around problems.

And this is also why they need policy (and should be [in a box](https://brooker.co.za/blog/2026/01/12/agent-box.html)).

> Alas! speedily he runs and fetches!
> If only you were a broom as before!
> He keeps rushing in
> With more and more water,
> Alas! a hundred rivers
> Pour down on my head! 

Policy layers like AgentCore Policy and structured steering like Strands Steering allow us to define limits on the agent's behavior. They allow us to make sure that agents stop when the basin is full, and to avoid pouring water all over the floor. That's important even if your agent is insulated from adversaries, and if your model is free from hallucinations. In fact, it becomes more and more important as models become more powerful, and able to solve longer-running problems.

Now, jump ahead to here:

<iframe width="560" height="315" src="https://www.youtube-nocookie.com/embed/40xhyXscddY?si=P4Vm3Ol3IhwNKlUj&amp;start=650" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

> Ah, my master comes at last!
> Sir, I'm in desperate straits!
> The spirits I summoned -
> I can't get rid of them.