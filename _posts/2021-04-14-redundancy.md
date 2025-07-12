---
layout: post
title: "Redundant against what?"






related_posts:
  - "/2019/06/20/redundancy.html"
  - "/2016/01/03/correlation.html"
  - "/2019/05/01/emergent.html"
---
{{ page.title }}
================

<p class="meta">Threat modeling thinking to distributed systems.</p>

There's basically one fundamental reason that distributed systems can achieve better availability than single-box systems: redundancy. The software, state, and other things needed to run a system are present in multiple places. When one of those places fails, the others can take over. This applies to replicated databases, load-balanced stateless systems, serverless systems, and almost all other common distributed patterns.

One problem with redundancy is that it [adds complexity](https://brooker.co.za/blog/2019/06/20/redundancy.html), which may reduce availability. Another problem, and the one that people tend to miss the most, is that redundancy isn't one thing. Like *security*, redundancy is a single word that we mean that our architectures and systems are resistant to different kinds of failures. That can mean infrastructure failures, where redundancy could mean multiple machines, multiple racks, multiple datacenters or even multiple continents. It can mean software failures, where common techniques like canary deployments help systems to be redundant when one software version failures. I can also mean logical failures, where we recognize that *state* can affect the performance or availability of our system, and we try ensure that the same *state* doesn't go to every host. Sometimes that state is configuration, sometimes it's stored data or requests and responses.

**An Example**

Unfortunately, when we talk about system designs, we tend to forget these multiple definitions of redundancy and instead just focus on infrastructure. To show why this matters, let's explore an example.

Event logs are rightfully a popular way to build large-scale systems. In these kinds of systems there's an ordered log which all changes (writes) flows through, and the changes are then applied to some systems that hang off the log. That could be read copies of the data, workflow systems taking action on the changes, and so on. In the simple version of this pattern one thing is true: every host in the log, and every consumer, sees the same changes in the same order.

![Event bus architecture, with three replicas hanging off the bus](https://mbrooker-blog-images.s3.amazonaws.com/bus_arch_0.jpg)

One advantage of this architecture is that it can offer a lot of redundancy against infrastructure failures. Common event log systems (like Kafka) can easily handle the failure of a single host. Surviving the failure of a single replica is also easy, because the architecture makes it very easy to keep multiple replicas in sync.

![Event bus architecture, with three replicas hanging off the bus, with host failures](https://mbrooker-blog-images.s3.amazonaws.com/bus_arch_1.jpg)

Now, consider the case where one of the events that comes down the log is a *poison pill*. This simply means that the consumers don't know how to process it. Maybe it says something that's illegal ("I can't decrement this unsigned 0!"), or doesn't make sense ("what's this data in column X? I've never heard of column X!"). Maybe it says something that only makes sense in a future, or past, version of the software. When faced with a poison pill, replicas have basically two options: ignore it, or stop.

![Event bus architecture, with three replicas hanging off the bus, with logical failures](https://mbrooker-blog-images.s3.amazonaws.com/bus_arch_2.jpg)

Ignoring it could lead to data loss, and stopping leads to writes being unavailable. Nobody wins. The problem here is a lack of redundancy: running the same (deterministic) software on the same state is going to have the same bad outcome every time.

**More Generally**

This problem doesn't only apply to event log architectures. Replicated state machines, famously, suffer from the same problem. So does primary/backup replication. It's not a problem with one architecture, but a problem with distributed systems designs in general. As you design systems, it's worth asking the question about what you're getting from your redundancy, and what failures it protects you against. In some sense, this is the same kind of thinking that security folks use when they do [threat modeling](https://en.wikipedia.org/wiki/Threat_model):

> Threat modeling answers questions like “Where am I most vulnerable to attack?”, “What are the most relevant threats?”, and “What do I need to do to safeguard against these threats?”.

A few years ago, I experimented with building a [threat modeling framework for distributed system designs](https://brooker.co.za/blog/2015/06/20/calisto.html), called CALISTO, but I never found something I loved. I do love the way of thinking, though. "What failures am I vulnerable to?", "Which are the most relevant failures?", "What do I need to do to safeguard against those failures?"

If your answer to "What failures am I vulnerable to?" doesn't include software bugs, you're more optimistic than me.