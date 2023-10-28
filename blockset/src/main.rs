use blockset_lib::run;
use io_impl::RealIo;

fn main() -> Result<(), String> {
    run(&RealIo::default())
}
