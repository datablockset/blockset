use blockset::{run, RealIo};

fn main() -> Result<(), String> {
    run(&mut RealIo::default())
}
