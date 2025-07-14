---
layout: post
title: Java's Atomic and volatile, under the hood on x86









related_posts:
  - "/2013/01/06/volatile.html"
  - "/2012/09/10/volatile.html"
  - "/2012/09/10/locking.html"
dissimilar_posts:
  - "/2021/08/11/arecibo.html"
---
{{ page.title }}
================

<p class="meta">How exactly do AtomicInteger and volatile do their magic in Java?</p>

It's well known that *volatile* in Java doesn't mean the same thing as *atomic*. As [Jeremy Manson says](http://jeremymanson.blogspot.com/2007/08/volatile-does-not-mean-atomic.html):

> If you do an increment of a volatile integer, you are actually performing three separate operations:
> 1) Read the integer to a local.
> 2) Increment the local
> 3) Write the integer back out to the volatile field.

On the other hand, the Java memory model is known to offer looser guarantees than some real hardware implementations. Most modern desktops (and most current servers) are powered by x86-family processors, which have fairly predictable memory behavior, especially [compared to some other processor families](http://preshing.com/20121019/this-is-why-they-call-it-a-weakly-ordered-cpu). Despite these stronger guarantees, *volatile int* doesn't behave like AtomicInteger, even on x86 and even for a very simple operation like counting.

Why not?

To understand what is going on under the hood, let's start with a very simple piece of code, which does only three things:

 1. Launches M threads
 2. Loops a large number of times (N) in each thread, incrementing a shared variable.
 3. Joins the threads.

The three threads are running in parallel, and sharing the value of the variable. For it to end up with the right value at the end (M\*N), two things need to be true. First, changes made by one threads must be immediately *visible* to the other threads. Second, changes made to the variable must be *atomic* - each threads must perform the load, increment and save as one effective operation. Visibility is not enough, because it allows something like this to happen:

 1. Thread 1 loads 5
 2. Thread 1 increments its private copy to 6.
 3. Thread 2 stores 6
 4. Thread 1 stores 6

Even if changes are immediately visible, increments done by done by one thread can be lost by others. To see the effects of this, we can start with a version of the program with a non-*volatile* shared variable, which offers neither *visibility* nor *atomicity*. On my system, based on a four-core Intel Core2 Q6600 CPU, with 3 threads each counting to 5 million the result is:

> Final value 7147559, expected 15000000, time 0.05ms

That worked rather poorly, but was nice and quick. Adding *volatile* adds visibility, but not atomicity. With the same parameters:

> Final value 5191650, expected 15000000, time 2286ms

In this particular sample, it's actually worse, and much much slower. It's worth noting that both of these return some seemingly random value between 5 million and 15 million, mostly clustered near to 5 million. Obviously [AtomicInteger](http://docs.oracle.com/javase/6/docs/api/java/util/concurrent/atomic/AtomicInteger.html), which guarantees both *atomicity* and *visibility*, will solve the problem:

> Final value 15000000, expected 15000000, time 3041ms

The Atomic version is correct. It's also slower, but not by a huge margin. Clearly these three versions are leading to very different CPU behavior, so let's turn to [-XX:+PrintAssembly](https://wikis.oracle.com/display/HotSpotInternals/PrintAssembly) to see if we can figure out what's going on. First, the increment code from the non-*volatile* version:

    add    $0x10,%ecx

Notice how the increment has turned into an add of 16. The compiler has optimized away a lot of the looping, and is doing 1/16 of as many passes through the actual loop as we specified. I'm somewhat surprised that it even does that amount of work. On to the *volatile* version. Here, we can see the three separate steps:

    mov    0xc(%r10),%r8d ; Load
    inc    %r8d           ; Increment
    mov    %r8d,0xc(%r10) ; Store
    lock addl $0x0,(%rsp) ; StoreLoad Barrier

I wrote about the StoreLoad barrier in [my previous post on Java's volatile](http://brooker.co.za/blog/2012/09/10/volatile.html). It's exact semantics [can be subtle](http://preshing.com/20120710/memory-barriers-are-like-source-control-operations), but the quick version is that it does two things: makes every store before the *lock addl* visible to other processors, and ensures that every load after the *lock addl* gets at least the version visible at the time it is executed. In this case, volatile gives *visibility*, in that each of the processors immediately gets the version from the other processors after each increment. What it doesn't give is *atomicity*. Any stores that happen on other processes between the load and the store are lost, as in the example above. [AtomicInteger](http://docs.oracle.com/javase/6/docs/api/java/util/concurrent/atomic/AtomicInteger.html) fixes this problem by seeking the processor's help to be truly atomic.

    mov    0xc(%r11),%eax       ; Load
    mov    %eax,%r8d            
    inc    %r8d                 ; Increment
    lock cmpxchg %r8d,0xc(%r11) ; Compare and exchange

To understand how this works, we need to understand what *cmpxchg* does. The [Intel Software Developer's Manual](http://download.intel.com/products/processor/manual/325383.pdf) describes it as:

> Compares the value in the EAX register with the destination operand. If the two values are equal, the source operand is loaded into the destination operand. Otherwise, the destination operand is loaded into the EAX register.

So, we're loading the value, incrementing it, then only writing it into memory if nobody else has overwritten it with a different value since we loaded the value. Obviously, the operation needs to be retried if the store to memory fails. Indeed, the Java does just that. I'll spare you the verbose assembly version, and present that Java version from OpenJDK:

    public final int incrementAndGet() {
      for (;;) {
        int current = get();
        int next = current + 1;
        if (compareAndSet(current, next))
          return next;
      }
    }

The *compareAndSet* function is a native implementation from Unsafe.java. Counting the instructions from the disassemblies would suggest that the Atomic version should be, at most, 100 times slower than the non-Atomic non-Volatile version. So why is it tens of thousands of times slower? The best evidence comes from modifying the programs so the threads don't contend - simply running one, then the other. Removing the contention doesn't do anything to the non-Atomic non-volatile version: it still runs in about 0.05ms. The performance difference when contention is removed from the volatile and Atomic versions is rather startling. The volatile version drops from 2286ms to 0.1ms, and the Atomic version drops from 3041ms to 0.15ms. In both cases, the serial version is around twenty thousand times faster. In trivial examples like this, it's clear that [Amdahl's Law](http://en.wikipedia.org/wiki/Amdahl%27s_law), which assumes that paralellizing a program doesn't make it slower, is highly over-optimistic. This is [parallel slowdown](http://en.wikipedia.org/wiki/Parallel_slowdown) in action.

To understand what's making this amazing 10-thousand-fold difference in program runtime, we can turn to [Linux performance counters](https://perf.wiki.kernel.org/index.php/Main_Page), and run the JVM under the *perf stat* program. Here are some highlights of the report for the two versions, with the no-contention (fast) version first:

       806,970,762 instructions              #    0.52  insns per cycle 
       165,637,507 branches                  #   57.996 M/sec         
         2,293,172 branch-misses             #    1.38% of all branches
       195,282,430 L1-dcache-loads           #   68.375 M/sec          
         4,348,143 L1-dcache-load-misses     #    2.23% of all L1-dcache hits
         1,978,387 LLC-loads                 #    0.693 M/sec                
           110,472 LLC-load-misses           #    5.58% of all LL-cache hits 
       0.713189740 seconds time elapsed

Then the slow version, with contention:

     1,066,079,264 instructions              #    0.08  insns per cycle      
       227,089,065 branches                  #   17.347 M/sec                
        10,659,632 branch-misses             #    4.69% of all branches      
       408,262,745 L1-dcache-loads           #   31.187 M/sec                
        24,905,458 L1-dcache-load-misses     #    6.10% of all L1-dcache hits
        21,168,959 LLC-loads                 #    1.617 M/sec                
        12,133,097 LLC-load-misses           #   57.32% of all LL-cache hits 
       3.271893871 seconds time elapsed

We must be careful interpreting these results, because they are polluted with data not related to our program under test, like the JVM startup sequence. Still, the differences are quite obvious. Our first smoking gun is instructions per cycle (the first line) - this measures how many CPU instructions were executed per clock cycle. This is the amount of time the CPU could actually do work without being blocked on other things, like loading data from RAM. Higher is obviously better. The fast no-contention version gets a respectable 0.52 instructions per cycle. The slow version scores a terrible 0.08 - indicating 92% of cycles were wasted. Moving down the report indicates why this is the case, with the last line being the most damning. We went from 110 thousand LLC (last-level cache) misses in the fast version, to over 12 million in the slow version. The slow version is spending most of its time waiting for data from RAM.

Why does contention cause an otherwise identical program to behave so differently? The answer lies in the way that the CPUs in my multi-core system are forced to keep their caches in sync to meet the requirement of *visibility*. The short version is that each core keeps track of which cache lines (chunks of memory in cache) are shared with other CPUs, and puts in extra effort to tell other CPUs about writes it makes to those lines. If you want to the long version, Ulrich Drepper explains it very well in section 3 of his excellent [What Every Programmer Should Know About Memory](http://www.akkadia.org/drepper/cpumemory.pdf). I highly recommend reading it.

The most important lesson here is how critical profiling is to writing high-performing parallel programs. Sometimes our intuition (more parallel equals faster) can be dangerously incorrect. The second lesson is the importance of minimizing contention and communication between cores.