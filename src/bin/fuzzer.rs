extern crate bincode_fuzz;

use std::env;

use bincode_fuzz::corpus::Corpus;
use bincode_fuzz::fuzzer::Fuzzer;

/*
#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Running,
    Stopped,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FuzzResult {
    Success,
    Timeout,
    Crashed,
}

struct TypeInstance {}

const HEARTBEAT: u64 = 100;
const TIMEOUT: u64 = 1000;
const LOST_CHILD: u64 = TIMEOUT / HEARTBEAT;

fn spawn() -> Result<FuzzResult> {
    let mut child = Command::new("cargo")
        .arg("build")
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()
        .expect("failed to build");
    if !child.success() {
        return Err(Error::new(ErrorKind::Other, "failed to build instance"));
    }

    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("instance")
        .stderr(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start instance");
    let mut stdout = child.stdout.take().unwrap();

    let child: Arc<Mutex<Child>> = Arc::new(Mutex::new(child));
    let state: Arc<Mutex<Option<State>>> = Arc::new(Mutex::new(None));

    let heartbeat = {
        let state = state.clone();
        thread::spawn(move || {
            let mut byte = [0];
            while let Ok(_) = stdout.read_exact(&mut byte) {
                if let Ok(mut state) = state.lock() {
                    *state = Some(State::Running);
                }
            }
            let mut state = state.lock().unwrap();
            *state = Some(State::Stopped);
        })
    };

    let watchdog = {
        let state = state.clone();
        let child = child.clone();
        thread::spawn(move || {
            let mut skipped = 0;
            while skipped <= LOST_CHILD {
                thread::sleep(time::Duration::from_millis(HEARTBEAT));
                if let Some(state) = state.lock().unwrap().take() {
                    if state == State::Stopped {
                        break;
                    }
                    skipped = 0;
                } else {
                    skipped = skipped + 1;
                }
            }
            let mut child = child.lock().unwrap();
            if skipped >= LOST_CHILD {
                child.kill().unwrap();
                child.wait().unwrap();
                FuzzResult::Timeout
            } else {
                if child.wait().unwrap().success() {
                    FuzzResult::Success
                } else {
                    FuzzResult::Crashed
                }
            }
        })
    };

    heartbeat.join().unwrap();
    watchdog.join().map_err(|_| Error::new(ErrorKind::Other, ""))
}
*/

fn main() {
    // let corpus = Corpus::new()
    //     .seed(thread_rng().gen())
    //     .structs(10)
    //     .serializations(500)
    //     .mutations(50);

    let path = env::args().skip(1).next().unwrap();
    let corpus = Corpus::open_or_new(path).unwrap();
    let fuzzer = Fuzzer::new(corpus);
    fuzzer.run().unwrap();
}
