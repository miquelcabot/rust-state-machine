use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, One, Zero};

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<AccountID, BlockNumber, Nonce> {
    /// The current block number.
    block_number: BlockNumber,
    /// A map from an account to their nonce.
    nonce: BTreeMap<AccountID, Nonce>,
}

impl<AccountID, BlockNumber, Nonce> Pallet<AccountID, BlockNumber, Nonce>
where
    AccountID: Ord + Clone,
    BlockNumber: Zero + One + CheckedSub + CheckedAdd + Copy,
    Nonce: Zero + One + Ord + Clone + Copy,
{
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        Self { block_number: BlockNumber::zero(), nonce: BTreeMap::new() }
    }

    /// Get the current block number.
    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        // crashes if the block number overflows
        self.block_number =
            self.block_number.checked_add(&BlockNumber::one()).unwrap();
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: &AccountID) {
        let nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero());
        self.nonce.insert(who.clone(), nonce + Nonce::one());
    }

    pub fn get_nonce(&self, who: &AccountID) -> Nonce {
        *self.nonce.get(who).unwrap_or(&Nonce::zero())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn init_system() {
        let system = super::Pallet::<String, u32, u32>::new();
        assert_eq!(system.block_number(), 0);
    }

    #[test]
    fn inc_block_number() {
        let mut system = super::Pallet::<String, u32, u32>::new();
        system.inc_block_number();
        assert_eq!(system.block_number(), 1);
    }

    #[test]
    fn inc_nonce() {
        let alice = String::from("alice");
        let mut system = super::Pallet::<String, u32, u32>::new();
        system.inc_nonce(&alice);
        assert_eq!(system.get_nonce(&alice), 1);
        system.inc_nonce(&alice);
        assert_eq!(system.get_nonce(&alice), 2);
    }
}
