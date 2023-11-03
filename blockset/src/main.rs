use std::io;

use blockset_lib::run;
use io_impl::RealIo;

fn main() -> io::Result<()> {
    run(&RealIo::default())
}
