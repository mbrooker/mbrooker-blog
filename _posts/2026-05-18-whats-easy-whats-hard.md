---
layout: post
title: "What's Easy Now? What's Hard Now?"

---
{{ page.title }}
================

<p class="meta">Take it easy.</p>

*This is the fourth in a series about how AI is changing software development, after [It’s time to be right.](https://brooker.co.za/blog/2026/04/30/be-right.html), [What about juniors?](https://brooker.co.za/blog/2026/03/25/ic-junior.html), and [My heuristics are wrong. What now?](https://brooker.co.za/blog/2026/03/20/ic-leadership.html). It stands alone, but if you found this interesting you may also find those interesting.*

I've been spending a lot of time thinking about the shape of the capabilities of coding agents. What they're good at now, what they're going to be good at. What they're bad at now, how much of that is inherent and how much is transient. This is worth thinking about, because it's the most important question shaping the future of software, and of software engineering. I don't pretend to have an answer, but am coming to a conclusion that may be deeply counter-intuitive.

Coding agents are becoming very good indeed, and can build meaningful and correct software very quickly and at transformatively low cost. They have super-human abilities on some coding tasks. Of course, computer systems have had super human abilities for *at least* 85 years<sup>[1](#foot1)</sup>. I think we're going to find, as we have over those nine decades, that this new technology we're building is vastly super-human in some areas<sup>[2](#foot2)</sup>, and not nearly as capable as humans in others.

Which raises the important question of how, and why.

**Feedback is powerful**

Early on in my EE education, one of my professors drew a simple circuit on the board that's been stuck in my mind ever since. It looked like this<sup>[3](#foot3)</sup>:

![](/blog/images/op_amp_sqrt.png)

Apply a voltage on the left, and on the right you get the square root of that voltage<sup>[4](#foot4)</sup>. The two components are an opamp and an analog multiplier IC (e.g. the deeply obsolete [MC1495](https://www.onsemi.com/download/data-sheet/pdf/mc1495-d.pdf)). This simple circuit encapsulates possibly the most important idea in electrical engineering: *feedback is uniquely powerful*. Maybe unreasonably powerful. It's the idea that makes nearly every electronic device work, it keeps planes in the sky, and stops your oven from burning your dinner.

Components inside feedback loops can be made to behave significantly differently from their basic *open loop* behavior. Excellent outputs can be extracted from poor components. Multipliers can become square rooters. Feedback changes everything.

AI agents are just feedback loops. They're built around a component with useful, but flawed, open loop behavior (an LLM), and use feedback to make that component able to do things that it's not able to do without feedback. This is the basic idea behind the transformation that has happened in developer tooling in the last two years or so: a move from open loop AI (the smart autocomplete mode in IDEs) to agents. The moving of the feedback from the human developer (build, test, go back to IDE), into the agent itself (build, test, iterate).

Much of the conversation about long-term coding agent capabilities is about open loop model behavior. But that's only half the picture. I may even stretch to saying it's the less important half of the picture. Feedback is the thing that's going to drive long-term capabilities.

**The feedback loop hypothesis**

*In the long term, coding agents will find tasks with effective feedback 'easy', and tasks without effective feedback 'hard'. The availability of accurate feedback will determine the limits on their capabilities*.

On one hand, we should see this as uncontroversial. Anybody who has built code with agents knows that good error messages help keep agents unstuck. We're seeing how tools like Rust guide agents towards writing correct code by providing explicit and immediate feedback about incorrectness of some kinds. We're seeing agents be great at performance work, where good benchmarks exist. We're seeing tools like [property-based testing](https://kiro.dev/docs/specs/correctness/) be uniquely valuable. We're also seeing that agents aren't great at *architecture* (where feedback tends to be of the 'I know it when I see it' kind), or writing concurrent programs (where feedback tends to be of the 'it silently corrupted data at runtime' kind).

But let's look forward a little bit, and compare two problems:

* Building a delightful ergonomic photo editing website.
* Building a correct high-performance database storage engine<sup>[5](#foot5)</sup>.

For open-loop models, the former is easier than the latter. At least in that you'll get closer to real success with a pure *vibe coding* workflow, and much closer to success on the former after a single shot. The feedback loop hypothesis, however, makes me think that the latter is actually the *easier* long-term problem.

To understand why, consider their feedback loops. The website's feedback loop, beyond maybe some automation that tests if the buttons do what they should, requires a human in the loop. It needs to be easy to use for humans, and humans are notoriously slow, squishy, and inconsistent feedback providers. The latter, however, has a rather simple specification, including the API, safety properties, and liveness properties. With the right tools in the feedback loop, iteration towards success requires no humans.

**What does it mean?**

I think this is different from the intuition many people have about coding agents. They see websites and UIs as 'easy' (see the SaaSpocalypse), and system software as 'hard'. The feedback loop hypothesis says that this is backwards. That, in fact, we're going to find that SaaS is 'hard' and system software is 'easy'.

This is going to raise the importance of specification (the writing down of what good looks like to drive the feedback loop), and of tools that apply that specification to code. Compile-time tools like Rust, [Hydro](https://hydro.run/), and [Verus](https://github.com/verus-lang/verus). Modelling-time tools like [TLA+](https://lamport.azurewebsites.net/tla/tla.html) and [P](https://github.com/p-org/P/). Specification tools like [Kiro's spec analyzer](https://kiro.dev/blog/deep-spec-analysis/). Testing tools, simulators, mocks, etc.

The future of software development is building these feedback loops. Many hard problems remain.

**Footnotes**

1. <a name="foot1"></a> Dating back to the work of folks like [Marian Rejewski](https://en.wikipedia.org/wiki/Marian_Rejewski) in the 1930s.
2. <a name="foot2"></a> The MacBook on my desk can add 64 bit numbers about something like 100,000,000,000 times faster than I can.
3. <a name="foot3"></a> Drawn with [CircuitLab](https://www.circuitlab.com), and adapted from this [Electronics StackExchange Answer](https://electronics.stackexchange.com/a/78298). In reality, a few more passive components are needed.
4. <a name="foot4"></a> If you're not familiar with this stuff, here's an intuition for how this works. The *opamp* (the triangle) tries to adjust its output (on the right) so the two inputs are the same. So if you take the output, and multiply it by itself, then feed it into one of the inputs, it'll set the output to the square root of the input. If you are familiar with this stuff, I apologize deeply for that explanation.
5. <a name="foot5"></a> I mean something on the scale of, say, [RocksDB](https://rocksdb.org/) or InnoDB, not something on the scale of [Aurora DSQL](https://aws.amazon.com/rds/aurora/dsql/) or even PostgreSQL. I think these large-scale distributed systems are going to be harder to hill climb to, at least for the future I can see.
