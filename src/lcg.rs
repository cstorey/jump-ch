use rand::Rng;
use RandFromKey;
pub struct LcgRng(u64);

impl RandFromKey for LcgRng {
    fn from_key(key: u64) -> LcgRng {
        LcgRng(key)
    }
}
impl Rng for LcgRng {
    fn next_u32(&mut self) -> u32 {
        let &mut LcgRng(ref mut state) = self;
        *state = state.wrapping_mul(2862933555777941757) + 1;
        *state as u32
    }
}
