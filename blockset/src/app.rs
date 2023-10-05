use std::process::ExitCode;

use crate::{io::Io, base32::{StrEx, ToBase32}, digest224::Digest224};

pub fn run(io: &mut impl Io) -> ExitCode {
    let mut a = io.args();
    a.next().unwrap();
    let command = a.next().unwrap();
    match command.as_str() {
        "validate" => {
            let b32 = a.next().unwrap();
            let d = b32.from_base32::<Digest224>();
            match d {
                Some(d) => {
                    io.print("valid: ");
                    io.println(&d.to_base32());
                    ExitCode::SUCCESS
                }
                None => {
                    io.println("invalid");
                    ExitCode::FAILURE
                }
            }
        },
        _ => {
            io.println("unknown command");
            ExitCode::FAILURE
        }
    }
}
