---
layout: post
title: "Software Deployment, Speed, and Safety"



related_posts:
  - "/2024/03/04/mousetrap"
  - "/2024/06/04/scale"
  - "/2024/04/17/formal"
---{{ page.title }}
================

<p class="meta">There's one right answer that applies in all situations, as always.</p>

*Disclaimer: Sometime around a 2015, I wrote AWS's official internal guidance on balancing deployment speed and safety. This blog post is not that. It's not official guidance from AWS (nothing on this blog is), and certainly not guidance for AWS. Instead, it's my own take on deployments and safety, and how I think about the space.*

You'll find a lot of opinions about deployments on the internet. Some folks will say that teams have to be able to deploy from commit to global production in minutes. Others will point out that their industry has multi-year product cycles. It's a topic that people feel strongly about, and for good reason. As usual with these kinds of topics, most of the disagreement doesn't come from actual disagreement, but from people with wildly different goals and tradeoffs in mind. Without being explicit about what we're trying to achieve, our risk tolerance, and our desired reward, it's impossible to have a productive conversation on this topic. This post is an attempt to disentangle that argument a little bit, and explain my perspective.

That perspective is clearly focused on the world I work in - offering cloud-based services to large groups of customers. Some of that applies to software more generally, and some applies only to that particular context. I have also used the word *deployment* here to stand in for all production changes, including both software and configuration changes, and the actions of operators in general.

**Tradeoffs exist**

In my experience, software teams are happiest when they're shipping code. That could mean code to production, or to testing, or to validation, but nothing seems to destroy the morale of a team quite a surely as making changes with no end date in sight. Folks want to see their changes have an impact on the real world. It's what I like to see too. Shipping often also sometimes means shipping smaller, better understood, increments, potentially increasing safety. Speed and agility are also important for reliability and security. Flaws in systems, whether in our software or the software, firmware, and hardware it's built on, are an unfortunate fact of working on complex systems. Once flaws are found, it's important to be able to address them quickly. Especially so when it comes to security, where an *adversary* may learn about flaws at the same time we do. Businesses also want to get changes in the hands of customers quickly - after all, that's what most of us are doing here. Customers want new features, improvements, better performance, or whatever else we've been working on. And, for the most part, they want it *now*.

These factors argue that faster is better.

Balancing the need for speed is risk. Let's ignore, for the rest of this post, the risk of developing software fast (and, presumably, skipping out on testing, validation, careful deployment practices, etc) and focus only on the act of deployment. Getting changed software out into production. Clearly, at some level, deploying software reduces risk by giving us an opportunity to address known flaws in the system. Despite this opportunity to improve, deployment brings risk, introduced both by the act of deploying and by the fact that new software is going out to meet the world for the first time. That new software is tested and validated, of course, but the real world is more complex and weirder than even the most ambitious testing program, and therefore will contain new flaws.

