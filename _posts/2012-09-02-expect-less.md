---
layout: post
title: Expect Less, Get More?
---

{{ page.title }}
================

<p class="meta">On what newly hired engineers think they need to do.</p>

Hiring good software engineers is really difficult. It's hard to find good people, hard to filter good hires from bad hires, and possibly even harder to decide what 'good' really means. When we find a good hire, we want to make sure they can reach their potential as quickly as possible. Most good software managers and leaders realize that even excellent people may take some time to really contribute. They have a lot to learn, and must be given the time to learn, however eager we are to get them contributing to our projects.

I recently came across a pair of papers from [Andrew Begel](http://research.microsoft.com/en-us/um/people/abegel/) at Microsoft Research, and [Beth Simon](http://cseweb.ucsd.edu/~bsimon/) at UCSD. In [Novice Software Developers, All Over Again](http://research.microsoft.com/en-us/um/people/abegel/papers/icer-begel-2008.pdf) and [Struggles of New College Graduates in their First Software Development Job](http://research.microsoft.com/en-us/um/people/abegel/papers/sigcse-begel-2008.pdf), they present the results from a study where they followed eight new Microsoft hires for a total of 85 hours.

All of the material in these studies is very interesting, but the section I found most relevant to my interests is the list of *Misconceptions Which Hinder*. The universal misconceptions they list all seem to match my own experiences early in my career, and seem to align well with what I see colleagues struggling with. Perhaps the most poignant of these is the first one:

> *1. I must do everything myself so that I look good to my manager.*
> This misconception is particularly dangerous, especially in large, complex development environments. \[...\] the perceived need to “perform” and not “reveal deficiencies” makes for much wasted time. It also seems to contribute to poor communication and a longer acclimatization. Communication suffered both by waiting too long to seek help and by trying to cover up issues that the \[engineer\] perhaps felt he “should know.”

The fact that this was universally felt is especially interesting given the diverse educational backgrounds of the study subjects:

> Subjects W, X, Y and Z had BS degrees, V had an MS, and U, R, and T had PhDs, all in computer science or software engineering. 2 were educated in the US, 2 in China, 1 in Mexico, 1 in Pakistan, 1 in Kuwait, and 1 in Australia. All 3 PhDs were earned in US universities.

Starting in industry after completing my PhD, I felt the same pressure that these candidates report. I felt like I had been hired for ''what I knew'', and by admitting that I didn't know something, I'd make my manager and mentor reconsider the decision to hire me. Until reading Begel and Simon's study, I hadn't really thought that this was a widely spread feeling amongst people early in their careers. Thinking about this now, I realized two things. First, this pressure came to from me, and not from the outside. What feedback I received on my performance was almost uniformly positive. Second, it's clear that I wasn't hired for my intimate knowledge of the proprietary systems I was working on.

While Begel and Simon's study is a small one, it is good evidence that this is a widespread problem amongst newly hired engineers. As mentors and managers, we need to make requirements more clear to new hires. It's not surprising that somebody who has been around for only a few months doesn't have a complete knowledge of our system, and we should clearly communicate that we don't expect them to. As software designers, we can also mitigate this problem by making sure our systems are loosely coupled and interfaces well defined - the amount of knowledge that a newbie needs to make effective changes to our software is a useful metric for good design. 

After reading these papers, I found the same research covered in the excellent book [Making Software: What Really Works, and Why We Believe It](http://www.amazon.com/Making-Software-Really-Works-Believe/dp/0596808321). It's well worth picking up if you have any interest in what we know about the process of making software.