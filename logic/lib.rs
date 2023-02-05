#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod logic {
    use ink::storage::Lazy;

    #[ink(storage)]
    pub struct Proxy {
        admin: Lazy<AccountId>,
    }

    impl Proxy {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut lazy = Lazy::default();
            lazy.set(&AccountId::from([0x00; 32]));
            Self { admin: lazy }
        }

        #[ink(message)]
        pub fn admin(&self) -> AccountId {
            let admin = self.admin.get().unwrap();
            ink::env::debug_println!("Logic {:?}", &admin);
            admin
        }

        #[ink(message)]
        pub fn update_admin(&mut self) -> AccountId {
            let new_admin = AccountId::from([0x01; 32]);
            self.admin.set(&new_admin);
            let admin = self.admin.get().unwrap();
            ink::env::debug_println!("Logic {:?}", &admin);
            admin
        }
    }
}
