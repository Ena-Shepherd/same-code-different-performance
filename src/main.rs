#![feature(fn_align)]

use std::{hint::black_box, time::Instant};

use paste::paste;
use same_code_different_performance::make_asm_nops;

// Creates __asm_nops() functions with sequence of NOP instructions. The number of instructions
// is given in NOP_COUNT env variable at compile time
make_asm_nops!();

/// This factorial function must always be inlined to produce different aligned version of the same function
#[inline(always)]
fn factorial<const N: u64>(mut n: u64) -> u64 {
    // This is a dummy code needed to prevent from collapsing all the factorial functions into one by linker
    unsafe { std::ptr::read_volatile(&N) };

    let mut m = 1u64;
    while n > 1 {
        m = m.saturating_mul(n);
        n -= 1;
        unsafe {
            // Those nops are dummy payload to produce the loop of a specific length
            // The number of nops is the same for all the versions of factorial functions.
            // But because different functions have different alignment in memory the loops are
            // also aligned differently. This has significant impact on the performance.
            __asm_nops();
        }
    }
    m
}

macro_rules! factorial_benchmark {
    ($n:expr, $ctx:ident) => {
        paste! {
            $ctx.bench_function(concat!("factorial_", $n), |b| b.iter(|| [<factorial_ $n>](black_box(100))));
        }
    };
}

macro_rules! factorial {
    ($n:expr, $ctx:ident) => {
        paste! {
            #[inline(never)]
            #[repr(align(16))]
            fn [<factorial_ $n>](n: u64) -> u64 {
                factorial::<$n>(n)
            }
        }
    };
}

// Helper macro to produce the same code multiple times with different values
macro_rules! define_multiple {
    ($macro:ident, $ctx:ident, $n:expr) => {
        $macro!($n, $ctx);
    };
    ($macro:ident, $ctx:ident, $n:expr, $($rest:expr),*) => {
        $macro!($n, $ctx);
        define_multiple!($macro, $ctx, $($rest),*);
    };
}

// Defining multiple identical factorial functions with different names
define_multiple!(factorial, skip, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);

#[cfg(feature = "criterion")]
mod criterion_support {
    use super::*;
    use criterion::{black_box, Criterion};
    use std::time::Duration;

    pub fn bench(c: &mut Criterion) {
        let mut g = c.benchmark_group("factorials");
        g.measurement_time(Duration::from_secs(1));
        g.warm_up_time(Duration::from_millis(100));

        // Sanechecking that all the factorial functions are producing the same results
        assert_eq!(factorial_1(10), factorial_10(10));

        define_multiple!(factorial_benchmark, g, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    }
}

#[cfg(feature = "criterion")]
criterion::criterion_group!(benches, criterion_support::bench);

#[cfg(feature = "criterion")]
criterion::criterion_main!(benches);

#[cfg(not(feature = "criterion"))]
fn main() {
    let mut min = u64::max_value();
    let mut max = u64::min_value();

    let value = measure(factorial_1);
    println!("factorial_1 = {}", value);
    min = min.min(value);
    max = max.max(value);

    let value = measure(factorial_2);
    println!("factorial_2 = {}", value);
    min = min.min(value);
    max = max.max(value);

    let value = measure(factorial_3);
    println!("factorial_3 = {}", value);
    min = min.min(value);
    max = max.max(value);

    let value = measure(factorial_4);
    println!("factorial_4 = {}", value);
    min = min.min(value);
    max = max.max(value);

    let value = measure(factorial_5);
    println!("factorial_5 = {}", value);
    min = min.min(value);
    max = max.max(value);

    let value = measure(factorial_6);
    println!("factorial_6 = {}", value);
    min = min.min(value);
    max = max.max(value);

    let value = measure(factorial_7);
    println!("factorial_7 = {}", value);
    min = min.min(value);
    max = max.max(value);

    let value = measure(factorial_8);
    println!("factorial_8 = {}", value);
    min = min.min(value);
    max = max.max(value);

    let value = measure(factorial_9);
    println!("factorial_9 = {}", value);
    min = min.min(value);
    max = max.max(value);

    let value = measure(factorial_10);
    println!("factorial_10 = {}", value);
    min = min.min(value);
    max = max.max(value);

    println!("{}, {} = {}", min, max, max - min)
}

fn measure(f: fn(u64) -> u64) -> u64 {
    const ITER: usize = 1000;
    let mut min = u64::max_value();
    for _ in 0..ITER {
        let time = Instant::now();
        for _ in 0..10 {
            black_box(f(black_box(100)));
        }
        min = min.min(time.elapsed().as_nanos() as u64 / 10);
    }

    min
}
