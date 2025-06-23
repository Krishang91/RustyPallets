use core::fmt::Debug;
use std::collections::BTreeMap;

use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
    type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }

    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        match self.get_claim(&claim) {
            Some(_) => Err("Claim already exists"),
            None => {
                self.claims.insert(claim, caller);
                Ok(())
            }
        }
    }

    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        match self.get_claim(&claim) {
            Some(existing_caller) if *existing_caller == caller => {
                self.claims.remove(&claim);
                Ok(())
            }
            Some(_) => Err("Caller does not own the claim"),
            None => Err("Claim does not exist"),
        }
    }
}

pub enum Call<T: Config> {
    CreateClaim { claim: T::Content },
    RevokeClaim { claim: T::Content },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
        match call {
            Call::CreateClaim { claim } => self.create_claim(caller, claim),
            Call::RevokeClaim { claim } => self.revoke_claim(caller, claim),
        }
    }
}

#[cfg(test)]
mod tests {
    struct TestConfig;
    impl super::Config for TestConfig {
        type Content = String;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = String;
        type Nonce = u32;
        type BlockNumber = u32;
    }

    #[test]
    fn basic_proof_of_existence() {
        let mut pallet: super::Pallet<TestConfig> = super::Pallet::new();
        let alice = "Alice".to_string();
        let claim = "Unique Document Hash".to_string();

        // Create a claim
        assert!(pallet.create_claim(alice.clone(), claim.clone()).is_ok());
        // Check if the claim exists
        assert!(pallet.get_claim(&claim).is_some());
        // Try to create the same claim again
        assert!(pallet.create_claim(alice.clone(), claim.clone()).is_err());
        // Revoke the claim
        assert!(pallet.revoke_claim(alice.clone(), claim.clone()).is_ok());
        // Check if the claim has been removed
        assert!(pallet.get_claim(&claim).is_none());
    }
}
