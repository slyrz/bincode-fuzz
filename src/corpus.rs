use std::io::{Result, Error, ErrorKind};
use std::path::Path;
use std::fs::File;
use serde_json;

use types::Struct;
use random::{Rng, seeded_rng, thread_rng};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Corpus {
    pub seed: u64,
    pub serializations: usize,
    pub mutations: usize,
    pub structs: Vec<Struct>,
}

impl Corpus {
    pub fn new() -> Corpus {
        Corpus {
            seed: thread_rng().gen(),
            serializations: 2048,
            mutations: 65536,
            structs: Vec::new(),
        }
    }

    pub fn seed(&mut self, seed: u64) -> Corpus {
        self.seed = seed;
        self.clone()
    }

    pub fn serializations(&mut self, serializations: usize) -> Corpus {
        self.serializations = serializations;
        self.clone()
    }

    pub fn mutations(&mut self, mutations: usize) -> Corpus {
        self.mutations = mutations;
        self.clone()
    }

    pub fn structs(&mut self, size: usize) -> Corpus {
        let mut rng = seeded_rng(self.seed);
        self.structs = (0..size).map(|_| rng.gen()).collect();
        self.clone()
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Corpus> {
        let file = File::open(path)?;
        serde_json::from_reader(file).map_err(|_| { Error::new(ErrorKind::Other, "failed to open corpus")})
    }

    pub fn open_or_new<P: AsRef<Path>>(path: P) -> Result<Corpus> {
        if path.as_ref().exists() {
            Corpus::open(path)
        } else {
            let corpus = Corpus::new().structs(32);
            corpus.save(path)?;
            Ok(corpus)
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut file = File::create(path)?;
        serde_json::to_writer(&mut file, self).map_err(|_| {Error::new(ErrorKind::Other, "failed to save corpus")})
    }
}
