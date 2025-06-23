use std::{collections::BTreeMap, ops::AddAssign};

use num::{CheckedAdd, CheckedSub, One, Zero};

// This is a simple system pallet that manages block numbers and nonces for accounts.
// It provides functionality to increment the block number and manage nonces for accounts.
pub trait Config {
    type AccountId: Ord + Clone;
    type Nonce: Copy + Zero + Ord + Clone + One;
    type BlockNumber: Copy + Zero + One + CheckedAdd + CheckedSub + AddAssign;
}
#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_numer: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            block_numer: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn inc_block_number(&mut self) {
        self.block_numer += T::BlockNumber::one();
    }
    pub fn block_number(&self) -> T::BlockNumber {
        self.block_numer
    }
    pub fn get_nonce(&self, who: &T::AccountId) -> T::Nonce {
        *self.nonce.get(who).unwrap_or(&T::Nonce::zero())
    }

    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let current_nonce = self.get_nonce(who);
        self.nonce
            .insert(who.clone(), current_nonce + T::Nonce::one());
    }
}

#[cfg(test)]
mod tests {
    struct TestConfig;
    impl super::Config for TestConfig {
        type AccountId = String;
        type Nonce = u32;
        type BlockNumber = u32;
    }
    #[test]
    fn init_system() {
        let pallet: super::Pallet<TestConfig> = super::Pallet::new();
        assert_eq!(pallet.block_number(), 0);
        assert_eq!(pallet.get_nonce(&"Alice".to_string()), 0);
    }

    #[test]
    fn increment_block_number() {
        let mut pallet: super::Pallet<TestConfig> = super::Pallet::new();
        pallet.inc_block_number();
        assert_eq!(pallet.block_number(), 1);
    }

    #[test]
    fn increment_nonce() {
        let mut pallet: super::Pallet<TestConfig> = super::Pallet::new();
        let alice = "Alice".to_string();
        pallet.inc_nonce(&alice);
        assert_eq!(pallet.get_nonce(&alice), 1);
        pallet.inc_nonce(&alice);
        assert_eq!(pallet.get_nonce(&alice), 2);
    }
}
