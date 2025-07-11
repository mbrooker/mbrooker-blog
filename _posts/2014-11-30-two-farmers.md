---
layout: post
title: "Two Farmers and Common Knowledge"



related_posts:
  - "/2014/10/25/ice-cream"
  - "/2015/03/03/sybil"
  - "/2014/11/15/exactly-once"
---{{ page.title }}
================

<p class="meta">A legislative solution to a technical problem.</p>

When their beloved father passed away, Jan and Marie inherited his famous wine estate. Jan and his family were given the historic homestead and half the grape harvesting equipment. Marie's family were given the other half, and a graceful home with stunning views down a craggy valley in the Bottelary Hills. As a final practical joke, the old man put strange condition into his will: one small vineyard was split between them, and they could only keep their farms if they met once a year on the same morning with all their inherited equipment to harvest the grapes together. The will insisted that they meet together at the vineyard, at a time when the grapes were perfectly ripe.

The first few years went well. When either decided that the next day would be right for the harvest, they would send a farm worker on his bicycle to tell the other. Farm workers are highly reliable, and they harvested simultaneously for many years. Then, one year around Christmas, a Karaoke joint opened the the village between the two homes. It's a poorly kept secret that farmers and farm workers love Karaoke, and the temptation to drop in to croon along with Sinatra often turned out to be too strong for the passing bicycling farmer worker. The workers loved karaoke so much, they could sing essentially forever.

In early July of the next year, Jan and Marie met to make a plan for that year's harvest. Their farm's remote locations meant that the telephone was out of the question, and SMS and email were yet to be discovered. They needed some way to make their cycling workers reliable again, and that meant finding a technical solution to the problem.

"Ok," said Jan. "All you need to do is send another message back when you get my message. When I get that message, I know you got my message."

"Don't be dumb Jan. What if that one doesn't arrive?"

"Oh. Then you need to send a message back saying you got that message"

"Ja. Then you know that I know that you know about the harvest, and I know that you know."

"But if that one doesn't come, then you don't know that I know that you know that I know."

"One more message, and I know you know that I know, and you know that I know that you know that I know. Right?"

Some hours, and most of a bottle of pot-stilled brandy, later Jan and Marie were still arguing.

"Then one more guy, and you know I know you know I know you know I know you know I know..." Marie counted off the "you knows" and "I knows", and Jan kept the tally with bottle corks. As the cork pile grew, the pair realized the approach wasn't going to work, and talk shifted to discouraging Karaoke in the community. After making a plan to ask for it to be mentioned in that Sunday's sermon, and drafting a list of reasons it wasn't moral, Marie realized it was in vain.

"Any chance, Jan. Any small chance and it's all for [niks](http://en.wiktionary.org/wiki/niks#Afrikaans). The messenger doesn't have to get lost, it's enough for one of us not to be sure. Besides, nobody listens to the [dominee](http://en.wiktionary.org/wiki/dominee#Afrikaans)."

The two farmers continued to struggle with the problem for days, without any result. Eventually, on a visit to the local library, they came across a [paper from 1979](http://65.54.113.26/Publication/3768450/some-issues-in-distributed-processes-communication) by Yemini and Cohen. In it, they found their worst fears confirmed: they were going to lose the farm. As long as there is any probability that a messenger gets lost, no algorithm can guarantee they meet on the same morning to harvest. It got worse. In a 1984 [paper by Halpern and Moses](https://www.cs.cornell.edu/home/halpern/papers/common_knowledge.pdf) they found that even if all the messengers did eventually arrive, they still couldn't agree, unless the amount of Karaoke they sung was bounded. They felt like all hope was lost.

Just as they were ready to leave the library in desperation, Jan read something that made his heart jump:

> On the other hand, if messages are guaranteed to be delivered within ε units of time, then ε-coordinated attack can be accomplished.

"Hey Marie! Did you see that paper by [Fagin](http://researcher.watson.ibm.com/researcher/files/us-fagin/apal99.pdf)? We need to read that Halpern and Moses one again!"

"Moses?"

"Probably not the same Moses."

Jan and Marie made a copy of the paper, and headed home. Putting the Halpern and Moses paper next to their father's will, they discovered something amazing. The will didn't say they have to meet at the field at exactly the same time! It just had to be the same morning. If they could set the maximum amount of Karaoke a messenger could sing to ε, they could meet at the fields within ε of each other. As long as ε is less than a morning, they could keep the farm. Being a well-funded land owner, Marie's run for local government was as quick as her career was short. She stayed long enough to propose only one law. To this day, it's illegal to sing karaoke for more than ε seconds in that small [dorp](http://en.wiktionary.org/wiki/dorp#English) in that valley in the Bottelary Hills.