---
layout: post
title: "Traffic mix matters"

---
{{ page.title }}
================

<p class="meta">Wanna listen to my mixtape?</p>

The [preliminary report on last week's M365 outage](https://www.365fanatic.com/wp-content/uploads/2026/11/postincidentreport.pdf) contains one statement that really caught my eye:

> Our normal load testing did not detect the latent issue because [of] the traffic mix and SKUs used in our labs.

There's a lot going on in this report (as there is in any report for a complex system), but I wanted to zoom in on this one issue: *traffic mix*. It's important, because its the one thing that makes the difference between a great load test and a complete waste of time.

The performance of computer systems is extremely sensitive to workload mix.

Let's start with a simple simulator, looking at a system with a table of locks. Requests arrive with a key, hold the lock for their key for some period of time, and then free the lock and complete. This isn't an unreasonable mental model of a relational database, where each transaction takes a lock during its execution.

<div markdown="0" style="margin: 1.5em 0; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif;">
<svg viewBox="0 0 760 220" width="760" style="max-width: 100%; height: auto; display: block; margin-bottom: 1em;" role="img" aria-label="Block diagram: Poisson arrivals enter a FIFO backlog, are picked up by 16 worker threads, and each thread waits for and holds the lock for its key in a lock table before completing.">
  <defs>
    <marker id="mixArrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="#999"/></marker>
  </defs>
  <g font-size="13" fill="#444">
    <text x="55" y="105" text-anchor="middle">arrivals</text>
    <text x="165" y="80" text-anchor="middle">backlog</text>
    <text x="325" y="30" text-anchor="middle">16 worker threads</text>
    <text x="545" y="30" text-anchor="middle">lock table (one lock per key)</text>
    <text x="712" y="119">done</text>
  </g>
  <g font-size="11" fill="#999">
    <text x="55" y="135" text-anchor="middle">Poisson</text>
    <text x="165" y="158" text-anchor="middle">FIFO, never empty</text>
    <text x="165" y="172" text-anchor="middle">at saturation</text>
    <text x="509" y="56" font-size="10" text-anchor="middle">holder</text>
    <text x="530" y="56" font-size="10">FIFO waiters</text>
    <text x="646" y="174" text-anchor="end">hold ~Exp(mean 1 ms)</text>
  </g>
  <g stroke="#999" stroke-width="1.5" fill="none">
    <line x1="10" y1="115" x2="106" y2="115" marker-end="url(#mixArrow)"/>
    <line x1="222" y1="115" x2="246" y2="115" marker-end="url(#mixArrow)"/>
    <line x1="402" y1="115" x2="426" y2="115" marker-end="url(#mixArrow)"/>
    <line x1="662" y1="115" x2="704" y2="115" marker-end="url(#mixArrow)"/>
  </g>
  <!-- Backlog: open on the left, five slots. -->
  <g stroke="#bbb" stroke-width="1.5">
    <rect x="110" y="90" width="110" height="50" fill="rgba(0,0,0,0.03)" stroke="none"/>
    <line x1="110" y1="90" x2="220" y2="90"/>
    <line x1="110" y1="140" x2="220" y2="140"/>
    <line x1="220" y1="90" x2="220" y2="140"/>
    <line x1="132" y1="90" x2="132" y2="140"/>
    <line x1="154" y1="90" x2="154" y2="140"/>
    <line x1="176" y1="90" x2="176" y2="140"/>
    <line x1="198" y1="90" x2="198" y2="140"/>
  </g>
  <!-- Threads: teal holds its key's lock, orange is blocked waiting for one. -->
  <rect x="250" y="40" width="150" height="150" rx="8" fill="none" stroke="#bbb" stroke-width="1.5"/>
  <g stroke-width="1.5">
    <rect x="265" y="55" width="24" height="24" rx="3" fill="rgba(13,122,110,0.18)" stroke="#0d7a6e"/>
    <rect x="297" y="55" width="24" height="24" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="329" y="55" width="24" height="24" rx="3" fill="rgba(13,122,110,0.18)" stroke="#0d7a6e"/>
    <rect x="361" y="55" width="24" height="24" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="265" y="87" width="24" height="24" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="297" y="87" width="24" height="24" rx="3" fill="rgba(13,122,110,0.18)" stroke="#0d7a6e"/>
    <rect x="329" y="87" width="24" height="24" rx="3" fill="rgba(13,122,110,0.18)" stroke="#0d7a6e"/>
    <rect x="361" y="87" width="24" height="24" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="265" y="119" width="24" height="24" rx="3" fill="rgba(13,122,110,0.18)" stroke="#0d7a6e"/>
    <rect x="297" y="119" width="24" height="24" rx="3" fill="rgba(13,122,110,0.18)" stroke="#0d7a6e"/>
    <rect x="329" y="119" width="24" height="24" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="361" y="119" width="24" height="24" rx="3" fill="rgba(13,122,110,0.18)" stroke="#0d7a6e"/>
    <rect x="265" y="151" width="24" height="24" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="297" y="151" width="24" height="24" rx="3" fill="rgba(13,122,110,0.18)" stroke="#0d7a6e"/>
    <rect x="329" y="151" width="24" height="24" rx="3" fill="rgba(13,122,110,0.18)" stroke="#0d7a6e"/>
    <rect x="361" y="151" width="24" height="24" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
  </g>
  <!-- Lock table: one row per key, the holder then its queue of waiters. -->
  <rect x="430" y="40" width="230" height="150" rx="8" fill="none" stroke="#bbb" stroke-width="1.5"/>
  <g font-size="12" fill="#444">
    <text x="444" y="78">key 1</text>
    <text x="444" y="104">key 2</text>
    <text x="444" y="130">key 3</text>
    <text x="460" y="150" fill="#999">&#8942;</text>
    <text x="444" y="174">key N</text>
  </g>
  <g stroke-width="1.5">
    <rect x="500" y="64" width="18" height="18" rx="3" fill="rgba(13,122,110,0.18)" stroke="#0d7a6e"/>
    <rect x="528" y="64" width="18" height="18" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="550" y="64" width="18" height="18" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="572" y="64" width="18" height="18" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="594" y="64" width="18" height="18" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="500" y="90" width="18" height="18" rx="3" fill="rgba(13,122,110,0.18)" stroke="#0d7a6e"/>
    <rect x="528" y="90" width="18" height="18" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="500" y="116" width="18" height="18" rx="3" fill="none" stroke="#ccc" stroke-dasharray="3 3"/>
    <rect x="500" y="160" width="18" height="18" rx="3" fill="rgba(13,122,110,0.18)" stroke="#0d7a6e"/>
  </g>
  <!-- Legend and timing note. -->
  <g font-size="11" fill="#555">
    <rect x="250" y="201" width="11" height="11" rx="2" fill="rgba(13,122,110,0.18)" stroke="#0d7a6e" stroke-width="1.5"/>
    <text x="266" y="210">holding its key's lock</text>
    <rect x="395" y="201" width="11" height="11" rx="2" fill="rgba(232,98,61,0.18)" stroke="#e8623d" stroke-width="1.5"/>
    <text x="411" y="210">blocked behind the holder (thread still occupied)</text>
  </g>
</svg>
<p>
  Keys (N): <input type="number" id="mixKeysInput" value="1000" min="1" max="1000000" step="1" style="width: 7em;">
  &nbsp;&nbsp;
  <button id="mixSimulateBtn">Simulate</button>
</p>
<canvas id="mixGraph" width="760" height="380" style="max-width: 100%;"></canvas>
<p id="mixSummary" style="font-size: 0.9em; color: #555;">
  Each request takes one of 16 worker threads, waits in a FIFO queue for the lock on its key, holds the lock for an exponentially distributed time (mean 1 ms), then releases it and frees its thread.
  Keys are drawn from a Zipf distribution over N keys; a coefficient of 0 is uniform.
</p>
</div>

As we may expect, the more skewed the distribution, the less throughput you get out of the system. A small number of keys are highly contended, leading to under utilization of the rest of the resources of the machine, driven by the global concurrency limit.

Now, let's consider a different mental model of a relational database (at least a single-system one like Postgres). A fast, local, in-memory cache containing some of the accessed data backed by a slower store containing the rest of the data.

<div markdown="0" style="margin: 1.5em 0; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif;">
<svg viewBox="0 0 760 220" width="760" style="max-width: 100%; height: auto; display: block; margin-bottom: 1em;" role="img" aria-label="Block diagram: Poisson arrivals enter a FIFO backlog and are picked up by 16 worker threads. Each request either hits a fast LRU cache or misses and goes to slower storage, which fills the cache, before completing.">
  <defs>
    <marker id="cacheArrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="#999"/></marker>
    <marker id="cacheArrowTeal" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="#0d7a6e"/></marker>
    <marker id="cacheArrowOrange" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="#e8623d"/></marker>
  </defs>
  <g font-size="13" fill="#444">
    <text x="55" y="105" text-anchor="middle">arrivals</text>
    <text x="165" y="80" text-anchor="middle">backlog</text>
    <text x="325" y="30" text-anchor="middle">16 worker threads</text>
    <text x="722" y="119">done</text>
  </g>
  <g font-size="11" fill="#999">
    <text x="55" y="135" text-anchor="middle">Poisson</text>
    <text x="165" y="158" text-anchor="middle">FIFO, never empty</text>
    <text x="165" y="172" text-anchor="middle">at saturation</text>
  </g>
  <g stroke="#999" stroke-width="1.5" fill="none">
    <line x1="10" y1="115" x2="106" y2="115" marker-end="url(#cacheArrow)"/>
    <line x1="222" y1="115" x2="246" y2="115" marker-end="url(#cacheArrow)"/>
    <polyline points="662,72 690,72 690,115 714,115" marker-end="url(#cacheArrow)"/>
    <polyline points="662,157 690,157 690,115"/>
  </g>
  <!-- Backlog: open on the left, five slots. -->
  <g stroke="#bbb" stroke-width="1.5">
    <rect x="110" y="90" width="110" height="50" fill="rgba(0,0,0,0.03)" stroke="none"/>
    <line x1="110" y1="90" x2="220" y2="90"/>
    <line x1="110" y1="140" x2="220" y2="140"/>
    <line x1="220" y1="90" x2="220" y2="140"/>
    <line x1="132" y1="90" x2="132" y2="140"/>
    <line x1="154" y1="90" x2="154" y2="140"/>
    <line x1="176" y1="90" x2="176" y2="140"/>
    <line x1="198" y1="90" x2="198" y2="140"/>
  </g>
  <!-- Threads: teal is serving a cache hit, orange is waiting on storage. -->
  <rect x="250" y="40" width="150" height="150" rx="8" fill="none" stroke="#bbb" stroke-width="1.5"/>
  <g stroke-width="1.5">
    <rect x="265" y="55" width="24" height="24" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="297" y="55" width="24" height="24" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="329" y="55" width="24" height="24" rx="3" fill="rgba(13,122,110,0.18)" stroke="#0d7a6e"/>
    <rect x="361" y="55" width="24" height="24" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="265" y="87" width="24" height="24" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="297" y="87" width="24" height="24" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="329" y="87" width="24" height="24" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="361" y="87" width="24" height="24" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="265" y="119" width="24" height="24" rx="3" fill="rgba(13,122,110,0.18)" stroke="#0d7a6e"/>
    <rect x="297" y="119" width="24" height="24" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="329" y="119" width="24" height="24" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="361" y="119" width="24" height="24" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="265" y="151" width="24" height="24" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="297" y="151" width="24" height="24" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="329" y="151" width="24" height="24" rx="3" fill="rgba(232,98,61,0.18)" stroke="#e8623d"/>
    <rect x="361" y="151" width="24" height="24" rx="3" fill="rgba(13,122,110,0.18)" stroke="#0d7a6e"/>
  </g>
  <!-- Hit and miss paths. -->
  <g stroke-width="1.5" fill="none">
    <polyline points="402,105 430,105 430,72 456,72" stroke="#0d7a6e" marker-end="url(#cacheArrowTeal)"/>
    <polyline points="402,125 430,125 430,157 456,157" stroke="#e8623d" marker-end="url(#cacheArrowOrange)"/>
    <line x1="505" y1="130" x2="505" y2="103" stroke="#999" stroke-dasharray="4 3" marker-end="url(#cacheArrow)"/>
  </g>
  <g font-size="11">
    <text x="406" y="98" fill="#0d7a6e">hit</text>
    <text x="406" y="140" fill="#e8623d">miss</text>
    <text x="513" y="119" fill="#999">fill on miss</text>
  </g>
  <!-- Cache and storage. -->
  <rect x="460" y="45" width="200" height="55" rx="8" fill="rgba(13,122,110,0.08)" stroke="#0d7a6e" stroke-width="1.5"/>
  <rect x="460" y="130" width="200" height="55" rx="8" fill="rgba(232,98,61,0.08)" stroke="#e8623d" stroke-width="1.5"/>
  <g text-anchor="middle">
    <text x="560" y="67" font-size="13" fill="#444">LRU cache</text>
    <text x="560" y="86" font-size="11" fill="#555"><tspan id="cacheDiagPct">5</tspan>% of keys, ~Exp(mean <tspan id="cacheDiagHitMs">0.01</tspan> ms)</text>
    <text x="560" y="152" font-size="13" fill="#444">storage</text>
    <text x="560" y="171" font-size="11" fill="#555">all N keys, ~Exp(mean 1 ms)</text>
  </g>
  <!-- Legend. -->
  <g font-size="11" fill="#555">
    <rect x="250" y="201" width="11" height="11" rx="2" fill="rgba(13,122,110,0.18)" stroke="#0d7a6e" stroke-width="1.5"/>
    <text x="266" y="210">serving a cache hit</text>
    <rect x="380" y="201" width="11" height="11" rx="2" fill="rgba(232,98,61,0.18)" stroke="#e8623d" stroke-width="1.5"/>
    <text x="396" y="210">waiting on storage</text>
  </g>
</svg>
<p style="line-height: 2.1;">
  Keys (N): <input type="number" id="cacheKeysInput" value="1000" min="1" max="1000000" step="1" style="width: 7em;">
  &nbsp;&nbsp;
  Cache size: <input type="number" id="cacheSizeInput" value="5" min="0" max="100" step="any" style="width: 4em;"> % of keys
  &nbsp;&nbsp;
  Cache is <input type="number" id="cacheRatioInput" value="100" min="1" max="10000" step="any" style="width: 5em;">&times; faster than storage
  &nbsp;&nbsp;
  <button id="cacheSimulateBtn">Simulate</button>
</p>
<canvas id="cacheGraph" width="760" height="380" style="max-width: 100%;"></canvas>
<p id="cacheSummary" style="font-size: 0.9em; color: #555;">
  Each request takes one of the same 16 worker threads and reads its key. A hit is served from an LRU cache; a miss goes to storage (exponentially distributed, mean 1 ms) and the key is added to the cache.
  Keys are drawn from a Zipf distribution over N keys; a coefficient of 0 is uniform. Note the log scale on the throughput axis.
</p>
</div>

This curve goes the other way! The more skewed the distribution, the better the performance, over something of a broad range. This is what makes it so important that workload mix in load tests and benchmarks matches real production traffic. This is especially true in relational database-shaped things, but applies to systems of all kinds.

**Aside**: This is also a reason I like optimistic concurrency control, and architectures with a single cost/latency *mode*. [Aurora DSQL](https://aws.amazon.com/rds/aurora/dsql/) reflects this thinking on how to make things less workload sensitive: all reads go to a single fast storage layer, and nobody is holding resources waiting on locks.

<script>
(function () {
  const canvas = document.getElementById('mixGraph');
  const ctx = canvas.getContext('2d');
  const keysInput = document.getElementById('mixKeysInput');
  const simulateBtn = document.getElementById('mixSimulateBtn');
  const summary = document.getElementById('mixSummary');

  const THREADS = 16;        // c: worker threads
  const HOLD_MS = 1;         // mean lock hold (service) time, exponential
  const MAX_THROUGHPUT = THREADS * 1000 / HOLD_MS;  // req/s with every thread doing useful work
  const S_POINTS = 41;       // Zipf coefficients 0, 0.05, ..., 2
  const S_MAX = 2;
  const WARMUP = 20000;      // completions discarded before measuring
  const MEASURE = 200000;    // completions measured at each coefficient
  const MAX_KEYS = 1000000;

  const COLOR_SIM = '#0d7a6e';
  const COLOR_UNIFORM = '#e8623d';
  const INK = '#444';
  const GRID = '#ececec';
  const FONT = "13px -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif";
  const FONT_SMALL = "12px -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif";

  // Small seeded PRNG so a given N always produces the same curve.
  function mulberry32(seed) {
    return function () {
      seed |= 0; seed = seed + 0x6D2B79F5 | 0;
      let t = Math.imul(seed ^ seed >>> 15, 1 | seed);
      t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t;
      return ((t ^ t >>> 14) >>> 0) / 4294967296;
    };
  }

  // CDF of a Zipf distribution over key ranks 1..n: P(k) proportional to k^-s.
  function zipfCdf(n, s) {
    const cdf = new Float64Array(n);
    let total = 0;
    for (let k = 0; k < n; k++) {
      total += Math.pow(k + 1, -s);
      cdf[k] = total;
    }
    for (let k = 0; k < n; k++) cdf[k] /= total;
    cdf[n - 1] = 1;
    return cdf;
  }

  function sampleKey(cdf, u) {
    let lo = 0, hi = cdf.length - 1;
    while (lo < hi) {
      const mid = (lo + hi) >>> 1;
      if (cdf[mid] > u) hi = mid; else lo = mid + 1;
    }
    return lo;
  }

  // Saturation throughput is the highest Poisson arrival rate the system can
  // sustain. Past that point the arrival queue never drains, so every thread
  // picks up a new request the moment it frees up. We simulate that saturated
  // regime directly: THREADS requests are always in the system, each either
  // holding its key's lock or queued behind the holder.
  function saturationThroughput(n, s, rand) {
    const cdf = zipfCdf(n, s);
    const inSystem = new Int32Array(n);   // holder + waiters, per key
    // Min-heap of lock-release events. At most one per key, so at most THREADS.
    const heapTime = new Float64Array(THREADS + 1);
    const heapKey = new Int32Array(THREADS + 1);
    let heapSize = 0;
    let now = 0;

    function push(t, k) {
      let i = heapSize++;
      while (i > 0) {
        const p = (i - 1) >> 1;
        if (heapTime[p] <= t) break;
        heapTime[i] = heapTime[p]; heapKey[i] = heapKey[p];
        i = p;
      }
      heapTime[i] = t; heapKey[i] = k;
    }

    function pop() {
      const k = heapKey[0];
      now = heapTime[0];
      const t = heapTime[--heapSize], lastKey = heapKey[heapSize];
      let i = 0;
      while (true) {
        let c = 2 * i + 1;
        if (c >= heapSize) break;
        if (c + 1 < heapSize && heapTime[c + 1] < heapTime[c]) c++;
        if (heapTime[c] >= t) break;
        heapTime[i] = heapTime[c]; heapKey[i] = heapKey[c];
        i = c;
      }
      heapTime[i] = t; heapKey[i] = lastKey;
      return k;
    }

    const holdTime = () => -Math.log(1 - rand()) * HOLD_MS;

    // A thread picks up a new request. It takes the lock if the key is free,
    // otherwise it waits (still occupying its thread) behind the holder.
    function admit() {
      const k = sampleKey(cdf, rand());
      if (inSystem[k]++ === 0) push(now + holdTime(), k);
    }

    for (let i = 0; i < THREADS; i++) admit();
    let start = 0;
    for (let done = 0; done < WARMUP + MEASURE; done++) {
      if (done === WARMUP) start = now;
      const k = pop();
      // The next waiter for this key, if any, takes the lock.
      if (--inSystem[k] > 0) push(now + holdTime(), k);
      admit();
    }
    return MEASURE / (now - start) * 1000;
  }

  function niceStep(range, target) {
    const raw = range / target;
    const mag = Math.pow(10, Math.floor(Math.log10(raw)));
    const norm = raw / mag;
    const step = norm < 1.5 ? 1 : norm < 3 ? 2 : norm < 7 ? 5 : 10;
    return step * mag;
  }

  const formatRate = r => r >= 1000 ? (r / 1000).toFixed(r >= 10000 ? 1 : 2) + 'k' : r.toFixed(0);

  function draw(results, message) {
    const padLeft = 80, padRight = 24, padTop = 56, padBottom = 52;
    const W = canvas.width, H = canvas.height;
    const plotW = W - padLeft - padRight;
    const plotH = H - padTop - padBottom;
    const yMax = MAX_THROUGHPUT * 1.08;
    const xOf = s => padLeft + (s / S_MAX) * plotW;
    const yOf = r => padTop + plotH - (r / yMax) * plotH;

    ctx.clearRect(0, 0, W, H);

    // Horizontal gridlines and throughput labels.
    ctx.lineWidth = 1;
    ctx.font = FONT_SMALL;
    ctx.textAlign = 'right';
    ctx.textBaseline = 'middle';
    const yStep = niceStep(MAX_THROUGHPUT, 5);
    for (let v = 0; v <= yMax + 1e-9; v += yStep) {
      const y = yOf(v);
      ctx.strokeStyle = GRID;
      ctx.beginPath(); ctx.moveTo(padLeft, y); ctx.lineTo(padLeft + plotW, y); ctx.stroke();
      ctx.fillStyle = '#999';
      ctx.fillText(formatRate(v), padLeft - 10, y);
    }

    // Vertical gridlines and coefficient labels.
    ctx.textAlign = 'center';
    ctx.textBaseline = 'top';
    for (let i = 0; i <= 8; i++) {
      const s = i * 0.25;
      const x = xOf(s);
      ctx.strokeStyle = GRID;
      ctx.beginPath(); ctx.moveTo(x, padTop); ctx.lineTo(x, padTop + plotH); ctx.stroke();
      ctx.fillStyle = '#999';
      ctx.fillText(s.toFixed(2), x, padTop + plotH + 8);
    }

    // The ceiling: every thread holding a lock and doing useful work.
    ctx.strokeStyle = '#bbb';
    ctx.lineWidth = 1;
    ctx.setLineDash([2, 4]);
    ctx.beginPath(); ctx.moveTo(padLeft, yOf(MAX_THROUGHPUT)); ctx.lineTo(padLeft + plotW, yOf(MAX_THROUGHPUT)); ctx.stroke();
    ctx.setLineDash([]);
    ctx.fillStyle = '#999';
    ctx.textAlign = 'right';
    ctx.textBaseline = 'bottom';
    ctx.fillText('all ' + THREADS + ' threads busy: ' + formatRate(MAX_THROUGHPUT) + ' req/s',
      padLeft + plotW, yOf(MAX_THROUGHPUT) - 3);

    // Uniform traffic (coefficient 0) as a reference across the whole sweep.
    if (results.length > 0) {
      const uniform = results[0].rate;
      ctx.strokeStyle = COLOR_UNIFORM;
      ctx.lineWidth = 2;
      ctx.setLineDash([8, 6]);
      ctx.beginPath(); ctx.moveTo(padLeft, yOf(uniform)); ctx.lineTo(padLeft + plotW, yOf(uniform)); ctx.stroke();
      ctx.setLineDash([]);
    }

    // Simulated saturation throughput.
    if (results.length > 0) {
      ctx.strokeStyle = COLOR_SIM;
      ctx.lineWidth = 2.5;
      ctx.lineJoin = 'round';
      ctx.beginPath();
      results.forEach((r, i) => {
        const x = xOf(r.s), y = yOf(r.rate);
        if (i === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
      });
      ctx.stroke();
      ctx.fillStyle = COLOR_SIM;
      results.forEach(r => {
        ctx.beginPath();
        ctx.arc(xOf(r.s), yOf(r.rate), 3, 0, 2 * Math.PI);
        ctx.fill();
      });
    }

    if (message) {
      ctx.fillStyle = '#999';
      ctx.font = FONT;
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.fillText(message, padLeft + plotW / 2, padTop + plotH / 2);
    }

    // Axis titles.
    ctx.fillStyle = INK;
    ctx.font = FONT;
    ctx.textAlign = 'center';
    ctx.textBaseline = 'alphabetic';
    ctx.fillText('Zipf coefficient (0 = uniform)', padLeft + plotW / 2, H - 10);
    ctx.save();
    ctx.translate(16, padTop + plotH / 2);
    ctx.rotate(-Math.PI / 2);
    ctx.textBaseline = 'middle';
    ctx.fillText('saturation throughput (req/s)', 0, 0);
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
    legend(COLOR_SIM, false, 'saturation throughput');
    legend(COLOR_UNIFORM, true, 'uniform keys');
  }

  function describe(n, results) {
    const uniform = results[0].rate;
    const at = s => results.find(r => Math.abs(r.s - s) < 1e-9);
    const pct = r => (100 * r.rate / uniform).toFixed(0) + '%';
    const one = at(1), two = at(S_MAX);
    summary.innerHTML =
      'With ' + n.toLocaleString() + (n === 1 ? ' key' : ' keys') + ', uniform traffic saturates at <strong>' +
      formatRate(uniform) + ' req/s</strong>. ' +
      'At a Zipf coefficient of 1 it saturates at <strong>' + formatRate(one.rate) +
      ' req/s</strong> (' + pct(one) + ' of uniform), and at 2 at <strong>' +
      formatRate(two.rate) + ' req/s</strong> (' + pct(two) + ' of uniform).';
  }

  function simulate() {
    const n = Math.round(parseFloat(keysInput.value));
    if (!(n >= 1 && n <= MAX_KEYS)) {
      summary.textContent = 'Choose a number of keys between 1 and ' + MAX_KEYS.toLocaleString() + '.';
      return;
    }
    keysInput.value = n;
    simulateBtn.disabled = true;
    keysInput.disabled = true;
    const rand = mulberry32(n);
    const results = [];
    let i = 0;

    // One coefficient per frame, so the curve draws as it goes.
    function step() {
      const s = i * S_MAX / (S_POINTS - 1);
      results.push({ s, rate: saturationThroughput(n, s, rand) });
      i++;
      draw(results);
      if (i < S_POINTS) {
        summary.textContent = 'Simulating Zipf coefficient ' + s.toFixed(2) + '…';
        requestAnimationFrame(step);
      } else {
        describe(n, results);
        simulateBtn.disabled = false;
        keysInput.disabled = false;
      }
    }
    requestAnimationFrame(step);
  }

  simulateBtn.addEventListener('click', simulate);
  keysInput.addEventListener('keydown', e => { if (e.key === 'Enter') simulate(); });
  draw([], 'Choose a number of keys and press Simulate');
})();
</script>

<script>
(function () {
  const canvas = document.getElementById('cacheGraph');
  const ctx = canvas.getContext('2d');
  const keysInput = document.getElementById('cacheKeysInput');
  const sizeInput = document.getElementById('cacheSizeInput');
  const ratioInput = document.getElementById('cacheRatioInput');
  const simulateBtn = document.getElementById('cacheSimulateBtn');
  const summary = document.getElementById('cacheSummary');
  const inputs = [keysInput, sizeInput, ratioInput];

  const THREADS = 16;        // c: same concurrency limit as the lock simulator
  const STORAGE_MS = 1;      // mean storage (miss) time, exponential
  const S_POINTS = 41;       // Zipf coefficients 0, 0.05, ..., 2
  const S_MAX = 2;
  const WARMUP = 20000;      // completions discarded before measuring (plus 2x cache size)
  const MEASURE = 200000;    // completions measured at each coefficient
  const MAX_KEYS = 1000000;
  const MAX_RATIO = 10000;

  const COLOR_SIM = '#0d7a6e';
  const COLOR_UNIFORM = '#e8623d';
  const INK = '#444';
  const GRID = '#ececec';
  const FONT = "13px -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif";
  const FONT_SMALL = "12px -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif";

  // Small seeded PRNG so a given N always produces the same request stream.
  function mulberry32(seed) {
    return function () {
      seed |= 0; seed = seed + 0x6D2B79F5 | 0;
      let t = Math.imul(seed ^ seed >>> 15, 1 | seed);
      t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t;
      return ((t ^ t >>> 14) >>> 0) / 4294967296;
    };
  }

  // CDF of a Zipf distribution over key ranks 1..n: P(k) proportional to k^-s.
  function zipfCdf(n, s) {
    const cdf = new Float64Array(n);
    let total = 0;
    for (let k = 0; k < n; k++) {
      total += Math.pow(k + 1, -s);
      cdf[k] = total;
    }
    for (let k = 0; k < n; k++) cdf[k] /= total;
    cdf[n - 1] = 1;
    return cdf;
  }

  function sampleKey(cdf, u) {
    let lo = 0, hi = cdf.length - 1;
    while (lo < hi) {
      const mid = (lo + hi) >>> 1;
      if (cdf[mid] > u) hi = mid; else lo = mid + 1;
    }
    return lo;
  }

  // An LRU cache over integer keys, as a doubly-linked list in typed arrays.
  function makeLru(n, capacity) {
    const prev = new Int32Array(n).fill(-1);
    const next = new Int32Array(n).fill(-1);
    const cached = new Uint8Array(n);
    let head = -1, tail = -1, size = 0;

    function unlink(k) {
      if (prev[k] >= 0) next[prev[k]] = next[k]; else head = next[k];
      if (next[k] >= 0) prev[next[k]] = prev[k]; else tail = prev[k];
      prev[k] = next[k] = -1;
    }
    function pushFront(k) {
      prev[k] = -1;
      next[k] = head;
      if (head >= 0) prev[head] = k;
      head = k;
      if (tail < 0) tail = k;
    }

    // Returns true on a hit. On a miss, the key is inserted and the least
    // recently used key evicted if the cache is full.
    function access(k) {
      if (cached[k]) {
        if (head !== k) { unlink(k); pushFront(k); }
        return true;
      }
      if (capacity === 0) return false;
      pushFront(k);
      cached[k] = 1;
      if (++size > capacity) {
        const victim = tail;
        unlink(victim);
        cached[victim] = 0;
        size--;
      }
      return false;
    }

    // Start with the most popular keys cached, to shorten warm-up.
    for (let k = Math.min(capacity, n) - 1; k >= 0; k--) access(k);
    return access;
  }

  // As in the lock simulator, saturation throughput is measured with the
  // arrival queue never draining: every thread picks up a new request the
  // moment it finishes one. With no locks, threads never wait on each other,
  // so throughput depends only on how often requests hit the cache.
  function saturationThroughput(n, capacity, ratio, s, rand) {
    const cdf = zipfCdf(n, s);
    const access = makeLru(n, capacity);
    const hitMs = STORAGE_MS / ratio;
    const heap = new Float64Array(THREADS + 1);   // completion times
    let heapSize = 0;
    let now = 0;

    function push(t) {
      let i = heapSize++;
      while (i > 0) {
        const p = (i - 1) >> 1;
        if (heap[p] <= t) break;
        heap[i] = heap[p];
        i = p;
      }
      heap[i] = t;
    }
    function pop() {
      now = heap[0];
      const t = heap[--heapSize];
      let i = 0;
      while (true) {
        let c = 2 * i + 1;
        if (c >= heapSize) break;
        if (c + 1 < heapSize && heap[c + 1] < heap[c]) c++;
        if (heap[c] >= t) break;
        heap[i] = heap[c];
        i = c;
      }
      heap[i] = t;
    }

    let hits = 0, measuring = false;
    function admit() {
      const hit = access(sampleKey(cdf, rand()));
      if (hit && measuring) hits++;
      push(now - Math.log(1 - rand()) * (hit ? hitMs : STORAGE_MS));
    }

    for (let i = 0; i < THREADS; i++) admit();
    const warmup = WARMUP + 2 * capacity;
    let start = 0;
    for (let done = 0; done < warmup + MEASURE; done++) {
      if (done === warmup) { start = now; measuring = true; }
      pop();
      admit();
    }
    return { rate: MEASURE / (now - start) * 1000, hitRate: hits / MEASURE };
  }

  function formatRate(r) {
    const trim = x => x.toFixed(x >= 10 ? 0 : x >= 1 ? 1 : 2).replace(/\.0+$/, '');
    if (r >= 1e6) return trim(r / 1e6) + 'M';
    if (r >= 1e3) return trim(r / 1e3) + 'k';
    return trim(r);
  }

  function draw(results, params, message) {
    const padLeft = 80, padRight = 24, padTop = 56, padBottom = 52;
    const W = canvas.width, H = canvas.height;
    const plotW = W - padLeft - padRight;
    const plotH = H - padTop - padBottom;
    const { ratio } = params;
    const allMiss = THREADS * 1000 / STORAGE_MS;
    const allHit = allMiss * ratio;
    // Log-scale throughput axis, with a little room around the floor and ceiling.
    const logLo = Math.log10(allMiss / 2), logHi = Math.log10(allHit * 2);
    const xOf = s => padLeft + (s / S_MAX) * plotW;
    const yOf = r => padTop + plotH - (Math.log10(r) - logLo) / (logHi - logLo) * plotH;

    ctx.clearRect(0, 0, W, H);

    // Horizontal gridlines at 1, 2, 5 x 10^k (decades only when the range is wide).
    const decades = logHi - logLo;
    const mantissas = decades > 4 ? [1] : [1, 2, 5];
    ctx.lineWidth = 1;
    ctx.font = FONT_SMALL;
    ctx.textAlign = 'right';
    ctx.textBaseline = 'middle';
    for (let e = Math.floor(logLo); e <= Math.ceil(logHi); e++) {
      for (const m of mantissas) {
        const v = m * Math.pow(10, e);
        const lv = Math.log10(v);
        if (lv < logLo || lv > logHi) continue;
        const y = yOf(v);
        ctx.strokeStyle = GRID;
        ctx.beginPath(); ctx.moveTo(padLeft, y); ctx.lineTo(padLeft + plotW, y); ctx.stroke();
        ctx.fillStyle = '#999';
        ctx.fillText(formatRate(v), padLeft - 10, y);
      }
    }

    // Vertical gridlines and coefficient labels.
    ctx.textAlign = 'center';
    ctx.textBaseline = 'top';
    for (let i = 0; i <= 8; i++) {
      const s = i * 0.25;
      const x = xOf(s);
      ctx.strokeStyle = GRID;
      ctx.beginPath(); ctx.moveTo(x, padTop); ctx.lineTo(x, padTop + plotH); ctx.stroke();
      ctx.fillStyle = '#999';
      ctx.fillText(s.toFixed(2), x, padTop + plotH + 8);
    }

    // Floor and ceiling: every request going to storage, or hitting the cache.
    function bound(v, label) {
      const y = yOf(v);
      ctx.strokeStyle = '#bbb';
      ctx.lineWidth = 1;
      ctx.setLineDash([2, 4]);
      ctx.beginPath(); ctx.moveTo(padLeft, y); ctx.lineTo(padLeft + plotW, y); ctx.stroke();
      ctx.setLineDash([]);
      ctx.fillStyle = '#999';
      ctx.font = FONT_SMALL;
      ctx.textAlign = 'right';
      ctx.textBaseline = 'bottom';
      ctx.fillText(label + ': ' + formatRate(v) + ' req/s', padLeft + plotW, y - 3);
    }
    bound(allHit, 'every request hits the cache');
    bound(allMiss, 'every request goes to storage');

    // Uniform traffic (coefficient 0) as a reference across the whole sweep.
    if (results.length > 0) {
      const y = yOf(results[0].rate);
      ctx.strokeStyle = COLOR_UNIFORM;
      ctx.lineWidth = 2;
      ctx.setLineDash([8, 6]);
      ctx.beginPath(); ctx.moveTo(padLeft, y); ctx.lineTo(padLeft + plotW, y); ctx.stroke();
      ctx.setLineDash([]);
    }

    // Simulated saturation throughput.
    if (results.length > 0) {
      ctx.strokeStyle = COLOR_SIM;
      ctx.lineWidth = 2.5;
      ctx.lineJoin = 'round';
      ctx.beginPath();
      results.forEach((r, i) => {
        const x = xOf(r.s), y = yOf(r.rate);
        if (i === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
      });
      ctx.stroke();
      ctx.fillStyle = COLOR_SIM;
      results.forEach(r => {
        ctx.beginPath();
        ctx.arc(xOf(r.s), yOf(r.rate), 3, 0, 2 * Math.PI);
        ctx.fill();
      });
    }

    if (message) {
      ctx.fillStyle = '#999';
      ctx.font = FONT;
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.fillText(message, padLeft + plotW / 2, padTop + plotH / 2);
    }

    // Axis titles.
    ctx.fillStyle = INK;
    ctx.font = FONT;
    ctx.textAlign = 'center';
    ctx.textBaseline = 'alphabetic';
    ctx.fillText('Zipf coefficient (0 = uniform)', padLeft + plotW / 2, H - 10);
    ctx.save();
    ctx.translate(16, padTop + plotH / 2);
    ctx.rotate(-Math.PI / 2);
    ctx.textBaseline = 'middle';
    ctx.fillText('saturation throughput (req/s, log scale)', 0, 0);
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
    legend(COLOR_SIM, false, 'saturation throughput');
    legend(COLOR_UNIFORM, true, 'uniform keys');
  }

  function describe(params, results) {
    const uniform = results[0];
    const at = s => results.find(r => Math.abs(r.s - s) < 1e-9);
    const hit = r => r.hitRate === 1 ? '100%' : (100 * r.hitRate).toFixed(r.hitRate > 0.99 ? 2 : 0) + '%';
    const times = r => (r.rate / uniform.rate).toFixed(1) + '&times;';
    const one = at(1), two = at(S_MAX);
    summary.innerHTML =
      'Caching ' + params.capacity.toLocaleString() + ' of ' + params.n.toLocaleString() +
      (params.n === 1 ? ' key' : ' keys') + ', uniform traffic hits the cache ' + hit(uniform) +
      ' of the time and saturates at <strong>' + formatRate(uniform.rate) + ' req/s</strong>. ' +
      'At a Zipf coefficient of 1 the hit rate is ' + hit(one) + ' and it saturates at <strong>' +
      formatRate(one.rate) + ' req/s</strong> (' + times(one) + ' uniform); at 2, ' + hit(two) +
      ' and <strong>' + formatRate(two.rate) + ' req/s</strong> (' + times(two) + ' uniform).';
  }

  function readParams() {
    const n = Math.round(parseFloat(keysInput.value));
    const pct = parseFloat(sizeInput.value);
    const ratio = parseFloat(ratioInput.value);
    if (!(n >= 1 && n <= MAX_KEYS)) {
      return { error: 'Choose a number of keys between 1 and ' + MAX_KEYS.toLocaleString() + '.' };
    }
    if (!(pct >= 0 && pct <= 100)) {
      return { error: 'Choose a cache size between 0% and 100% of keys.' };
    }
    if (!(ratio >= 1 && ratio <= MAX_RATIO)) {
      return { error: 'Choose a speed ratio between 1 and ' + MAX_RATIO.toLocaleString() + '.' };
    }
    return { n, pct, ratio, capacity: Math.round(n * pct / 100) };
  }

  function simulate() {
    const params = readParams();
    if (params.error) {
      summary.textContent = params.error;
      return;
    }
    keysInput.value = params.n;
    simulateBtn.disabled = true;
    inputs.forEach(input => { input.disabled = true; });
    const rand = mulberry32(params.n);
    const results = [];
    let i = 0;

    // One coefficient per frame, so the curve draws as it goes.
    function step() {
      const s = i * S_MAX / (S_POINTS - 1);
      const r = saturationThroughput(params.n, params.capacity, params.ratio, s, rand);
      results.push({ s, rate: r.rate, hitRate: r.hitRate });
      i++;
      draw(results, params);
      if (i < S_POINTS) {
        summary.textContent = 'Simulating Zipf coefficient ' + s.toFixed(2) + '…';
        requestAnimationFrame(step);
      } else {
        describe(params, results);
        simulateBtn.disabled = false;
        inputs.forEach(input => { input.disabled = false; });
      }
    }
    requestAnimationFrame(step);
  }

  simulateBtn.addEventListener('click', simulate);
  inputs.forEach(input => input.addEventListener('keydown', e => { if (e.key === 'Enter') simulate(); }));

  // Keep the block diagram's cache labels in step with the inputs.
  const diagPct = document.getElementById('cacheDiagPct');
  const diagHitMs = document.getElementById('cacheDiagHitMs');
  function updateDiagram() {
    const pct = parseFloat(sizeInput.value);
    const ratio = parseFloat(ratioInput.value);
    if (pct >= 0 && pct <= 100) diagPct.textContent = String(pct);
    if (ratio >= 1) diagHitMs.textContent = String(Number((STORAGE_MS / ratio).toPrecision(2)));
  }
  sizeInput.addEventListener('input', updateDiagram);
  ratioInput.addEventListener('input', updateDiagram);
  updateDiagram();
  draw([], { ratio: parseFloat(ratioInput.value) || 100 }, 'Choose a cache size and speed, and press Simulate');
})();
</script>

