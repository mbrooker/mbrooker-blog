---
layout: post
title: "Latency Sneaks Up On You"
related_posts:
  - "/2020/08/06/erlang"
  - "/2021/08/27/caches"
  - "/2018/06/20/littles-law"
---{{ page.title }}
================

<p class="meta">And is a bad way to measure efficiency.</p>

As systems get big, people very reasonably start investing more in increasing efficiency and decreasing costs. That's a good thing, for the business, for the environment, and often for the customer. Much of the time efficient systems have lower and more predictable latencies, and everybody enjoys lower and more predictable latencies.

Most folks around me think about latency using percentiles or other order statistics. Common practice is to look at the median (50<sup>th</sup> percentile), and some high percentile like the 99.9<sup>th</sup> or 99.99<sup>th</sup>. As they do efficiency work, they often see that not only does their 50<sup>th</sup> percentile improve a lot, but so does the high percentiles. Then, just a short while later, the high percentiles have crept back up without the code getting slower again. What's going on?

A lot of things, as usual, but one of them is the non-linear effect of utilization. To explain that, let's consider a simple system with one server that can serve one thing at a time, and a queue in front of it<sup>[1](#foot1)</sup>.

    ┌────────┐     ┌─────┐    ┌──────┐
    │ Client │────▶│Queue│───▶│Server│
    └────────┘     └─────┘    └──────┘

Simple as it comes, really. We can then define the Utilization of the server (calling it ⍴ for traditional reasons) in terms of two other numbers: the mean arrival rate λ, and the mean completion rate μ, both in units of jobs/second.

    ⍴ = λ/μ

Another way to think of ⍴ is that the server is idle (no work to do, empty queue) 1-⍴ of the time. So if ⍴=0.4, then the server is idle 60% of the time. Clearly, if ⍴>1 for a long time then the queue grows without bound, because more stuff is arriving than leaving. Let's ignore that for now, because infinite queues are silly things.

To understand what happens here, we need to look at the diagram above, and notice that there's no feedback loop. The client sends work<sup>[2](#foot2)</sup> at random, on it's own schedule. Sometimes that's when the server is idle, and sometimes when it's busy. When the server is busy, that work is added to the queue. Because ⍴<1 in our world, the work will eventually get done, but may have to wait behind other things in the queue. Waiting in the queue<sup>[3](#foot3)</sup> for service is a common cause of outlier latency.

As we think about it this way, we realize that the closer ⍴ gets to 1, the more likely it is that an incoming item of work will find a busy server, and so will be queued. So increasing ⍴ increases queue depth, which increases latency. By a lot. In fact, it increases latency by an alarming amount as ⍴ goes it 1. One way to think about this is in terms of the number of items of work in the system, including being serviced by the server, and in the queue. For tradition's sake, we'll call this N and its mean (expectation) E\[N\].

    E[N] = ⍴/(1-p)

Maybe we need to draw that to show how alarming it is.

![Graph of ⍴/(1-p)](https://mbrooker-blog-images.s3.amazonaws.com/queue_length.png)

To give you some sense of this, at ⍴=0.5 (about half utilized), E\[N\] is 1. At ⍴=0.99, it's 99.

When people do efficiency work, they typically increase the rate at which the system can do work μ, without changing the arrival rate λ. Going back to our definition:

    ⍴ = λ/μ

We can see that increasing μ drives down ⍴ and pushes us to the left and down on the curve above. Even relatively modest changes in μ can lead to a very big change in E\[N\], and if queuing dominates our latency, a big win on latency. An especially big win on outlier latency.

Next, the system grows for a while (increasing λ), or we reduce the number of servers (decreasing μ) to realize our efficiency gains. That causes ⍴ to pop back up, and latency to return to where it was. This often leads people to be disappointed about the long-term effects of efficiency work, and sometimes under-invest in it.

The system we consider above is a gross simplification, both in complexity, and in kinds of systems. Streaming systems will behave differently. Systems with backpressure will behave differently. Systems whose clients *busy loop* will behave differently. These kinds of dynamics are common, though, and worth looking out for.

The bottom line is that high-percentile latency is a bad way to measure efficiency, but a good (leading) indicator of pending overload. If you must use latency to measure efficiency, use [mean (avg) latency](https://brooker.co.za/blog/2017/12/28/mean.html). Yes, average latency<sup>[4](#foot4)</sup>. 

**Footnotes**

 1. <a name="foot1"></a> I'm intentionally glossing over the details here. The system I'm considering is M/M/1, with a single server, unbounded queue, Poisson arrival process, and exponentially distributed service time. And yes, real systems aren't like this. I know.
 2. <a name="foot2"></a> In this case according to a Poisson process, which isn't entirely realistic, but isn't so far off the reality either. I'm fudging something else here: single clients don't tend to be Poisson processes, but the sum of very large numbers of independent clients do. If you care about that, sub 'clients' every time I say 'client'.
 3. <a name="foot3"></a> When I say *queue* that may be an explicit actual queue, or could just be a bunch of threads waiting on a lock, or an async task waiting for an IO to complete, or whatever. Implicit queues are everywhere.
 4. <a name="foot4"></a> Yes, those people on the internet that tell you never to measure average latency are wrong. And don't get me started on the trimmers and winsorizers.