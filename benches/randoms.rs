#![feature(test)]
extern crate test;
extern crate rand;
extern crate jump_ch;

use test::Bencher;
use rand::XorShiftRng;
use rand::chacha::ChaChaRng;
use jump_ch::RandFromKey;

fn bench_for<R:RandFromKey>(b: &mut Bencher, buckets: u64) {
    let k = rand::random::<u64>();
    b.iter(|| jump_ch::jump_ch::<R>(k, buckets));
}
#[bench] fn ch_chacha_2pow00(b: &mut Bencher) { bench_for::<ChaChaRng>(b, 1 << 0)}
#[bench] fn ch_chacha_2pow02(b: &mut Bencher) { bench_for::<ChaChaRng>(b, 1 << 2)}
#[bench] fn ch_chacha_2pow04(b: &mut Bencher) { bench_for::<ChaChaRng>(b, 1 << 4)}
#[bench] fn ch_chacha_2pow06(b: &mut Bencher) { bench_for::<ChaChaRng>(b, 1 << 6)}
#[bench] fn ch_chacha_2pow08(b: &mut Bencher) { bench_for::<ChaChaRng>(b, 1 << 8)}
#[bench] fn ch_chacha_2pow10(b: &mut Bencher) { bench_for::<ChaChaRng>(b, 1 << 10)}

#[bench] fn ch_xorshift_2pow00(b: &mut Bencher) { bench_for::<XorShiftRng>(b, 1 << 0)}
#[bench] fn ch_xorshift_2pow02(b: &mut Bencher) { bench_for::<XorShiftRng>(b, 1 << 2)}
#[bench] fn ch_xorshift_2pow04(b: &mut Bencher) { bench_for::<XorShiftRng>(b, 1 << 4)}
#[bench] fn ch_xorshift_2pow06(b: &mut Bencher) { bench_for::<XorShiftRng>(b, 1 << 6)}
#[bench] fn ch_xorshift_2pow08(b: &mut Bencher) { bench_for::<XorShiftRng>(b, 1 << 8)}
#[bench] fn ch_xorshift_2pow10(b: &mut Bencher) { bench_for::<XorShiftRng>(b, 1 << 10)}
