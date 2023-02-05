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
            Self {
                admin: AccountId::from([0x00; 32]),
            }
        }

        #[ink(message)]
        pub fn admin(&self) -> AccountId {
            let admin = self.admin;
            ink::env::debug_println!("Logic {:?}", &admin);
            admin
        }

        #[ink(message)]
        pub fn update_admin(&mut self) -> AccountId {
            let new_admin = AccountId::from([0x01; 32]);
            self.admin = new_admin;
            let admin = self.admin;
            ink::env::debug_println!("Logic {:?}", &admin);
            admin
        }
    }
}
