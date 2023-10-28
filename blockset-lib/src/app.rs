use std::io::{self, Read, Write};

use io_trait::{DirEntry, Io, Metadata};

use crate::{
    base32::{StrEx, ToBase32},
    file_table::{FileTable, CDT0, PARTS, ROOTS},
    level_storage::LevelStorage,
    state::{mb, progress, State},
    storage::{Null, Storage},
    table::{Table, Type},
    tree::Tree,
    u224::U224,
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

fn read_to_tree<T: Storage>(
    s: T,
    mut file: impl Read,
    len: u64,
    stdout: &mut impl Write,
    display_new: bool,
) -> Result<String, String> {
    let mut tree = Tree::new(s);
    let mut i = 0;
    let mut state = State::new(stdout);
    let mut total = 0;
    loop {
        let mut buf = [0; 1024];
        let p = if len == 0 { 100 } else { i * 100 / len };
        let s = if display_new {
            "New data: ".to_owned() + &mb(total) + ". "
        } else {
            String::new()
        } + "Processed: "
            + &progress(i, p as u8);
        state.set(&s).to_string_result()?;
        let size = file.read(buf.as_mut()).to_string_result()?;
        if size == 0 {
            break;
        }
        i += size as u64;
        for c in buf[0..size].iter() {
            total += tree.push(*c).to_string_result()?;
        }
    }
    Ok(tree.end().to_string_result()?.0.to_base32())
}

fn print(w: &mut impl Write, s: &str) -> Result<(), String> {
    w.write_all(s.as_bytes()).to_string_result()
}

fn println(w: &mut impl Write, s: &str) -> Result<(), String> {
    print(w, s)?;
    print(w, "\n")
}

fn add<'a, T: Io, S: 'a + Storage>(
    io: &'a T,
    a: &mut T::Args,
    storage: impl Fn(&'a T) -> S,
    display_new: bool,
) -> Result<(), String> {
    let stdout = &mut io.stdout();
    let path = a.next().ok_or("missing file name")?;
    let len = io.metadata(&path).to_string_result()?.len();
    let f = io.open(&path).to_string_result()?;
    let k = read_to_tree(storage(io), f, len, stdout, display_new)?;
    println(stdout, &k)?;
    Ok(())
}

fn calculate_total(io: &impl Io) -> io::Result<u64> {
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

pub fn run(io: &impl Io) -> Result<(), String> {
    let stdout = &mut io.stdout();
    let mut a = io.args();
    a.next().unwrap();
    let command = a.next().ok_or("missing command")?;
    match command.as_str() {
        "validate" => {
            let b32 = a.next().ok_or("missing address")?;
            let d = b32.from_base32::<U224>().ok_or("invalid address")?;
            print(stdout, "valid: ")?;
            println(stdout, &d.to_base32())?;
            Ok(())
        }
        "address" => add(io, &mut a, |_| Null(), false),
        "add" => add(io, &mut a, |io| LevelStorage::new(FileTable(io)), true),
        "get" => {
            let b32 = a.next().ok_or("missing address")?;
            let d = b32.from_base32::<U224>().ok_or("invalid address")?;
            let path = a.next().ok_or("missing file name")?;
            let mut f = io.create(&path).to_string_result()?;
            let table = FileTable(io);
            table
                .restore(Type::Main, &d, &mut f, stdout)
                .to_string_result()?;
            Ok(())
        }
        "info" => {
            let total = calculate_total(io).to_string_result()?;
            let s = "size: ".to_owned() + &total.to_string() + " B.";
            println(stdout, &s)?;
            Ok(())
        }
        _ => Err("unknown command".to_string()),
    }
}

#[cfg(test)]
mod test {
    use io_test::VirtualIo;
    use io_trait::Io;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        base32::ToBase32,
        run,
        sha224::{compress, compress_one},
        u256::{to_u224, U256},
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
        io.write("a.txt", "Hello, world!".as_bytes()).unwrap();
        let e = run(&mut io);
        assert_eq!(e, Ok(()));
        let d: U256 = [
            0x00000021_646c726f_77202c6f_6c6c6548,
            0x68000000_00000000_00000000_00000000,
        ];
        let s = to_u224(&compress([d, [0, 0]])).unwrap().to_base32();
        assert_eq!(io.stdout.to_stdout(), s + "\n");
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_add() {
        let mut io = VirtualIo::new(&["add", "a.txt"]);
        io.write("a.txt", "Hello, world!".as_bytes()).unwrap();
        let e = run(&mut io);
        assert_eq!(e, Ok(()));
        let d: U256 = [
            0x00000021_646c726f_77202c6f_6c6c6548,
            0x68000000_00000000_00000000_00000000,
        ];
        let s = compress_one(&d).to_base32();
        assert_eq!(io.stdout.to_stdout(), s.clone() + "\n");
        let v = io
            .read(&("cdt0/roots/".to_owned() + &s[..2] + "/" + &s[2..4] + "/" + &s[4..]))
            .unwrap();
        assert_eq!(v, " Hello, world!".as_bytes());
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_get() {
        let d: U256 = [
            0x00000021_646c726f_77202c6f_6c6c6548,
            0x68000000_00000000_00000000_00000000,
        ];
        let s = compress_one(&d).to_base32();
        let mut io = VirtualIo::new(&["get", s.as_str(), "b.txt"]);
        // io.create_dir("cdt0").unwrap();
        io.write_recursively(
            &("cdt0/roots/".to_owned() + &s[..2] + "/" + &s[2..4] + "/" + &s[4..]),
            " Hello, world!".as_bytes(),
        )
        .unwrap();
        run(&mut io).unwrap();
        let v = io.read("b.txt").unwrap();
        assert_eq!(v, "Hello, world!".as_bytes());
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_info() {
        let d: U256 = [
            0x00000021_646c726f_77202c6f_6c6c6548,
            0x68000000_00000000_00000000_00000000,
        ];
        let s = compress_one(&d).to_base32();
        let mut io = VirtualIo::new(&["info"]);
        // io.create_dir("cdt0").unwrap();
        io.write_recursively(
            &("cdt0/roots/".to_owned() + &s[..2] + "/" + &s[2..4] + "/" + &s[4..]),
            " Hello, world!".as_bytes(),
        )
        .unwrap();
        io.write_recursively("cdt0/roots/ab", " Hello, world!".as_bytes())
            .unwrap();
        run(&mut io).unwrap();
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_add_empty() {
        let mut io = VirtualIo::new(&["add", "a.txt"]);
        io.write("a.txt", "".as_bytes()).unwrap();
        let e = run(&mut io);
        assert_eq!(e, Ok(()));
        let d: U256 = [0, 0];
        let s = compress_one(&d).to_base32();
        assert_eq!(io.stdout.to_stdout(), s.clone() + "\n");
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_get_empty() {
        let d: U256 = [0, 0];
        let s = compress_one(&d).to_base32();
        let mut io = VirtualIo::new(&["get", &s, "a.txt"]);
        let e = run(&mut io);
        assert_eq!(e, Ok(()));
    }

    fn add_get(src: String) {
        let mut io = VirtualIo::new(&["add", "a.txt"]);
        io.write("a.txt", src.as_bytes()).unwrap();
        let e = run(&mut io);
        assert_eq!(e, Ok(()));
        let x = &io.stdout.to_stdout()[..45];
        io.args = ["blockset", "get", x, "b.txt"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let e = run(&mut io);
        assert_eq!(e, Ok(()));
        let v = io.read("b.txt").unwrap();
        assert_eq!(v, src.as_bytes());
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_big() {
        add_get("Hello, world!".repeat(95000));
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_repeat() {
        for i in 0..1000 {
            add_get("X".repeat(i));
        }
    }
}
