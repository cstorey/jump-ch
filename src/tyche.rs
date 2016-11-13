use rand::Rng;
use RandFromKey;

pub struct TycheRng {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl RandFromKey for TycheRng {
    fn from_key(key: u64) -> TycheRng {
        TycheRng::new_unmixed(key, 0)
    }
}
impl TycheRng {
    pub fn new(seed: u64, idx: u32) -> Self {
        let mut t = Self::new_unmixed(seed, idx);
        for _ in 0..20 {
            t.mix()
        }
        t
    }
    fn new_unmixed(seed: u64, idx: u32) -> Self {
        TycheRng {
            a: (seed << 32) as u32,
            b: (seed & (!0u32) as u64) as u32,
            c: 2654435769,
            d: 1367130551 ^ idx,
        }
    }
    fn mix(&mut self) {
        let &mut TycheRng { mut a, mut b, mut c, mut d } = self;
        a = a.wrapping_add(b);
        d = (d ^ a).rotate_left(16);
        c = c.wrapping_add(d);
        b = (b ^ c).rotate_left(12);

        a = a.wrapping_add(b);
        d = (d ^ a).rotate_left(8);
        c = c.wrapping_add(d);
        b = (b ^ c).rotate_left(7);

        *self = TycheRng {
            a: a,
            b: b,
            c: c,
            d: d,
        };
    }
}
impl Rng for TycheRng {
    fn next_u32(&mut self) -> u32 {
        // let &mut LcgRng(ref mut state) = self;
        // *state = state.wrapping_mul(2862933555777941757) + 1;
        // *state as u32
        self.mix();
        self.b
    }
}

pub struct Tyche1Rng {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl RandFromKey for Tyche1Rng {
    fn from_key(key: u64) -> Tyche1Rng {
        Tyche1Rng::new_unmixed(key, 0)
    }
}
impl Tyche1Rng {
    pub fn new(seed: u64, idx: u32) -> Self {
        let mut t = Self::new_unmixed(seed, idx);
        for _ in 0..20 {
            t.mix()
        }
        t
    }
    fn new_unmixed(seed: u64, idx: u32) -> Self {
        Tyche1Rng {
            a: (seed << 32) as u32,
            b: (seed & (!0u32) as u64) as u32,
            c: 2654435769,
            d: 1367130551 ^ idx,
        }
    }
    fn mix(&mut self) {
        let &mut Tyche1Rng { mut a, mut b, mut c, mut d } = self;
        b = (b ^ c).rotate_right(7);
        c = c.wrapping_sub(d);
        d = (d ^ a).rotate_left(8);
        a = a.wrapping_sub(b);
        b = (b ^ c).rotate_right(12);
        c = c.wrapping_sub(d);
        d = (d ^ a).rotate_left(16);
        a = a.wrapping_sub(b);

        *self = Tyche1Rng {
            a: a,
            b: b,
            c: c,
            d: d,
        };
    }
}
impl Rng for Tyche1Rng {
    fn next_u32(&mut self) -> u32 {
        // let &mut LcgRng(ref mut state) = self;
        // *state = state.wrapping_mul(2862933555777941757) + 1;
        // *state as u32
        self.mix();
        self.b
    }
}
