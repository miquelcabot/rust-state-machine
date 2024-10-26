use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u32>,
}

impl Pallet {
    pub fn new() -> Self {
        Pallet { balances: BTreeMap::new() }
    }
}
