use crate::{
    base32::{StrEx, ToBase32},
    digest224::Digest224,
    io::Io,
};

pub fn run(io: &mut impl Io) -> Result<(), &str> {
    let mut a = io.args();
    a.next().unwrap();
    let command = a.next().ok_or("missing command")?;
    match command.as_str() {
        "validate" => {
            let b32 = a.next().ok_or("missing address")?;
            let d = b32.from_base32::<Digest224>().ok_or("invalid address")?;
            io.print("valid: ");
            io.println(&d.to_base32());
            Ok(())
        }
        _ => Err("unknown command"),
    }
}

#[cfg(test)]
mod test {
    use crate::{run, virtual_io::VirtualIo};

    #[test]
    fn test() {
        let mut io = VirtualIo::new(&[]);
        let e = run(&mut io);
        assert_eq!(e, Err("missing command"));
    }

    #[test]
    fn test_unknown_command() {
        let mut io = VirtualIo::new(&["x"]);
        let e = run(&mut io);
        assert_eq!(e, Err("unknown command"));
    }

    #[test]
    fn test_missing_address() {
        let mut io = VirtualIo::new(&["validate"]);
        let e = run(&mut io);
        assert_eq!(e, Err("missing address"));
    }

    #[test]
    fn test_invalid_address() {
        let mut io = VirtualIo::new(&["validate", "0"]);
        let e = run(&mut io);
        assert_eq!(e, Err("invalid address"));
    }

    #[test]
    fn test_valid_address() {
        let mut io = VirtualIo::new(&["validate", "3v1d4j94scaseqgcyzr0ha5dxa9rx6ppnfbndck971ack"]);
        let e = run(&mut io);
        assert_eq!(e, Ok(()));
    }
}
