---
layout: post
title: "DynamoDB's Best Feature: Predictability"









related_posts:
  - "/2022/07/12/dynamodb.html"
  - "/2025/08/15/dynamo-dynamodb-dsql.html"
  - "/2025/11/02/thinking-dsql.html"
dissimilar_posts:
  - "/2015/05/24/sodium-carbonate.html"
---
{{ page.title }}
================

<p class="meta">Happy birthday!</p>

It's [10 years since the launch of DynamoDB](https://www.amazon.science/latest-news/amazons-dynamodb-10-years-later), Amazon's fast, scalable, NoSQL database. Back when DynamoDB launched, I was leading the team rethinking the control plane of [EBS](https://aws.amazon.com/ebs/). At the time, we had a large number of manually-administered MySQL replication trees, which were giving us a lot of operational pain. Writes went to a single primary, and reads came from replicas, with lots of eventual consistency and weird anomalies in the mix. Our code, based on an in-house framework, was also hard to work with. We weren't happy with our operational performance, or our ability to deliver features and improvements. Something had to change. We thought a lot about how to use MySQL better, and in the end settled on ditching it entirely. We rebuilt the whole thing, from the ground up, using DynamoDB. At the time my main attraction to DynamoDB was *somebody else gets paged for this*, with a side order of *it's fast and consistent*. DynamoDB turned out to be the right choice, but not only for those reasons.

To understand the real value of DynamoDB, I needed to think more deeply about one of the reasons the existing system was painful. It wasn't just the busywork of DB operations, and it wasn't just the eventual consistency. The biggest pain point was behavior under load. A little bit of unexpected traffic and things went downhill fast. Like this:

![](https://mbrooker-blog-images.s3.amazonaws.com/goodput_curve.jpeg)

Our system had two stable modes (see my posts [on metastability](https://brooker.co.za/blog/2021/05/24/metastable.html) and on [cache behavior](https://brooker.co.za/blog/2021/08/27/caches.html)): one where it was ticking along nicely, and one where it had collapsed under load and wasn't able to make progress. That collapsing under load was primarily driven by the database itself, with buffer/cache thrashing and IO contention the biggest drivers, but that wasn't the real cause. The real cause was that we couldn't reject work well enough to avoid entering that mode. Once we knew - based on queue lengths or latency or other output signals - the badness had already started. The unexpectedly expensive work had already been let in, and the queries were already running. Sometimes cancelling queries helped. Sometimes failing over helped. But it was always a pain.

Moving to DynamoDB fixed this for us in two ways. One is that DynamoDB is great at rejecting work. When a table gets too busy you don't get weird long latencies or lock contention or IO thrashing, you get a nice clean HTTP response. The net effect of DynamoDB's ability to reject excess load (based on per-table settings) is that the offered load/goodput graph has a nice flat "top" instead of going up and then sharply down. That's great, because it gives systems more time to react to excess load before tipping into overload. Rejections are a clear leading signal of excess load.

More useful than that is another property of DynamoDB's API: each call to the database does a clear, well-defined unit of work. Get these things. Scan these items. Write these things. There's never anything open-ended about the work that you ask it to do. That's quite unlike SQL, where a single *SELECT* or *JOIN* can do a great deal of work, depending on things like index selection, cache occupancy, key distribution, and the skill of the query optimizer. Most crucially, though, the amount of work that a SQL database does in response to a query depends on what data is already in the database. The programmer can't know how much work a query is going to trigger unless they can also predict what data is going to be there. And, to some extent, what other queries are running at the same time. These properties make it hard for the programmer to build a good mental model of how their code will work in production, especially as products grow and conditions change.

The same unpredictability has another effect. In typical web services, requests need to be accepted or rejected at the front door. That means that services need to be able to look at a request, and decide whether it should be rejected (for example to prevent overload or because of user quotas) without being able to accurately predict the cost of the database queries it will trigger.

This all comes together to make it much easier to write stable, well-conditioned, systems and services against DynamoDB than against relational databases. SQL and relational databases definitely have their place, including in scalable service architectures, but significant extra effort needs to be spent to make the systems that depend on them stable under unexpected load. That's work that most developers aren't deeply familiar with. DynamoDB's model, on the other hand, forces stability and load to be considered up front, and makes them easier to reason about. In some environments that's well worth it.

It took me a while to realize it, but that's my favorite thing about DynamoDB.