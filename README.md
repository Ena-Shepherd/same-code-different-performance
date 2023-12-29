# How the same machine code can have different performance

This repo contains minimum reproducible examples for the following phenomenon:

> The same machine code can have different performance when it placed in different location in virtual memory.

Using some linker shenenigans, we produce several identical versions of the same machine code. The only difference is the location in virtual memory where the code is placed.

## How to reproduce

```console
$ cargo run --release -- --bench

factorials/factorial_1  time:   [331.11 ns 333.89 ns 337.15 ns]
factorials/factorial_2  time:   [255.63 ns 258.37 ns 261.76 ns]
factorials/factorial_3  time:   [252.26 ns 254.12 ns 255.92 ns]
factorials/factorial_4  time:   [331.93 ns 334.25 ns 336.48 ns]
factorials/factorial_5  time:   [331.58 ns 333.40 ns 335.14 ns]
factorials/factorial_6  time:   [254.01 ns 255.98 ns 257.92 ns]
factorials/factorial_7  time:   [253.24 ns 255.03 ns 256.61 ns]
factorials/factorial_8  time:   [334.26 ns 335.79 ns 337.54 ns]
factorials/factorial_9  time:   [330.33 ns 332.61 ns 334.70 ns]
factorials/factorial_10 time:   [256.85 ns 258.82 ns 260.62 ns]
```

In this setup functions 1, 4, 5, 8, 9 are predictabily slower than functions 2, 3, 6, 7, 10. The effect is persistent across multiple runs.

## Perpoducibility

This effect is present to some extent on all modern x86-64 CPUs although circumstances required to reporouce it differs slightly on different microarchitectures. The effect is most pronounced on Intel Sandy Bridge and newer CPUs with addition of Decoded Stream Buffer.

Tested on:

- `Intel Core i7-1068NG7` on `darwin 23.2.0`
- `Intel Core i7-1068NG7` on `linux 6.5.13`
