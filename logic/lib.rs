#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod logic {
    use ink::storage::traits::ManualKey;
    use ink::storage::Lazy;

    const VALUE_OFFSET: u32 = 1337;

    #[ink(storage)]
    pub struct Logic {
        admin: AccountId,
        value: Lazy<bool, ManualKey<VALUE_OFFSET>>,
    }

    impl Logic {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut value = Lazy::default();
            value.set(&false);

            Self {
                admin: AccountId::from([0x00; 32]),
                value,
            }
        }

        #[ink(message, selector = 1)]
        pub fn admin(&self) -> AccountId {
            let admin = self.admin;
            ink::env::debug_println!("Logic {:?}", &admin);
            admin
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            let value = self.value.get().unwrap();
            self.value.set(&!value)
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value.get().unwrap()
        }
    }
}
