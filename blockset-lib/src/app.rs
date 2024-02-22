use std::io::{self, ErrorKind, Read, Write};

use io_trait::Io;

use crate::{
    cdt::{main_tree::MainTreeAdd, node_type::NodeType, tree_add::TreeAdd},
    common::{
        base32::{StrEx, ToBase32},
        eol::ToPosixEol,
        progress::{self, Progress},
        status_line::{mb, StatusLine},
    },
    forest::{file::FileForest, node_id::ForestNodeId, tree_add::ForestTreeAdd, Forest},
    info::calculate_total,
    uint::u224::U224,
};

fn set_progress(
    state: &mut StatusLine<'_, impl Io>,
    display_new: bool,
    new: u64,
    progress::State { current, total }: progress::State,
) -> io::Result<()> {
    let p = if total == 0 {
        1.0
    } else {
        (current as f64) / (total as f64)
    };
    let s = if display_new {
        "New data: ".to_owned() + &mb(new) + ". "
    } else {
        String::new()
    } + "Processed: "
        + &mb(current)
        + ", ";
    state.set_progress(&s, p)
}

fn file_read(
    file: &mut (impl Read + Progress),
    tree: &mut MainTreeAdd<impl TreeAdd>,
    new: &mut u64,
) -> io::Result<bool> {
    let mut buf = [0; 1024];
    let size = file.read(buf.as_mut())?;
    if size == 0 {
        return Ok(true);
    }
    for c in buf[0..size].iter() {
        *new += tree.push(*c)?;
    }
    Ok(false)
}

fn read_to_tree<T: TreeAdd>(
    s: T,
    mut file: impl Read + Progress,
    io: &impl Io,
    display_new: bool,
) -> io::Result<String> {
    let mut tree = MainTreeAdd::new(s);
    let mut state = StatusLine::new(io);
    let mut new = 0;
    loop {
        let pr = file.progress();
        set_progress(&mut state, display_new, new, pr?)?;
        if file_read(&mut file, &mut tree, &mut new)? {
            break;
        }
    }
    Ok(tree.end()?.0.to_base32())
}

fn print(w: &mut impl Write, s: &str) -> io::Result<()> {
    w.write_all(s.as_bytes())
}

fn println(w: &mut impl Write, s: &str) -> io::Result<()> {
    print(w, s)?;
    print(w, "\n")
}

fn invalid_input(s: &str) -> io::Error {
    io::Error::new(ErrorKind::InvalidInput, s)
}

fn is_to_posix_eol(a: &mut impl Iterator<Item = String>) -> io::Result<bool> {
    Ok(if let Some(option) = a.next() {
        if option != "--to-posix-eol" {
            return Err(invalid_input("unknown option"));
        }
        true
    } else {
        false
    })
}

fn read_to_tree_file(
    to_posix_eol: bool,
    s: impl TreeAdd,
    f: impl Read + Progress,
    io: &impl Io,
    display_new: bool,
) -> io::Result<String> {
    if to_posix_eol {
        // this may lead to incorrect progress bar because, a size of a file with replaced CRLF
        // is smaller than `len`. Proposed solution:
        // a Read implementation which can also report a progress.
        read_to_tree(s, ToPosixEol::new(f), io, display_new)
    } else {
        read_to_tree(s, f, io, display_new)
    }
}

