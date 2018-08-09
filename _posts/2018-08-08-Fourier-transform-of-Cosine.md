---
layout: post
mathjax: true
---

The Fourier transform maps a signal to its frequency spectrum, denoting exactly at which amplitude a given frequency appears in the signal.
The underlying assumption is that any signal out there can be decomposed in to an infinite sum of trigonometric functions with varying amplitude, phase, and frequency.
But what happens if one of the most basic functions, the cosine itself, is transformed?

## Prerequisites and Notation

I will use $\widehat{f}$ to refer to the Fourier Transform of $f$.

I will use the following definition of the Fourier transform:

Let $f \in L_2(\mathbb{R})$ be a signal. Then:

$$
\widehat{f}(\xi) = \int_{-\infty}^{\infty}f(t)e^{-i\xi t}dt
$$

## Transforming the Cosine

### Plugging the Cosine into the definition of the Fourier Transform

$$
\widehat{(cos)}(\xi) = \\
\int_{-\infty}^{\infty}cos(t)e^{-i\xi t}dt \\
\int_{-\infty}^{\infty}\frac{e^{it}+e^{-it}}{2}e^{-i\xi t}dt \\
\frac{1}{2}\int_{-\infty}^{\infty}e^{it}e^{-i\xi t}dt + \frac{1}{2}\int_{-\infty}^{\infty}e^{-it}e^{-i\xi t}dt\\
\frac{1}{2}\int_{-\infty}^{\infty}e^{it(1-\xi)}dt + \frac{1}{2}\int_{-\infty}^{\infty}e^{-it(1-\xi)}dt\\
\frac{1}{2}\int_{-\infty}^{\infty}e^{-it(\xi-1)}dt + \frac{1}{2}\int_{-\infty}^{\infty}e^{-it(1-\xi)}dt\\
\frac{1}{2}\int_{-\infty}^{\infty}1 e^{-it(\xi-1)}dt + \frac{1}{2}\int_{-\infty}^{\infty}1 e^{-it(1-\xi)}dt\\
\frac{1}{2}\widehat{1}(\xi-1) + \frac{1}{2}\widehat{1}(1-\xi)\\
$$

So we see that the spectrum of the cosine is comprised of two other certain spectra, which are shifted in time.

### But what is the fourier transform of 1?

This can be approached using intuition. Given that the value of the spectrum of a function at frequency $\xi = 0$ is equal to the mean of the function:

$$
\widehat{1}(0) = \int_{-\infty}^{\infty}1dt = \infty
$$

We can conclude that our spectrum is infinite at 0. But what about the rest of the function?

The 1-function does not have any other spectral components - it is boring and flat, completely defined without any frequency components other than 0.

A function that is infinite at 0 and 0 everywhere else - sounds familiar? The Dirac Impulse function!

$$
\delta(t) =
\begin{cases}
0,  & t \lt 0 \\
\infty,  & t = 0 \\
0,  & t \gt 0
\end{cases}
$$

So, now that we have the transform of the 1-function (albeit the derivation was rather hand-wavy), we can state the fourier transform of the cosine function.

### Final Result

$$
\frac{1}{2}\widehat{1}(\xi-1) + \frac{1}{2}\widehat{1}(1-\xi)\\
\frac{1}{2}\delta(\xi-1) + \frac{1}{2}\delta(1-\xi)\\
$$

The result: Two Impulses, one at 1, one at -1, suffice to describe the spectrum of the cosine function!
