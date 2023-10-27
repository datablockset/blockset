#[cfg(test)]
mod test {
    use io_trait::Io;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::virtual_io::VirtualIo;

    #[wasm_bindgen_test]
    #[test]
    fn test() {
        let io = VirtualIo::new(&[]);
        io.write("test.txt", "Hello, world!".as_bytes()).unwrap();
        let result = io.read_to_string("test.txt").unwrap();
        assert_eq!(result, "Hello, world!");
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_dir_fail() {
        let io = VirtualIo::new(&[]);
        assert!(io.write("a/test.txt", "Hello, world!".as_bytes()).is_err());
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_write_recursively() {
        let io = VirtualIo::new(&[]);
        assert!(io
            .write_recursively("a/test.txt", "Hello, world!".as_bytes())
            .is_ok());
        assert!(io
            .write_recursively("a/test2.txt", "Hello, world!".as_bytes())
            .is_ok());
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_dir_rec() {
        let io = VirtualIo::new(&[]);
        assert!(io
            .write_recursively("a/b/test.txt", "Hello, world!".as_bytes())
            .is_ok());
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_err() {
        let io = VirtualIo::new(&[]);
        assert!(io
            .write_recursively("?", "Hello, world!".as_bytes())
            .is_err());
    }
}
