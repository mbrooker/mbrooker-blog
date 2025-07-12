---
layout: post
title: "What is a simple system?"






related_posts:
  - "/2022/12/15/thumb.html"
  - "/2015/01/25/patterns.html"
  - "/2024/06/04/scale.html"
---
{{ page.title }}
================

<p class="meta">Is this pretentious?</p>

Why do I need cryptography when I could simply hide the contents of my communications rotating every letter by 13? Why do I need a distributed storage system when I could simply store my files on this one server? Why do I need a database when I could simply use a flat file?

Do any of those things, and feel joy in a job well done. A simple solution. Perhaps you're hiding your communications from a child, storing little data with low value, and avoiding concurrency. Simplicity in a goal achieved. But useless in the face of an adult adversary, or a desire for persistence beyond the fallibility of hardware, or even of two people trying to do a job at once.

This presents us with something of a challenge: we know that simplicity is good, and excess simplicity is useless.

> Everything should be as simple as can be,
> Says Einstein,
> But not simpler.<sup>[1](#foot1)</sup>

Indeed, but that doesn't get us much further in understanding how simple things can be. It is only possible to evaluate simplicity in context of the complete closure of the world. The problems a system solves, technical, organizational, educational, and historical. This presents a problem, due to the difficulties of encapsulating the world. 

> When we try to pick out anything by itself, we find it hitched to everything else in the Universe.<sup>[2](#foot2)</sup>

Brooks turned to Aristotle to attempt to answer this question, and found complexities accidental and essential. The accidental complexities are introduced by our human failings: ignorance, pride, and curiosity. The essential are produced by our environment, as separate from ourselves and our technology. A perfect jewel, dirtied only by our clumsy hands. We are encouraged to avoid excessive curiosity, as if learning may take us further from the light of perfect simplicity. Could a perfect craftsman with perfect tools produce a perfect product?

> Successive theories in any mature science will be such that they 'preserve' the theoretical relations and the apparent referents of earlier theories (i.e., earlier theories will be 'limiting cases' of later theories).<sup>[3](#foot3)</sup>

Simplicity exists in a historical context. It refers to the needs handed down by the sages of the past, whether or not we understand their wisdom. C and Unix are simple. Any appearance that the solve problems no longer relevant, or fail to solve the problems of today, is simply due to your ignorance. Windows and Excel are complicated, and any semblance of simplicity is illusion.

Simplicity may also refer to our ability to shed the demands of the past, and focus only on the transient present and rapidly approaching future. Immediately deny the past, its lessons irrelevant and theories inapplicable. Simplicity is achieved by the new. C is a tool too simple for our adversarial world, and too complex for our abstracted one.

> "Looks pretty much the same, yeah," Armstrong replied of the lunar module. "You know, there's an old saying in aviation that 'if it looks good it flies good.' And this has to be the exception to the rule. Because it flew very well. But it is the probably the ugliest flying machine that was ever been designed."<sup>[4](#foot4)</sup>

Simplicity then, exists only in the eye of the beholder. Hopefully more like a contact lens, and less like an eyelash. Despite the correlation of appearance and result, exceptions abound. Ugly and functional. Beautiful but useless. Exceptions, but common enough to challenge any definition based on aesthetics alone.

Each culture, company, team, and organization has their own aesthetic sense. What's simpler: Go, Scheme, or assembly?

Goodhart warns that attempts to measure simplicity, let along success, will lead to the measurements becoming useless. After all, any attempt to quantify anything about software or systems is doomed to inevitable failure, and we may as well not try. Opinion is better than data, especially mine.

> If we postulate, and we just have, that within un-, sub- or supernatural forces the probability is that the law of probability will not operate as a factor<sup>[5](#foot5)</sup>

Defining simplicity as outcome fails when faced with the improbable. Simplicity is easily achieved within reasonable probabilities, and fails when probabilities become unreasonable. The guard on the grinder and the harness on the climber are accidental complexity, or maybe simplicity only in the face of accidents. Complexity that handles the improbable is only complexity until it becomes essential. Good luck makes everything look unnecessarily complex.

What is a simple system?

 **Footnotes**

 1. <a name="foot1"></a> My copy of Zukofsky's "A" quotes [Hugh Kenner](https://en.wikipedia.org/wiki/Hugh_Kenner) as calling it "the most hermetic poem in English". I haven't read every poem in English, and so can't vouch for this superlative.
 2. <a name="foot2"></a> From Muir, [apparently accurately](https://vault.sierraclub.org/john_muir_exhibit/writings/misquotes.aspx#1).
 3. <a name="foot3"></a> From Laudan's [A Confutation of Convergent Realism](https://philosophy.hku.hk/courses/dm/phil2130/AConfutationOfConvergentRealism2_Laudan.pdf), which made a big dent in my world view in my early 20s.
 4. <a name="foot4"></a> From ["Man on the Moon": The 50th anniversary of the Apollo 11 landing](https://www.cbsnews.com/news/man-on-the-moon-50th-anniversary-of-the-apollo-11-landing-cbs-news-special/)
 5. <a name="foot5"></a> Guildenstern, on their way to a sticky end.