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
    fn from_key(key: u64) -> Self {
        let seed = [0, 0, (key >> 32) as u32, key as u32];
        XorShiftRng::from_seed(seed)
    }
}

impl RandFromKey for ChaChaRng {
    fn from_key(key: u64) -> Self {
        let seed = [0, 0, (key >> 32) as u32, key as u32];
        ChaChaRng::from_seed(&seed)
    }
}

#[cfg(feature="xoroshiro")]
impl RandFromKey for xoroshiro::XoroShiroRng {
    fn from_key(key: u64) -> Self {
        let seed = [0, key];
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
        j = ((((b + 1) as u64) << 32) / (1 + g.next_u32() as u64)) as u32;
    }

    b
}

#[cfg(test)]
mod test {}
