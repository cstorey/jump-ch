extern crate rand;
#[cfg(feature="xoroshiro")]
extern crate xoroshiro;

use rand::{Rng, SeedableRng};
use rand::XorShiftRng;
use rand::chacha::ChaChaRng;

pub trait RandFromKey: Rng {
    fn from_key(key: u64) -> Self;
}

impl RandFromKey for XorShiftRng {
    fn from_key(mut key: u64) -> Self {
        key = key.saturating_add(1);
        let hi = (key >> 32) as u32;
        let lo = key as u32;

        let seed = [hi.wrapping_mul(0x193a6754),
                    lo.wrapping_mul(0xa8a7d469),
                    hi.wrapping_mul(0x97830e05),
                    lo.wrapping_mul(0x113ba7bb)];
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
        key = key.saturating_add(1);
        let seed = [key.wrapping_mul(0x193a6754a8a7d469), key.wrapping_mul(0x97830e05113ba7bb)];
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
        // let r = g.next_f32() as f64;
        // j = ((b + 1) as f64 / r).floor() as u32;
        let r = g.next_u32() as u64;
        j = ((((b + 1) as u64) << 32) / (1 + r)) as u32;
        // println!("b:{}; j:{}; r:{}", b, j, r);
    }

    b
}

    pub struct LcgRng(u64);

    impl RandFromKey for LcgRng {
        fn from_key(key: u64) -> LcgRng {
            LcgRng(key)
        }
    }
    impl Rng for LcgRng {
        fn next_u32(&mut self) -> u32 {
            let &mut LcgRng(ref mut state) = self;
            *state = state.wrapping_mul(2862933555777941757) +1;
            *state as u32
        }
    }


#[cfg(test)]
pub mod test {
    use std::collections::BTreeMap;
    use std::iter;
    use super::{jump_ch, RandFromKey,LcgRng};
    use rand::{Rng, SeedableRng, XorShiftRng};
    #[cfg(feature="xoroshiro")]
    use xoroshiro::XoroShiroRng;

    fn test_distribution<R:RandFromKey>() {
        let nbuckets: u32 = 16;
        let nkeys = 1 << 16;
        let mut histogram = iter::repeat(0).take(nbuckets as usize).collect::<Vec<usize>>();

        for k in 0..nkeys {
            let bucket = jump_ch::<R>(k, nbuckets);
            // println!("{} ->{}", k, bucket);
            histogram[bucket as usize] += 1;
        }

        println!("xorshift Dist for {} keys, {} buckets: {:#?}",
                 nkeys,
                 nbuckets,
                 histogram);
    }

    #[test]
    fn test_distribution_xorshift() {
        test_distribution::<XorShiftRng>();
    }

    #[test]
    fn test_distribution_lcg() {
        test_distribution::<LcgRng>();
    }


    #[cfg(feature="xoroshiro")]
    #[test]
    fn test_distribution_xoroshiro() {
        test_distribution::<XoroShiroRng>();
    }
}
