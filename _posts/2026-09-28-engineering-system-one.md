---
layout: post
title: "Small Decisions: Engineering a Leading Model"

---
{{ page.title }}
================

<p class="meta">Or trying to, at least.</p>

My day job has been primarily in AI for three years now, but I'd be the first to admit that's been almost entirely in one corner of AI: infrastructure, safety, and tools for AI agents. That work has brought me in contact with a lot of the AI science (and I'd dabbled there over the previous decade), but I'm super far from the day-to-day of work like model building. I wanted to catch up a little (after all [you have to know what you're talking about](https://brooker.co.za/blog/2026/03/20/ic-leadership.html)), and the last couple weeks provided a perfect opportunity.

On the 15th of this month, the TypeSafe AI folks announced [Jev](https://typesafe.ai/blog/introducing-system-one-models-and-jev), a kind of general purpose calibrated classifier. You can see this as something exciting or not, but it sure has captured the world's attention. And mine. I was particularly interested in the calibration, combined with low latency and the ability to answer questions in parallel, it's a great building block for the more *workflowy* end of the spectrum of agents.

In an effort to understand these things well, it was time to build my own model: Hobson. A small one, because I wanted to use the GPU I have at home, and because I wanted to see if I could push the bounds on accuracy and calibration at very low latency. I decided to limit myself to about 2 billion parameters.

*How have I done so far?*

![](/blog/images/hobson_jevbench_trajectory.svg)

Fairly well, I think. You can read that as a trajectory of how versions of my model have performed on accuracy (on *x*) and calibration (on *y*) as I've made improvements. The pareto optimal is the bottom right. 

On the [jevbench public set](https://benchmarkheaven.com/jev-models) I'm at the top in my size range<sup>[1](#foot1)</sup>. There's a new version of [decider-2b](https://huggingface.co/Mapika/decider-2b) which beats me, but hasn't been added to the leaderboard yet. I handily beat Qwen3.5-2B on both calibration and accuracy. I'm measuring calibration with the multi-class [Brier score](https://en.wikipedia.org/wiki/Brier_score), roughly the mean-square prediction error in a range between 0 (perfect) and 2 (confidently wrong). Perhaps most usefully, Brier on the JevBench *easy* set is only 0.009 and accuracy is 100%, so we're very well calibrated for easy tasks.

![](/blog/images/hobson_jevbench_tiers.svg)

*How does it work?*

The core idea is that we take a pre-trained LLM torso (in this case Qwen3.5-2B), and rip off the LM head, and so remove its ability to generate text. The LM head is replaced with a *pointer head* which scores the answers offered by the torso for each option. It does this by scoring the hidden state at each option position against the hidden state at the `<answer>` position. This head is pretty small, just over a million total parameters. The torso is fine-tuned with a rank-16 LoRA adapter.

![](/blog/images/hobson_architecture.svg)

This approach appears to give better calibration than the simple approach of reading the logits at the output of the equivalent size LLM. It's fairly similar to the approach [Kev](https://github.com/jaredpalmer/kev) takes. 

My first attempt (inspired by a conversation with a colleague at work, so not original to me) was a *slot head* which only read the hidden states for each `<answer>` and passed it through a single linear layer with 24 fixed output slots. This was smaller (53k total parameters), but had some real disadvantages: a limit of 24 options, positional bias (it would learn things like 'the first option is often right'), and ignored the extra information in each option's hidden states. I initially experimented with a more complex slot head (2.1M params with a hidden layer), but that approach seemed like a dead end.

*Training*

The training approach is a fairly standard LoRA fine-tuning, with some self-distillation. The self-distillation was introduced to limit forgetting: the training process tends to make the fine-tuned model forget how to do tasks that it's already good at. The basic recipe is to use [KL](https://en.wikipedia.org/wiki/Kullback%E2%80%93Leibler_divergence) between the trainee and a frozen version of the torso (basic distillation), and KL to a previous version of the model on some tasks where I was seeing regressions. The rest is pretty standard: one epoch, cross-entropy to the gold options in the training set, and options shuffled on every example to stop learning positional lessons.

The data set is 115,000 rows, about 113k from public datasets, and 2k from synthetic 'hard' questions. None of the jevbench set is trained on, and the synthesis process doesn't know about it either (the model used for synthesis is about six months old). On the other hand, I have seen the *jevbench* examples, and I designed the synthesis process, so it all comes down to how sub subconsciously intellectually honest I am. Science is hard. 

I expected synthesis to be a big needle mover, by creating hard examples with the right structure. It helped, but wasn't huge. I suspect there's a good amount of juice left in that approach.

![](/blog/images/hobson_training_loss.svg)

Again, I'm not an expert, but the training loss graph looks pretty normal. Validation accuracy (not pictured) keeps climbing towards the end, even though loss stalls out. So no huge surprises.

After training, part of the held out data (so data I didn't train on) is used to calibrate scores. One *temperature* per question type (binary/noul, choice, score) is calculated that minimizes log loss on that question type (and so ideally improves Brier, but isn't guaranteed to). At inference time, this temperature is used to scale the confidence scores (by dividing the raw logits by the temperatures).

*Evaluation*

The rest of the held-out set is used for evaluation, including some examples from tasks in the training set (i.e. the kind of work is in the training set, but not the individual example), and some tasks entirely held out.

![](/blog/images/hobson_fit_vs_generalisation.svg)

One of the most important things we learn at this stage is how well the model generalizes. Can it do tasks that it hasn't seen before? After all, that's what makes this kind of model interesting versus a custom classifier. The answer is that even at this small size it generalizes usefully, but isn't great. As I've evolved the model, in-task accuracy has been much easier to move than generalization. I suspect this would be much easier with a bigger torso, but the rules of the game don't allow that approach.

*What's Next?*

There are a few things I want to try. Starting with more data synthesis, especially of harder problems. I think we're not yet close to tapped out on capabilities with this number of parameters. The other big one is some form of reinforcement learning, mostly seeing if that can help calibration and generalization, especially on end-to-end decision utility (e.g. with a workflow that 'does the thing if confidence >0.9', which can't be differentiated). Smaller ones include trying a few architectural tweaks, evaluating a second epoch or partial epoch, evaluating some different training schedules, larger LoRA ranks, and experimenting with other torsos (I tried *instruct* variants early on with negative results, but I'm not sold on that yet).

*Some of the Development Steps*

 - *v2* scaled the torso from Qwen3-1.7B to Qwen3-4B (before I set my 2B goal). Actually made performance worse, because the hold out set didn't have the right kind of tasks to see if it was better. Who among us hasn't come to the wrong conclusion from a bad benchmark?
 - *v6* was where I introduced the distillation technique, moving accuracy slightly.
 - *v7* started to feel like I was going somewhere. I switched here from the slot head to the pointer head, and that was our single biggest win so far.
 - *v8* through *v11* was a series of failed experiments: switching to Qwen3-1.7B-instruct, closed-form programmatic synthesis, playing with option order.
 - *v12* Switching to Qwen3-8B improved performance a lot (and matched SemIf's performance at this size), but I decided not to follow that path.
 - *v13* brought us back to the gold path, by switching to Qwen3.5-2B. I'd resisted that because it meant I needed to throw out some earlier work on optimizing inference, but it moved the needle significantly.
 - *v14*, *v16*, and *v18* expanded the training corpus substantially, with public data sets (ContractNLI, BoardgameQA, MuSiQue), and LLM-generated examples (Qwen3.5-27B) validated by an even bigger model (Qwen3.5-397B). This is fundamentally a data game, and these were also big wins.
 - *v17* was another change to training: to stop going backwards on performance on multi-step tasks, use *v14* as the teacher for some examples rather than *Qwen3.5-4B*. decider-2B does something similar (replay toward a parent rather than the base teacher). I don't love it, but it seems to work a little bit.

What worked: the head rearchitecture, a more modern and slightly bigger torso, document data with a teacher or verifier. What didn't: templated data synthesis, distillation on small/easy problems, some training data additions (notably ShARC and ConditionalQA). I think this is compatible with what [Zhang et al](https://arxiv.org/abs/2402.17193) found about fine tuning: more base params, more types of problems (different skills). More of the same data doesn't help a lot. 

*Lessons*

Maybe the biggest lesson here is how much easier it is to learn this stuff now than a year or so ago. Being able to ask Kiro or Claude to step me through concepts and then quiz me on my understanding was exceptionally helpful - it's like having a custom textbook about exactly this problem at just the right level for me. Every line of code was written by an agent, but at each step I tried to make sure the core ideas and insights were mine, or at least I understood them. I might not set such a bar for a project at work, but for this project the outcome was mostly about me learning.

I think I'll need to do this a few times before all the new concepts stick. I'm not yet at the point I could stand at a white board and walk through each decision, but I'm way further along that path than a week ago.

The other lesson is that even at 2B and below, we can build useful models of this class. That's obvious from the JevBench website too, but getting hands-on has really helped calibrate my thinking about this problem.

Finally, while this was fun, it showed how easy it is to get obsessed with this *number go up* model building game. People who had a bit of a, ah, *problem* with World of Warcraft or Diablo II should probably find another way to spend their time.

**Footnotes**

1. <a name="foot1"></a> joint first of 30 at 2B or below on the v1.4.2 board, level with decider-2b's v10 entry. There are two slightly larger models, around 2.5B, that do beat my model. I think I was legitimately in the lead for models around 2B for a while, maybe 24h. Interesting times.