---
layout: post
mathjax: true
---

Filtering data and signals is a great application of mathematics in signal processing. But with all the Fourier transforms, Convolutions, and various properties like causality, time-invariance, linearity... It can get confusing. In this post, I want to clarify the relationship between a filter, it's impulse response and transfer function, and why applying a filter is the same as convoluting a transformed signal with a transfer function. Don't worry, it will sound less intimidating in the end.

## Prerequisites and Notation

I will use $\widehat{f}$ to refer to the Fourier Transform of $f$ and $f \ast c$ to refer to the Convolution of $f$ and $c$. Their raw definitions are:

Let $f \in l_2(\mathbb{Z})$ be a signal. Then:

$$
\widehat{f}(\xi) = \sum_{k \in \mathbb{Z}}f(k)e^{-i\xi k}
$$

Let $f, c \in l_2(\mathbb{Z})$. Then:

$$
(f \ast c)(\tau) = \sum_{k \in \mathbb{Z}}f(\tau - k)c(k)
$$

A fundamental theorem in Signal Processing is the Convolution Theorem. I will only state it here, but it will be important later:

Let $f, c \in l_2(\mathbb{Z})$. Then:

$$
\widehat{(f \ast c)}(\xi) = \widehat{f}\widehat{c}
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

A simple example: set all values of the filter to 1/N. Then a sample in a filtered signal is the arithmetic mean of the samples around it, removing high-frequency content.

Notice the filter coefficients where 1/N. Otherwise we might not preserve the energy of the signal. Obviously, you don't want a filtered signal to be amplified or diminished.
