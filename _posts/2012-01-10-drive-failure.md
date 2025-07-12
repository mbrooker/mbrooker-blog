---
layout: post
title: The benefits of having data





related_posts:
  - "/2016/01/03/correlation/"
  - "/2021/03/25/latency-bandwidth/"
  - "/2014/07/04/iostat-pct/"
---
{{ page.title }}
================

<p class="meta">Two ways to look at drive failures and temperature.</p>

Almost all recent articles and papers I have read on hard drive failure rates refer to either [Failure Trends in a Large Disk Drive Population](http://www.usenix.org/events/fast07/tech/full_papers/pinheiro/pinheiro_html/) from Google, or [Estimating Drive Reliability in Desktop Computers and Consumer Electronics Systems](http://www.seagate.com/docs/pdf/whitepaper/drive_reliability.pdf) from Seagate. Despite both sounding and looking authoritative, these papers come to some wildly different conclusions, and couldn't be more different in their approach.

How does temperature affect drive failure rate? The Seagate paper says an increase from 25C to 30C increases it by 27%. The Google paper suggests a decrease of around 10%. How can the two most widely used studies differ by so much? It's really because these papers use completely different approaches: the Google study uses simple analysis, while the Seagate paper uses powerful and sophisticated models, accelerated aging, and complex statistical tools. Despite sounding less authoritative, the Google paper is much better.

The Seagate paper doesn't actually present the results of testing drives at different temperatures. Instead, all the drives were tested using a standard [accelerated aging](http://en.wikipedia.org/wiki/Accelerated_aging) approach, in an oven heated to 42C. Another standard accelerated aging technique, the [Arrhenius Model](http://en.wikipedia.org/wiki/Arrhenius_equation), was used to estimate the effect of temperature on failure rates. The Seagate paper goes on to use Weibull modeling, and a fairly sophisticated Bayesian approach to estimating the Weibull parameters. The underlying, and unmentioned, assumption is that the failure rate of drives is proportional to the [reaction rate constant](http://en.wikipedia.org/wiki/Reaction_rate_constant), or the speed that an unlimited chemical reaction would proceed at a given temperature. No attempt is made to justify this choice, other than appealing to standard textbooks describing the approach.

The Google paper, on the other hand, doesn't use any statistical concepts that would be unfamiliar to an undergraduate engineering student. Instead, they use the failure data from over a hundred thousand disk drives collected over nearly a year of constant measurement.

The difference between these paper's conclusions on a rather fundamental question - how does temperature affect drive failure? - is a great example of how there is no substitute for data. Statistical tools and small-sample laboratory testing are certainly useful, and should not be ignored, but their conclusions are a poor approximation of the real thing.

The Seagate paper is, sadly, an excellent example of why so many engineers (in our industry, and outside of it) are suspicious of statistical approaches. The paper not only stretches a little data into big conclusions, but does it while sounding authoritative. It uses complex statistical jargon not to explain, but to shield. *How dare you question my conclusions? You haven't seen a Weibull model since college, and probably don't even know what Eta looks like!*