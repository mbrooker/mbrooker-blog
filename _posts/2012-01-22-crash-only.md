---
layout: post
title: The properties of crash-only software









related_posts:
  - "/2025/11/20/what-now.html"
  - "/2021/05/24/metastable.html"
  - "/2025/06/02/hotos.html"
dissimilar_posts:
  - "/2020/07/28/fish.html"
---
{{ page.title }}
================

<p class="meta">My thoughts about a classic paper</p>

[Crash-only software](http://www.usenix.org/events/hotos03/tech/full_papers/candea/candea.pdf) by Candea and Fox is a very interesting paper which is well worth your time if you spend any time designing software or services. Re-reading it today, I noticed how useful the section headers of section 3 *Properties of Crash-Only Software* appear outside the context of the paper.

The properties the authors list are:

 - All important non-volatile state is managed by dedicated state stores
 - Components have externally enforced boundaries
 - All interactions between components have a timeout
 - All resources are leased, rather than permanently allocated
 - Requests are entirely self-describing

Regardless of what you think of the value of crash-only software, it is difficult to argue with this list of properties. Even outside of the context of the paper, each of these makes sense to me as a good design practice. The way I understand them is like this:

**All important non-volatile state is managed by dedicated state stores**. Either you care about your state or you don't. If you do, store it somewhere safe where it won't get lost and can be recovered quickly in event of failure. If you don't, and your data is either purely volatile or a cache, then be explicit in your design that you don't care about it. Don't half-care about data. Store data explicitly and not implicitly.

**Components have externally enforced boundaries**. Keep logically separate components as separate as possible, ensuring that they don't interact except via a well-defined API. Try to limit implicit side channels.

**All interactions between components have a timeout**. Not only that, but consider the possibility that what you are trying to do will never succeed. All individual calls should have a timeout, and all attempts to retry should be limited. Waiting forever is not a substitute for knowing how to handle failure.

**All resources are leased, rather than permanently allocated**. If you own something, don't give it away to somebody else if you still value it. You can lend it to them for as long as they can justify it's use. If you no longer care about something, you can give it away. If something still has value to your business, then keep it.

**Requests are entirely self-describing**. Don't keep implicit context in protocols and interactions. To me, self-describing doesn't imply 'has no schema' or 'doesn't correspond to a well defined API'. It means that requests should contain all the context that is needed to complete them, and if that is not possible keep the state in a dedicated state store. The API should also be explicit about idempotency and the safety of retries.