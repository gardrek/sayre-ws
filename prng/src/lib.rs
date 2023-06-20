/**
`Prng32` is an implementation of a pseudo-random number generator. Specifically, it is an
adaptation of the xoshiro128++ generator found
[here](https://prng.di.unimi.it/xoshiro128plusplus.c).
*/
#[derive(Clone, Debug)]
pub struct Prng32 {
    state: [u32; 4],
}

pub use Prng32 as Prng;

impl Prng32 {
    /// Creates a new generator from a seed.
    pub fn new(state: [u32; 4]) -> Self {
        Self { state }
    }

    /**
    Creates a new generator, treating the bytes of a 16-byte string as the seed.

    # Panics

    Panics if the string is not exactly 16 bytes long.
    */
    pub fn from_string(st: &str) -> Self {
        Self::from_byte_slice(st.as_bytes())
    }

    /**
    Creates a new generator, treating a 16-byte slice as the seed.

    # Panics

    Panics if the slice is not exactly 16 bytes long.
    */
    pub fn from_byte_slice(bytes: &[u8]) -> Self {
        use std::convert::TryInto;

        assert_eq!(bytes.len(), 16);

        let mut state = [0; 4];

        let mut chunks = bytes.chunks(4);

        for i in 0..4 {
            state[i] = u32::from_be_bytes(chunks.next().unwrap().try_into().unwrap());
        }

        Self::new(state)
    }

    /// Directly set the seed of the generator.
    pub fn seed(&mut self, seed: [u32; 4]) {
        self.state = seed;
    }

    /// Advance the internal state of the generator and return a new item.
    fn next(&mut self) -> u32 {
        let result = self.state[0]
            .wrapping_add(self.state[3])
            .rotate_left(7)
            .wrapping_add(self.state[0]);

        let t = self.state[1] << 9;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;

        self.state[3] = self.state[3].rotate_left(11);

        result
    }

    /**
    This is the jump function for the generator. It is equivalent
    to 2^64 calls to `next`; it can be used to generate 2^64
    non-overlapping subsequences for parallel computations.
    */
    pub fn jump(&mut self) {
        let jump_table: [u32; 4] = [0x8764000b, 0xf542d2d3, 0x6fa035c3, 0x77f2db5b];

        let mut s0 = 0;
        let mut s1 = 0;
        let mut s2 = 0;
        let mut s3 = 0;

        for i in 0..4 {
            for b in 0..32 {
                if (jump_table[i] & 1 << b) != 0 {
                    s0 ^= self.state[0];
                    s1 ^= self.state[1];
                    s2 ^= self.state[2];
                    s3 ^= self.state[3];
                }
                self.next();
            }
        }

        self.state[0] = s0;
        self.state[1] = s1;
        self.state[2] = s2;
        self.state[3] = s3;
    }

    /**
    This is the long-jump function for the generator. It is equivalent to
    2^96 calls to `next`; it can be used to generate 2^32 starting points,
    from each of which `jump` will generate 2^32 non-overlapping
    subsequences for parallel distributed computations.
    */
    pub fn long_jump(&mut self) {
        let jump_table: [u32; 4] = [0xb523952e, 0x0b6f099f, 0xccf5a0ef, 0x1c580662];

        let mut s0 = 0;
        let mut s1 = 0;
        let mut s2 = 0;
        let mut s3 = 0;

        for i in 0..4 {
            for b in 0..32 {
                if (jump_table[i] & 1 << b) != 0 {
                    s0 ^= self.state[0];
                    s1 ^= self.state[1];
                    s2 ^= self.state[2];
                    s3 ^= self.state[3];
                }
                self.next();
            }
        }

        self.state[0] = s0;
        self.state[1] = s1;
        self.state[2] = s2;
        self.state[3] = s3;
    }
}

/**
`Prng16` is an implementation of a pseudo-random number generator. Specifically, it is an
adaptation of the xoroshiro32++ generator, as seen
[here](https://github.com/ZiCog/xoroshiro).
*/
#[derive(Clone, Debug)]
pub struct Prng16 {
    state: [u16; 2],
}

impl Prng16 {
    /// Creates a new generator from a seed.
    pub fn new(state: [u16; 2]) -> Self {
        Self { state }
    }

    /// Directly set the seed of the generator.
    pub fn seed(&mut self, seed: [u16; 2]) {
        self.state = seed;
    }

    /// Advance the internal state of the generator and return a new item.
    fn next(&mut self) -> u16 {
        let a = 13;
        let b = 5;
        let c = 10;
        let d = 9;

        let result = self.state[0]
            .wrapping_add(self.state[1])
            .rotate_left(d)
            .wrapping_add(self.state[0]);

        self.state[1] ^= self.state[0];

        self.state[0] = self.state[0].rotate_left(a) ^ self.state[1] ^ (self.state[1] << b);

        self.state[1] = self.state[1].rotate_left(c);

        result
    }
}

impl Iterator for Prng16 {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next())
    }
}

impl Iterator for Prng32 {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_seed_32() {
        let mut p = Prng32::from_string("[___sixteen____]");
        assert_eq!(3133062588, p.next());
    }

    #[test]
    fn test_seed_32() {
        let mut p = Prng32::new([1, 1, 1, 1]);

        assert_eq!(257, p.next());

        p.seed([10, 10, 10, 10]);

        assert_eq!(2570, p.next());
    }

    #[test]
    fn first_values_16() {
        let mut prng = Prng16::new([1, 0]);

        let expected_values = [
            0x0201, 0x6269, 0xae16, 0x12a2, 0x4ae8, 0xd719, 0x0c52, 0x984b, 0x1df1, 0x743c, 0xdba0,
            0xbcc6, 0x34c9, 0x746c, 0x3643, 0x07ff,
        ];

        for (i, &value) in expected_values.iter().enumerate() {
            let n = prng.next();
            assert!(
                value == n,
                "failed at index {}: expected 0x{:04x}, found 0x{:04x}\n{value:016b}\n{n:016b}",
                i,
                value,
                n
            );
        }
    }
}
