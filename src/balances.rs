use std::collections::BTreeMap;

use num::traits::{CheckedAdd, CheckedSub, Zero};

pub trait Config: crate::system::Config {
    type Balance: Copy + Zero + CheckedAdd + CheckedSub;
}
#[derive(Debug)]
pub struct Pallet<T: Config> {
    pub balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }
    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn get_balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.get_balance(&caller);

        let to_balance = self.get_balance(&to);
        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Insufficient balance")?;
        let new_to_balance = to_balance
            .checked_add(&amount)
            .ok_or("Overflow when adding to balance")?;
        // Update balances
        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}

pub enum Call<T: Config> {
    Transfer {
        to: T::AccountId,
        amount: T::Balance,
    },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(
        &mut self,
        caller: Self::Caller,
        call: Self::Call,
    ) -> crate::support::DispatchResult {
        match call {
            Call::Transfer { to, amount } => self.transfer(caller, to, amount)?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::system;

    struct TestConfig;

    impl system::Config for TestConfig {
        type AccountId = String;
        type Nonce = u32;
        type BlockNumber = u32;
    }
    impl super::Config for TestConfig {
        type Balance = u128;
    }

    #[test]
    fn init_balances() {
        let mut pallet: super::Pallet<TestConfig> = super::Pallet::new();
        pallet.set_balance(&"Alice".to_string(), 100);
        pallet.set_balance(&"Bob".to_string(), 200);

        assert_eq!(pallet.get_balance(&"Alice".to_string()), 100);
        assert_eq!(pallet.get_balance(&"Bob".to_string()), 200);
        assert_eq!(pallet.get_balance(&"Charlie".to_string()), 0);
    }

    #[test]
    fn transfer_success() {
        let mut pallet: super::Pallet<TestConfig> = super::Pallet::new();
        pallet.set_balance(&"Alice".to_string(), 100);
        pallet.set_balance(&"Bob".to_string(), 50);

        assert!(pallet
            .transfer("Alice".to_string(), "Bob".to_string(), 30)
            .is_ok());
        assert_eq!(pallet.get_balance(&"Alice".to_string()), 70);
        assert_eq!(pallet.get_balance(&"Bob".to_string()), 80);
    }

    #[test]
    fn transfer_insufficient_balance() {
        let mut pallet: super::Pallet<TestConfig> = super::Pallet::new();
        pallet.set_balance(&"Alice".to_string(), 100);

        let amount: u128 = 150;

        // Also verify the transfer fails
        let result = pallet.transfer("Alice".to_string(), "Bob".to_string(), amount);
        assert!(result.is_err());
    }
}
