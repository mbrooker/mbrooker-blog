---
layout: post
title: "Animations for System Builders: Metastability and Lost Writes"

---
{{ page.title }}
================

<p class="meta">Pretty pictures help us learn, maybe?</p>

<script>
  MathJax = {
    tex: {inlineMath: [['\\(', '\\)'], ['$', '$']]}
  };
</script>
<script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>

One of the things I love using LLMs for is helping me understand systems better. In the [Strands Decider post](https://brooker.co.za/blog/2026/09/28/engineering-system-one.html) I wrote about how I often ask *ask me questions to check I understand this*. The other technique I'm enjoying right now, and am delighted to see folks around me using too, is building animations.

Animations can help us build an intuition for how and why systems work, or how and why they fail.

Here's one example: did you know that a Raft cluster can lose writes if even one of the cluster members has a storage layer that lies about `fsync()`?

<video width="640" height="360" controls>
  <source src="https://brooker.co.za/blog/images/lying-fsync.mp4" type="video/mp4">
  Your browser does not support the video tag.
</video>
 
When I built [Stability Sim](https://stability-sim.systems/), I wanted to help engineers understand metastable failures better. That worked pretty well on an abstract level (e.g. [retries and queues make metastability really easy to trigger](https://stability-sim.systems/?example=metastable-retry)), but I still hear doubts that these behaviors happen in real systems. After all, those real systems are production-proven, and wouldn't we have shaken out the behaviors by now if they existed?

Let's start with Postgres.

<video width="640" height="360" controls>
  <source src="https://brooker.co.za/blog/images/pg-thrash.mp4" type="video/mp4">
  Your browser does not support the video tag.
</video>

In this example, captured from traces on real Postgres 16 running in a real MicroVM. Before a load spike, the database is easily able to handle its 20 queries-per-second load, thanks to the whole working set sitting in the page cache. But after the load spike, we've filled up a lot of the memory running queries (each takes 12MB), which now run slower because they're served off the disk rather than the page cache. I like this example, because it shows that even simple, non-distributed systems have these behaviors.

What about `etcd`?

<video width="640" height="360" controls>
  <source src="https://brooker.co.za/blog/images/etcd-metastable.mp4" type="video/mp4">
  Your browser does not support the video tag.
</video>

Again, we have load spike triggering the metastable state. Writes, in this case. The overloaded peers look like they're unavailable, which triggers a storm of application-level resends. I like this example, because `etcd` is widely-used quality software. Operational experience doesn't seem to effectively shake out these kinds of issues.

When I wrote [Garbage Collection and Metastability](https://brooker.co.za/blog/2024/08/14/gc-metastable.html), a widely-known systems academic told me that it was a *fantasy*. I've seen it happen in the wild in ways I couldn't share, and I can forgive people for not believing this can happen based on the hand waving in the blog.

So it's nice to have a real world example, picking on `etcd` again.

<video width="640" height="360" controls>
  <source src="https://brooker.co.za/blog/images/etcd-gc.mp4" type="video/mp4">
  Your browser does not support the video tag.
</video>

At the start, we're ticking along nicely at 500 reads/second, before spiking load to 1000 reads/s. The read spike increases the time spent in GC, increasing average tenure, and making less CPU available for the read workload. This is a self-reinforcing loop, and the system doesn't recover without intercession.

I believe the same thing could happen on a runtime without GC, due to increasing work in the allocator, but haven't been able to reproduce that yet.

The animations here are all generated with [Kiro](https://kiro.dev) and [KiroCrew](https://github.com/kirodotdev/KiroCrew), mostly with Opus 5.5. The actual results came from a real test environment, which captures the traces that back the animations.