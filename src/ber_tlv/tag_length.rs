#[derive(Debug)]
pub(super) struct TagLength {
    pub(super) _tag: Vec<u8>,
    pub(super) _len: u64,
    pub(super) _val_offset: usize,
}
