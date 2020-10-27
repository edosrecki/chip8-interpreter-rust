use std::collections::BTreeSet;

pub struct Input<'a> {
    pub pressed_keycodes: &'a BTreeSet<u8>,
}
