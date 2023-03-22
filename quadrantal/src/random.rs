mod prng;
use prng::Prng;

use std::collections::VecDeque;

use super::piece::Piece;

const NUM_BASIC_PIECES: usize = 7;

pub struct Bag {
    next: VecDeque<Piece>,
    rng: Prng,
    bag_size: usize,
}

impl Default for Bag {
    fn default() -> Self {
        Self::new(2)
    }
}

impl Bag {
    fn new(bag_size: usize) -> Bag {
        let seed = get_prng_seed();
        let rng = Prng::new(seed);

        let next = vec![].into();

        let mut bag = Bag {
            next,
            rng,
            bag_size,
        };

        bag.add_scrambled_set();

        bag
    }

    pub fn next(&mut self) -> Piece {
        if self.next.len() <= self.bag_size * NUM_BASIC_PIECES {
            self.add_scrambled_set();
        }

        self.next.pop_front().unwrap().shrink_wrap_square()
    }

    pub fn peek(&self, len: usize) -> Option<&[Piece]> {
        let slices = self.next.as_slices().0;

        if slices.len() <= len {
            return None;
        }

        Some(slices)
    }

    /*
    pub fn peek_next(&self) -> &Piece {
        &self.peek(1).unwrap()[0]
    }
    */

    fn add_scrambled_set(&mut self) {
        let mut tmp = vec![];

        for _ in 0..self.bag_size {
            for pcid in 1..=NUM_BASIC_PIECES {
                tmp.push(Piece::new_basic(pcid, vfc::Subpalette::new(pcid as u8)));
            }
        }

        self.randomize(&mut tmp);

        self.next.extend(tmp.into_iter());

        self.next.make_contiguous();
    }

    // implements a Fisher-Yates shuffle
    fn randomize(&mut self, vec: &mut Vec<Piece>) {
        // To shuffle an array a of n elements (indices 0..n-1):
        for i in (1..vec.len()).rev() {
            let j = self.rng.next().unwrap() as usize % (i + 1); // such that 0 ≤ j ≤ i

            vec.swap(i, j);
        }
    }
}

#[cfg(not(target_family = "wasm"))]
fn get_prng_seed() -> [u16; 2] {
    use std::time::SystemTime;

    let duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Duration since UNIX_EPOCH failed");

    let number0 = duration.as_secs();
    let number1 = duration.subsec_nanos() & 0xffff0000 >> 16;

    [number0 as u16, number1 as u16]
}

#[cfg(target_family = "wasm")]
fn get_prng_seed() -> [u16; 2] {
    let number0 = (crate::js::random() * 65536.0).floor();
    let number1 = (crate::js::random() * 65536.0).floor();

    [number0 as u16, number1 as u16]
}
