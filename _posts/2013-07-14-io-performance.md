---
layout: post
title: "Beyond iostat: Storage performance analysis with blktrace"









related_posts:
  - "/2014/07/04/iostat-pct.html"
  - "/2021/03/25/latency-bandwidth.html"
  - "/2014/12/06/random.html"
dissimilar_posts:
  - "/2020/07/28/fish.html"
---
{{ page.title }}
================

<p class="meta">An under appreciated set of IO analysis tools.</p>

If you've spent much time at all investigating IO performance on Linux, you're no doubt already familiar with *iostat* from the venerable [sysstat](http://sebastien.godard.pagesperso-orange.fr/) package. *iostat* is the go-to tool for Linux storage performance monitoring with good reason: it's available nearly everywhere, it works on the vast majority of Linux machines, and it's relatively easy to use and understand. Some of what it measures can be [subtle](http://dom.as/2009/03/11/iostat/), and the exact definitions of its measurements can be [confusing, and even contentious](http://www.xaprb.com/blog/2010/09/06/beware-of-svctm-in-linuxs-iostat/), but it's still a great start.

Sometimes, though, you need to go into more detail than iostat can provide. The aggregate view from *iostat* is simple, but makes it difficult to tell which processes are doing which IOs. Averaging over a period of time can hide subtle performance issues and the real causes of may IO-related problems. To get around these issues, you'll want to go deeper. If you have the guts for it, a recent kernel, and a good understanding of IO performance issues, you'll want to reach for [blktrace](http://git.kernel.org/cgit/linux/kernel/git/axboe/blktrace.git/tree/README) and friends. The blktrace toolkit provides an extremely powerful way to look at the exact IO performance of a Linux machine, at a wide range of levels of detail, and is vastly more capable than the simple *iostat*.

For a start, let's look at the performance of a random read workload to a magnetic drive, with 16k IOs. The manufacturer's spec sheet says this drive should be delivering about 120 IOs per second on a completely random load. *iostat -x* has this to say about the drive:

    Device:       rrqm/s   wrqm/s     r/s     w/s   rsec/s   wsec/s avgrq-sz
    sdb             0.00     0.00  124.67    0.00  3989.33     0.00    32.00
      avgqu-sz   await  svctm  %util
         1.00    8.01   8.02 100.00

As expected, we're doing about 125 random IOs per second, each at 16k (32.00 512 byte sectors), at a mean service time of around 8ms. That's pretty much exactly what we would expect from a 7200 RPM magnetic drive. Nothing to see there, then. Next up, is a drive I'm a little bit suspicious about. The demanding IO application I've been running on it has been sluggish, but other than that I have no real evidence that it's bad. SMART checks out, for as little as that's worth. *iostat -x* indicates that things are a little slow, but not off the charts:

    Device:        rrqm/s   wrqm/s     r/s     w/s   rsec/s   wsec/s avgrq-sz
    sdf              0.00     0.00   41.33   50.67  1322.67  1621.33    32.00
      avgqu-sz   await  svctm  %util
          1.00   10.90  10.86  99.87

Time to turn to *blktrace* and see what we can find out. The first step to using *blktrace* is to capture an IO trace for a period of time. Here, I've chosen 30 seconds:

    blktrace -w 30 -d /dev/sdf -o sdf

The *blktrace* command writes down, in a group of files starting with *sdf*, a trace of all the IOs being performed to that disk. The trace is stored in a binary format, which obviously doesn't make for convenient reading. The tool for that job is *blkparse*, a simple interface for analyzing the IO traces dumped by *blktrace*. They are packaged together, so you'll have *blkparse* if you have *blktrace*. When given a *blktrace* file, *blkparse* outputs a stream of events like these:

    8,32   0    19190    28.774795629  2039  D   R 94229760 + 32 [fio]
    8,32   0    19191    29.927624071     0  C   R 94229760 + 32 [0]

At this point I'll have to come clean and admit that the "demanding IO application" is actually the IO benchmark tool, [fio](http://freecode.com/projects/fio), but that doesn't change the results. What you are looking at, in each of those events, is a fixed-field format like this:

    major,minor cpu sequence timestamp pid action rwbs offset + size [process_name]

This is nothing more than a stream of events - "this thing happened at this time". The first one means "At 28.774 seconds, a read (R) was issued to the driver (D)". The second one means "At 29.92 seconds, a read (R) was completed (C)". This is just two example events among many. *blktrace* writes down a large number of event types, so you'll end up with multiple events for each IO. Events include when the IO is queued (Q), merges (M), [plugging](http://lwn.net/Articles/438256/) (P) and unplugging (U) and others. Let's look at a second example, a single traced direct read from this device:

    dd if=/dev/sdf bs=1k of=/dev/null count=1 iflag=direct

That should be simple, right? It turns out that the Linux block IO layer is actually doing a bunch of work here:

    8,32   3        1     0.000000000  2208  Q   R 0 + 2 [dd]
    8,32   3        2     0.000002113  2208  G   R 0 + 2 [dd]
    8,32   3        3     0.000002891  2208  P   N [dd]
    8,32   3        4     0.000004193  2208  I   R 0 + 2 [dd]
    8,32   3        5     0.000004802  2208  U   N [dd] 1
    8,32   3        6     0.000005487  2208  D   R 0 + 2 [dd]
    8,32   0        1     0.000744872     0  C   R 0 + 2 [0]

Here, the IO is queued (Q), a request struct is allocated (G), the queue is [plugged](http://lwn.net/Articles/438256/) (P), the IO is scheduled (I), the queue is unplugged (U), the IO is dispatched to the device (D), and the IO is completed (C). All of that took only 744us, which makes me suspect that it was served out of cache by the device. That's a really simple example. Once merging and read ahead behaviors come into play, the traces can be difficult to understand. There's still a really big gap between having this IO trace, and being able to say something about the performance of the drive. If you're anything like me, you're considering the possibilities of writing a tool to parse these traces and come up with aggregate statistics about the whole trace. Luckily, one has already been written: [btt](http://www.cse.unsw.edu.au/~aaronc/iosched/doc/btt.html).

Passing our trace through *btt* gives us a whole lot of output, but the really interesting stuff (in this case) is in the first section. In fact, two lines tell us a huge amount about this disk:

                ALL           MIN           AVG           MAX           N
    --------------- ------------- ------------- ------------- -----------
    D2C               0.000580332   0.010877224   1.152828442        2744
    Q2C               0.000584308   0.010880923   1.152832326        2744

Here, Q2C is the total IO time (time from being queued to being completed, just like in the example above), and D2C is the IO time spent in the device. The values are in seconds, and N is the number of IOs observed. As was obvious from the *iostat* output, the queue time isn't very high, so most of what is going on with performance is the device (D2C) time. Here, that's shown by the relatively small difference between the D2C and Q2C lines. The minimum IO time, 584us, is very short. Those IOs must be served from cache somewhere. The mean time, 10.8ms, is slightly high for what we would expect from this drive (it's sibling averaged just over 8ms), but isn't crazy. The maximum, at 1.15s, clearly shows that there's something amiss about this drive. Our healthy drive's maximum D2C over the same test was only 160ms.

If you want even more IO latency detail, *btt* is capable of exporting nearly all the statistics it calculates in raw form. For example, the -l flag outputs all the samples of D2C latencies. Combined with the plotting capabilities of [R](http://www.r-project.org/) or [matplotlib](http://matplotlib.org/), you can quickly make graphs of the finest details of a system's IO performance. Two lines of R gave me this IO latency histogram:

![](https://s3.amazonaws.com/mbrooker-blog-images/io_latency_hist.png)

Another useful capability of *btt* is extracting the offsets of IOs (with the *-B* flag). The offsets can then be plotted, showing an amazing amount of detail about the current IO workload. In this example, I did an *apt-get update* then *apt-get install libssl1.0.0* on my aging Ubuntu desktop. The whole thing took about 90s, and issued about 7100 IO operations. Here's what the IO pattern looked like:

![](https://s3.amazonaws.com/mbrooker-blog-images/aptget-io-pattern.png)

That's an incredible set of capabilities. Creating a plot of the IO latency histogram of a running system is tremendously powerful, as is graphing access patterns over time. It's also just scratching the surface of the uses of the *blktrace* family. Other capabilities include counting seeks (though that's getting less interesting as SSDs take over the world), understanding merge behavior, and analyzing the overhead of various parts of the Linux IO subsystem. This is a set of tools that should be more widely known and used.