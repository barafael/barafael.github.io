+++
title = "Impulse Response and Transfer Function"
date = 2018-07-08

+++

Applying filters to signals is one of the most important applications of mathematics in signal processing.
But with all the Fourier transforms, Convolutions, and various properties like causality, time-invariance, linearity...  It can get confusing.
In this post, I want to clarify the relationship between a filter, it's impulse response and transfer function, and why applying a filter is the same as convoluting a transformed signal with an impulse response.
Don't worry, it will sound less intimidating in the end.

## Prerequisites and Notation

I will use {% katex() %}\widehat{c}{% end %} to refer to the Fourier Transform of {% katex() %}c{% end %} (and {% katex() %}(c + d)\hat{ }{% end %} for the transform of larger terms).
Conversely, I will use the inverted hat for the inverse of the Fourier Transform: {% katex() %}c(t) = (\widehat{c}(\cdot))\vee(t){% end %}.

To denote the Convolution of signals {% katex() %}c{% end %} and {% katex() %}d{% end %}, I will use {% katex() %}c \ast d{% end %}.

The raw definitions are (incomplete for brevity):

Let {% katex() %} c \in l_2(\mathbb{Z}) {% end %} be a signal. Then:

{% katex(block=true) %}
\widehat{c}(\xi) = \sum_{k \in \mathbb{Z}}c(k)e^{-i\xi k}
{% end %}

Let {% katex() %}c, d \in l_2(\mathbb{Z}){% end %}. Then:

{% katex(block=true) %}
(c \ast d)(\tau) = \sum_{k \in \mathbb{Z}}c(\tau - k)d(k)
{% end %}

A fundamental theorem in Signal Processing is the Convolution Theorem:

Let {% katex() %}c, d \in l_2(\mathbb{Z}){% end %}. Then:

{% katex(block=true) %}
\widehat{(c \ast d)}(\xi) = \widehat{c}(\xi)\widehat{d}(\xi)
{% end %}

The proof is included here for completeness, however it is **not particularly important for the rest of the post, so feel free to skip it**:

{% katex(block=true) %}
\def\colonequals{:=}
\widehat{(c \ast d)}(\xi) = \\
\sum_{k \in \mathbb{Z}}(c \ast d)(k)e^{-i\xi k}= \\
\sum_{k \in \mathbb{Z}}\sum_{s\in\mathbb{Z}}c(k-s)d(s) e^{-i\xi k}=\\ \sum_{k \in \mathbb{Z}}\sum_{s\in\mathbb{Z}}c(k-s)d(s) e^{-i\xi (k-s+s)}=\\
\sum_{k \in \mathbb{Z}}\sum_{s\in\mathbb{Z}}c(k-s)d(s) e^{-i\xi (k-s)}e^{-i\xi s}=\\
\sum_{k \in \mathbb{Z}}\sum_{s\in\mathbb{Z}}c(k-s)e^{-i\xi (k-s)}d(s) e^{-i\xi s}=\\
\sum_{s \in \mathbb{Z}}\sum_{k\in\mathbb{Z}}c(k-s)e^{-i\xi (k-s)}d(s) e^{-i\xi s}=\\
\sum_{s \in \mathbb{Z}}d(s) e^{-i\xi s}\sum_{k\in\mathbb{Z}}c(k-s)e^{-i\xi (k-s)}\\
\kappa\colonequals k-s;\\
\sum_{s \in \mathbb{Z}}d(s) e^{-i\xi s}\sum_{\kappa\in\mathbb{Z}}c(\kappa)e^{-i\xi \kappa}=\\
\sum_{\kappa\in\mathbb{Z}}c(\kappa)e^{-i\xi \kappa}\sum_{s \in \mathbb{Z}}d(s) e^{-i\xi s}=\\
\widehat{c}(\xi)\widehat{d}(\xi)
{% end %}

## What's a Filter?

A filter is just a vector of some length N (or function on some interval), where we interpret the values as coefficients of the filter. The vector of filter coefficients will be called a "filter window" from here on.

Filtering a signal can be achieved like this:

```
for each sample:
    place the window on the signal, such that the sample is at the window center
    for each value of the window:
        multiply the value with the corresponding signal sample
        add the result to an accumulator A
    
    set the value of the center sample to the value of A
```

