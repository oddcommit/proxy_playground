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
    #[ink(storage)]
    pub struct Proxy {
        admin: AccountId,
        implementation: AccountId,
    }

    /// The Proxy error type.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        UnauthorizedCaller,
    }

    /// The Proxy Result type.
    pub type Result<T> = core::result::Result<T, Error>;

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

        #[ink(message)]
        pub fn upgrade_to(&mut self, new_code: AccountId) -> Result<()> {
            let caller = self.env().caller();
            ensure!(caller == self.admin, Error::UnauthorizedCaller);

            Ok(self.implementation = new_code)
        }

        /// Forwards any call which doesn't match a message in the contract.
        ///
        /// # Note
        ///
        /// There are two key things which are required for this to work correctly.
        ///
        /// 1. set_tail_call(false)
        /// 2. The `&mut self` signature
        ///
        /// We need (1) in order to ensure that the `Logic` contract returns control to the
        /// `Proxy`. This allows the storage write to happen in the context of the `Proxy`.
        ///
        /// It follows that in order for us to write to storage that we have a mutable reference to
        /// the `Proxy` storage, which is why we require `&mut self` in the function signature.
        #[ink(message, selector = _)]
        pub fn fallback(&mut self) {
            use ink::env::call::build_call;
            use ink::env::DefaultEnvironment;

            ink::env::debug_println!("Proxying Call");

            let code_hash = self.env().code_hash(&self.implementation).unwrap();
            build_call::<DefaultEnvironment>()
                .delegate(code_hash)
                .call_flags(
                    ink::env::CallFlags::default()
                        .set_forward_input(true)
                        .set_tail_call(false),
                )
                .invoke();
        }
    }
}
