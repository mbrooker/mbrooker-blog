---
layout: post
title: "Ten Years of AWS Lambda"





related_posts:
  - "/2023/05/23/snapshot-loading/"
  - "/2024/12/03/aurora-dsql/"
  - "/2024/07/29/aurora-serverless/"
---
{{ page.title }}
================

<p class="meta">Everything starts somewhere.</p>

Today, Werner Vogels shared his annotated version of the [original AWS Lambda PRFAQ](https://www.allthingsdistributed.com/2024/11/aws-lambda-turns-10-a-rare-look-at-the-doc-that-started-it.html). This is a great inside look into how product development happens at AWS - the real *working backwards* process in action. This was, in some ways, the start of serverless computing<sup>[2](#foot2)</sup>. Tim Wagner, Ajay Nair, and others really saw the future when they wrote this PRFAQ<sup>[3](#foot3)</sup>.

I wanted to take the opportunity to dive a little deeper into some of the things Werner highlights.

> We made the hard decision to only launch with support for Node, which was popular at the time

Another thing Node had going for it in 2014 was a great dependency management story (via *npm*). It's really easy to zip up a NodeJS program along with its dependencies into a nice self-contained unit, without changing anything about the developer flow. This makes NodeJS my go-to for developing small Lambda functions to this day. It's really this easy:

    zip -r function.zip index.mjs node_modules

Over time, languages like Go and Rust, which are mostly statically linked have made this easy for binary programs too, but those languages weren't very well known a decade ago. C and C++ remain rather difficult to package up, given their rather deep ties into the OS userland, and we had to wait for container support to do a great job there.

> Go support (Jan 2018)
> Custom Runtimes (Nov 2018)

A few days ago, I did an episode of the AWS developer podcast where I talked about some of the surprises that came as we built the Lambda business.

<iframe allow="autoplay *; encrypted-media *; fullscreen *; clipboard-write" frameborder="0" height="175" style="width:100%;max-width:660px;overflow:hidden;border-radius:10px;" sandbox="allow-forms allow-popups allow-same-origin allow-scripts allow-storage-access-by-user-activation allow-top-navigation-by-user-activation" src="https://embed.podcasts.apple.com/us/podcast/aws-lambda-a-decade-of-transformation/id1574162669?i=1000675303451&theme=auto"></iframe>

Custom runtimes was one of those. We knew that customers wanted more flexibility around languages, but we also thought that wrapping custom code in little NodeJS or Python scripts wasn't too onerous. The launch of Go support, which was really custom runtime support under the covers, opened the flood gates of customers doing interesting and cool things to bring their own languages (C++, Rust, different JVMs, etc), and made it super clear to us that we needed official custom runtime support.

We should have noticed this earlier, because the same thing happened with EC2 nearly a decade before. EC2 customers did [all kinds of clever tricks](https://www.daemonology.net/blog/2011-07-08-FreeBSD-on-EC2-via-defenestration.html) to bring their own OSs and kernels to EC2 before it was officially supported.

> Applications in steady use have typical latencies in the range of 20-50ms, determined by timing a simple “echo” application from a client hosted in Amazon EC2.

Today, Lambda invokes are much faster than this. We've written a lot over the years about optimizing tail latencies for Lambda, but in a lot of ways the *median* latency was a much bigger deal for customers and somewhere we've invested just as much. Especially customers building microservice and SoA architectures, where median latencies stack up. 

In [our talk at reInvent 2018](https://www.youtube.com/watch?v=QdzV04T_kec) Holly Mesrobian talked about Lambda's *worker manager*, the core component that allocates incoming Lambda invokes to available capacity. The original 2014 worker manager was an entirely in-memory process, which was great for latency and allowed it to optimize across a broad segment of the workload. The downside to it was that if a worker manager machine failed all that in-memory state was lost, and it was very expensive to reconstruct. We ended up redesigning *worker manager* to be persistent across AZs, and doing that while decreasing median latency was really challenging. It's one of the places we use AWS's internal Journal service<sup>[1](#foot1)</sup> to great effect.

> How does Lambda protect me from overcrowding effects?

There's another interesting story here. From the beginning, Lambda's design was smart about heat management on compute hardware, and avoiding noisy-neighbor effects on functions themselves. One thing we overlooked, though, was the effect of the shared queues in the system (such as the ones that back [event invoke](https://docs.aws.amazon.com/lambda/latest/api/API_Invoke.html)). Heavy hitters, especially with functions that were failing a lot, could fill up these queues and cause a lot of cross-customer latency impact.

In [our talk at reInvent 2019](https://www.youtube.com/watch?v=xmacMfbrG28) Holly and I cover a bit of what we ended up doing about that, and Julian and Chris's [2022 talk](https://www.youtube.com/watch?v=0_jfH6qijVY) goes into some more detail. Working with some [Amazon Scholars](https://www.amazon.science/scholars) and our team, we developed an algorithm that combined a variant of [stochastic fairness queuing](https://ieeexplore.ieee.org/document/91316) (SFQ) with a [best-of-k](https://brooker.co.za/blog/2018/01/01/balls-into-bins.html) placement approach to design a system that provided tight bounds on noisy neighbor effects without requiring any shared state between the machines in the queue fleet. Stochastic fairness is a super powerful approach, and one we've since deployed in a load of other places.

Anyway, go check out [Werner's post](https://www.allthingsdistributed.com/2024/11/aws-lambda-turns-10-a-rare-look-at-the-doc-that-started-it.html). It's a rare look at how the sausage is made at AWS.

*Footnotes*

1. <a name="foot1"></a> Which I've written about before [in context of MemoryDB](https://brooker.co.za/blog/2024/04/25/memorydb.html) and the MemoryDB team's excellent [SIGMOD'23 paper](https://www.amazon.science/publications/amazon-memorydb-a-fast-and-durable-memory-first-cloud-database).
2. <a name="foot2"></a> But not the start of the use of the word *serverless*. For that, I think we have to go all the way back to the 1995 paper [Serverless Network Filesystems](https://dl.acm.org/doi/10.1145/225535.225537) by Anderson et al. From what I can see, *serverless* was in fairly common use in the storage world for decades before it really started being applied to compute. Incidentally, in our [On-Demand Container Loading in AWS Lambda](https://www.usenix.org/conference/atc23/presentation/brooker) paper we cite Douceur et al's 2002 paper [Reclaiming Space from Duplicate Files in a Serverless Distributed File System](https://www.microsoft.com/en-us/research/publication/reclaiming-space-from-duplicate-files-in-a-serverless-distributed-file-system/), a paper that I happened to stumble across a few years ago by chance because it uses the word 'serverless' in the title.
3. <a name="foot3"></a> If it wasn't so trite, I'd quote Alan Kay's "the best way to predict the future is to invent it" here.