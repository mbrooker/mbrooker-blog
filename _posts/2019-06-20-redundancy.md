---
layout: post
title: "When Redundancy Actually Helps"


related_posts:
  - "/2020/01/02/why-distributed"
  - "/2022/01/31/deployments"
  - "/2024/06/04/scale"
---{{ page.title }}
================

<p class="meta">Redundancy can harm more than it helps.</p>

Just after I joined the EBS team at AWS in 2011, the service [suffered a major disruption](https://aws.amazon.com/message/65648/) lasting more than two days to full recovery. Recently, on Twitter, [Andrew Certain said](https://twitter.com/tacertain/status/1152459506464329729):

> We were super dependent on having a highly available network to make the replication work, so having two NICs and a second network fabric seemed to be a way to improve availability. But the lesson of this event is that only some forms of redundancy improve availability.

I've been thinking about the second part of that a lot recently, as my team starts building a new replicated system. When does redundancy actually help availability? I've been breaking that down into four rules:

1. The complexity added by introducing redundancy mustn't cost more availability than it adds.
2. The system must be able to run in degraded mode.
3. The system must reliably detect which of the redundant components are healthy and which are unhealthy.
4. The system must be able to return to fully redundant mode.

This might seem like obvious, even tautological, but each serves as the trigger of deeper thinking and conversation.

## Don't add more risk than you take away

Andrew (or Kerry Lee, I'm not sure which) introduced this to the EBS team as *don't be weird*.

<blockquote class="twitter-tweet" data-conversation="none" data-dnt="true"><p lang="en" dir="ltr">So I think it reinforces two lessons:<br><br>1/ Don&#39;t be weird<br>2/ Modality is bad</p>&mdash; Andrew Certain (@tacertain) <a href="https://twitter.com/tacertain/status/1152460786171707393?ref_src=twsrc%5Etfw">July 20, 2019</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script> 

This isn't a comment on people (who are more than welcome to be weird), but on systems. Weirdness and complexity add risk, both risk that we don't understand the system that we're building, and risk that we don't understand the system that we are operating. When adding redundancy to a system, it's easy to fall into the mistake of adding too much complexity, and underestimating the ways in which that complexity adds risk.

## You must be able to run in degraded mode

Once you've failed over to the redundant component, are you sure it's going to be able to take the load? Even in one of the simplest cases, active-passive database failover, this is a complex question. You're going from warm caches and full buffers to cold caches and empty buffers. Performance can differ significantly.

As systems get larger and more complex, the problem gets more difficult. What components do you expect to fail? How many at a time? How much traffic can each component handle? How do we stop our cost reduction and efficiency efforts from taking away the capacity needed to handle failures? How do we continuously test that the failover works? What mechanism do we have to make sure there's enough failover capacity? There's typically at least as much investment in answering these questions as building the redundant system in the first place.

Chaos testing, gamedays, and other similar approaches are very useful here, but typically can't test the biggest failure cases in a continuous way.

## You've got to fail over in the right direction

When systems suffer partial failure, it's often hard to tell what's *healthy* and what's *unhealthy*. In fact, different systems in different parts of the network often completely disagree on health. If your system sees partial failure and fails over towards the truly *unhealthy* side, you're in trouble. The complexity here comes from the distributed systems fog of war: telling the difference between bad networks, bad software, bad disks, and bad NICs can be surprisingly hard. Often, systems flap a bit before falling over.

## The system must be able to return to fully redundant mode

If your redundancy is a single shot, it's not going to add much availability in the long term. So you need to make sure the system can safely get from one to two, or N to N+1, or N to 2N. This is relatively easy in some kinds of systems, but anything with a non-zero RPO or asynchronous replication or periodic backups can make it extremely difficult. In small systems, human judgement can help. In larger systems, you need an automated plan. Most likely, you're going to make a better automated plan during daylight in the middle of the week during your design phase than at 3AM on a Saturday while trying to fix the outage.