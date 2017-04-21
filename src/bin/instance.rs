#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate bincode_fuzz;

use std::env;

use bincode_fuzz::random::*;
use bincode_fuzz::util::heartbeat;
use bincode::{serialize, deserialize, Infinite};

include!(concat!(env!("OUT_DIR"), "/type.rs"));

fn perform_serializations<R: Rng>(rng: &mut R, n: usize) {
    for _ in 0..n {
        heartbeat();
        let original: Test = rng.gen();
        let encoded: Vec<u8> = serialize(&original, Infinite).unwrap();
        let decoded: Test = deserialize(&encoded[..]).unwrap();
        assert_eq!(decoded, original);
    }
}

fn perform_mutations<R: Rng>(rng: &mut R, n: usize) {
    let mut buffer = serialize(&rng.gen::<Test>(), Infinite).unwrap();
    for _ in 0..n {
        heartbeat();
        mutate_bytes(rng, &mut buffer);
        let _ = deserialize::<Test>(&buffer[..]).is_ok();
        if buffer.is_empty() || (rng.next_u32() & 0xfff) == 0 {
            buffer = serialize(&rng.gen::<Test>(), Infinite).unwrap();
        }
    }
}

fn main() {
    let args: Vec<u64> = env::args()
        .skip(1)
        .take(3)
        .map(|arg| arg.parse().unwrap())
        .collect();

    let seed = args[0];
    let serializations = args[1] as usize;
    let mutations = args[2] as usize;

    let mut rng = seeded_rng(seed);
    perform_serializations(&mut rng, serializations);
    perform_mutations(&mut rng, mutations);
}
