---
layout: post
mathjax: true
---

Filtering data and signals is a great application of mathematics in signal processing. But with all the Fourier transforms, Convolutions, and various properties like causality, time-invariance, linearity... It can get confusing. In this post, I want to clarify the relationship between a filter, it's impulse response and transfer function, and why applying a filter is the same as convoluting a transformed signal with a transfer function. Don't worry, it will sound less intimidating in the end.

## Prerequisites and Notation

I will use $\widehat{f}$ to refer to the Fourier Transform of $f$ and $f \ast c$ to refer to the Convolution of $f$ and $c$. Their raw definitions are:

Let $c \in l_2(\mathbb{Z})$ be a signal. Then:

$$
\widehat{c}(\xi) = \sum_{k \in \mathbb{Z}}c(k)e^{-i\xi k}
$$

Let $c, d \in l_2(\mathbb{Z})$. Then:

$$
(c \ast d)(\tau) = \sum_{k \in \mathbb{Z}}c(\tau - k)d(k)
$$

A fundamental theorem in Signal Processing is the Convolution Theorem. I will only state it here, but it will be important later:

Let $c, d \in l_2(\mathbb{Z})$. Then:

$$
\widehat{(c \ast d)}(\xi) = \widehat{c}\widehat{d}
$$

## What's a filter?

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

A simple example: set all values of the filter to $1/N$. Then a sample in a filtered signal is the arithmetic mean of the samples around it, removing high-frequency content.

## Properties of Filters

Notice the filter coefficients in the previous example where $1/N$. Otherwise we might not preserve the energy of the signal. Obviously, you don't want a filtered signal to be amplified or diminished. This kind of filter is called "Energy-Preserving".

There are many more properties, one of which is linearity. A linear filter has the property that you can apply it to a sum of signals and get the same result as applying it to each  signal individually and then summing the result. Put in another way:

Let $c, d \in l_2(\mathbb{Z})$ and $a, b \in \mathbb{R}$ . Then:

$$
F(ac+bd) = aFc + bFd
$$

This property will become important later.

## The Unit Impulse and why it is important

The Unit Impulse function is... not actually a function, but a distribution. However, we will treat it as the following function here:

$$
\delta(t) =
\begin{cases}
0,  & t \lt 0 \\
1,  & t == 0\\
0,  & t \gt 0
\end{cases}
$$

The impulse contains frequencies from every wavelength with the same amplitude. Intuitively, it is a "bang", like a gunshot. It's Fourier Transform is... just $1$. In other words, we can get a unit impulse by summing up cosines of ever-increasing frequency. So what happens if we apply a filter to a unit impulse? Well, by above linearity property, that would be the same as applying the filter to each of the frequencies the unit impulse is comprised of - which is, all of them. You read that right - with the unit impulse, we can see what a filter does to ANY frequency. Does it diminish high frequency? Only the low frequencies from the impulse get through. Is there a band that the filter attenuates? Those frequencies will be dimmed in the result. By the way, the result of applying a filter to a unit impulse is aptly called the "Impulse Response".

Now let's think about what the Fourier Transform of an impulse response contains. The impulse response contains every frequency with the amplitude that the filter permits for this frequency. So it's spectral content directly displays how the filter behaves for a given frequency.

I will denote the impulse response $F\delta$ of a filter $F$ with $f$.

## Convolution and applying filters

Now let's clear up why applying filters can be done as a convolution. The best explanation I found is rather mathematical, but "easy to understand".

First, let's look at a signal $c$ in a slightly contrived way:

$$
c(\cdot) = \sum_{k \in \mathbb{Z}}c(k)\delta(\cdot-k)
$$

Equivalently, where $\tau_{s}c(\cdot) = c(\cdot + s)$:

$$
c(\cdot) = \sum_{k \in \mathbb{Z}}c(k)\tau_{-k}\delta
$$

Which is to say: you can picture a signal at a point as a unit impulse at exactly that point multiplied with the signal.

Now what happens if we apply a filter $F$ to that signal and simplify the result?

$$Fc(\cdot) = F\Big[\sum_{k \in \mathbb{Z}}c(k)\tau_{-k}\delta\Big] = \sum_{k \in \mathbb{Z}}c(k)F[\tau_{-k}\delta] = \sum_{k \in \mathbb{Z}}c(k)\tau_{-k}F[\delta] = \sum_{k \in \mathbb{Z}}c(k)\tau_{-k}f = \sum_{k \in \mathbb{Z}}c(k)f(\cdot - k) = (c \ast f)(\cdot)
$$
