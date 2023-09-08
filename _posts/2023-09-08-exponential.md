---
layout: post
title: "Exponential Value at Linear Cost"
---

{{ page.title }}
================

<p class="meta">What a deal!</p>

<script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
<script>
  MathJax = {
    tex: {inlineMath: [['$', '$'], ['\\(', '\\)']]}
  };
</script>
<script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
<script src="https://cdn.jsdelivr.net/npm/vega@5"></script>
<script src="https://cdn.jsdelivr.net/npm/vega-lite@4"></script>
<script src="https://cdn.jsdelivr.net/npm/vega-embed@6"></script>

Binary search is kind a of a magical thing. With each additional search step, the size of the haystack we can search doubles. In other words, the value of a search is *exponential* in the amount of effort. That's a great deal. There are a few similar deals like that in computing, but not many. How often, in life, do you get exponential value at linear cost?

Here's another important one: redundancy.

If we have $N$ hosts, each with availability $A$, any one of which can handle the full load of the system, the availability of the total system is:

$A_{system} = 1 - (1 - A)\^N$

It's hard to overstate how powerful this mechanism is, and how important it has been to the last couple decades of computer systems design. From RAID to cloud services, this is the core idea that makes them work. It's also a little hard to think about, because our puny human brains just can't comprehend the awesome power of exponents (mine can't at least).

If you want to try some numbers, give this a whirl:

<div id="vis"></div>

<script type="text/javascript">
  function make_data(n, host_avail, dc_avail) {
      let data = [];
      for (let i = 0; i < n; i++) {
        data.push({
          "x": i,
          "y": dc_avail * (1 - (1 - host_avail)**i),
        });
      }
      return data;
  }

  function updateView(view) {
    let new_data = make_data(view.signal('Hosts'), view.signal('HostAvail'), 1.0);
    view.change('points', vega.changeset().remove(vega.truthy).insert(new_data)).runAsync();
  }

  var spec = "https://brooker.co.za/blog/resources/redundancy_vega_lite_spec.json";
  vegaEmbed('#vis', spec).then(function(result) {
    updateView(result.view);
    result.view.addSignalListener('HostAvail', function(name, value) {
      updateView(result.view);
    });
    result.view.addSignalListener('Hosts', function(name, value) {
      updateView(result.view);
    });
  }).catch(console.error);
</script>

What you'll realize pretty quickly is that this effect is very hard to compete with. No matter how high you make the availability for a single host, even a very poor cluster quickly outperforms it in this simple model. Exponentiation is extremely powerful.

Unfortunately it's not all good news. This exponentially powerful effect only works when all these hosts fail independently. Let's extend the model just a little bit to include the effect of them being in the same datacenter, and that datacenter having availability $D$. We can easily show that the availability of the total system then becomes:

$A_{system} = D * (1 - (1 - A)\^N)$

Which doesn't look nearly as good.

<div id="vis2"></div>

<script type="text/javascript">
  function updateView2(view) {
    let new_data = make_data(view.signal('Hosts'), view.signal('HostAvail'), view.signal('DCAvail'));
    view.change('points', vega.changeset().remove(vega.truthy).insert(new_data)).runAsync();
  }

  var spec = "https://brooker.co.za/blog/resources/redundancy_2_vega_lite_spec.json";
  vegaEmbed('#vis2', spec).then(function(result) {
    updateView(result.view);
    result.view.addSignalListener('HostAvail', function(name, value) {
      updateView2(result.view);
    });
    result.view.addSignalListener('Hosts', function(name, value) {
      updateView2(result.view);
    });
    result.view.addSignalListener('DCAvail', function(name, value) {
      updateView2(result.view);
    });
  }).catch(console.error);
</script>

Which goes to show how quickly things go wrong when there's some correlation between the failures of redundant components. System designers must pay careful attention to ensuring that designs consider this effect, almost beyond all others, when designing distributed systems. Exponential goodness is our most powerful ally. Correlated failures are its kryptonite.

This observation is obviously fairly basic. It's also critically important.
