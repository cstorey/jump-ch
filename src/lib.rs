extern crate rand;
use rand::{Rng, SeedableRng, StdRng};
use rand::XorShiftRng;
use rand::chacha::ChaChaRng;

pub trait RandFromKey {
    type Rng: Rng;
    fn from_key(key: u64) -> Self::Rng;
}

impl RandFromKey for XorShiftRng {
    type Rng = XorShiftRng;
    fn from_key(key: u64) -> Self {
        let seed = [0, 0, (key >> 32) as u32, key as u32];
        XorShiftRng::from_seed(seed)
    }
}

impl RandFromKey for ChaChaRng {
    type Rng = ChaChaRng;
    fn from_key(key: u64) -> Self {
        let seed = [0, 0, (key >> 32) as u32, key as u32];
        ChaChaRng::from_seed(&seed)
    }
}


pub fn jump_ch<R = XorShiftRng>(key: u64, nbuckets: u64) -> u64
    where R: RandFromKey
{
    let mut g = R::from_key(key);
    let mut b = 0;
    let mut j = 0;
    while j < nbuckets {
        b = j;
        j = ((b + 1) as f64 / g.next_f64()).floor() as u64;
    }

    b
}
