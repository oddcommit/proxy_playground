#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod logic {

    #[ink(storage)]
    pub struct Logic {
        admin: AccountId,
    }

    impl Logic {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { admin: AccountId::from([0x00; 32]) }
        }

        #[ink(message)]
        pub fn admin(&self) -> AccountId {
            self.admin
        }
    }

    #[cfg(test)]
    mod tests {
        // /// Imports all the definitions from the outer scope so we can use them here.
        // use super::*;

        // /// We test a simple use case of our contract.
        // #[ink::test]
        // fn it_works() {
        //     todo!()
        // }
    }
}
