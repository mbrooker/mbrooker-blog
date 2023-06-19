---
layout: post
title: "What is a container?"
---

{{ page.title }}
================

<p class="meta">What are words, anyway?</p>

One big area of confusion and miscommunication I frequently see is caused by different people using different definitions of words. Sometimes the definitions are subtly different (as with [availability](https://brooker.co.za/blog/2018/02/25/availability-liveness.html)). Sometimes they're completely different, and we're just talking about different things entirely. A very common example is the word *container*, a popular term for a popular technology that means at least four different things.

 1. An approach to packaging an application along with its dependencies (sometimes a whole operating system user space), that can then run on a minimal runtime environment with a clear contract.
 2. A set of development, deployment, architectural, and operational approaches built around applications packaged this way.
 3. A set of operational, security, and performance isolation tools that allow multiple applications to share an operating system without interfering with each other. On Linux, this tools include *chroot*, *cgroups*, *namespaces*, *[seccomp](https://man7.org/linux/man-pages/man2/seccomp.2.html)*, and others.
 4. A set of implementations of the practices (the proper nouns, like Docker, Kubernetes, ECS, etc).

These four definitions are surprisingly independent. The idea of packaging applications this way predates the other three, and will likely be around after they are gone. The practices and approaches are enabled by the tools, but don't really require them. The kernel-level interfaces, and the semantics and security they provide, are a basis for many of the implementations today, but most of the semantics are available in different ways<sup>[1](#foot1)</sup>. 

To pick an example, when we talk about [container image support in AWS Lambda](https://aws.amazon.com/blogs/aws/new-for-aws-lambda-container-image-support/) we mostly mean the first one - enabling customers to get the advantages of packaging their code that way, with a small overlap with practices (some become easier to use with this support available), and the fourth (some of these tools can be used to create the images in ways that fit into a broader ecosystem).

Or, to pick another example, when people say *containers are not a security boundary*<sup>[2](#foot2)</sup>, they are mostly talking about the third category (with some overlap into the fourth). It barely touches on the first and second category, which are generally a big win for security. That full conversation is subtle, so I won't go into it here.

When you use the word *container*, consider whether your audience is using the same definition as you.

**Footnotes**

1. <a name="foot1"></a> For example, with MicroVMs like [Firecracker](https://github.com/firecracker-microvm/firecracker).
2. <a name="foot2"></a> Those people include me.
