---
layout: post
title: "Two traps in iostat: %util and svctm"
---

{{ page.title }}
================

<p class="meta">These commonly-used fields in iostat shouldn't be commonly-used.</p>

[iostat](), from the excellent [sysstat](http://sebastien.godard.pagesperso-orange.fr/) suite of utilities, is the go-to tool for evaluating IO performance on Linux. It's obvious why that's the case: sysstat is very useful, solid, and widely installed. System administrators can go a lot worse than taking a look at *iostat -x*. There are some serious caveats lurking in *iostat*'s output, two of which are greatly magnified on newer machines with solid state drives.

To explain what's wrong, let me compare two lines of *iostat* output:

    Device:     rrqm/s   wrqm/s       r/s     w/s    rkB/s    wkB/s avgrq-sz 
    sdd           0.00     0.00  13823.00    0.00 55292.00     0.00     8.00
                 avgqu-sz   await r_await w_await  svctm  %util
                     0.78    0.06    0.06    0.00   0.06  78.40

    Device:     rrqm/s   wrqm/s       r/s     w/s    rkB/s    wkB/s avgrq-sz
    sdd           0.00     0.00  72914.67    0.00 291658.67    0.00     8.00
                 avgqu-sz   await r_await w_await  svctm  %util
                    15.27    0.21    0.21    0.00   0.01 100.00

Both of these lines are from the same device (a [Samsung 840 EVO](http://www.samsung.com/global/business/semiconductor/minisite/SSD/global/html/about/SSD840EVO.html) SSD), and both are from read-only 4kB random read loads. What differs here is the level parallelism: in the first load the mean queue depth is only 0.78, and in the second it's 15.27. Same pattern, more threads.

The first problem we run into with this output is the *svctm* field, widely taken to mean *the amount of time an operation takes*. The iostat man page describes it as:

> The average service time (in milliseconds) for I/O requests that were issued to the device.

and goes on to say:

> The average service time (svctm field) value is meaningless, as I/O statistics are now calculated at block level, and we don't know when the disk driver starts to process a request.

The reasons the man page states for this field being meaningless are true, as are the warnings in the sysstat code. The calculation behind *svctm* is fundamentally broken, and doesn't really have a clear meaning. Inside iostat, svctm in an interval is calculated as *time the device was doing some work* / *number of IOs*, that is the amount of time we were doing work, divided by the amount of work that we being done. If we go back to our two workloads, we can compare their service times:

    svctm
    0.06
    0.01

Taken literally, this means the device was responding in 60µs when under little load, and 10µs when under a lot of load. That seems unlikely, and indeed the load generator [fio](https://github.com/axboe/fio) tells us it's not true. So what's going on?

![Hard Drive Exposed By Evan-Amos (Own work) CC-BY-SA-3.0 via Wikimedia Commons](https://s3.amazonaws.com/mbrooker-blog-images/Laptop-hard-drive-exposed-Evan-Amos.jpg)

Magnetic hard drives are serial beings. They have a few tiny heads, ganged together, that move over a spinning platter to a single location where they do some IO. Once the IO is done, and no sooner, they move on. Over the years, they've gathered some shiny capabilities like [NCQ](http://en.wikipedia.org/wiki/Native_Command_Queuing) and [TCQ](http://en.wikipedia.org/wiki/Tagged_Command_Queuing) that make them appear parallel (mostly to allow reordering), but they're still the same old horse-and-carriage sequential devices they've always been. Modern hard drives expose some level of concurrency, but no true parallelism. SSDs, like the Samsung 840 EVO in this test, are different. SSDs can and do perform operations in parallel. In fact, the only way to achieve their peak performance is to offer them parallel work to do.

While SSDs vary in the details of their internal construction, most have the ability to access multiple flash *packages* (groups of chips) at a time. This is a big deal for SSD performance. Individual flash chips actually don't have great bandwidth, so the ability to group the performance of many chips together is essential. The chips are completely independent, and because the controller doesn't need to block on requests to the chip, the drive is truly doing multiple things at once. Without the single electromechanical head as a bottleneck, even single SSDs can have a fairly large amount of internal parallelism. This diagram from [Agarwal, et al](http://research.microsoft.com/pubs/63596/usenix-08-ssd.pdf) shows the high-level architecture:

![](https://s3.amazonaws.com/mbrooker-blog-images/agrawal-ssd-arch.png)

If Jane does one thing at a time, and doing ten things takes Jane 20 minutes, each thing has taken Jane an average of two minutes. The mean time between asking Jane to do something and Jane completing it is two minutes. Alice, like Jane, can do ten things in twenty minutes, but she works on multiple things in parallel. Looking only at Alice's throughput (the number of things she gets done in a period of time) what can we say about Alice's latency (the amount of time it takes her from start to finish for a task)? Very little. We know its less than 10 minutes. If she's busy the whole time, we know it's 2 minutes or more minutes. That's it.

Let's go back to that iostat output:

    Device:     rrqm/s   wrqm/s       r/s     w/s    rkB/s    wkB/s avgrq-sz 
    sdd           0.00     0.00  13823.00    0.00 55292.00     0.00     8.00
                 avgqu-sz   await r_await w_await  svctm  %util
                     0.78    0.06    0.06    0.00   0.06  78.40

    Device:     rrqm/s   wrqm/s       r/s     w/s    rkB/s    wkB/s avgrq-sz
    sdd           0.00     0.00  72914.67    0.00 291658.67    0.00     8.00
                 avgqu-sz   await r_await w_await  svctm  %util
                    15.27    0.21    0.21    0.00   0.01 100.00

What's going on with *%util*, then? The first line is telling us that the drive is 78.4% utilized at 13823 reads per second. The second line is telling us that the drive is 100% utilized at 72914 reads per second. If it takes 14 thousand to fill it to 78.4%, wouldn't we expect it to only be able to do 18 thousand in total? How is it doing 73 thousand?

The problem here is the same - parallelism. When iostat says *%util*, it means "Percentage of CPU time during which I/O requests were issued to the device". The percentage of the time the drive was doing *at least one thing*. If it's doing 16 things at the same time, that doesn't change. Once again, this calculation works just fine for magnetic drives (and Jane), which do only one thing at a time. The amount of time they spend doing one thing is a great indication of how busy they really are. SSDs (and RAIDs, and Alice), on the other hand, can do multiple things at once. If you can do multiple things in parallel, the percentage of time you're doing *at least one thing* isn't a great predictor of your performance potential. The iostat man page does provide a warning:

> Device saturation occurs when this value is close to 100% for devices serving requests serially.  But for devices serving requests in parallel, such as RAID arrays and modern SSDs, this number does not reflect their performance limits.

As a useful measure of general IO busyness *%util* is fairly handy, but as an indication of how much the system is doing compared to what it can do, it's terrible. Iostat's *svctm* has even fewer redeeming strengths. It's just extremely misleading for most modern storage systems and workloads. Both of these fields are likely to mislead more than inform on modern SSD-based storage systems, and their use should be treated with extreme care.

<sub>Hard drive image by Evan-Amos (Own work) [CC-BY-SA-3.0 (http://creativecommons.org/licenses/by-sa/3.0) or GFDL (http://www.gnu.org/copyleft/fdl.html)], via Wikimedia Commons</sub>
