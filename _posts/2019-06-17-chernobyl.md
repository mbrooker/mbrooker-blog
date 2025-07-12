---
layout: post
title: "Is Anatoly Dyatlov to blame?"






related_posts:
  - "/2014/06/29/rasmussen.html"
  - "/2019/08/12/kind-wicked.html"
  - "/2016/01/03/correlation.html"
---
{{ page.title }}
================

<p class="meta">Without a good safety culture, operators are bound to fail.</p>

(Spoiler warning: containers spoilers for the HBO series Chernobyl, and for history).

Recently, I enjoyed watching HBO's new series Chernobyl. Like everybody else on the internet, I have some thoughts about it. I'm not a nuclear physicist or engineer, but I do think a lot about safety and the role of operators.

The show tells the story of the accident at Chernobyl in April 1986, the tragic human impact, and the cleanup and investigation in the years that followed. One of the villains in the show is Anatoly Dyatlov, the deputy chief engineer of the plant. Dyatlov was present in the control room of reactor 4 when it exploded, and received a huge dose of radiation (the second, or perhaps third, large dose in his storied life of being near reactor accidents). HBO's portrayal of Dyatlov is of an arrogant and aggressive man whose refusal to listen to operators was a major cause of the accident. Some first-hand accounts agree<sup>[2](#foot2), [3](#foot3), [6](#foot6)</sup>, and others disagree<sup>[1](#foot1)</sup>. Either way, Dyatlov spent over three years in prison for his role in the accident.

There's little debate that the reactor's design was deeply flawed. The International Nuclear Safety Advisory Group (INSAG) found<sup>[4](#foot4)</sup> that certain features of the reactor "had a primary influence on the course of the accident and its consequences". During the time before the accident, operators had put the reactor into a mode where it was unstable, with reactivity increases leading to higher temperatures, and further reactivity increases. The IAEA (and Russian scientists) also found that the design of the control rods was flawed, both in that they initially increased (rather than decreasing) reactivity when first inserted, and in that they machinery to insert them moved too slowly. They also found issues with the control systems, cooling systems, and the fact that some critical safety measures could be manually disabled. Authorities had been aware of many of these issues since an accident at the Ignalina plant in 1983<sup>[4, page 13](#foot4)</sup>, but no major design or operational practice changes had been made by the time of the explosion in 1986.

In the HBO series' telling of the last few minutes before the event, Dyatlov was shown to dismiss concerns from his team that the reactor shouldn't be run for long periods of time at low power. Initially, Soviet authorities claimed that the dangers of doing this was made clear to operators (and Dyatlov ignored procedures). Later investigations by IAEA found no evidence that running the reactor in this dangerous mode was forbidden <sup>[4, page 11](#foot4)</sup>. The same is true of other flaws in the plant. Operators weren't clearly told that pushing the emergency shutdown (aka SCRAM, aka AZ-5) button could temporarily increase the reaction rate in some parts of the reactor. The IAEA also found that the reactors were safe in "steady state", and the accident would not have occurred without the actions of operators.

Who is to blame for the 1986 explosion at Chernobyl?

In 1995, Dyatlov wrote an article in which he criticized both the Soviet and IAEA investigations<sup>[5](#foot5)</sup>, and asked a powerful question:

> How and why should the operators have compensated for design errors they did not know about?

If operators make mistakes while operating systems which have flaws they don't know about, is that "human error"? Does it matter if their ignorance of those flaws is because of their own inexperience, bureaucratic incompetence, or some vast KGB-lead conspiracy? Did Dyatlov deserve death for his role in the accident, as the series suggests? As Richard Cook says in "How Complex Systems Fail"<sup>[7](#foot7)</sup>:

> Catastrophe requires multiple failures – single point failures are not enough. ... Each of these small failures is necessary to cause catastrophe but only the combination is sufficient to permit failure.

And

> After accidents, the overt failure often appears to have been inevitable and the practitioner’s actions as blunders or deliberate willful disregard of certain impending failure. ... That practitioner actions are gambles appears clear after accidents; in general, post hoc analysis regards these gambles as poor ones. But the converse: that successful outcomes are also the result of gambles; is not widely appreciated.

If Dyatlov and the other operators of the plant had known about the design issues with the reactor that had been investigated following the accident at Ignalina in 1983, would they have made the same mistake? It's hard to believe they would have. If the reactor design had been improved following the same accident, would the catastrophe had occurred? The consensus seems to be that it wouldn't have, and if it did then it would have taken a different form.

From "How Complex Systems Fail":

> Most initial failure trajectories are blocked by designed system safety components. Trajectories that reach the operational level are mostly blocked, usually by practitioners.

The show's focus on the failures of practitioners to block the catastrophe, and maybe on their unintentional triggering of the catastrophe seems unfortunate to me. The operators - despite their personal failings - had not been set up for success, either by arming them with the right knowledge, or by giving them the right incentives. 

From my perspective, the show is spot-on in it's treatment of the "cost of lies". Lies, and the incentive to lie, almost make it impossible to build a good safety culture. But not lying is not enough. A successful culture needs to find the truth, and then actively use it to both improve the system and empower operators. Until the culture can do that, we shouldn't be surprised when operators blunder or even bluster their way into disaster.

## Footnotes

 1. <a name="foot1"></a> BBC, [Chernobyl survivors assess fact and fiction in TV series](https://www.bbc.com/news/world-europe-48580177), 2019
 1. <a name="foot2"></a> Svetlana Alexievich, "Voices from Chernobyl".
 1. <a name="foot3"></a> Serhii Plokhy, "Chernobyl: The History of a Nuclear Catastrophe". This is my favorite book about the disaster (I've probably read over 20 books on it), covering a good breadth of history and people without being too dramatic. There are a couple of minor errors in the book (like confusing GW and GWh in multiple places), but those can be overlooked.
 1. <a name="foot4"></a> [INSAG-7 The Chernobyl Accident: Updating of INSAG-1](https://www-pub.iaea.org/MTCD/publications/PDF/Pub913e_web.pdf), IAEA, 1992 
 1. <a name="foot5"></a> Anatoly Dyatlov, [Why INSAG has still got it wrong](https://www.neimagazine.com/features/featurewhy-insag-has-still-got-it-wrong), NEI, 1995 
 1. <a name="foot6"></a> Adam Higginbotham, "Midnight in Chernobyl: The Untold Story of the World's Greatest Nuclear Disaster"
 1. <a name="foot7"></a> Richard Cook, [How Complex Systems Fail](https://web.mit.edu/2.75/resources/random/How%20Complex%20Systems%20Fail.pdf)