use rand_core::{impls, Error, RngCore};

/// This implementation is a xoroshiro128+ that is serde serializable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gen {
    state: [u64; 2],
}

/// This implementation heavily based on MIT-Licensed:
/// https://github.com/mscharley/rust-xoroshiro128/blob/master/src/xoroshiro.rs
impl Gen {
    /// Generator from new whole state:
    pub fn new(seed: [u64; 2]) -> Gen {
        Gen { state: seed }
    }
    /// Generator from current generator?
    pub fn new_child(other: &mut Gen) -> Gen {
        Gen {
            state: [other.next_u64(), other.next_u64()],
        }
    }
    /// Generate next 64 pseudo-random bits.
    pub fn _next_u64(&mut self) -> u64 {
        let s0: u64 = self.state[0];
        let mut s1: u64 = self.state[1];
        let result: u64 = s0.wrapping_add(s1);

        s1 ^= s0;
        self.state[0] = s0.rotate_left(55) ^ s1 ^ (s1 << 14); // a, b
        self.state[1] = s1.rotate_left(36); // c

        result
    }
    /// Generate next 32 pseudo-random bits.
    pub fn _next_u32(&mut self) -> u32 {
        self._next_u64() as u32
    }
    /// Create a generator from a single, 32-bit seed.
    pub fn new_from_seed(seed: u32) -> Gen {
        Gen::new([0x193a6754a8a7d469 ^ (seed as u64), 0x97830e05113ba7bb])
    }
    pub fn reset_seed(&mut self, seed: u32) {
        self.state = [0x193a6754a8a7d469 ^ (seed as u64), 0x97830e05113ba7bb]
    }
}

impl RngCore for Gen {
    fn next_u32(&mut self) -> u32 {
        self._next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        self._next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}
