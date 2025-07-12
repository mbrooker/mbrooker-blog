---
layout: post
title: "Getting into formal specification, and getting my team into it too"






related_posts:
  - "/2014/08/09/formal-methods.html"
  - "/2022/06/02/formal.html"
  - "/2015/03/29/formal.html"
---
{{ page.title }}
================

<p class="meta">Getting started is the hard part</p>

*Sometimes I write long email replies to people at work asking me questions. Sometimes those emails seem like they could be useful to more than just the recipient. This is one of those emails: a reply to a software engineer asking me how they could adopt formal specification in their team, and how I got into it.*

Sometime around 2011 I was working on some major changes to the EBS control plane. We had this *anti-entropy* system, which had the job of converging the actual system state (e.g. the state of the volumes on the storage fleet, and clients on the EC2 fleet<sup>[1](#foot1)</sup>) with the intended system state in the control plane (e.g. the customer requested that this volume is deleted). We had a mess of ad-hoc code that took four sources of state (two storage servers, one EC2 client, the control plane), applied a lot of logic, and tried to figure out the steps to take to converge the states. Lots and lots of code. Debugging it was hard, and bugs were frequent.

Most painfully, I think, wasn't that the bugs were frequent. It's that they came in bursts. The code would behave for months, then there would be a network partition, or a change in another system, and loads of weird stuff would happen all at once. Then we'd try to fix something, and it'd just break in another way.

So we all took a day and drew up a huge state table on this big whiteboard in the hall, and circles and arrows showing the state transitions we wanted. A day well spent: we simplified the code significantly, and whacked a lot of bugs. But I wanted to do better. Specifically, I wanted to be able to know whether this mess of circles and arrows would always converge the state. I went looking for tools, and found and used [Alloy](https://alloytools.org/) for a while. Then Marc Levy introduced me to [Spin](https://spinroot.com/spin/whatispin.html), which I used for a while but never became particularly comfortable with.

The next year we were trying to reason through some changes to replication in EBS, and especially the control plane's role in ensuring correctness<sup>[2](#foot2)</sup>. I was struggling to use Alloy to demonstrate the properties I cared about<sup>[3](#foot3)</sup>. As something of a stroke of luck, I went to a talk by Chris Newcombe and Tim Rath titled "Debugging Designs" about their work applying formal specification to DynamoDB and Aurora. That talk gave me the tool I needed: [TLA+](https://lamport.azurewebsites.net/tla/tla.html).

Over the next couple years, I used TLA+ heavily on EBS, and got a couple of like-minded folks into it too. It resonated best with people who saw the same core problem I did: it was too hard to get the kinds of distributed software we were building right, and testing wasn't solving our problems. I think of this as a kind of mix of hubris (*software can be correct*), humility (*I can't write correct software*) and laziness (*I don't want to fix this again*). Some people just didn't believe that it was a battle that could be won, and some hadn't yet burned their fingers enough to believe they couldn't win it without help.

Somewhere along the line, Chris lead us in writing the paper that became [How Amazon Web Services Uses Formal Methods](https://cacm.acm.org/magazines/2015/4/184701-how-amazon-web-services-uses-formal-methods/fulltext), which appeared on Leslie Lamports's website in 2014 and eventually in CACM in 2015. We spent some time with Leslie Lamport talking about the paper (which was a real thrill), and he wrote [Who Builds a House Without Drawing Blueprints?](https://cacm.acm.org/magazines/2015/4/184705-who-builds-a-house-without-drawing-blueprints/fulltext), framing our paper. I also tried to convince him that TLA+ would be nicer to write with a Scheme-style s-expression syntax<sup>[7](#foot7)</sup>. He didn't buy it.

Since then, I've used TLA+ to specify core properties of things I care about in every team I've been on at AWS. More replication work in EBS, state convergence work in Lambda, better configuration distribution protocols, trying to prevent VM snapshots [returning duplicate random numbers](https://arxiv.org/abs/2102.12892), and now a lot of work in distributed databases. Byron Cook, Neha Rungta<sup>[4](#foot4)</sup>, Murat Demirbas, and many other people who are actual formal methods experts (unlike me) joined, and have been doing some great work across the company. Overall, I probably reach for TLA+ (or, increasingly, [P](https://github.com/p-org/P)) every couple months, but when I do it adds a lot of value. Teams around me are looking at [Shuttle](https://github.com/awslabs/shuttle) and [Dafny](https://github.com/dafny-lang/dafny), and some other tools. And, of course, there's the work S3 continues to do on [lightweight formal methods](https://www.amazon.science/publications/using-lightweight-formal-methods-to-validate-a-key-value-storage-node-in-amazon-s3). I'm also using [simulation](https://brooker.co.za/blog/2022/04/11/simulation.html) more and more (or getting back into it, my PhD work was focused on simulation).

So how do you get into it? First, recognize that it's going to take some time. P is a little easier to pick up, but TLA+ does take a bit of effort to learn<sup>[5](#foot5)</sup>. It also requires some math. Not a lot - just logic and basic set theory - but some. For me, spending that effort requires a motivating example. The best ones are where there's a clear customer benefit to improving the quality of the code, the problem is a tricky distributed protocol or security boundary<sup>[6](#foot6)</sup> or something else that really really needs to be right, and there's a will to get it right. Sometimes, you have to create the will. Talk about the risks of failure, and how teams across the company have found it hard to build correct systems without formal specification. Get people on your side. Find the folks in the team with the right level of hubris and humility, and try get them excited to join you.

Whether formal specification will be worth it depends a lot on your problems. I've mostly used it for distributed and concurrent protocols. Tricky business logic (like the volume state merge I mentioned) can definitely benefit. I'm not very experienced in code verification, but clearly there's a lot of value in tools that can reason directly about code. I've been meaning to get into that when I have some time. But mostly, you need to have an example where correctness really matters to your customers, your business, or your team. Those aren't hard to find around here, but there might happen to not be many of them near you.

 **Footnotes**

 1. <a name="foot1"></a> If you're interested in what these words mean, Marc Olson and Prarthana Karmaker did a talk at ReInvent 2021 titled [Amazon EBS under the hood: A tech deep dive](https://www.youtube.com/watch?v=kaWzAEVZ6k8). Some of the background is also covered in our [Millions of Tiny Databases](https://www.usenix.org/conference/nsdi20/presentation/brooker) paper.
 2. <a name="foot2"></a> This work eventually morphed into Physalia, as we describe in [Millions of Tiny Databases](https://www.usenix.org/conference/nsdi20/presentation/brooker).
 3. <a name="foot3"></a> My choice of Alloy was inspired by reading Pamela Zave's [work on Chord](http://www.pamelazave.com/chord.html), especially [Using Lightweight Modeling To Understand Chord](http://www.pamelazave.com/chord-ccr.pdf), but it's never felt like the right tool for that kind of job. It's really nice for other things, though.
 4. <a name="foot4"></a> There's a nice interview with Neha about her career path [here](https://www.amazon.science/working-at-amazon-from-nasa-ames-research-center-to-automated-reasoning-group-aws-neha-rungta).
 5. <a name="foot5"></a> Although resources like Hillel Wayne's [Learn TLA+](https://www.learntla.com/) have made it a lot more approachable. Lamport's [Specifying Systems](https://smile.amazon.com/Specifying-Systems-Language-Hardware-Engineers/dp/032114306X/) isn't a hard book, and is well worth picking up, but doesn't hold your hand.
 6. <a name="foot6"></a> See, for example, the work the Kani folks have done on Firecracker in [Using the Kani Rust Verifier on a Firecracker Example](https://model-checking.github.io/kani-verifier-blog/2022/07/13/using-the-kani-rust-verifier-on-a-firecracker-example.html), or this video with [Byron Cook talking about formal methods and security](https://www.youtube.com/watch?v=J9Da3VsLH44).
 7. <a name="foot7"></a> I still don't like the TLA+ syntax. It's nice to read, but the whitespace rules are weird, and the operators are a bit weird, and I think that makes it less accessible for no particularly good reason. And don't get me started on the printed documentation using a different character set (e.g. real ∃, ∀, ∈ rather than their escaped variants). It seems like a minor thing, but boy did I find it challenging starting out.