(This explanation is probably very engineer-like and insufficient for mathematicians. What can I do.)

A simple example: set all values of the filter to {% katex() %}1/N{% end %}. Then a sample in a filtered signal is the arithmetic mean of the samples around it, removing high-frequency content.

## Properties of Filters

Notice the filter coefficients in the previous example where {% katex() %}1/N{% end %}. Otherwise, we might modify the energy of the signal. Obviously, you don't want a filtered signal to be amplified or diminished. This kind of filter is called "Energy-Preserving".

There are many more properties, one of which is linearity. A linear filter has the property that you can apply it to a sum of signals and get the same result as applying it to each  signal individually and then summing the result. Put in another way:

Let {% katex() %}c, d \in l_2(\mathbb{Z}){% end %} and {% katex() %}a, b \in \mathbb{R}{% end %} . Then:

{% katex(block=true) %}
F(ac+bd) = aFc + bFd
{% end %}

This property will become important later.

## The Unit Impulse and why it is important

The Unit Impulse function (or dirac delta) is... not actually a function, but a distribution. However, we will treat it as the following function here:

{% katex(block=true) %}
\delta(t) =
\begin{cases}
0,  & t \lt 0 \\
1,  & t = 0 \\
0,  & t \gt 0
\end{cases}
{% end %}

The impulse contains frequencies from every wavelength with the same amplitude.
Intuitively, it is a "bang", like a gunshot. It's Fourier Transform is... just {% katex() %}1{% end %}:

{% katex(block=true) %}
\widehat{\delta}(\xi) = \int_{-\infty}^{\infty}\delta (t)e^{-i\xi t}dt = 1
{% end %}

In other words, we can get a unit impulse by summing up cosines of ever-increasing frequency.
So what happens if we apply a filter to a unit impulse?
Well, by the linearity property stated above, that would be the same as applying the filter to each of the frequencies the unit impulse is comprised of - which is, all of them.
You read that right - with the unit impulse, we can see what a filter does to ANY frequency.
Does it diminish high frequency? Only the low frequencies from the impulse get through.
Is there a band that the filter attenuates? Those frequencies will be dimmed in the result. By the way, the result of applying a filter to a unit impulse is aptly called the "Impulse Response".

Now, let's think about the Fourier Transform of an impulse response.
The impulse response contains every frequency with the amplitude that the filter permits for this frequency.
So it's spectral content directly displays how the filter behaves for a given frequency.

I will denote the impulse response {% katex() %}F\delta{% end %} of a filter {% katex() %}F{% end %} with {% katex() %}f{% end %}.

## Convolution and Filter Application

Now let's clarify why applying filters can be achieved by Convolution. The best explanation I found is rather mathematical, but somewhat easy to understand.

First, let's look at a signal {% katex() %}c{% end %} in a slightly contrived way:

{% katex(block=true) %}
c(\cdot) = \sum_{k \in \mathbb{Z}}c(k)\delta(\cdot-k)
{% end %}

Equivalently, where {% katex() %}\tau_{s}c(\cdot) = c(\cdot + s){% end %}:

{% katex(block=true) %}
c(\cdot) = \sum_{k \in \mathbb{Z}}c(k)\tau_{-k}\delta
{% end %}

Which is to say: you can picture a signal at a point as a unit impulse shifted to exactly that point multiplied with the entire signal.

Now let's see what happens if we apply a filter {% katex() %}F{% end %} to that signal and simplify the result.

{% katex(block=true) %}
Fc(\cdot) = F\Big[\sum_{k \in \mathbb{Z}}c(k)\tau_{-k}\delta\Big] = \\
\sum_{k \in \mathbb{Z}}c(k)F[\tau_{-k}\delta] = \\
\sum_{k \in \mathbb{Z}}c(k)\tau_{-k}F\delta = \\
\sum_{k \in \mathbb{Z}}c(k)\tau_{-k}f = \\
\sum_{k \in \mathbb{Z}}c(k)f(\cdot - k) = \\
(c \ast f)(\cdot)
{% end %}

And there you have it! Applying a filter to a signal is the same as multiplying the transformed signal with the transfer function and taking the inverse Fourier Transform of the result:

{% katex(block=true) %}
Fc(t) = (c \ast f)(t) = (\hat{c}\hat{f})\vee(t)
{% end %}
