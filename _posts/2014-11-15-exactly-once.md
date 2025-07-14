---
layout: post
title: "Exactly-Once Delivery May Not Be What You Want"









related_posts:
  - "/2014/01/12/ben-or.html"
  - "/2014/05/10/lynch-pub.html"
  - "/2021/04/14/redundancy.html"
dissimilar_posts:
  - "/2012/01/10/drive-failure.html"
---
{{ page.title }}
================

<p class="meta">It's hard to get, but that's OK, because you don't want it.</p>

Last week, there was a good discussion on [lobste.rs](http://lobste.rs) about [why exactly-once messaging is not possible](https://lobste.rs/s/ecjfcm/why_is_exactly-once_messaging_not_possible_in_a_distributed_queue). The discussion was kicked off with a link to a paper from Patel et al titled [Towards In-Order and Exactly-Once Delivery using Hierarchical Distributed Message Queues](http://datasys.cs.iit.edu/publications/2014_SCRAMBL14_HDMQ.pdf), which claims to contribute:

> ... a highly scalable distributed queue service using hierarchical architecture that supports exactly once delivery, message order, large message size, and message resilience.

I haven't evaluated the author's other claims in detail, but the claim of exactly once delivery caught my eye.

> There is no chance of getting two get requests for the same message. When a HTTP message request comes in, a message is sent through HTTP response and the message is deleted at the same time.

While I'm not fully satisfied about their *at the same time*, they don't seem to be claiming to break any fundamental laws here. What I do feel is fundamental, though, is that this definition of *exactly once delivery* isn't the one that most systems builders would find useful. The effect that most people are interested in is actually exactly-once processing: a message having a particularly side-effect exactly once per message.

I like to think about this in terms of redundancy. Fault-tolerant distributed systems deal with all kinds of failures, but it's often practically useful to break them into two categories: node failure and message loss. Node failures can be tolerated with redundancy *in space*, having multiple copies of a piece of data on multiple nodes. Message loss can be tolerated with redundancy *in time*, sending the same message multiple times if it doesn't seem to have been received. Replicated databases are redundant in space, and [TCP](http://en.wikipedia.org/wiki/Transmission_Control_Protocol) is a great example of redundancy in time. *Side note: I stole this characterization from the excellent talk titled [Outwards From The Middle of the Maze](http://www.youtube.com/watch?v=ggCffvKEJmQ) by Peter Alvaro.*

Think about a simple system making use of an *exactly once* queue. There's some producer (which we'll mostly ignore), the queue (which we'll pretend has magic durability and availability properties), and the focus of the discussion: a fleet of consumers. The producer makes tasks asking the consumers to alter the world in some way. We could build this system in a few ways:

 - The queue passes each task to exactly one consumer exactly once. If the consumer fails the task is lost, and the system does each task *at most once*.
 - We could ask the consumer to acknowledge the message once it's been processed. If the consumer fails to do that after some amount of time, the queue will offer it to another consumer. This makes system tolerant to consumer failure, but a consumer just stalling could cause it to pick up the work when it recovers, causing multiple delivery. This system ends up being *at least once*.
 - To fix the stall problem, we could put a timeout on the task itself, saying "don't perform this task at all if you can't get it done by five o'clock on Friday". While we can do this in a way that doesn't require the queue and consumer to synchronize their clocks, at least we have to depend on the relative rates of their clocks being bounded.
 - We could pass the task to multiple consumers, and ask them to co-ordinate amongst themselves which will execute it. That's a reasonable solution from the queue's perspective, but just moves the problem down to the consumer.

And so on. There's always a place to slot in [one more turtle](http://en.wikipedia.org/wiki/Turtles_all_the_way_down). The bad news is that I'm not aware of a nice solution to the general problem for all side effects, and I suspect that no such solution exists. On the bright side, there are some very nice solutions that work really well in practice. The simplest is [idempotence](http://queue.acm.org/detail.cfm?id=2187821). This is a very simple idea: we make the tasks have the same effect no matter how many times they are executed. 

Consider Bob, distributed systems enthusiast and pizza restaurateur. When people order from Bob, their orders go into a persistent queue. Bob's workers take a pizza order off the queue, bake it, deliver it, and go back to the queue. Occasionally one of Bob’s workers gets bored and leaves early in the middle of a task, in which case Bob gives the order to a different worker. Sometimes, this means that multiple pizzas arrive at the customer’s house (though never less than one pizza). Bob doesn't want people to end up with excess pizza, so he does something very smart: gives each order a unique identifier. On arriving, the pizza delivery guy asks the home owner if they had received a pizza with that order ID before. If the home owner says yes, the pizza guy takes the duplicate pie with him. If not, he leaves the pie. Each home owner gets exactly one pie, and everybody is happy.

In Bob's world, pizza baking and delivery is an *at least once* operation, but pizza delivery into the customer's house happens *exactly once* thanks to the fact that his deliveries are idempotent. Bob's obviously got a strong incentive to reduce pizza waste. He tries to make sure that *at least once* is also *approximately once*, which is easy most of the year, but can be a real challenge when it's stormy out and the big game is on.

I think there are two lessons here for people building distributed systems. One is that end-to-end system semantics matter much more than the semantics of an individual building block, and sometimes what seems like a very desirable semantic for a building block may end up making the end-to-end problem harder. The other is that simple, practical, solutions like unique IDs can make really hard problems much easier, and allow us to build and ship real systems that work in predictable ways.