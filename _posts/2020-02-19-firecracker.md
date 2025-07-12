---
layout: post
title: "Firecracker: Lightweight Virtualization for Serverless Applications"





related_posts:
  - "/2024/11/14/lambda-ten-years/"
  - "/2020/06/08/virtualization/"
  - "/2022/07/12/dynamodb/"
---
{{ page.title }}
================

<p class="meta">Our second paper for NSDI'20.</p>

In 2018, we announced [Firecracker](https://firecracker-microvm.github.io/), an [open source](https://github.com/firecracker-microvm/firecracker) VMM optimized for multi-tenant serverless and container workloads. We heard some interest from the research community, and in response wrote up our reasoning behind building Firecracker, and how its used inside AWS Lambda.

That paper was accepted to NSDI'20, and is [available here](https://www.amazon.science/publications/firecracker-lightweight-virtualization-for-serverless-applications). Here's the abstract:

> Serverless containers and functions are widely used for deploying and managing software in the cloud. Their popularity is due to reduced cost of operations, improved utilization of hardware, and faster scaling than traditional deployment methods. The economics and scale of serverless applications demand that workloads from multiple customers run on the same hardware with minimal overhead, while preserving strong security and performance isolation. The traditional view is that there is a choice between virtualization with strong security and high overhead, and container technologies with weaker security and minimal overhead. This tradeoff is unacceptable to public infrastructure providers, who need both strong security and minimal overhead. To meet this need, we developed Fire-cracker, a new open source Virtual Machine Monitor (VMM)specialized for serverless workloads, but generally useful for containers, functions and other compute workloads within a reasonable set of constraints. We have deployed Firecracker in two publically available serverless compute services at Amazon Web Services (Lambda and Fargate), where it supports millions of production workloads, and trillions of requests per month. We describe how specializing for serverless in-formed the design of Firecracker, and what we learned from seamlessly migrating Lambda customers to Firecracker.

Like any project the size of Firecracker, it was developed by a team of people from vision to execution. I played only a small role in that, but it's been great to work with the team (and the community) on getting Firecracker out, adding features, and using it in production at pretty huge scale.

Firecracker is a little bit unusual among software projects of having an explicit goal of being simple and well-suited for a relatively small number of tasks. That doesn't mean it's simplistic. Choosing what to do, and what not to do, was some of the most interesting decisions to be made in it's development. I'm particularly proud of how well the team made those decisions, and continues to make them.