use nanvm_lib::{big_numbers::big_int::BigInt, common::cast::Cast};

struct OctetString(Vec<u8>);

struct ObjectIdentifier {
    a0: u8,
    a1: u8,
    a2: Vec<BigInt>,
}

struct BitString {
    padding: u8,
    value: Vec<u8>,
}

struct Sequence(Vec<Any>);

enum Any {
    Bool(bool),
    Integer(BigInt),
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
