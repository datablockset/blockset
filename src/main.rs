use blockset::{run, RealIo};

fn main() -> Result<(), String> {
    run(&RealIo::default())
}
