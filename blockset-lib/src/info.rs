use std::io;

use io_trait::{DirEntry, Io, Metadata};

use crate::{
    file_table::{CDT0, PARTS, ROOTS},
    state::{mb, State},
};

pub fn calculate_total(io: &impl Io) -> io::Result<u64> {
    let stdout = &mut io.stdout();
    let f = |d| {
        io.read_dir_type(&(CDT0.to_owned() + "/" + d), true)
            .unwrap_or_default()
    };
    let mut a = f(ROOTS);
    a.extend(f(PARTS));
    let an = a.len() as u64;
    let mut total = 0;
    let state = &mut State::new(stdout);
    for (ai, ia) in a.iter().enumerate() {
        let b = io.read_dir_type(&ia.path(), true)?;
        let bn = b.len() as u64;
        for (bi, ib) in b.iter().enumerate() {
            let c = io.read_dir_type(&ib.path(), false)?;
            for ic in c.iter() {
                let d = ic.metadata()?.len();
                total += d;
            }
            let p = (bn * ai as u64 + bi as u64 + 1) as f64 / (an * bn) as f64;
            let e = total as f64 / p;
            let s = "size: ~".to_string()
                + &mb(e as u64)
                + ". "
                + &((p * 100.0) as u64).to_string()
                + "%.";
            state.set(&s)?;
        }
    }
    Ok(total)
}
