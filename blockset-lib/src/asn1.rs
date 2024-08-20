use nanvm_lib::common::cast::Cast;

// it should be replaced by JsInt in the future.
struct Integer(Vec<u8>);

struct OctetString(Vec<u8>);

struct ObjectIdentifier {
    a0: u8,
    a1: u8,
    a2: Vec<Integer>,
}

struct BitString {
    padding: u8,
    value: Vec<u8>,
}

struct Sequence(Vec<Any>);

enum Any {
    Bool(bool),
    Integer(Integer),
    OctetString(OctetString),
    ObjectIdentifier(ObjectIdentifier),
    BitString(BitString),
    Sequence(Vec<Any>),
}

trait Serailize {
    fn tag() -> u8;
    fn serialize(self) -> Vec<u8>;
}

impl Serailize for bool {
    fn tag() -> u8 {
        1
    }

    fn serialize(self) -> Vec<u8> {
        [if self { 0xFF } else { 0 }].cast()
    }
}
