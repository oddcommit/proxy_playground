#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod logic {
    use ink::storage::traits::StorageKey;
    use ink::storage::Lazy;

    #[ink(storage)]
    pub struct Logic {
        /// This ends up getting overwritten by the `Proxy` contract when control is returned at the
        /// end of the call.
        ///
        /// This is because the `Proxy` also stores `Packed` values starting at the  `0x00000000`
        /// storage cell.
        admin: AccountId,
        value: Lazy<bool>,
    }

    impl Logic {
        /// Creates a new instance of the `Logic` contract.
        ///
        /// # Note
        ///
        /// In theory we shouldn't need this since the contract should be initialized from the
        /// `Proxy`. However, we don't have the ability to do this yet so we still a) require that
        /// all contracts have a constructor, and b) we don't have a way to initialize storage
        /// otherwise.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                admin: AccountId::from([0x00; 32]),
                value: Default::default(),
            }
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            let key = self.value.key();
            ink::env::debug_println!("Logic::get: Key {:?}", &key);

            let value = self.value.get().unwrap();
            ink::env::debug_println!("Logic::get: {:?}", &value);
            value
        }

        #[ink(message)]
        pub fn set(&mut self, value: bool) {
            let key = self.value.key();
            ink::env::debug_println!("Logic::set: Key {:?}", &key);

            self.value.set(&value);
            ink::env::debug_println!("Logic::set: {:?}", &self.value.get());
        }

        #[ink(message)]
        pub fn get_admin(&self) -> AccountId {
            let key = self.admin.key();
            ink::env::debug_println!("Logic::get_admin: Key {:?}", &key);

            let value = self.admin;
            ink::env::debug_println!("Logic::get_admin: {:?}", &value);
            value
        }

        #[ink(message)]
        pub fn set_admin(&mut self, new_admin: AccountId) {
            let key = self.admin.key();
            ink::env::debug_println!("Logic::set_admin: Key {:?}", &key);

            self.admin = new_admin;
            ink::env::debug_println!("Logic::set_admin: {:?}", &self.admin);
        }
    }
}
