---
layout: post
title: "Better Benchmarks Through Graphs"
---

{{ page.title }}
================

<p class="meta">Isn't the ambiguity in the word *graphs* fun?</p>

<script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
<script>
  MathJax = {
    tex: {inlineMath: [['$', '$'], ['\\(', '\\)']]}
  };
</script>
<script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>

*This is a blog post version of a talk I gave at the Northwest Database Society meeting last week. The [slides are here](https://brooker.co.za/blog/resources/nwds_mbrooker_feb_2024.pdf), but I don't believe the talk was recorded.*

I believe that one of the things that's holding back databases as an engineering discipline (and why so much remains stubbornly opinion-based) is a lack of good benchmarks, especially ones available at the design stage. The gold standard is designing for and benchmarking against real application workloads, but there are some significant challenges achieving this ideal. One challenge<sup>[1](#foot1)</sup> is that, as in any system with concurrency, *traces* capture the behavior of the application running on another system, and they might have issued different operations in a different order running on this one (for example, think about how in most traces it's hard to tell the difference between *application thinking* and *application waiting for data*, which could heavily influence results if we're trying to understand the effect of speeding up the *waiting for data* portion). Running real applications is better, but is costly and raises questions of access (not all customers, rightfully, are comfortable handing their applications over to their DB vendor).

Industry-standard benchmarks like TPC-C, TPC-E, and YCSB exist. They're widely used, because they're easy to run, repeatable, and form a common vocabulary for comparing the performance of systems. On the other hand, these benchmarks are known to be poorly representative of real-world workloads. For the purposes of this post, mostly that's because they're *too easy*. We'll get to what that means later. First, here's why it matters.

Designing, optimizing, or improving a database system requires a lot of choices and trade-offs. Some of these are big (optimistic vs pessimistic, distributed vs single machine, multi-writer vs single-writer, optimizing for reads or writes, etc), but there are also thousands of small ones ("*how much time should I spend optimizing this critical section?*"). We want benchmarks that will shine light on these decisions, allowing us to make them in a quantitative way.

Let's focus on just a few of the decisions the database system engineer makes: how to implement *atomicity*, *isolation*, and *durability* in a distributed database. Three of the factors that matter there are transaction size (*how many rows?*), locality (*is the same data accessed together all the time?*), and coordination (*how many machines need to make a decision together?*). Just across these three factors, the design that's *best* can vary widely.

![](/blog/images/wsz_axes.png)

If we think of these three factors as ones that define a space<sup>[2](#foot2)</sup>. At each point in this space, keeping other concerns constant, some design is *best*. Our next challenge is generating synthetic workloads---fake applications---for each point of the space. Standard approaches to benchmarking sample this space sparsely, and the industry-standard ones do it extremely poorly. In the search for a better way, we can turn, as computer scientist so often do, to graphs.

![](/blog/images/wsz_graph.png)

In this graph, each row (or other object) in our database is a node, and the edges mean *transacted with*. So two nodes are connected by a (potentially weighted) edge if they appear together in a transaction. We can then generate example transactions by taking a random walk through this graph of whatever length we need to get transactions of the right size.

The graph model seems abstract, but is immediately useful in allowing us to think about why some of the standard benchmarks are so easy. Here's the graph of write-write edges for TPC-C *neworder* (with one warehouse), for example.

![](/blog/images/wsz_tpcc.png)

Notice how it has 10 disjoint islands. One thing that allows us to see is that we could immediately partition this workload into 10 shards, without ever having to execute a distributed protocol for *atomicity* or *isolation*. Immediately, that's going to look flattering to a distributed database architecture. This graph-based way of thinking is generally a great way of thinking about the partitionability of workloads. Partitioning is trying to draw a line through that graph which cuts as few edges as possible<sup>[3](#foot3)</sup>.

If we're comfortable that graphs are a good way of modelling this problem, and random walks over those graphs<sup>[4](#foot4)</sup> are a good way to generate workloads with a particular shape, we can ask the next question: how do we generate graphs with the properties we want? Generating graphs with particular shapes is a classic problem, but one approach I've found particularly useful is based on [the small-world networks model](http://worrydream.com/refs/Watts-CollectiveDynamicsOfSmallWorldNetworks.pdf) from Watts and Strogatz<sup>[6](#foot6)</sup>. This model gives us a parameter $p$ which, which allows us to vary between *ring lattices* (the simplest graph with a particular constant degree), and completely random graphs. Over the range of $p$, long-range connections form across broad areas of the graph, which seem to correlate very well with the *contention* patterns we're interested in exploring.

![](/blog/images/wsz_ws.png)

That gives us two of the parameters we're interested in: transaction size is set by the length of random walks we do, and coordination which is set by adjusting $p$. We haven't yet solved *locality*. In our experiments, locality is closely related to *degree distribution*, which the Watts-Strogatz model doesn't control very well. We can easily control the central tendency of that distribution (by setting the initial degree of the ring lattice we started from), but can't really simulate the outliers in the distribution that model things like *hot keys*.

In the procedure for creating these Watts-Strogatz graph, the targets of the *rewirings* from the ring lattice are chosen uniformly. We can make the degree distribution more extreme by choosing non-uniformly, such as with a Zipf distribution (even though Zipf [seems to be a poor match for real-world distributions in many cases](https://brooker.co.za/blog/2023/02/07/hot-keys.html)). This lets us create a Watt-Strogatz-Zipf model.

![](/blog/images/wsz_wsz.png)

Notice how we have introduced a hot key (near the bottom right). Even if we start our random walk uniformly, we're quite likely to end up there. This kind of internal hot key is fairly common in relational transactional workloads (for example, secondary indexes with low cardinality, or dense auto-increment keys).

This approach to generating benchmark loads has turned out to be very useful. I like how flexible it is, how we can generate workloads with nearly any characteristics, and how well it maps to other graph-based ways of thinking about databases. I don't love how the relationship between the parameters and the output characteristics is non-linear in a potentially surprising way. Overall, this post and talk were just scratching the surface of a deep topic, and there's a lot more we could talk about.

**Play With the Watts-Strogatz Model**

<!-- Generated by GPT-4 with the prompt: "write an html5/js file that does the following:

large square canvas
draw a 20 node graph, follows the "small world networks" model
add a slider that allows the user to change the value of the p parameter" -->

<canvas id="graphCanvas" width="600" height="600"></canvas>
<input type="range" id="pSlider" min="0" max="1" step="0.01" value="0">
<script>
        const canvas = document.getElementById('graphCanvas');
        const ctx = canvas.getContext('2d');
        const slider = document.getElementById('pSlider');
        const nodeCount = 20;
        const radius = 250; // Radius for nodes layout in a circle
        const centerX = canvas.width / 2;
        const centerY = canvas.height / 2;

        function generateGraph(p) {
            let nodes = [];
            let edges = new Map();

            // Initialize nodes and place them in a circle
            for (let i = 0; i < nodeCount; i++) {
                let angle = (i / nodeCount) * 2 * Math.PI;
                nodes.push({
                    x: centerX + radius * Math.cos(angle),
                    y: centerY + radius * Math.sin(angle),
                });
            }

            // Create a ring lattice with k/2 neighbors each side
            let k = 4; // Number of nearest neighbors (assumed even for simplicity)
            for (let i = 0; i < nodeCount; i++) {
                for (let j = 1; j <= k / 2; j++) {
                    let neighbor = (i + j) % nodeCount;
                    if (!edges.has(i)) edges.set(i, new Set());
                    if (!edges.has(neighbor)) edges.set(neighbor, new Set());
                    edges.get(i).add(neighbor);
                    edges.get(neighbor).add(i); // Assuming undirected graph
                }
            }

            // Rewire edges with probability p
            edges.forEach((value, key) => {
                value.forEach(neighbor => {
                    if (Math.random() < p) {
                        let oldNeighbor = neighbor;
                        let newNeighbor;
                        do {
                            newNeighbor = Math.floor(Math.random() * nodeCount);
                        } while (newNeighbor === key || edges.get(key).has(newNeighbor));
                        edges.get(key).delete(oldNeighbor);
                        edges.get(key).add(newNeighbor);
                        edges.get(newNeighbor).add(key); // Assuming undirected graph
                    }
                });
            });

            return { nodes, edges };
        }

        function drawGraph(graph) {
            ctx.clearRect(0, 0, canvas.width, canvas.height); // Clear the canvas

            // Draw edges
            graph.edges.forEach((value, key) => {
                value.forEach(neighbor => {
                    ctx.beginPath();
                    ctx.moveTo(graph.nodes[key].x, graph.nodes[key].y);
                    ctx.lineTo(graph.nodes[neighbor].x, graph.nodes[neighbor].y);
                    ctx.stroke();
                });
            });

            // Draw nodes
            graph.nodes.forEach(node => {
                ctx.beginPath();
                ctx.arc(node.x, node.y, 5, 0, 2 * Math.PI);
                ctx.fill();
            });
        }

        function updateGraph() {
            const p = parseFloat(slider.value);
            const graph = generateGraph(p);
            drawGraph(graph);
        }

        slider.addEventListener('input', updateGraph);

        // Initial drawing
        updateGraph();
</script>

**Footnotes**

1. <a name="foot1"></a> There's an excellent discussion of more problems with traces in Traeger et al's [A Nine Year Study of File System and Storage Benchmarking](https://www.fsl.cs.sunysb.edu/docs/fsbench/fsbench-tr.html#sec:traces).
2. <a name="foot2"></a> I've drawn them here as orthogonal, which they aren't in reality. Let's hand-wave our way past that.
3. <a name="foot3"></a> This general way of thinking dates back to at least 1992's [On the performance of object clustering techniques](https://dl.acm.org/doi/pdf/10.1145/130283.130308) by Tsangaris et al (this paper's *Expansion Factor*, from section 2.1, is a nice way of thinking about distributed databases scalability in general). Thanks to Joe Hellerstein for pointing this paper out to me. More recently, papers like [Schism](https://dl.acm.org/doi/10.14778/1920841.1920853) and [Chiller](https://dl.acm.org/doi/abs/10.1145/3471485.3471490) have made use of it.
4. <a name="foot4"></a> There's a lot to be said about the relationship between the shape of graphs and the properties of random walks over those graphs. Most of it would need to be said by somebody more competent in this area of mathematics than I am.
5. <a name="foot5"></a> The degree distribution of these small-world networks is a whole deep topic of its own. Roughly, there's a big spike at the degree of the original ring lattice, and the distribution decays exponentially away from that (with the exponent related to $p$).
6. <a name="foot6"></a> Google Scholar lists nearly 54000 citations for this paper, so its not exactly obscure.