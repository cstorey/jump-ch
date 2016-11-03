extern crate rand;
#[cfg(feature="xoroshiro")]
extern crate xoroshiro;

use rand::{Rng, SeedableRng, StdRng};
use rand::XorShiftRng;
use rand::chacha::ChaChaRng;

pub trait RandFromKey: Rng {
    fn from_key(key: u64) -> Self;
}

impl RandFromKey for XorShiftRng {
    fn from_key(mut key: u64) -> Self {
        let hi = (key >> 32) as u32;
        let lo = key as u32;

        let seed = [0x193a6754 ^ hi , 0xa8a7d469 ^ lo , 0x97830e05 ^ hi, 0x113ba7bb ^ lo];
        XorShiftRng::from_seed(seed)
    }
}

impl RandFromKey for ChaChaRng {
    fn from_key(key: u64) -> Self {
        let hi = (key >> 32) as u32;
        let lo = key as u32;
        let seed = [hi, lo, hi, lo];
        ChaChaRng::from_seed(&seed)
    }
}

#[cfg(feature="xoroshiro")]
impl RandFromKey for xoroshiro::XoroShiroRng {
    fn from_key(mut key: u64) -> Self {
        let seed = [0x193a6754a8a7d469 ^ key, 0x97830e05113ba7bb ^ key];
        xoroshiro::XoroShiroRng::from_seed(seed)
    }
}

pub fn jump_ch<R>(key: u64, nbuckets: u32) -> u32
    where R: RandFromKey
{
    let mut g = R::from_key(key);
    let mut b = 0;
    let mut j = 0;
    while j < nbuckets {
        b = j;
        let r = g.next_f32() as f64;
        j = ((b + 1) as f64 / r).floor() as u32;
        // j = ((((b + 1) as u64) << 32) / (1 + g.next_u32() as u64)) as u32;
        // println!("b:{}; j:{}; r:{}", b, j, r);
    }

    b
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;
    use std::iter;
    use super::{jump_ch, RandFromKey};
    use rand::{Rng, SeedableRng, XorShiftRng};
    #[cfg(feature="xoroshiro")]
    use xoroshiro::XoroShiroRng;

    #[test]
    fn test_distribution_xorshift() {
        let nbuckets: u32 = 16;
        let nkeys = 1 << 16;
        let mut histogram = iter::repeat(0).take(nbuckets as usize).collect::<Vec<usize>>();

        for k in 0..nkeys {
            let bucket = jump_ch::<XorShiftRng>(k, nbuckets);
            println!("{} ->{}", k, bucket);
            histogram[bucket as usize] += 1;
        }

        println!("xorshift Dist for {} keys, {} buckets: {:#?}",
                 nkeys,
                 nbuckets,
                 histogram);
    }

    #[cfg(feature="xoroshiro")]
    #[test]
    fn test_distribution_xoroshiro() {
        let nbuckets: u32 = 16;
        let nkeys = 1 << 16;
        let mut histogram = iter::repeat(0).take(nbuckets as usize).collect::<Vec<usize>>();

        for k in 0..nkeys {
            let bucket = jump_ch::<XoroShiroRng>(k, nbuckets);
            println!("{} ->{}", k, bucket);
            histogram[bucket as usize] += 1;
        }

        println!("xoroshiro Dist for {} keys, {} buckets: {:#?}",
                 nkeys,
                 nbuckets,
                 histogram);
    }



    #[test]
    fn test_weat() {
        for k in 0..24 {
            let mut r = XorShiftRng::from_key(k);
            println!("r:{:?}", r.gen_iter::<f64>().take(20).collect::<Vec<_>>());
        }
    }
}
