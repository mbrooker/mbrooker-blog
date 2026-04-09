---
layout: post
title: "Spec Driven Development isn't Waterfall"

---
{{ page.title }}
================

<p class="meta">Write down what you mean.</p>

After spending a few months writing (e.g. on the [Kiro Blog](https://kiro.dev/blog/kiro-and-the-future-of-software-development/)), and speaking (e.g. [Real Python Podcast](https://realpython.com/podcasts/rpp/277/), [SE Radio](https://se-radio.net/2026/03/se-radio-710-marc-brooker-on-spec-driven-ai-dev/)) about spec-driven development, I've noticed a common misconception: spec driven development is a return to a waterfall style of software development.

Specification driven development ([in Kiro, for example](https://kiro.dev/docs/specs/)) isn't about pulling designs *up-front*, it's about pulling designs *up*. Making specifications explicit, versioned, living artifacts that the implementation of the software flows from, rather than static artifacts.

This distinction is important, because software development (like all complex product development and engineering tasks) is a fundamentally iterative process. It is extremely rare for a software project to know all of the requirements up-front. It's much more common for one of the goals of the development process being to discover requirements, most frequently through engaging users in the cycle of feedback. This is a point that's missed in strict waterfall software development processes, and missed in critiques (like Dijkstra's) of natural language specification (as [I have written about before](https://brooker.co.za/blog/2025/12/16/natural-language.html)). The Agile movement is often presented as a high-minded set of ideas, but I think it's more accurate to see it as a reflection of a simple fact: as software became more complex, and filled more roles in society, top-down approaches to design simply no longer work.

From the Agile Manifesto:

> Customer collaboration over contract negotiation
> Responding to change over following a plan

These are simple reflections of reality. Software specifications are complex, dynamically changing, internally conflicting, and invariably incomplete. In specification driven development, the specification is the thing being iterated on, rather than the implementation. The iteration cycle is the same as before, but potentially much quicker because of the accelerating effect of AI.

So if specifications aren't up-front designs, what are they?

Specifications are an explicit statement of requirements and key design choices, separated from the low-level implementation. They are a raising of the level of abstraction from code to words, and increasingly to a mix of words, pictures, snippets, and even mathematics. The words can be free-form, or structured (e.g. [RFC2119](https://datatracker.ietf.org/doc/html/rfc2119) or [EARS](https://alistairmavin.com/ears/)). It can (rarely) pull in exact statements (e.g. in [Lean](https://lean-lang.org/) or [TLA+](https://lamport.azurewebsites.net/tla/tla.html)) when needed. The implementation is then derived from this specification, reflecting iterative changes in the specification, by AI alone or human developers working with AI. Increasingly, these tasks are done autonomously end-to-end by AI agents. Specs stay in sync with implementation by being *upstream* of implementation for most changes. 

This approach has several advantages which I've written about in the past: keeping context on the bigger picture (a map, versus the turn-by-turn directions of vibe coding prompts), the ability to mix levels of formality and detail to meet the needs of a particular piece of software, serving as always-in-sync documentation, allowing implementation of the same code in multiple languages or with multiple frameworks, and the ability to lift what matters out of the muck of the implementation. One advantage, though, is looking to override all of these in importance: we're seeing the largest improvements in velocity and delivery in teams and processes that can allow agents to run autonomously for long periods of time. Specifications do exactly that. By providing the agent with a clear map, we can set an agent off building without a human inside the tight loop of development and testing. The agent can also write higher quality, better designed, and better tested code by seeing the big picture. It knows what to test, and what good looks like.

Specifications aren't up-front designs because you don't need to, and probably shouldn't, develop the entire specification upfront. Instead, specifications should be at the core of an iterative software development practice. Humans are still critical to this outer loop of software development, driven by refining and extending the specification. Perhaps most crucially, they own the *internally conflicting* nature of software requirements. Where conflicts and trade-offs exist, either technical or in product requirements, expertise and experience come into play.

We are still very early in this revolution in software development. I believe that the future is specification-driven. Software development at a level above what we currently think of as implementation, just as it's already been raised from switches, to gates, to instructions, to lines of code<sup>[‡](#foot1)</sup>. It's going to take while to find exactly the right recipe, and I suspect we're not going to converge on a single answer across all domains. But the big picture is clear.

<p class="footnote">
<a name="foot1">‡</a> In a future post I'll tackle the *determinism question*, and how the non-deterministic nature of LLM-powered spec-to-code translation isn't actually a big deal.
</p>