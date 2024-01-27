# How the same machine code can have different performance

This repo contains minimum reproducible examples for the following phenomenon:

> The same machine code can have different performance when it placed in different location in virtual memory.

Using some linker shenenigans, we produce several identical versions of the same machine code. The only difference is the location in virtual memory where the code is placed.

## How to reproduce

Code in this repo produces severeal identical factorial functions and tests their performance reporting discrepancy
(slowest function time - fastest function time).

```console
$ ./run
NOP_COUNT=1 max-min difference = 4
NOP_COUNT=2 max-min difference = 3
NOP_COUNT=3 max-min difference = 2
NOP_COUNT=4 max-min difference = 3
NOP_COUNT=5 max-min difference = 4
NOP_COUNT=6 max-min difference = 2
NOP_COUNT=7 max-min difference = 2
NOP_COUNT=8 max-min difference = 3
NOP_COUNT=9 max-min difference = 2
NOP_COUNT=10 max-min difference = 6
NOP_COUNT=11 max-min difference = 3
NOP_COUNT=12 max-min difference = 4
NOP_COUNT=13 max-min difference = 3
NOP_COUNT=14 max-min difference = 59
NOP_COUNT=15 max-min difference = 59
NOP_COUNT=16 max-min difference = 39
NOP_COUNT=17 max-min difference = 31
NOP_COUNT=18 max-min difference = 57
NOP_COUNT=19 max-min difference = 57
NOP_COUNT=20 max-min difference = 39
NOP_COUNT=21 max-min difference = 30
NOP_COUNT=22 max-min difference = 56
NOP_COUNT=23 max-min difference = 57
NOP_COUNT=24 max-min difference = 45
NOP_COUNT=25 max-min difference = 45
NOP_COUNT=26 max-min difference = 46
NOP_COUNT=27 max-min difference = 46
NOP_COUNT=28 max-min difference = 52
NOP_COUNT=29 max-min difference = 1
NOP_COUNT=30 max-min difference = 2
NOP_COUNT=31 max-min difference = 2
NOP_COUNT=32 max-min difference = 3
NOP_COUNT=33 max-min difference = 2
NOP_COUNT=34 max-min difference = 3
NOP_COUNT=35 max-min difference = 3
NOP_COUNT=36 max-min difference = 3
NOP_COUNT=37 max-min difference = 3
NOP_COUNT=38 max-min difference = 3
NOP_COUNT=39 max-min difference = 3
NOP_COUNT=40 max-min difference = 3
NOP_COUNT=41 max-min difference = 3
NOP_COUNT=42 max-min difference = 3
NOP_COUNT=43 max-min difference = 3
NOP_COUNT=44 max-min difference = 43
NOP_COUNT=45 max-min difference = 47
NOP_COUNT=46 max-min difference = 46
NOP_COUNT=47 max-min difference = 46
NOP_COUNT=48 max-min difference = 46
NOP_COUNT=49 max-min difference = 46
NOP_COUNT=50 max-min difference = 46
```

Here we can see that for `NOP_COUNT=14` there is ~60ns of discrepancy. We can check performance of the individual
functions

```
$ NOP_COUNT=14 cargo run --release --features=criterion -- --bench
factorials/factorial_1  time:   [351.05 ns 351.14 ns 351.23 ns]
factorials/factorial_2  time:   [295.58 ns 295.69 ns 295.92 ns]
factorials/factorial_3  time:   [348.73 ns 348.80 ns 348.88 ns]
factorials/factorial_4  time:   [295.61 ns 295.66 ns 295.71 ns]
factorials/factorial_5  time:   [350.39 ns 350.44 ns 350.51 ns]
factorials/factorial_6  time:   [295.16 ns 295.25 ns 295.38 ns]
factorials/factorial_7  time:   [348.79 ns 348.85 ns 348.91 ns]
factorials/factorial_8  time:   [295.70 ns 295.79 ns 295.92 ns]
factorials/factorial_9  time:   [351.01 ns 351.07 ns 351.14 ns]
factorials/factorial_10 time:   [295.17 ns 295.22 ns 295.26 ns]
```

In this setup functions 1, 4, 5, 8, 9 are predictabily slower than functions 2, 3, 6, 7, 10. The effect is persistent across multiple runs.

## Perpoducibility

This effect is present to some extent on all modern x86-64 CPUs although circumstances required to reporouce it differs slightly on different microarchitectures. The effect is most pronounced on Intel Sandy Bridge and newer CPUs with addition of Decoded Stream Buffer. To reproduce the result on some particular microarchitecture you may need to play with the number of `nop`s placed in a loop.

Tested on:

- `Intel Core i7-1068NG7` on `darwin 23.2.0`
- `Intel Core i7-1068NG7` on `linux 6.5.13`
- `Intel(R) Xeon(R) CPU E5-2651 v2 @ 1.80GHz` on AWS c1.medium
