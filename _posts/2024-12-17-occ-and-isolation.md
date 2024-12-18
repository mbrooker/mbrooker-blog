---
layout: post
title: "Snapshot Isolation vs Serializability"
---

{{ page.title }}
================

<script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
<script>
  MathJax = {
    tex: {inlineMath: [['$', '$'], ['\\(', '\\)']]}
  };
</script>
<script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>

<p class="meta">Getting into some fundamentals.</p>

In my [re:Invent talk on the internals of Aurora DSQL](https://www.youtube.com/watch?v=huGmR_mi5dQ) I mentioned that I think snapshot isolation is a sweet spot in the database isolation spectrum for most kinds of applications. Today, I want to dive in a little deeper into why I think that, and some of the trade-offs of going stronger and weaker.

This post is going to be a little deeper than the last few. If you're not deeply familiar with SQL's isolation levels, I recommend checking out Crooks et al's [Seeing is Believing: A Client-Centric Specification of Database Isolation](https://dl.acm.org/doi/10.1145/3087801.3087802), Berenson et al's [A Critique of ANSI SQL Isolation Levels](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/tr-95-51.pdf), or Adya et al's [Generalized Isolation Level Definitions](https://pmg.csail.mit.edu/papers/icde00.pdf).

Specifically, I'm going to talk about one very specific mental model of transaction isolation: read-write conflicts, and write-write conflicts.

Let's start our journey with a transaction, `T1`:

    BEGIN;
    SELECT amnt FROM food WHERE id = 1 OR id = 2;
    UPDATE food SET amnt = amnt - 1 WHERE id = 1;
    COMMIT;

There are a few things to notice about this transaction, when run in a database system that offers interactive (i.e. back-and-forth with the client) transactions:

 * It directly reads two rows from the database, the rows `1` and `2` from the table `food`.
 * It directly writes one row in the database, the rows `1` from the table food.
 * It starts and ends at different times: starting during `BEGIN` and ending during `COMMIT`.
 * Depending on the way that database is implemented, it also likely reads other data (e.g. the system catalog which tells it which tables exist), and may write other data (e.g. a secondary index on the `amnt` column of `food`).

The forth point here is critical in real systems, but let's ignore it for now and focus only on the first three. Before we do that, let's introduce a second transaction, the first one's parallel universe clone, `T2`. We'll also assume these are the only transactions running at this time.

    BEGIN;
    SELECT amnt FROM food WHERE id = 1 OR id = 2;
    UPDATE food SET amnt = amnt - 1 WHERE id = 2;
    COMMIT;

We'll go a step further and explicitly interleave these transactions (as though they were happening at the same time on two different connections to the same database), taking a page from [Hermitage](https://github.com/ept/hermitage/blob/master/postgres.md).

    BEGIN; -- T1, t1_start
    BEGIN; -- T2, t2_start
    SELECT amnt FROM food WHERE id = 1 OR id = 2; -- T1
    SELECT amnt FROM food WHERE id = 1 OR id = 2; -- T2
    UPDATE food SET amnt = amnt - 1 WHERE id = 1; -- T1
    UPDATE food SET amnt = amnt - 1 WHERE id = 1; -- T2
    COMMIT; -- T1, t1_commit
    COMMIT; -- T2, t2_commit, WHAT HAPPENS HERE?

What should happen to the second commit?

Accepting that second commit is an example of *write skew*, and is generally a good minimal example of the difference between snapshot isolation (SI) and serializability.

But that's well documented, and not what I'm focussing on. What I'm focussing on is how we might prevent write skew.

*Read Sets and Write Sets*

Let's abstract `T1` and `T2` a step further: into read sets and write sets. We'll simplify everything more here by pretending our database has a single table (`food`).

`T1` then has the read set $R_1 = {1,2}$ and the write set $W_1 = {1}$

`T2` then has the read set $R_2 = {1,2}$ and the write set $W_2 = {2}$

Under serializability, once `T1` is committed, `T2` can only commit if $R_2 \cap W_1 = \emptyset$ (or, more generally, the set of all writes accepted between `t2_start` and `t2_commit` does not intersect with `R_2`<sup>[1](#foot1)</sup>).

Under snapshot isolation, once `T1` is committed, `T2` can only commit if $W_2 \cap W_1 = \emptyset$ (or, more generally, the set of all writes accepted between `t2_start` and `t2_commit` does not intersect with `W_2`).

Ok?

Notice how similar those two statements are: one about $R_2 \cap W_1$ and one about $W_2 \cap W_1$. That's the only real difference in the rules.

But it's a *crucial* difference.

It's a crucial difference because of one of the cool and powerful things that SQL databases make easy: `SELECT`s. You can grow a transaction's write set with `UPDATE` and `INSERT` and friends, but most OLTP applications don't tend to. You can grow a transaction's read set with any `SELECT`, and many applications do that. If you don't believe me, go look at the ratio between predicate (i.e. not exact PK equality) `SELECT`s in your code base versus predicate `UPDATE`s and `INSERT`s. If the ratios are even close, you're a little unusual.

*Concurrency versus Isolation*

This is where we enter a world of trade-offs: avoiding SI's write skew requires the database to abort (or, sometimes, just block) transactions based on what they *read*. That's true for OCC, for PCC, or for nearly any scheme you can devise. To get good performance (and scalability) in the face of concurrency, applications using serializable isolation need to be extremely careful about *reads*.

The trouble is that isolation primarily exists to simplify the lives of application programmers, and make it so they don't have to deal with concurrency. SQL-like isolation models do that quite effectively, and are (in my opinion) one of the best ideas in the history of computing. But as we move up the isolation levels to serializability, we start pushing more complexity onto the application programmer by forcing them to worry more about concurrency from a performance and throughput perspective.

This is the cause of my belief that snapshot isolation, combined with strong consistency, is the right default for most applications and most teams of application programmers: it provides a useful minimum in the sum of worries about anomalies and performance.

Fundamentally, it does that by observing that read sets are smaller than write sets, for the majority of OLTP applications.

There is a set of folks online who don't believe that these trade-offs exist, but I believe those people are fundamentally wrong.

*How Low Can We Go? How Low Should We Go?*

That raises the question of whether its worth going even lower in the isolation spectrum. I don't have as crisp a general answer, but in Aurora DSQL's case the answer is, mostly *no*. Reducing the isolation level from SI to `read committed` does not save any distributed coordination, because of the use of physical clocks and MVCC to provide a consistent read snapshot to transactions without coordination. It does save some local coordination on each storage replica, and the implementation of multiversioning in storage, but our measurements indicate those are minimal.

If you'd like to learn more about that architecture, here's me talking about it at re:Invent 2024:

<iframe width="560" height="315" src="https://www.youtube-nocookie.com/embed/huGmR_mi5dQ?si=qXMxiImNZ5MGf9Co" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

*Footnotes*

1. <a name="foot1"></a> This general framing is my OCC backward validation bias showing, but it extends to most pessimistic approaches too.