fn add<'a, T: Io, S: 'a + TreeAdd>(
    io: &'a T,
    a: &mut T::Args,
    storage: impl Fn(&'a T) -> S,
    display_new: bool,
) -> io::Result<()> {
    let stdout = &mut io.stdout();
    let path = a.next().ok_or(invalid_input("missing file name"))?;
    let to_posix_eol = is_to_posix_eol(a)?;
    // let len = io.metadata(&path)?.len();
    let f = io.open(&path)?;
    let k = read_to_tree_file(to_posix_eol, storage(io), f, io, display_new)?;
    println(stdout, &k)
}

fn get_hash(a: &mut impl Iterator<Item = String>) -> io::Result<U224> {
    let b32 = a.next().ok_or(invalid_input("missing hash"))?;
    b32.from_base32::<U224>()
        .ok_or(invalid_input("invalid hash"))
}

fn validate(a: &mut impl Iterator<Item = String>, stdout: &mut impl Write) -> io::Result<()> {
    let d = get_hash(a)?.to_base32();
    println(stdout, &("valid: ".to_owned() + &d))
}

pub fn run(io: &impl Io) -> io::Result<()> {
    let stdout = &mut io.stdout();
    let mut a = io.args();
    a.next().unwrap();
    let command = a.next().ok_or(invalid_input("missing command"))?;
    match command.as_str() {
        "validate" => validate(&mut a, stdout),
        "hash" => add(io, &mut a, |_| (), false),
        "add" => add(io, &mut a, |io| ForestTreeAdd::new(FileForest(io)), true),
        "get" => {
            let d = get_hash(&mut a)?;
            let path = a.next().ok_or(invalid_input("missing file name"))?;
            let w = &mut io.create(&path)?;
            FileForest(io).restore(&ForestNodeId::new(NodeType::Root, &d), w, io)
        }
        "info" => println(
            stdout,
            &("size: ".to_owned() + &calculate_total(io)?.to_string() + " B."),
        ),
        _ => Err(invalid_input("unknown command")),
    }
}

#[cfg(test)]
mod test {
    use io_test::VirtualIo;
    use io_trait::Io;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{cdt::node_id::root, common::base32::ToBase32, run, uint::u256::U256};

    #[wasm_bindgen_test]
    #[test]
    fn test() {
        let mut io = VirtualIo::new(&[]);
        let e = run(&mut io);
        assert_eq!(e.unwrap_err().to_string(), "missing command");
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_unknown_command() {
        let mut io = VirtualIo::new(&["x"]);
        let e = run(&mut io);
        assert_eq!(e.unwrap_err().to_string(), "unknown command");
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_missing_address() {
        let mut io = VirtualIo::new(&["validate"]);
        let e = run(&mut io);
        assert_eq!(e.unwrap_err().to_string(), "missing hash");
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_invalid_address() {
        let mut io = VirtualIo::new(&["validate", "0"]);
        let e = run(&mut io);
        assert_eq!(e.unwrap_err().to_string(), "invalid hash");
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_valid_address() {
        let mut io = VirtualIo::new(&["validate", "3Vld4j94scaseqgcyzrOha5dxa9rx6ppnfbndck97iack"]);
        let e = run(&mut io);
        assert!(e.is_ok());
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_address() {
        let mut io = VirtualIo::new(&["hash", "a.txt"]);
        io.write("a.txt", "Hello, world!".as_bytes()).unwrap();
        let e = run(&mut io);
        assert!(e.is_ok());
        let d: U256 = [
            0x00000021_646c726f_77202c6f_6c6c6548,
            0x68000000_00000000_00000000_00000000,
        ];
        let s = root(&d).to_base32();
        assert_eq!(io.stdout.to_stdout(), s + "\n");
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_add() {
        let mut io = VirtualIo::new(&["add", "a.txt"]);
        io.write("a.txt", "Hello, world!".as_bytes()).unwrap();
        let e = run(&mut io);
        assert!(e.is_ok());
        let d: U256 = [
            0x00000021_646c726f_77202c6f_6c6c6548,
            0x68000000_00000000_00000000_00000000,
        ];
        let s = root(&d).to_base32();
        assert_eq!(io.stdout.to_stdout()[..s.len()], s);
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
        let s = root(&d).to_base32();
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
        let s = root(&d).to_base32();
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
        assert!(e.is_ok());
        let d: U256 = [0, 0];
        let s = root(&d).to_base32();
        assert_eq!(io.stdout.to_stdout()[..s.len()], s);
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_get_empty() {
        let d: U256 = [0, 0];
        let s = root(&d).to_base32();
        let mut io = VirtualIo::new(&["get", &s, "a.txt"]);
        let e = run(&mut io);
        assert!(e.is_ok());
    }

    fn add_get_expected(src: &str, to_posix_eol: bool, expected: &str) {
        let mut io = VirtualIo::new(if to_posix_eol {
            &["add", "a.txt", "--to-posix-eol"]
        } else {
            &["add", "a.txt"]
        });
        io.write("a.txt", src.as_bytes()).unwrap();
        let e = run(&mut io);
        assert!(e.is_ok());
        let x = &io.stdout.to_stdout()[..45];
        io.args = ["blockset", "get", x, "b.txt"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let e = run(&mut io);
        assert!(e.is_ok());
        let v = io.read("b.txt").unwrap();
        assert_eq!(v, expected.as_bytes());
    }

    fn add_get(src: String, to_posix_eol: bool) {
        add_get_expected(&src, to_posix_eol, &src);
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_big() {
        add_get("Hello, world!".repeat(95000), false);
        add_get("Hello, world!".repeat(95000), true);
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_repeat() {
        for i in 0..1000 {
            add_get("X".repeat(i), false);
            add_get("X".repeat(i), true);
        }
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_eol() {
        add_get_expected(
            "Hello\rworld!\r\nGoodbye!\n\r\n",
            true,
            "Hello\rworld!\nGoodbye!\n\n",
        );
        add_get("Hello\rworld!\r\nGoodbye!\n\r\n".to_string(), false);
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_info_big() {
        let mut io = VirtualIo::new(&["add", "a.txt"]);
        let mut src = Vec::default();
        for i in 0..=0xFF {
            for j in 0..=0xFF {
                for k in 0..=3 {
                    src.push(i);
                    src.push(j);
                    src.push(k);
                }
            }
        }
        io.write("a.txt", &src).unwrap();
        let e = run(&mut io);
        assert!(e.is_ok());
        // "hk3c2pnsjyesmj441czj8s60d7vbsg5msyrz1h8kraghx"
        io.args = ["blockset".to_string(), "info".to_string()].to_vec();
        let e = run(&mut io);
        assert!(e.is_ok());
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_unknown_option() {
        let mut io = VirtualIo::new(&["add", "a.txt", "--x"]);
        let e = run(&mut io);
        assert_eq!(e.unwrap_err().to_string(), "unknown option");
    }
}
