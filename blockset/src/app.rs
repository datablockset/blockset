use std::io::Read;

use crate::{
    base32::{StrEx, ToBase32},
    io::Io,
    sha224::compress,
    storage::Null,
    tree::Tree,
    u224::U224,
    u256::to_u224,
};

trait ResultEx {
    type T;
    fn to_string_result(self) -> Result<Self::T, String>;
}

impl<T, E: ToString> ResultEx for Result<T, E> {
    type T = T;
    fn to_string_result(self) -> Result<Self::T, String> {
        self.map_err(|e| e.to_string())
    }
}

pub fn run(io: &mut impl Io) -> Result<(), String> {
    let mut a = io.args();
    a.next().unwrap();
    let command = a.next().ok_or("missing command")?;
    match command.as_str() {
        "validate" => {
            let b32 = a.next().ok_or("missing address")?;
            let d = b32.from_base32::<U224>().ok_or("invalid address")?;
            io.print("valid: ");
            io.println(&d.to_base32());
            Ok(())
        }
        "address" => {
            let path = a.next().ok_or("missing file name")?;
            let mut t = Tree::new(Null());
            {
                let mut f = io.open(&path).to_string_result()?;
                loop {
                    let mut buf = [0; 1024];
                    let size = f.read(buf.as_mut()).to_string_result()?;
                    if size == 0 {
                        break;
                    }
                    for c in buf[0..size].iter() {
                        t.push(*c);
                    }
                }
            }
            let d = t.end();
            let e = to_u224(&compress([d, [0, 0]])).unwrap();
            io.println(&e.to_base32());
            Ok(())
        }
        _ => Err("unknown command".to_string()),
    }
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        base32::ToBase32,
        run,
        sha224::compress,
        u256::{to_u224, U256},
        virtual_io::VirtualIo,
    };

    #[wasm_bindgen_test]
    #[test]
    fn test() {
        let mut io = VirtualIo::new(&[]);
        let e = run(&mut io);
        assert_eq!(e, Err("missing command".to_string()));
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_unknown_command() {
        let mut io = VirtualIo::new(&["x"]);
        let e = run(&mut io);
        assert_eq!(e, Err("unknown command".to_string()));
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_missing_address() {
        let mut io = VirtualIo::new(&["validate"]);
        let e = run(&mut io);
        assert_eq!(e, Err("missing address".to_string()));
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_invalid_address() {
        let mut io = VirtualIo::new(&["validate", "0"]);
        let e = run(&mut io);
        assert_eq!(e, Err("invalid address".to_string()));
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_valid_address() {
        let mut io = VirtualIo::new(&["validate", "3Vld4j94scaseqgcyzrOha5dxa9rx6ppnfbndck97iack"]);
        let e = run(&mut io);
        assert_eq!(e, Ok(()));
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_address() {
        let mut io = VirtualIo::new(&["address", "a.txt"]);
        io.file_map
            .insert("a.txt".to_string(), "Hello, world!".as_bytes().to_vec());
        let e = run(&mut io);
        assert_eq!(e, Ok(()));
        let d: U256 = [
            0x00000021_646c726f_77202c6f_6c6c6548,
            0x68000000_00000000_00000000_00000000,
        ];
        let s = to_u224(&compress([d, [0, 0]])).unwrap().to_base32();
        assert_eq!(io.stdout, s + "\n");
    }
}
