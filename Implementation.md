# Implementation details

## Performance optimizations

There are few tricks required to reach decent performance.

### Frequency resolution

Although the wavelet transform implementation allows *unbounded* frequency resolution, in practice for music audio
files we only need to know about the frequencies of each note. This means two things:

- We only need to perform 100 or so single-frequency wavelet transforms for music files.
- We can use wavelets with about 16 wavelengths to reach good enough frequency/time resolution balance

### Fourier for convolution

We can compute convolution in time between two signals by multiplying their fourier transforms and performing the
inverse transform on the result.

If the two signals have respectively N and M samples, then the fourier convolution can be done
in `O((N+M)' * log(N+M)')` while the time convolution would be `O(N*M)`. Note: `(N+M)'` here means the nearest power of
2 > N+M (due to 0 padding of the inputs).

Since we only gain a performance improvement if `(N+M)' * log(N+M)' < N * M` we should use this as criterion to decide
whether to use fourier-based convolution or not for every single frequency (taking into account constants).