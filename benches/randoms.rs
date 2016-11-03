#![feature(test)]
extern crate test;
extern crate rand;
extern crate jump_ch;
extern crate xoroshiro;

use jump_ch::RandFromKey;
use test::Bencher;

fn bench_for<R:RandFromKey>(b: &mut Bencher, buckets: u32) {
    let k = rand::random::<u64>();
    b.iter(|| jump_ch::jump_ch::<R>(k, buckets));
}

mod chacha {
    use test::Bencher;
    use rand::chacha::ChaChaRng;
    use super::bench_for;
    #[bench] fn ch_2pow00(b: &mut Bencher) { bench_for::<ChaChaRng>(b, 1 << 0)}
    #[bench] fn ch_2pow02(b: &mut Bencher) { bench_for::<ChaChaRng>(b, 1 << 2)}
    #[bench] fn ch_2pow04(b: &mut Bencher) { bench_for::<ChaChaRng>(b, 1 << 4)}
    #[bench] fn ch_2pow06(b: &mut Bencher) { bench_for::<ChaChaRng>(b, 1 << 6)}
    #[bench] fn ch_2pow08(b: &mut Bencher) { bench_for::<ChaChaRng>(b, 1 << 8)}
    #[bench] fn ch_2pow10(b: &mut Bencher) { bench_for::<ChaChaRng>(b, 1 << 10)}
}

mod xorshift {
    use test::Bencher;
    use rand::XorShiftRng;
    use super::bench_for;
    #[bench] fn ch_2pow00(b: &mut Bencher) { bench_for::<XorShiftRng>(b, 1 << 0)}
    #[bench] fn ch_2pow02(b: &mut Bencher) { bench_for::<XorShiftRng>(b, 1 << 2)}
    #[bench] fn ch_2pow04(b: &mut Bencher) { bench_for::<XorShiftRng>(b, 1 << 4)}
    #[bench] fn ch_2pow06(b: &mut Bencher) { bench_for::<XorShiftRng>(b, 1 << 6)}
    #[bench] fn ch_2pow08(b: &mut Bencher) { bench_for::<XorShiftRng>(b, 1 << 8)}
    #[bench] fn ch_2pow10(b: &mut Bencher) { bench_for::<XorShiftRng>(b, 1 << 10)}
}

#[cfg(feature="xoroshiro")]
mod xoroshiro_ {
    use test::Bencher;
    use super::bench_for;
    use xoroshiro::XoroShiroRng;
    #[bench] fn ch_2pow00(b: &mut Bencher) { bench_for::<XoroShiroRng>(b, 1 << 0)}
    #[bench] fn ch_2pow02(b: &mut Bencher) { bench_for::<XoroShiroRng>(b, 1 << 2)}
    #[bench] fn ch_2pow04(b: &mut Bencher) { bench_for::<XoroShiroRng>(b, 1 << 4)}
    #[bench] fn ch_2pow06(b: &mut Bencher) { bench_for::<XoroShiroRng>(b, 1 << 6)}
    #[bench] fn ch_2pow08(b: &mut Bencher) { bench_for::<XoroShiroRng>(b, 1 << 8)}
    #[bench] fn ch_2pow10(b: &mut Bencher) { bench_for::<XoroShiroRng>(b, 1 << 10)}
}
