#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod proxy {

    #[ink(storage)]
    pub struct Proxy {
        admin: AccountId,
        implementation: AccountId,
    }

    impl Proxy {
        #[ink(constructor)]
        pub fn new(admin: AccountId, implementation: AccountId) -> Self {
            Self {
                admin,
                implementation,
            }
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
            use ink::env::call::{build_call, ExecutionInput, Selector};
            use ink::env::DefaultEnvironment;

            let code_hash = self.env().code_hash(&self.implementation).unwrap();
            let admin_id: AccountId = build_call::<DefaultEnvironment>()
                .delegate(code_hash)
                .exec_input(ExecutionInput::new(Selector::new(ink::selector_bytes!(
                    "update_admin"
                ))))
                .returns::<AccountId>()
                .invoke();

            ink::env::debug_println!("Proxy {:?}", admin_id);
        }
    }
}
