#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod proxy {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Proxy {
        /// Stores a single `bool` value on the storage.
        admin: AccountId,
        implementation: AccountId,
    }

    impl Proxy {
        #[ink(constructor)]
        pub fn new(admin: AccountId, implementation: AccountId) -> Self {
            Self { admin, implementation }
        }

        #[ink(message)]
        pub fn admin(&self) -> AccountId {
            self.admin
        }

        #[ink(message)]
        pub fn implementation(&self) -> AccountId {
            self.implementation
        }

        #[ink(message, selector = _)]
        pub fn fallback(&mut self) {
            todo!()
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        // /// Imports all the definitions from the outer scope so we can use them here.
        // use super::*;

        // #[ink::test]
        // fn _it_works() {
        //     todo!()
        // }
    }
}
