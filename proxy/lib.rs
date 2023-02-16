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
        value: u32,
    }

    /// Distinguishes whether the return type comes from the `Proxy` contract or the `Logic`
    /// contract.
    ///
    /// This is needed since we cannot get the actual return type for any message from the `Logic`
    /// contract, but may still return from it.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum ProxyOrLogic<T> {
        /// The value returned by the `Proxy` contract message.
        Proxy(T),
        /// The message returned a value from the `Logic` contract. However, we do not know what
        /// this value actually was.
        Logic,
    }

    /// The Proxy error type.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// The `Proxy` admin attempted to proxy a call to the `Logic` contract.
        ///
        /// In the Transparent Proxy pattern the `Proxy` admin is not allowed to proxy any calls to
        /// the `Logic` contract.
        AdminAttemptedToProxy,
    }

    /// The Proxy Result type.
    pub type Result<T> = core::result::Result<T, Error>;

    const FALLBACK_ERR_MSG: &str = "Checked for a non-admin user before calling `fallback`.";

    impl Proxy {
        #[ink(constructor)]
        pub fn new(admin: AccountId, implementation: AccountId) -> Self {
            Self {
                admin,
                implementation,
                value: Default::default(),
            }
        }

        #[ink(message)]
        pub fn admin(&mut self) -> ProxyOrLogic<AccountId> {
            let caller = self.env().caller();
            if caller != self.admin {
                self.fallback().expect(FALLBACK_ERR_MSG);
                ProxyOrLogic::Logic
            } else {
                ProxyOrLogic::Proxy(self.admin)
            }
        }

        #[ink(message)]
        pub fn implementation(&mut self) -> ProxyOrLogic<AccountId> {
            let caller = self.env().caller();
            if caller != self.admin {
                self.fallback().expect(FALLBACK_ERR_MSG);
                ProxyOrLogic::Logic
            } else {
                ProxyOrLogic::Proxy(self.implementation)
            }
        }

        #[ink(message)]
        pub fn upgrade_to(&mut self, new_code: AccountId) {
            let caller = self.env().caller();
            if caller != self.admin {
                self.fallback().expect(FALLBACK_ERR_MSG);
            } else {
                self.implementation = new_code
            }
        }

        #[ink(message)]
        pub fn get(&mut self) -> ProxyOrLogic<u32> {
            let caller = self.env().caller();
            if caller != self.admin {
                self.fallback().expect(FALLBACK_ERR_MSG);
                ProxyOrLogic::Logic
            } else {
                use ink::storage::traits::StorageKey;
                let key = self.value.key();
                ink::env::debug_println!("Proxy::get: Key {:?}", &key);

                let value = self.value;
                ink::env::debug_println!("Proxy::get: {:?}", &value);

                ProxyOrLogic::Proxy(value)
            }
        }

        #[ink(message)]
        pub fn set(&mut self, value: u32) {
            let caller = self.env().caller();
            if caller != self.admin {
                self.fallback().expect(FALLBACK_ERR_MSG);
            } else {
                use ink::storage::traits::StorageKey;
                let key = self.value.key();
                ink::env::debug_println!("Proxy::set: Key {:?}", &key);

                self.value = value;
                ink::env::debug_println!("Proxy::set: {:?}", &self.value);
            }
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
        /// `Proxy`. This allows the `Proxy` contract to be the last to commit to storage. If we
        /// don't do this then the `Logic` contract (which has a different and possibly conflicting
        /// storage layout) will be last to commit, rendering the `Proxy` contract's storage
        /// useless.
        ///
        /// It follows that in order for us to write to storage that we have a mutable reference to
        /// the `Proxy` storage, which is why we require `&mut self` in the function signature.
        #[ink(message, selector = _)]
        pub fn fallback(&mut self) -> Result<()> {
            use ink::env::{call::build_call, DefaultEnvironment};

            // As part of the Transparent Proxy pattern we don't allow the `Proxy` admin to proxy
            // any messages to the `Logic` contract.
            ensure!(
                self.env().caller() != self.admin,
                Error::AdminAttemptedToProxy,
            );

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

            Ok(())
        }
    }
}
