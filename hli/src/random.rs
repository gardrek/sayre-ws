pub use prng::Prng16 as Prng;

//~ use std::collections::VecDeque;

pub fn new_prng() -> Prng {
    Prng::new(get_prng_seed())
}

// implements a Fisher-Yates shuffle
// TODO: return the number of times `next` was called?
pub fn shuffle<T>(rng: &mut Prng, vec: &mut Vec<T>) {
    // To shuffle an array a of n elements (indices 0..n-1):
    for i in (1..vec.len()).rev() {
        let j = rng.next().unwrap() as usize % (i + 1); // such that 0 ≤ j ≤ i

        vec.swap(i, j);
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
