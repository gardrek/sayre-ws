/**
`Prng` is an implementation of a pseudo-random number generator. Specifically, it is an
adaptation of the xoroshiro32++ generator, as seen
[here](https://github.com/ZiCog/xoroshiro).
*/
#[derive(Clone, Debug)]
pub struct Prng {
    state: [u16; 2],
}

impl Prng {
    /// Creates a new generator from a seed.
    pub fn new(state: [u16; 2]) -> Self {
        Self { state }
    }

    /*
    /// Directly set the seed of the generator.
    pub fn seed(&mut self, seed: [u16; 2]) {
        self.state = seed;
    }
    */

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

impl Iterator for Prng {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_values_16() {
        let mut prng = Prng::new([1, 0]);

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
