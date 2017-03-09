use std::time::Duration;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::sync::{Arc, Mutex};
use std::io::{Read, Result, Error, ErrorKind};

use corpus::Corpus;
use util::discrete;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instance {
    Running,
    Stopped,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Success,
    Timeout,
    Crashed,
}

pub struct Fuzzer {
    heartbeat: Duration,
    timeout: Duration,
    corpus: Corpus,
}

impl Fuzzer {
    pub fn new(corpus: Corpus) -> Fuzzer {
        Fuzzer {
            heartbeat: Duration::from_millis(100),
            timeout: Duration::from_millis(500),
            corpus: corpus,
        }
    }

    pub fn run(&self) -> Result<()> {
        let n = self.corpus.structs.len();
        for (i, corpus_struct) in
            self.corpus
                .structs
                .iter()
                .enumerate() {
            corpus_struct.save(concat!(env!("OUT_DIR"), "/type.rs"))?;

            self.build()?;
            let status = self.spawn()?;
            println!("Test {}/{}: {:?}", i + 1, n, status);
        }
        Ok(())
    }

    fn build(&self) -> Result<()> {
        let status = Command::new("cargo")
            .arg("build")
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .status()
            .expect("failed to build");

        if status.success() {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "failed to build instance"))
        }
    }

    fn spawn(&self) -> Result<Status> {
        // This is the path and not "cargo run --bin instance" because the latter got troubles
        // with killing processes somehow.
        // TODO: fresh seed for every child process?
        let mut child = Command::new("./target/debug/instance")
            .arg(self.corpus.seed.to_string())
            .arg(self.corpus.serializations.to_string())
            .arg(self.corpus.mutations.to_string())
            .stderr(Stdio::null())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to start instance");

        let mut stdout = child.stdout.take().unwrap();

        let child: Arc<Mutex<Child>> = Arc::new(Mutex::new(child));
        let state: Arc<Mutex<Option<Instance>>> = Arc::new(Mutex::new(None));

        let heartbeat = {
            let state = state.clone();
            thread::spawn(move || {
                let mut byte = [0];
                while let Ok(_) = stdout.read_exact(&mut byte) {
                    if let Ok(mut state) = state.lock() {
                        *state = Some(Instance::Running);
                    }
                }
                let mut state = state.lock().unwrap();
                *state = Some(Instance::Stopped);
            })
        };

        let watchdog = {
            let lost = discrete(self.timeout) / discrete(self.heartbeat);
            let heartbeat = self.heartbeat.clone();
            let state = state.clone();
            let child = child.clone();
            thread::spawn(move || {
                let mut skipped = 0;
                while skipped < lost {
                    thread::sleep(heartbeat);
                    if let Some(state) = state.lock().unwrap().take() {
                        if state == Instance::Stopped {
                            break;
                        }
                        skipped = 0;
                    } else {
                        skipped = skipped + 1;
                    }
                }

                let mut child = child.lock().unwrap();
                if skipped >= lost {
                    child.kill().unwrap();
                    child.wait().unwrap();
                    Status::Timeout
                } else {
                    if child.wait().unwrap().success() {
                        Status::Success
                    } else {
                        Status::Crashed
                    }
                }
            })
        };

        heartbeat.join().unwrap();
        watchdog.join().map_err(|_| Error::new(ErrorKind::Other, ""))
    }
}
