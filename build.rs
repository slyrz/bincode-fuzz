use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let directory = env::var("OUT_DIR").unwrap();
    let path = Path::new(&directory).join("type.rs");
    let mut file = File::create(path).unwrap();
    file.write(b"type Test = ();\n").unwrap();
}
