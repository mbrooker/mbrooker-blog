---
layout: post
title: "On the success of 'natural language programming'"

---
{{ page.title }}
================

<p class="meta">Specifications, in plain speech.</p>

I believe that specification is the future of programming.

Over the last four decades, we've seen the practice of building programs, and software systems grow closer and closer to the practice of specification. Details of the implementation, from layout in memory and disk, to layout in entire data centers, to algorithm and data structure choice, have become more and more abstract. Most application builders aren't writing frameworks, framework builders aren't building databases, database builders aren't designing protocols, protocol designers aren't writing kernels, and so on. Our modern software world is built on abstractions.

Significant advancements are made, from time to time, by cutting through these abstractions. But still, the abstractions dominate, and will continue to.

The practice of programming has become closer and closer to the practice of *specification*. Of crisply writing down what we want programs to do, and what makes them right. The *how* is less important.

I believe that *natural language* will form the core of the programming languages of the future.

*The Ambiguity Problem*

The most common objection to this view is that natural language is *ambiguous*. It's exact meaning is potentially unclear, and highly dependent on context. This is a real problem.

For example, in [The Bug in Paxos Made Simple](https://brooker.co.za/blog/2021/11/16/paxos.html), I look at a common bug in implementations of Paxos caused directly by the ambiguity of natural language.

> Pointing out this ambiguity isn’t criticizing [Lamport's] writing, but rather reminding you about how hard it is to write crisp descriptions of even relatively simple distributed protocols in text.

As [Lamport says](https://lamport.azurewebsites.net/pubs/pubs.html#paxos-simple):

> Prose is not the way to precisely describe algorithms.

Perhaps the most famous statement of this problem is Dijkstra's from [On the foolishness of "natural language programming"](https://www.cs.utexas.edu/~EWD/transcriptions/EWD06xx/EWD667.html):

> When all is said and told, the "naturalness" with which we use our native tongues boils down to the ease with which we can use them for making statements the nonsense of which is not obvious.

Dijkstra's argument goes beyond merely pointing out ambiguity, and the lack of precision of natural language, but also points out the power of symbolic tools. All of these arguments are true. Reasoning using the symbolic and formal tools of mathematics is indeed powerful. It is tempting to poke holes in this argument by pointing out that most programs don't need precise specification, and that there's a large opportunity for natural language to specify those programs. This argument is true, but doesn't go far enough.

Instead, I argue that ambiguity doesn't doom natural language programming for one simple reason: almost all programs are already specified in natural language. And always have been.

*Where Do Programs Come From?*

Programs come from requirements from people, and people specify the need for those programs using that least precise of tools: natural language. We talk to customers, to stake holders, to product managers, and other consumers of our programs and ask them what they want. Sometimes, we'll get a precise specification, like an OpenAPI spec or an RFC. More often, we'll get something fuzzy, incomplete, and ambiguous.

That tends to work for two reasons. First, we'll apply context. Common sense. Our understanding from similar successful projects about the requirements that users have in common. Or maybe even formal compliance requirements. Second, we'll have a conversation. *Hey, I didn't quite understand your requirement for the daily average, do you want the mean or median?* Or *can you make it so I don't lose data even if a machine fails?* Software teams and organizations have these conversations continuously.

This is how software is built professionally, how software construction is taught, and how open source and even hobby communities build their systems.

Sometimes, these conversations will become formal. A snippet of code. A SQL query. An example. But most often they're informal. A conversation. A napkin sketch. Some hand-waving over lunch.

LLMs allow us to include our computers in these conversations.

*Specifications are Loops*

Vibe coding is the ultimate embodiment of this: building a specification for a program based on a conversation of *yes, and* and *no, but*. A closed loop of a developer and an AI model or agent having a conversation about how to drive a code base forward. This conversation is much richer than a simple natural language programming language because of the loop. The loop is where the context is built up, and the magic happens.

Kiro-style [spec-driven development](https://kiro.dev/blog/kiro-and-the-future-of-software-development/) is similar. There, the loop is a little more formal, a little less ad-hoc. But the fundamental shape of the conversation is a similar one. [Property-based testing](https://kiro.dev/blog/property-based-testing/) adds a little bit of structure to this loop, and helps make sure its ratcheting forward as it goes.

This is, I think, the fundamental think that most of the takes on natural language and programming miss. They think we're solving a one-hit problem of going from an ambiguous messy, human, specification to a precise and perfect program. They see the trips around the loop as failures. I believe that the trips around the loop are fundamental to the success of the whole enterprise. They're what we've been doing all along.

To quote from the [Agile Manifesto](https://agilemanifesto.org/), and artifact of a similar time of change in our industry:

> Individuals and interactions over processes and tools

The loop is the interaction.

*Getting the Answer Right*

Now, this answer is easy enough for programs whose results don't matter all that much. Which is most programs. But there are some cases where precise and exact answers matter a great deal. They could matter because other layers of the system depend on them (as with the safety properties of Paxos), because of legal or compliance requirements, because of security requirements, or for many other reasons. More importantly, it may not be obvious that the current answer is wrong. That another trip around the loop is needed.

What do we do then, huh?

One thing we can do is slip back into symbolic representations. Back to writing Rust, SQL, or TLA+. We shouldn't see these cases as failures, and should expect to have these tools in our toolbox for the foreseeable future. Using them is not a failure because we've mostly avoided the need for them, and gotten the efficiency gains of bringing our programming practice closer to our customers and businesses. Piercing through the layers will always be a thing.

But there's another, rather tantalizing, direction. Here, let's turn to a paper by some colleagues of mine [A Neurosymbolic Approach to Natural Language Formalization and Verification](https://arxiv.org/pdf/2511.09008). Here *neurosymbolic* refers to the idea that we can effectively combine the ambiguous loosie goosie natural language LLM world with the precise world of symbolic reasoning *inside the machine* to provide highly accurate results not achievable via either technique. In this case, highly accurate results on policy following based on a natural language specification.

They resolve ambiguities in text by explicitly including the customer (the specifier) in the feedback loop, such as by having the customer review a restatement of the policy:

> Manual inspection allows users to review their generated policy model and verify its correctness, similar to code review in software development. Users can examine the policy variables with their types and descriptions, as well as the logical rules themselves.

Here, *the policy model* could be entirely formal (SMT-LIB), and could be semi-formal in structured natural language. *Let me say that back to you to check I got it*.

They're also applying other techniques, like looking for internal inconsistencies in specifications and asking users to proactively resolve them. Humans can also review automatically generated test cases, both at the individual case level and at the property level.

This is, again, the power of the conversation. A specification isn't created single-shot from the ambiguous language, but rather extracted after some back-and-forth. This seems to me like the programming model of the future. It's also the one we've always used.

*Specifications are Context*

Once a specification has been developed, it takes on a second life, as a new piece of context that future specifications can refer to. This is a technique as old as computing, and much much older. Once we've established a common understanding, whether it's about what we mean by *average* or what we mean by *authenticated*, we can then use those terms in future communication. When we talk to our customers about a feature request, we don't start from zero every time. Instead, we use our common understanding of the system as-is, and have a short conversation about what changes we want. Just like I don't provide you directions to the restaurant down the road starting at your birth place.

Back to Dijkstra:

> From one gut feeling I derive much consolation: I suspect that machines to be programmed in our native tongues —be it Dutch, English, American, French, German, or Swahili— are as damned difficult to make as they would be to use.

Today, in the year 2025, we've built those *machines to be programmed in our native tongues*. We've made significant progress on how to use them, and are seeing significant returns from that effort. There's more work to learn how to use them efficiently, but there's no doubt in my mind that we'll be using them. The future of programming looks like the past of programming: a natural language conversation, a feedback loop, with the occasional descent into mathematical precision.

When we succeed, we'll bring the power of computing to more places, more people, and more problems. I can't wait to see where we go from here.