use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use paste::paste;

macro_rules! factorial_bench {
    ($n:expr, $ctx:ident) => {
        paste! {
            $ctx.bench_function(concat!("factorial_", $n), |b| b.iter(|| [<factorial_ $n>](black_box(100))));
        }
    };
}

macro_rules! factorial {
    ($n:expr, $ctx:ident) => {
        paste! {
            fn [<factorial_ $n>](n: u64) -> u64 {
                factorial::<$n>(n)
            }
        }
    };
}

#[inline(always)]
fn factorial<const N: u64>(mut n: u64) -> u64 {
    let mut m = 1u64;
    while n > 1 {
        m = m.saturating_mul(n);
        n -= 1;
        unsafe {
            #[rustfmt::skip]
            std::arch::asm!{
                "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
                "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
                "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
                "nop", "nop", "nop", "nop", "nop", "nop",
            }
        }
    }
    m + N
}

macro_rules! define_multiple {
    ($macro:ident, $ctx:ident, $n:expr) => {
        $macro!($n, $ctx);
    };
    ($macro:ident, $ctx:ident, $n:expr, $($rest:expr),*) => {
        $macro!($n, $ctx);
        define_multiple!($macro, $ctx, $($rest),*);
    };
}

define_multiple!(factorial, skip, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("factorials");
    g.measurement_time(Duration::from_secs(1));
    g.warm_up_time(Duration::from_millis(1));

    define_multiple!(factorial_bench, g, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