I'm going to mostly ignore the risks of the act of deploying. Clare Liguori wrote [a great post for the Amazon Builder's Library on that topic](https://aws.amazon.com/builders-library/automating-safe-hands-off-deployments/?did=ba_card&trk=ba_card), and the state-of-the-art of technological and organizational solutions. I won't repeat that material here.

Even in a world where getting software out to production is perfectly safe, new software has risks that old software doesn't. More crucially, new components of old systems introduce change that may lead to emergent changes in the behavior of the entire system, in ways that prove difficult to predict. New features and components add complexity that wasn't there before. Even new performance improvements may have unexpected consequences, either by introducing new cases where performance is unexpectedly worse, or by moving the bottlenecks to somewhere they are less visible, or by introducing instability or metastability into the system.

Deploying software incrementally - to some places, customers, or machines - helps contain this risk. It doesn't reduce the probability that something goes wrong, but does reduce the blast radius when something does. Deploying incrementally only works *over time*. You need to allow enough time (measured in hours, requests, or both) to pass between steps to know if there is trouble. Monitoring, logging, and observability are needed to make that time valuable. If you're not looking for signs of trouble, you're wasting your time.

There's a tradeoff between speed and safety.

**Time finds problems, people fix them**

There are two hidden assumptions in the section above: problems happen (or become visible) some time after deployment, and that time is short enough that waiting between deployments will catch a significant proportion of problems. The first assumption seems true enough. Trivial problems are often caught in testing, especially integration testing, so what's left is more subtle things. It can take some time for a subtle signal in error rates or logs to become visible. The second is less obvious. If problems are caused by state or customer behavior that only exist in production but not in testing, then we may expect them to show themselves fairly quickly. More subtle issue may take longer, and longer than we're willing to wait. For example Roblox recently had a long outage triggered by a long latent design issue, [for reasons they describe in an excellent postmortem](https://blog.roblox.com/2022/01/roblox-return-to-service-10-28-10-31-2021/).

These assumptions lead to three of the biggest controversies in this space: how fast to move, how to measure speed, and the Friday question.

How fast we move, or how long we wait between increments, is a question that needs to be answered in two ways. One is driven by data on how long it takes to detect and react to real in-production software problems. That's going to depend a great deal on the system itself. The second answer has to come from customer expectations and promises, which I'll touch on later. The second controversy is how to measure speed. Do we measure in terms of wall-clock time, or requests, or coverage? The problem with wall-clock time is that software problems don't tend to be triggered just by time, but by the system actually doing work. So if you deploy and nobody is using it, then waiting doesn't help. Counting work, like requests, seems to be the obvious solution. The problem with that approach is that user patterns tend to be seasonal, and so you need to be quite specific about which requests to count (and doing that requires a level of foresight that may not be possible). Requests and coverage are also a little open-ended, which makes making promises somewhat difficult.

Then there's The Friday Question. This is typically framed two ways. One is that not deploying on Fridays and weekends is better because it respects people's time (because people will need to fix problems that arise), and better because folks being out for the weekend will increase time-to-recovery, and better because waking customer's oncalls up on Friday night is painful. The other framing is that avoiding deploying on Fridays is merely a symptom of bad practices or bad testing or bad tooling or bad observability leading to too much risk. A lot of the controversy comes from the fact that both of these positions are reasonable, and correct. Good leaders do need to constantly look out for, and sometimes avoid, short-term band-aids over long-term problems. On the other hand, our whole industry could do with being more respectful of people's lives, and doing the right thing by our customers is always the first goal. The passion on this question seems to be misguided.

**There are no take-backsies**

Rollback (taking a troublesome change out of production and replacing it with a previously known-good version) and *roll forward* (taking a troublesome change out of production and replacing it with a hopefully fixed new version) are important parts of managing deployment risk. In idealized stateless systems they might be all we need to manage that risk. Detect quickly if something bad is happening, and roll back. Limit the trouble caused by a bad deployment by limiting the time to recovery. Unfortunately, with stateful systems you can't take back some kinds of mistakes. Once state is corrupted, or lost, or leaked, or whatever, no amount of rolling back is going to fix the problem. That's obviously true of hard-state systems like databases and soft-state systems like caches, but also true of the softer and more implicit state in thing like queued requests, or inflight work, or running workflows or whatever. Rolling back requires fixing state, which may be impossible.

High quality validation and testing are a critical part of the way forward. Stateful systems need to have a different bar for software quality than many other kinds of systems. They also need to have deployment practices reflect the fact that one of the most powerful tools for risk management—rollback—simply doesn't work.

Immutable, append-only, or log-based systems are often cited as a solution to this problem. They may be, but rolling back your log causes your log to turn into a DAG, and that's a whole problem of its own.

**You can't talk about risk without talking about customers**

When we talk about the risk of a bad deployment, the only sensible way to do that is in context of the outcome. When I deploy changes to this blog, I do it live on the one production box. After all, if I mess up my blog may be broken, but nothing here is secret (the content is CC-BY) or particularly valuable.

But as systems get bigger and more critical the risk changes. A few minutes of downtime, or an unexpected behavior, for a critical cloud system could have a direct impact on millions of people, and an indirect impact on billions. The outcome is of mistakes is different, and therefore so is the risk. We can't rationally talk about deployment practices without acknowledging this fact. Any reasonable set of deployment practices must be based around both the benefits of moving fast, and the risk of quickly pushing out bad changes. This is where most online discussions about this topic fall down, and where people end up talking past each other.

The only place to start that makes sense is to understand the needs of the customers of the system. They want improvements, and fast. But they also want reliability and availability. How much is enough for them? Obviously there's no amount of down time that makes people happy, but there is an amount where it stops becoming an impediment to their use of the service. That's going to depend a great deal on what system we're talking about, and how customers use it.

**You can't talk about risk without talking about correlation**

Redundancy is the most powerful tool in our toolbox as distributed systems engineers. Have more that one place the data is stored, or requests are processed, or traffic can flow, and you can have a system that is more available and durable than any single component. Assuming, of course, that those components don't fail at the same time. If they do, then all that carefully designed-in redundancy is for nought.

What makes components fail at the same time? Could just be bad luck. The good news is that we can spend linear amounts of money to offset exponential amounts of bad luck. Correlated failures could be caused by common infrastructure, like power, cooling, or thin glass tubes. Correlated failures could be caused by data, load, or user behavior. But one of the dominant causes of correlated failure is software deployments. After all, deployments can cut across redundancy boundaries in ways that requests, data, and infrastructure often can't. Deployments are the one thing that breaks all our architecture assumptions.

To understand what a big deal is, we need to understand the exponential effects of redundancy. Say I have one box, which is down for a month a year, then I have a system with around 92% availability. If I have two boxes (either of which can handle the load), and they fail independently, then I have a system with 99.3% availability! On the other hand, if they tend to fail together, then I'm back to 92%. Three boxes independent gets me 99.95%. Three boxes at the same time get me 92%. And so on. Whether failures happen independently or at the same time matters a lot for availability.

Our deployment practices need to be aware of our redundancy assumptions. If we're betting that failures are uncorrelated, we need to be extremely careful about reintroducing that correlation. Similarly, and even more importantly, our architectures need to be sensitive to the need to deploy safely. This is one of the places that folks working on architecture can have the biggest long-term impact, by designing systems that can be changed safely and frequently with low impact, and isolating critical state-altering logic from other logic which we may want to change more quickly.

**You can't talk about risk without talking about correlation on behalf of customers**

Just like we build highly-available systems out of redundant components, customers of cloud services do too. It's typical practice to build systems which run in multiple availability zones or datacenters. Customers with more extreme availability needs may build architectures which cross regions, or even continents. Those designs only work if the underlying services don't fail at the same time, for all the same reasons that apply to each system in isolation. Making that true requires careful attention to system design, and careful attention to not re-introducing correlated failure modes during deployments.

**Conclusion**

This is a tricky space, because it combines social and organizational concerns with technical concerns with customer concerns, and even things like contractual obligations. Like any similar problem space, it's hard to come up with clear answers to these questions, because the answers are so dependent on context and details of your business. My advice is to write down those tensions explicitly, and be clear about what you're trying to balance and where you think the right balance is for your business or technology.