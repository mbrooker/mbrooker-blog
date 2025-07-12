---
layout: post
title: "One or Two? How Many Queues?"





related_posts:
  - "/2023/05/10/open-closed/"
  - "/2021/08/05/utilization/"
  - "/2022/10/21/nudge/"
---
{{ page.title }}
================


<script>
  MathJax = {
    tex: {inlineMath: [['$', '$'], ['\\(', '\\)']]}
  };
</script>
<script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>

<p class="meta">Very applied queue theory.</p>

There's a well-known rule of thumb that one queue is better than two. When you've got people waiting to check out at the supermarket, having a single shared queue improves utilization and reduces wait times. The reason for this is pretty simple: it avoids the case where somebody is waiting in a queue while there's a checker available to do the work. It also saves the sanity of the person standing behind a cheque writer or expired coupon negotiator.

But one queue isn't always better than two.

The other day, I was up at Crystal Mountain, and needed to visit the *ahem* facilities. In the base lodge there, the men's room includes a bank of urinals, and a row of stalls. As men's rooms often do. What it also features is a long queue at rush hour, and a sharp corner before its possible to see which of the facilities are occupied. The result is a queue with a mix of folks waiting for urinals and stalls, and often under-utilization of the urinals (because the person at the head of the queue is waiting on a stall). Single queue, lower utilization, longer waits.

This situation got me thinking<sup>[1](#foot1)</sup>: what's the crucial difference between these two cases? And are their bathroom usage patterns where one queue makes sense?

As far as I can tell, the difference between the "two is better" and "one is better" situation is precommitment. Supermarket checkouts are fungible, and so work can be allocated to any one of them. There's no need to decide which you want to visit before joining the queue. When visiting the WC, on the other hand, one tends to have a strong opinion about what sort of facility is desired, up front. Somebody wanting to visit the urinal would likely take a stall, but the vice versa is seldom desirable.

*Does One Queue Ever Make Sense?*

Are there cases where one queue still makes sense, even in this *precommitted* world? Not that I could find.

First, let's look at the results of wait time versus utilization (utilization $\rho$ calculated as the ratio between the arrival rate and service rate, $\rho = \frac{\lambda}{\mu}$). With the utilization of each type of lavatory calculated independently, and an assumption that a stall visit takes ten times longer than the alternative.

![](/blog/images/arrival_rate_avg_wait.png)

At low utilization, as expected, the queue length remains short and both systems offer reasonable service times (see [my 2021 post on this topic to dive a bit deeper here](https://brooker.co.za/blog/2021/08/05/utilization.html)). However, as utilization approaches 100%, the mean wait times for single queue variant increase much more quickly than the multi-queue variant. Keep in mind there's some bias on the mean here - because the utilization of each variant is calculated independently, there are a lot more total arrivals for a quicker visit.

Another way to think about it is how the result changes with the ratio of visit service times (i.e. the ratio between the mean latency of a standing or sitting visit). Perhaps counter-intuitively, the single queue variant is slower even when the service times are equal, and things degrade from there (with loads of simulation noise).

![](/blog/images/speed_ratio_avg_wait_linear.png)

*What Can We Learn?*

Perhaps one thing we'll learn is whether this was a wise choice of framing for a blog post.

But, more usefully to everybody but me, is that the go-to rule that one queue is better than multiple queues breaks down in this case of *precommitment*. If you have multiple different types of work in your system, a queue per type of work may be a good choice.


*Footnotes*

1. <a name="foot1"></a> What else is one supposed to do in such a situation?