use crate::{
    base32::{StrEx, ToBase32},
    digest224::Digest224,
    io::Io,
};

trait ToResult {
    fn to_result(self, e: &str) -> Result<(), &str>;
}

impl ToResult for Option<()> {
    fn to_result(self, e: &str) -> Result<(), &str> {
        self.ok_or(e)
    }
}

pub fn run(io: &mut impl Io) -> Result<(), &str> {
    let mut a = io.args();
    a.next().unwrap();
    let command = a.next().ok_or("missing command")?;
    match command.as_str() {
        "validate" => {
            let b32 = a.next().ok_or("missing base32")?;
            let d = b32.from_base32::<Digest224>().ok_or("invalid address")?;
            io.print("valid: ");
            io.println(&d.to_base32());
            Ok(())
        }
        _ => Err("unknown command")
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
}
