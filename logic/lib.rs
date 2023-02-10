#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod logic {
    use ink::storage::traits::StorageKey;
    use ink::storage::Lazy;

    #[ink(storage)]
    pub struct Logic {
        value: Lazy<bool>,
    }

    impl Logic {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
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
    }
}
