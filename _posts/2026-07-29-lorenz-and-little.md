---
layout: post
title: "Lorenz and Little: How Much Does Your Tail Cost?"

---
{{ page.title }}
================

<p class="meta">Lorenz and Little sounds like hipster burger bar from 2015.</p>

<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism.min.css">
<script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/prism.min.js"></script>
<script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-python.min.js"></script>
<script>
  MathJax = {
    tex: {inlineMath: [['\\(', '\\)'], ['$', '$']]}
  };
</script>
<script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>

It's time for Marc's Amateur Statistics Corner! Today: why I pay a lot of attention to tail latency when optimizing cost.

I've written before on the importance of tail latency for customer experience (e.g. [in 2026](https://brooker.co.za/blog/2026/06/19/waiting.html), [2021](https://brooker.co.za/blog/2021/10/20/simulation.html), and [2021](https://brooker.co.za/blog/2021/04/19/latency.html), and [2017](https://brooker.co.za/blog/2017/12/28/mean.html)). Today, I want to talk about tail latency from the perspective of cost and capacity.

Like many system operators, I think about tail latency using percentiles.

Here's a question: how much does each of my latency percentiles contributed to the mean latency? Intuitively, the answer is "quite a lot", but can we quantify that? We can! The thing we're looking for is the [empirical Lorenz Curve](https://en.wikipedia.org/wiki/Lorenz_curve). It directly calculates the answer to the question: given a latency percentile $P$ (e.g. p99=100ms), how much do requests taking shorter than $P$ contribute to the mean latency? (Let's call it $L(P)$ , so the real answer to our question is $1 - L(P)$).

Starting from latency samples, the calculation is pretty simple: `L = sum(sorted(x)[:k]) / sum(x)` (for a set of `n` latency samples `x`, and `k=p*n`). From a vector of quantiles, things get a little more complicated, because we have to choose how to interpolate between the samples and extrapolate out to the maximum. Here I'm interpolating using a power law, which is a little bit of a sin<sup>[1](#foot1)</sup>, but good enough for our purposes.

<style>
  .one-minus-l-code summary {
    cursor: pointer;
    list-style: none;
  }
  .one-minus-l-code summary::-webkit-details-marker {
    display: none;
  }
  .one-minus-l-code .code-preview {
    display: block;
    overflow-x: auto;
    padding: 1em;
    background: #f5f2f0;
    white-space: pre;
  }
  .one-minus-l-code .code-toggle {
    display: inline-block;
    margin: 0.4em 0 1em;
    color: #555;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif;
    font-size: 0.9em;
  }
  .one-minus-l-code .when-open,
  .one-minus-l-code[open] .code-preview,
  .one-minus-l-code[open] .when-closed {
    display: none;
  }
  .one-minus-l-code[open] .when-open {
    display: inline;
  }
</style>

<details class="one-minus-l-code">
<summary><code class="code-preview language-python"># Calculate 1 - L(p) for a vector of measured quantiles
# q - an array of quantiles (e.g. [1, 10, 200, 10000, 20000])
# p - an array of percentiles they're measured at (e.g. [0, 0.5, 0.9, 0.99, 0.999])
# OneMinusL - One minus the empirical Lorenz curve for each of the percentiles
def OneMinusL(q, p):
  ...</code><span class="code-toggle"><span class="when-closed">Show full implementation</span><span class="when-open">Hide implementation</span></span></summary>
<pre><code class="language-python">
# Calculate 1 - L(p) for a vector of measured quantiles
# q - an array of quantiles (e.g. [1, 10, 200, 10000, 20000])
# p - an array of percentiles they're measured at (e.g. [0, 0.5, 0.9, 0.99, 0.999])
# OneMinusL - One minus the empirical Lorenz curve for each of the percentiles
def OneMinusL(q, p):
  q, p = np.asarray(q, float), np.asarray(p, float)
  assert q.shape == p.shape and q.size >= 2, "q, p must be same-length, size >= 2"
  assert p[0] == 0 and p[-1] < 1, "p must start at 0 (else mass is dropped) and end below 1"
  assert np.all(np.diff(p) > 0), "p must be strictly increasing"
  assert q[0] > 0 and np.all(np.diff(q) > 0), "q must be positive and strictly increasing"
  w = q*(1-p)
  a = np.log((1-p[1:])/(1-p[:-1]))/np.log(q[:-1]/q[1:])
  assert np.all(np.abs(a-1) > 1e-9), "alpha == 1 in some cell: divide by zero"
  assert a[-1] > 1, f"tail alpha={a[-1]:.3f} <= 1: infinite mean, not estimable"
  c = np.r_[0, np.cumsum(a/(a-1)*(w[:-1]-w[1:]))]
  return 1 - c/(c[-1] + a[-1]/(a[-1]-1)*w[-1])
</code></pre>
</details>

For example:

<pre><code class="language-python">
print(OneMinusL([1, 10, 200, 10000, 20000], [0, 0.5, 0.9, 0.99, 0.999]))
[1. 0.99377318 0.93082759 0.51712644 0.10342529]
</code></pre>

That tells us that, for this distribution, requests at or longer than the median (p50) contribute about 99% of the mean latency, at requests at or longer than the p99 contribute about 52% of the mean latency<sup>[2](#foot2)</sup>.

That's fun, but why do I care? Because [Little's law](https://en.wikipedia.org/wiki/Little%27s_law) tells us that this same value (contribution to the mean) is also the contribution to the concurrency in the system. In a simple threaded system, if $1 - L(p) = k$ , then $100k$% of the busy threads in our service are busy with requests with a latency above the $p$th percentile. Queues, event-based implementations, etc complicate the mapping of concurrency to cost, so you'll need to think about those in context of your own system.

In my experience, it's not unusual in services for $1 - L(0.99)$ to be greater than $0.5$ or even $0.75$. That tells us that optimizing the tail could be a much bigger than expected contributor to reducing concurrency, and along with that reducing capacity demand, lock contention, and other things that come with higher concurrency. Tails tend to be disproportionately expensive to serve, and so disproportionately important to focus on as we think about optimization. We shouldn't make the mistake of trimming them off, because they're often the thing that's driving costs! (And, of course, bad customer experiences).

If you want to play with some values, type your percentiles in here, and see your curve:

<div markdown="0" style="margin: 1.5em 0; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif;">
<p style="line-height: 2.1;">
  p0: <input type="number" id="lorenzP0Input" value="1" min="0.001" step="any" style="width: 5em;"> ms
  &nbsp;
  p50: <input type="number" id="lorenzP50Input" value="10" min="0.001" step="any" style="width: 5em;"> ms
  &nbsp;
  p90: <input type="number" id="lorenzP90Input" value="200" min="0.001" step="any" style="width: 6em;"> ms
  &nbsp;
  p99: <input type="number" id="lorenzP99Input" value="10000" min="0.001" step="any" style="width: 7em;"> ms
  &nbsp;
  p99.9: <input type="number" id="lorenzP999Input" value="20000" min="0.001" step="any" style="width: 7em;"> ms
</p>
<canvas id="lorenzGraph" width="760" height="380" style="max-width: 100%;"></canvas>
<p id="lorenzError" style="display: none; font-size: 0.9em; color: #b42318;"></p>
<p id="lorenzSummary" style="font-size: 0.9em; color: #555;">
  Share of mean latency from requests at or above each percentile:
  p50 <strong><span id="lorenzP50Share">–</span></strong>,
  p90 <strong><span id="lorenzP90Share">–</span></strong>,
  p99 <strong><span id="lorenzP99Share">–</span></strong>, and
  p99.9 <strong><span id="lorenzP999Share">–</span></strong>.
  By Little's law, these are also their shares of system concurrency.
</p>
</div>

<script>
(function () {
  const canvas = document.getElementById('lorenzGraph');
  const ctx = canvas.getContext('2d');
  const inputs = [
    document.getElementById('lorenzP0Input'),
    document.getElementById('lorenzP50Input'),
    document.getElementById('lorenzP90Input'),
    document.getElementById('lorenzP99Input'),
    document.getElementById('lorenzP999Input')
  ];
  const percentiles = [0, 0.5, 0.9, 0.99, 0.999];
  const shareSpans = [
    document.getElementById('lorenzP50Share'),
    document.getElementById('lorenzP90Share'),
    document.getElementById('lorenzP99Share'),
    document.getElementById('lorenzP999Share')
  ];
  const error = document.getElementById('lorenzError');
  const summary = document.getElementById('lorenzSummary');

  const COLOR_CONTRIBUTION = '#0d7a6e';
  const COLOR_REQUESTS = '#e8623d';
  const FILL_CONTRIBUTION = 'rgba(13, 122, 110, 0.12)';
  const INK = '#444';
  const GRID = '#ececec';
  const FONT = "13px -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif";
  const FONT_SMALL = "12px -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif";

  // Fit a power law between each adjacent pair of measured quantiles, exactly
  // as in OneMinusL above. The final power law is continued out to p100.
  function fitModel(q) {
    if (q.some(x => !Number.isFinite(x) || x <= 0)) {
      throw new Error('All latency values must be positive numbers.');
    }
    for (let i = 1; i < q.length; i++) {
      if (q[i] <= q[i - 1]) {
        throw new Error('Latency values must increase from p0 through p99.9.');
      }
    }

    const alpha = [];
    const cumulative = [0];
    for (let i = 0; i < q.length - 1; i++) {
      const a = Math.log((1 - percentiles[i + 1]) / (1 - percentiles[i])) /
        Math.log(q[i] / q[i + 1]);
      if (!Number.isFinite(a) || Math.abs(a - 1) < 1e-9) {
        throw new Error('These values produce a power-law exponent of 1; adjust one of them slightly.');
      }
      alpha.push(a);
      const w0 = q[i] * (1 - percentiles[i]);
      const w1 = q[i + 1] * (1 - percentiles[i + 1]);
      cumulative.push(cumulative[i] + a / (a - 1) * (w0 - w1));
    }

    const tailAlpha = alpha[alpha.length - 1];
    if (tailAlpha <= 1) {
      throw new Error('The fitted tail has an infinite mean; increase p99.9 by less, or increase p99.');
    }
    const last = q.length - 1;
    const mean = cumulative[last] + tailAlpha / (tailAlpha - 1) *
      q[last] * (1 - percentiles[last]);

    function oneMinusL(p) {
      if (p <= 0) return 1;
      if (p >= 1) return 0;

      let i = percentiles.length - 1;
      for (let j = 0; j < percentiles.length - 1; j++) {
        if (p <= percentiles[j + 1]) { i = j; break; }
      }
      const survival0 = 1 - percentiles[i];
      const survival = 1 - p;
      const a = alpha[Math.min(i, alpha.length - 1)];
      const qAtP = q[i] * Math.pow(survival0 / survival, 1 / a);
      const contributionBelowP = cumulative[i] + a / (a - 1) *
        (q[i] * survival0 - qAtP * survival);
      return Math.max(0, Math.min(1, 1 - contributionBelowP / mean));
    }

    return { oneMinusL };
  }

  function draw() {
    const q = inputs.map(input => parseFloat(input.value));
    let model;
    try {
      model = fitModel(q);
      error.style.display = 'none';
      summary.style.display = '';
    } catch (e) {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      error.textContent = e.message;
      error.style.display = '';
      summary.style.display = 'none';
      return;
    }

    for (let i = 1; i < percentiles.length; i++) {
      shareSpans[i - 1].textContent = (100 * model.oneMinusL(percentiles[i])).toFixed(1) + '%';
    }

    const padLeft = 64, padRight = 24, padTop = 56, padBottom = 52;
    const W = canvas.width, H = canvas.height;
    const plotW = W - padLeft - padRight;
    const plotH = H - padTop - padBottom;
    const xOf = p => padLeft + p * plotW;
    const yOf = y => padTop + (1 - y) * plotH;

    ctx.clearRect(0, 0, W, H);

    // Grid and percentage labels.
    ctx.font = FONT_SMALL;
    ctx.lineWidth = 1;
    for (let i = 0; i <= 5; i++) {
      const v = i / 5;
      const y = yOf(v);
      ctx.strokeStyle = GRID;
      ctx.beginPath(); ctx.moveTo(padLeft, y); ctx.lineTo(padLeft + plotW, y); ctx.stroke();
      ctx.fillStyle = '#999';
      ctx.textAlign = 'right';
      ctx.textBaseline = 'middle';
      ctx.fillText(Math.round(v * 100) + '%', padLeft - 10, y);

      const x = xOf(v);
      ctx.strokeStyle = GRID;
      ctx.beginPath(); ctx.moveTo(x, padTop); ctx.lineTo(x, padTop + plotH); ctx.stroke();
      ctx.fillStyle = '#999';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'top';
      ctx.fillText('p' + Math.round(v * 100), x, padTop + plotH + 8);
    }

    const N = 500;

    // Shade the contribution from requests at or above each percentile.
    ctx.fillStyle = FILL_CONTRIBUTION;
    ctx.beginPath();
    ctx.moveTo(xOf(0), yOf(0));
    for (let i = 0; i <= N; i++) {
      const p = i / N;
      ctx.lineTo(xOf(p), yOf(model.oneMinusL(p)));
    }
    ctx.lineTo(xOf(1), yOf(0));
    ctx.closePath();
    ctx.fill();

    function curve(fn, color, dashed) {
      ctx.strokeStyle = color;
      ctx.lineWidth = 2.5;
      ctx.lineJoin = 'round';
      ctx.setLineDash(dashed ? [8, 6] : []);
      ctx.beginPath();
      for (let i = 0; i <= N; i++) {
        const p = i / N;
        const x = xOf(p), y = yOf(fn(p));
        if (i === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
      }
      ctx.stroke();
      ctx.setLineDash([]);
    }
    curve(model.oneMinusL, COLOR_CONTRIBUTION, false);
    curve(p => 1 - p, COLOR_REQUESTS, true);

    // Mark the supplied quantiles on the fitted curve.
    for (let i = 1; i < percentiles.length; i++) {
      const p = percentiles[i];
      ctx.fillStyle = COLOR_CONTRIBUTION;
      ctx.beginPath();
      ctx.arc(xOf(p), yOf(model.oneMinusL(p)), 3.5, 0, 2 * Math.PI);
      ctx.fill();
    }

    // Axis titles.
    ctx.fillStyle = INK;
    ctx.font = FONT;
    ctx.textAlign = 'center';
    ctx.textBaseline = 'alphabetic';
    ctx.fillText('latency percentile', padLeft + plotW / 2, H - 10);
    ctx.save();
    ctx.translate(16, padTop + plotH / 2);
    ctx.rotate(-Math.PI / 2);
    ctx.textBaseline = 'middle';
    ctx.fillText('share at or above percentile', 0, 0);
    ctx.restore();

    // Legend across the top.
    ctx.font = FONT;
    ctx.textAlign = 'left';
    ctx.textBaseline = 'middle';
    const ly = 22;
    let lx = padLeft;
    function legend(color, dashed, text) {
      ctx.strokeStyle = color;
      ctx.lineWidth = 2.5;
      ctx.setLineDash(dashed ? [8, 6] : []);
      ctx.beginPath(); ctx.moveTo(lx, ly); ctx.lineTo(lx + 30, ly); ctx.stroke();
      ctx.setLineDash([]);
      ctx.fillStyle = INK;
      ctx.fillText(text, lx + 38, ly);
      lx += 38 + ctx.measureText(text).width + 30;
    }
    legend(COLOR_CONTRIBUTION, false, 'share of mean latency / concurrency');
    legend(COLOR_REQUESTS, true, 'share of requests');
  }

  inputs.forEach(input => input.addEventListener('input', draw));
  draw();
})();
</script>

**Footnotes**

1. <a name="foot1"></a> There are two sins here: one of them is that I'm arbitrarily choosing a way to interpolate, and the other than I'm arbitrarily choosing a way to extrapolate. The denser your percentiles and closer your data fits a Pareto distribution, the less that matters. But if you want an exact empirical answer, you'll need to use a different approach (specifically, the approach based on the sorted raw samples). I did say it was amateur statistics corner.
2. <a name="foot2"></a> The other sin here, of course, is that our measured percentiles are estimates of the true percentiles, and the uncertainty goes up as the percentile gets larger. You might get very variable results if you have heavy tails and small samples. Not ideal, but doesn't change the conclusion.