---
layout: post
title: "Serial, Parallel, and Quorum Latencies"




related_posts:
  - "/2023/09/08/exponential"
  - "/2024/02/12/parameters"
  - "/2022/04/11/simulation"
---
{{ page.title }}
================

<p class="meta">Why are they letting me write Javascript?</p>

I've written [before](https://brooker.co.za/blog/2021/04/19/latency.html) about the latency effects of series (do X, then Y), parallel (do X and Y, wait for them both), and quorum (do X, Y and Z, return when two of them are done) systems. The effects of these different approaches to doing multiple things are quite intuitive. What may not be intuitive, though, is the impact of quorums, and how much quorums can reduce tail latency.

So I put together this little toy simulator.

The knobs are:

 - **serial** The number of things to do in a *chain*.
 - **parallel** The number of parallel *chains*.
 - **quorum** The number of chains we wait to complete before being done.
 - **runs** How many times to sample.

 So, for example, a traditional 3-of-5 Paxos system would have serial=1, parallel=5, and quorum=3. A length-3 chain replication system would have serial=3, parallel=1, quorum=1. The per-node service time distribution is (for now) assumed to be exponentially distributed with mean 1.

 <div id="vis"></div>

<script src="https://cdn.jsdelivr.net/npm/vega@5"></script>
<script src="https://cdn.jsdelivr.net/npm/vega-lite@4"></script>
<script src="https://cdn.jsdelivr.net/npm/vega-embed@6"></script>

<script type="text/javascript">
  // Generate `n` samples, exponentially distributed with `lambda = 1.0` (i.e. a mean of 1)
  function samples(n) {
    let data = [];
    for (let i = 0; i < n; i++) {
      data.push(-Math.log(Math.random()));
    }
    return data;
  }

  function makeSerial(n, serial) {
    let totals = [];
    for (let i = 0; i < n; i++) {
      let sample = samples(serial).reduce((a, b) => a + b, 0);
      totals.push(sample);
    }
    return totals;
  }

  function simulate(n, serial, parallel, quorum) {
    let results = [];
    for (let i = 0; i < n; i++) {
      // Each sample starts with `parallel` serial chains, each of length `serial`
      let serial_samples = makeSerial(parallel, serial);
      // Then we sort them, and take the highest `quorum`
      let sorted_samples = serial_samples.sort().slice(0, quorum);
      // And the result is the longest remaining sample
      results.push(sorted_samples[sorted_samples.length - 1]);
    }
    return results;
  }

  function arrayToData(arr) {
    return arr.map(function(v) { return {"u": v }; });
  }

  function updateView(view) {
    let new_data = simulate(view.signal('runs'), view.signal('serial'), view.signal('parallel'), view.signal('quorum'));
    view.change('points', vega.changeset().remove(vega.truthy).insert(arrayToData(new_data))).runAsync();
  }

  var spec = "https://brooker.co.za/blog/resources/simulation_vega_lite_spec.json";
  vegaEmbed('#vis', spec).then(function(result) {
    updateView(result.view);
    result.view.addSignalListener('serial', function(name, value) {
      updateView(result.view);
    });
    result.view.addSignalListener('parallel', function(name, value) {
      updateView(result.view);
    });
    result.view.addSignalListener('quorum', function(name, value) {
      updateView(result.view);
    });
    result.view.addSignalListener('runs', function(name, value) {
      updateView(result.view);
    });
  }).catch(console.error);
</script>

**Examples to Try**

 - Compare a 3-length chain to a 3-of-5 Paxos system. First, set serial=3, parallel=1, and quorum=1 and see how the 99th percentile latency is somewhere around 8s. Now, try serial=1, parallel=5, quorum=3. Notice how the 99th percentile is now just over 2ms. There's obviously a lot more to chain-vs-quorum in the real world than what is captured here.
 - Compare a 3-of-5 quorum to 4-of-7. The effect isn't as big here, but the bigger quorum leads to a nice reduction in high-percentile latency.
 - Check out the non-linear effect of longer serial chains. The 99th percentile doesn't increase by 10x between serial=1 and serial=10. Why?
 
 Have fun!