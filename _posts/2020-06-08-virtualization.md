---
layout: post
title: "Some Virtualization Papers Worth Reading"




related_posts:
  - "/2025/06/02/hotos"
  - "/2023/07/13/osdi"
  - "/2014/09/21/liskov-pub"
---
{{ page.title }}
================

<p class="meta">A short, and incomplete, survey.</p>

A while back, Cindy Sridharan asked on Twitter for pointers to papers on the past, present and future of virtualization. A picked a few of my favorites, and given the popularity of that thread I decided to collect some of them here. This isn't a literature survey by any means, just a collection of some papers I've found particularly interesting or useful. As usual, I'm biased towards papers I enjoyed reading, rather than those I had to slog through.

Popek and Goldberg's 1974 paper [Formal Requirements for Virtualizable Third Generation Architectures](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.141.4815&rep=rep1&type=pdf) is rightfully a classic. They lay out a formal framework of conditions that a computer architecture must fulfill to support virtual machines. It's 45 years old, so some of the information is dated, but the framework and core ideas have stood the test of time.

[Xen and the Art of Virtualization](https://www.cl.cam.ac.uk/research/srg/netos/papers/2003-xensosp.pdf), from 2003, described the Xen hypervisor and a novel technique for running secure virtualization on commodity x86 machines. The exact techniques are less interesting than they were then, mostly because of hardware virtualization features on x86 like [VT-x](https://en.wikipedia.org/wiki/X86_virtualization), but the discussion of the filed and trade-offs is enlightening. Xen's influence on the industry has been huge, especially because it was used as the foundation of Amazon EC2, which triggered the following decade's explosion in cloud computing. [Disco: Running Commodity Operating Systems on Scalable Multiprocessors](http://pages.cs.wisc.edu/~remzi/Classes/838/Spring2013/Papers/bugnion97disco.pdf) from 1997 is very useful from a similar perspective (and thanks to Pekka Enberg for the tip on that one). Any paper that has *"our approach brings back an idea popular in the 1970s"* in its abstract gets my attention immediately.

[A Comparison of Software and Hardware Techniques for x86 Virtualization](https://www.vmware.com/pdf/asplos235_adams.pdf), from 2006, looks at some of the early versions of that x86 virtualization hardware and compares it to software virtualization techniques. As above, hardware has moved on since this was written, but the criticisms and comparisons are still useful to understand.

The security, compatibility and performance trade-offs of different approaches to isolation are complex. On compatibility, [A study of modern Linux API usage and compatibility: what to support when you're supporting](https://dl.acm.org/doi/10.1145/2901318.2901341) is a very nice study of how much of the Linux kernel surface area actually gets touched by applications, and what is needed to be truly compatible with Linux. Randal's [The Ideal Versus the Real: Revisiting the History of Virtual Machines and Containers](https://arxiv.org/abs/1904.12226) surveys the history of isolation, and what that means in the modern world. Anjali's [Blending Containers and Virtual Machines: A Study of Firecracker and gVisor](https://dl.acm.org/doi/pdf/10.1145/3381052.3381315) is another of a related genre, with some great data comparing three methods of isolation.

[My VM is Lighter (and Safer) than your Container](https://dl.acm.org/doi/10.1145/3132747.3132763) from SOSP'17 has also been influential in changing they way a lot of people think about virtualization. A lot of people I talk to see virtualization as a heavy tool with multi-second boot times and very limited density, mostly because that's the way it's typically used in industry. Manco et al's work wasn't the first to burst that bubble, but they do it very effectively.

Our own paper [Firecracker: Lightweight Virtualization for Serverless Applications](https://www.amazon.science/publications/firecracker-lightweight-virtualization-for-serverless-applications) describes Firecracker, new open-source Virtual Machine Monitor (VMM) specialized for serverless workloads. The paper also covers how we use it in AWS Lambda, and some of what we see as the future challenges in this space. Obviously I'm biased here, being an author of that paper.