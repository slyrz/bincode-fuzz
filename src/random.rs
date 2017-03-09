use std::ops::Range;
use std::cmp;

pub use rand::*;

pub fn seeded_rng(seed: u64) -> XorShiftRng {
    XorShiftRng::from_seed([
        (seed ^ 0x4d3a1613c0077157) as u32,
        (seed ^ 0xaa99ab477fd5d862) as u32,
        (seed ^ 0xa16d13ad8b5d4420) as u32,
        (seed ^ 0x82e93fddc8167589) as u32
    ])
}

pub fn rand_range<R: Rng>(rng: &mut R, max: usize) -> Range<usize> {
    let length = rng.gen::<usize>() % max;
    (0..length)
}

pub fn rand_vector<R: Rng, T: Rand>(rng: &mut R) -> Vec<T> {
    rand_range(rng, 512).map(|_| rng.gen::<T>()).collect()
}

pub fn rand_string<R: Rng>(rng: &mut R) -> String {
    rand_range(rng, 512).map(|_| rng.gen::<char>()).collect()
}

pub fn rand_vector_string<R: Rng, T: Rand>(rng: &mut R) -> Vec<String> {
    rand_range(rng, 32).map(|_| rand_string(rng)).collect()
}

pub fn rand_option<R: Rng, T>(rng: &mut R, v: T) -> Option<T> {
    if rng.gen() { Some(v) } else { None }
}

pub fn mutate_bytes<R: Rng>(rng: &mut R, bytes: &mut Vec<u8>) {
    if bytes.is_empty() {
        return;
    }
    if rng.gen_weighted_bool(5) {
        let index = rng.gen_range(0, bytes.len());
        bytes[index] = bytes[index].wrapping_add(1);
    }
    if rng.gen_weighted_bool(5) {
        let index = rng.gen_range(0, bytes.len());
        bytes[index] = bytes[index].wrapping_sub(1);
    }
    if rng.gen_weighted_bool(5) {
        let index = rng.gen_range(0, bytes.len());
        bytes[index] = !bytes[index];
    }
    if rng.gen_weighted_bool(10) {
        let index = rng.gen_range(0, bytes.len());
        let byte = rng.gen();
        bytes.insert(index, byte);
    }
    if rng.gen_weighted_bool(10) {
        let index = rng.gen_range(0, bytes.len());
        bytes.remove(index);
    }
    if rng.gen_weighted_bool(20) {
        let count = rng.gen_range(0, 8);
        for _ in 0..count {
            bytes.push(rng.gen());
        }
    }
    if !bytes.is_empty() && rng.gen_weighted_bool(40) {
        let count = bytes.len() - rng.gen_range(0, cmp::min(32, bytes.len()));
        bytes.truncate(count);
    }
}
