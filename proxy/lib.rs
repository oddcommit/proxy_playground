#![cfg_attr(not(feature = "std"), no_std)]

macro_rules! ensure {
    ( $condition:expr, $error:expr $(,)? ) => {{
        if !$condition {
            return ::core::result::Result::Err(::core::convert::Into::into($error));
        }
    }};
}

#[ink::contract]
mod proxy {
    use ink::storage::traits::ManualKey;
    use ink::storage::Lazy;

    const VALUE_OFFSET: u32 = 1337;

    #[ink(storage)]
    pub struct Proxy {
        admin: AccountId,
        implementation: AccountId,
        value: Lazy<bool, ManualKey<VALUE_OFFSET>>,
    }

    // The Proxy error type.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        UnauthorizedCaller,
    }

    // The Proxy Result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl Proxy {
        #[ink(constructor)]
        pub fn new(admin: AccountId, implementation: AccountId) -> Self {
            let mut value = Lazy::default();
            value.set(&false);

            Self {
                admin,
                implementation,
                value,
            }
        }

        #[ink(message, selector = 1)]
        pub fn admin(&self) -> AccountId {
            let caller = self.env().caller();
            let admin = self.admin;
            if caller == admin {
                ink::env::debug_println!("Proxy {:?}", &admin);
                admin
            } else {
                self.fallback();
                unreachable!()
            }
        }

        #[ink(message)]
        pub fn implementation(&self) -> AccountId {
            let caller = self.env().caller();
            if caller == self.admin {
                self.implementation
            } else {
                self.fallback();
                unreachable!()
            }
        }

        #[ink(message)]
        pub fn upgrade_to(&mut self, new_code: AccountId) -> Result<()> {
            let caller = self.env().caller();
            ensure!(caller == self.admin, Error::UnauthorizedCaller);

            Ok(self.implementation = new_code)
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            let caller = self.env().caller();
            if caller == self.admin {
                self.value.get().unwrap()
            } else {
                self.fallback();
                unreachable!()
            }
        }

        #[ink(message, selector = _)]
        pub fn fallback(&self) {
            use ink::env::call::build_call;
            use ink::env::DefaultEnvironment;

            let code_hash = self.env().code_hash(&self.implementation).unwrap();
            build_call::<DefaultEnvironment>()
                .delegate(code_hash)
                .call_flags(
                    ink::env::CallFlags::default()
                        .set_forward_input(true)
                        .set_tail_call(true),
                )
                .invoke();
        }
    }
}
