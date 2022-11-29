---
layout: post
title: "Lambda Snapstart, and snapshots as a tool for system builders"
---

{{ page.title }}
================

<p class="meta">Durable memory?</p>

Yesterday, AWS announced [Lambda Snapstart](https://aws.amazon.com/blogs/aws/new-accelerate-your-lambda-functions-with-lambda-snapstart/), which uses VM snapshots to reduce cold start times for Lambda functions that need to do a lot of work on start (starting up a language runtime<sup>[3](#foot3)</sup>, loading classes, running *static* code, initializing caches, etc). Here's a short video about it:

<iframe width="560" height="315" src="https://www.youtube-nocookie.com/embed/AgxvrZLI1mc" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

Snapstart is a super useful capability for Lambda customers. I'm extremely proud of the work the team did to make it a reality. We've been talking about this work for [over two years](https://www.youtube.com/watch?v=ADOfX2LiEns), and working on it for longer. The team did some truly excellent engineering work to make Snapstart a reality.

Beyond Snapstart in Lambda, I'm particularly excited about the underlying technology (microVM snapshots), and the way they give us (as system builders and researchers) a powerful tool for building new kinds of secure and scalable systems. In this post, I talk about some interesting aspects of Snapstart, and how they point to interesting possible areas for research on systems, storage, and even cryptography.

*What is Snapstart?*

Without snapstart, the *cold start time* of a Lambda function is a combination of time time taken to download the function (or container), to start the language runtime<sup>[3](#foot3)</sup>, and to run any initialization code inside the function (including any *static* code, doing class loading, and even JIT compilation). The cold start time doesn't include MicroVM boot, because that time is not specialized to the application, and can be done in advance. A cold start looks like this<sup>[1](#foot1)</sup>: 

![](/blog/images/serverless_cold_start.png)

With snapstart, a snapshot of the microVM is taken after these initialization steps (including all the memory, device state, and CPU registers). This snapshot can be used multiple times (also called *cloned*), to create multiple sandboxes that are initialized in the same way. Cloning like this has two benefits:

 - The work of initialization is amortized over many sandboxes, rather than being done once for every sandbox. Lambda runs one sandbox for ever concurrent function execution, so for a function with N concurrent executions this reduces initialization work from O(N) to O(1).
 - The time taken by execution no longer accrues to the cold-start time, significantly reducing cold start latency for applications that do a lot of work on startup (which is typically applications that use large language runtimes like the JVM, or large frameworks and libraries).
 - JIT compilation and other *warmup* tasks can be done at initialization time, often avoiding the uneven performance that can be introduced by JITing soon after language runtime<sup>[2](#foot2)</sup> startup.

In diagram form, the snapstart startup regime looks like this<a name="foot1"></a>:
![](/blog/images/snapstart.png)

**The Problems of Uniqueness**

Perhaps the biggest challenge with clones is that they're, well, clones. They contain the same CPU, the same memory, and even the same CPU registers. [Being too alike can cause problems](https://www.youtube.com/watch?v=nuW0pUG3PrQ). For example, as we say in [Restoring Uniqueness in MicroVM Snapshots](https://arxiv.org/pdf/2102.12892.pdf):

> Most modern distributed systems depend on the ability of nodes to generate unique or random values. RFC4122 [20] version 4 UUIDs are widely used as unique database keys, and as request IDs used for log correlation and distributed tracing. ...

and

>  Many common distributed systems protocols, including consensus protocols like Paxos, and ordering protocols like vector clocks, rely on the fact that participants can uniquely identify themselves.

and

> Cryptography is the most critical application of unique data. Any predictability in the data used to generate cryptographic keys—whether long-lived keys for applications like storage encryption or ephemeral keys for protocols like TLS fundamentally compromise the confidentiality and authentication properties offered by cryptography.

In other words, its really important for systems to be able to generate random numbers, and MicroVM clones might find that difficult to do. If they rely on hardware features like `rdrand` then there's no problem, but any software-based PRNGs will simply create the same stream of numbers unless action is taken. To solve this problem, our team has been working with the OpenSSL, Linux, and Java open source communities to make sure that common PRNGs like `java.security.SecureRandom` and `/dev/urandom` reseed correctly when snapshots are cloned. 

**State**

Another challenge with working with clones is connection state in protocols like TCP. There are actually two problems here:

- *Time*. If a connection is established during initialization and the clone is used later, it's likely that the remote end has given up on the connection.
- *State*. Protocols like TCP provide reliable delivery using state at each end of a connection (like a sequence number), with the assumption that there is one client for the lifetime of the connection. If that one client suddenly becomes two clients, the protocol is broken and the connection must be dropped.

The simple solution is to reestablish connections after snapshots are restored. As with reseeding PRNGs, this requires time and work, especially for secure protocols like TLS which somewhat dilutes cold start benefit. There's a significant research and development opportunity here, focusing on fast reestablishment of secure protocols, clone-aware protocols, clone-aware proxies, and even deeply protocol-aware session managers (like RDS proxy).

**Moving Data**

Let's take another look at this snapstart diagram:
![](/blog/images/snapstart.png)

If the clones are running on the same machine the snapshot was created on, then this diagram is reasonably accurate. However, if we want to scale snapshot restores out over a system the size of AWS Lambda, then we need to make sure the snapshot data is available where and when it is needed. This hidden work - both in distributing data when it's needed and in sending restores to the right places to meet the data - was the largest challenge of building Snapstart. Turning memory reads into network storage accesses, as would happen with a naive demand-loading system, would very quickly cancel out the latency benefits of Snapstart.

We're going to say more about our particular solution to this problem in the near future, but I believe that there are interesting general challenges here for systems and storage researchers. Loading memory on demand can be done if the data layer offer low-enough latency, close enough to the latency of a local memory read. We can also avoid loading memory on demand by predicting memory access patterns, loading memory contents ahead of when they are needed. This seems hard to do in general, but is significantly simplified by the ability to learn from the behavior of multiple MicroVM clones.

**Hungry, Hungry Hippos**

Linux, [rather famously](https://www.linuxatemyram.com/), loves to eat all the memory it can lay its hands on. In the traditional single-system setting this is the right thing to do: the marginal cost of making an empty memory page full is very nearly zero, so there's no need to think much about the marginal benefit of keeping around a disk-backed page of memory (whether its an mmap mapping, or an IO buffer, or whatever). However, in cloud settings like Lambda Snapstart, this calculus is significantly different: keeping around disk-backed pages that are unlikely to be used again makes snapshots bigger, with little benefit. The same applies to caches at all layers, whether they're in the language runtime, in application code, or in libraries.

Tools like [DAMON](https://www.kernel.org/doc/html/v5.17/vm/damon/index.html) provide a good ability to monitor and control the kernel's behavior. I think, though, that there will be a major change in thinking required as systems like Snapstart become more popular. There seems to be an open area of research here, in adapting caching behaviors (perhaps dynamically) to handle changing marginal costs of full and empty memory pages. Linux's behavior - and the one most programmers build into their applications - is one behavior on a larger spectrum, that is only optimal at the point of zero marginal cost.

**Snapshots Beyond Snapstart**
I can't say anything here about future plans for using MicroVM snapshots at AWS. But I do believe that they are a powerful tool for system designers and researchers, which I think are currently under-used. Firecracker has the ability to restore a MicroVM snapshot in as little as 4ms (or about 10ms for a full decent-sized Linux system), and it's no doubt possible to optimize this further. I expect that sub-millisecond restore times are possible, as are restore times with a CPU cost not much higher than a traditional fork (or even a traditional thread start). This reality changes the way we think about what VMs can be used for - making them useful for much smaller, shorter-lived, and transient applications than most would assume.

Firecracker's full and incremental snapshot support [is already open source](https://github.com/firecracker-microvm/firecracker/blob/main/docs/snapshotting/snapshot-support.md). But Firecracker is far from the last word in restore-optimized VMMs. I would love to see more research in this area, exploring what is possible from user space and kernel space, and even how hardware virtualization support can be optimized for fast restores.

**In Video Form**
Most of this post covers material I also covered in this talk, if you'd prefer to consume it in video form.

<iframe width="560" height="315" src="https://www.youtube-nocookie.com/embed/ADOfX2LiEns" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

**Footnotes**

1. <a name="foot1"></a> Diagram from Brooker et al, *[Restoring Uniqueness in MicroVM Snapshots](https://arxiv.org/pdf/2102.12892.pdf)*, 2021.
2. <a name="foot2"></a> I particularly enjoyed Laurence Tratt's recent look at VM startup in [More Evidence for Problems in VM Warmup](https://tratt.net/laurie/blog/2022/more_evidence_for_problems_in_vm_warmup.html).
3. <a name="foot3"></a> In this post I've used the words *language runtime* to refer to language VMs like the JVM, to avoid confusion with virtualization VMs like Firecracker MicroVMs. This isn't quite the right word, but it seemed worth avoiding the potential for confusion.
