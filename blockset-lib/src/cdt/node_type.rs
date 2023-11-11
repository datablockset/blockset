#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum NodeType {
    Root = 0,
    Child = 1,
}
