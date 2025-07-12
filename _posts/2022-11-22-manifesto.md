---
layout: post
title: "Amazon's Distributed Computing Manifesto"




related_posts:
  - "/2024/06/04/scale"
  - "/2021/01/22/cloud-scale"
  - "/2019/03/17/control"
---
{{ page.title }}
================

<p class="meta">Manifesto made manifest.</p>

In the Johannesburg of 1998, I was rocking a middle parting, my friend group was abuzz about the news that there was water (and therefore monsters) on Europa, and all the cool kids were getting satellite TV at home<sup>[1](#foot1)</sup>. Over in Seattle, the folks at Amazon.com had started to notice that their architecture was in need of rethinking. [$147 million](https://d18rn0p25nwr6d.cloudfront.net/CIK-0001018724/96985bfb-79b1-41e9-b552-fd5ad5af6fd3.pdf) in sales in 1997, and over $600 million in 1998, were proving to be challenging to deal with. In 1998, as [Werner Vogels recently shared](https://www.allthingsdistributed.com/2022/11/amazon-1998-distributed-computing-manifesto.html) folks at Amazon wrote a *distributed computing manifesto* describing the problems they were seeing and the solutions they saw to those problems.

The document itself, which you can (and should!) [read in full over on Werner's blog](https://www.allthingsdistributed.com/2022/11/amazon-1998-distributed-computing-manifesto.html) is both something of a time capsule, and surprisingly relevant to many of the systems architecture debates going on today, and the challenges that nearly all growing architectures inevitably face.

From the manifesto:

> The applications that run the business access the database directly and have knowledge of the data model embedded in them. This means that there is a very tight coupling between the applications and the data model, and data model changes have to be accompanied by application changes even if functionality remains the same.

Despite being called a *distributed computing manifesto*, the Amazon of 1997 was already a distributed system by any reasonable measure. The problem was one of interfaces: the data store was serving as the interface between components and concerns, leading to tight coupling between storage and business logic. The architecture was difficult to scale, not in requests per second, but to adapt to new lines of business and the rate of overall change.

> This approach does not scale well and makes distributing and segregating processing based on where data is located difficult since the applications are sensitive to the interdependent relationships between data elements.

The proposed solution is services. This document predates the term microservices<sup>[2](#foot2)</sup>, but that's pretty much what they were talking about. Moving the data behind well-defined interface that encapsulate business logic, reducing the coupling between different parts of the system.

> We propose moving towards a three-tier architecture where presentation (client), business logic and data are separated. This has also been called a service-based architecture. The applications (clients) would no longer be able to access the database directly, but only through a well-defined interface that encapsulates the business logic required to perform the function.

Perhaps the most interesting part of the manifesto for me is the description of the cultural change that needs to go along with the change in architecture. Merely drawing a different block diagram wasn't going to be enough to get the outcome the authors wanted.

> There are several important implications that have to be considered as we move toward a service-based model.
> ...
> A second implication of a service-based approach, which is related to the first, is the significant mindset shift that will be required of all software developers. Our current mindset is data-centric, and when we model a business requirement, we do so using a data-centric approach. Our solutions involve making the database table or column changes to implement the solution and we embed the data model within the accessing application. The service-based approach will require us to break the solution to business requirements into at least two pieces. The first piece is the modeling of the relationship between data elements just as we always have. This includes the data model and the business rules that will be enforced in the service(s) that interact with the data. However, the second piece is something we have never done before, which is designing the interface between the client and the service so that the underlying data model is not exposed to or relied upon by the client.

This mindset shift - from database schema to API - has been fundamental to the rise of SoA and microservices as the default architecture over the last two decades. Now, in 2022, with embedded databases and two-tier architectures coming back in fashion, it's interesting to see data-centric thinking somewhat converge with API-centric thinking. A broad toolkit is a good thing, but one hopes that the architects of this new generation of two-tier systems are learning from the lessons of the multi-gigabyte monoliths of old<sup>[3](#foot3)</sup>.

Another groundbreaking part of the manifesto was thinking about the role of workflows in distributed architectures. Starting with the observation that the existing order flow, despite its tight coupling on the backend, was already a workflow:

> We already have an "order pipeline" that is acted upon by various business processes from the time a customer order is placed to the time it is shipped out the door. Much of our processing is already workflow-oriented, albeit the workflow "elements" are static, residing principally in a single database. 

the scaling challenges of that architecture:

> ...the current database workflow model will not scale well because processing is being performed against a central instance. As the amount of work increases..., the amount of processing against the central instance will increase to a point where it is no longer sustainable. A solution to this is to distribute the workflow processing so that it can be offloaded from the central instance.

and the prescription:

> Instead of processes coming to the data, the data would travel to the process.

When I started at Amazon a decade later, I found this way of thinking enlightening. Before I joined Amazon, I spent some time thinking about how to distribute radar simulations, an interestingly compute- and data-heavy workflow problem. Google's [MapReduce](http://static.googleusercontent.com/media/research.google.com/es/us/archive/mapreduce-osdi04.pdf) paper had come out in 2004, and had become something of a ubiquitous model for data-centric distributed communication. We made some attempts to apply MapReduce to our problems, without success. I can't help but wonder if I had seen this writing from Amazon about workflows at the time whether we would have had a lot more success with that model.

The *manifesto* is a fascinating piece of history, both of Amazon's technical approach, and of the effect that web scale was having on the architectures of distributed systems. A lot has changed in the industry since then, and Amazon's approach has evolved significantly, but there are still fascinating lessons here.

**Footnotes**

1. <a name="foot1"></a> Despite my rocking 'do, I was not one of the cool kids, if you can believe it.
2. <a name="foot2"></a> At least in its current definition. 
3. <a name="foot3"></a> As [James Hamilton has talked about](http://hpts.ws/papers/2022/JamesHamilton20221010.pdf), one of Amazon's monoliths (Obidos) was big enough in the early 2000s that it was becoming impossible to link on a 32 bit machine. In many ways the size, and unreliability, of Obidos informed a lot of the *reliable system from unreliable parts* thinking that went into AWS's early architecture later in the same decade.