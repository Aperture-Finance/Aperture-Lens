pub use ephemeral_storage_lens::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod ephemeral_storage_lens {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("extsload"),
                ::std::vec![::ethers::core::abi::ethabi::Function {
                    name: ::std::borrow::ToOwned::to_owned("extsload"),
                    inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("slots"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        ),),
                        internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32[]"),),
                    },],
                    outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                        name: ::std::string::String::new(),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                        ),),
                        internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes32[]"),),
                    },],
                    constant: ::core::option::Option::None,
                    state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                },],
            )]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static EPHEMERALSTORAGELENS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x01G\x80a\0 `\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0\x1EW`\x005`\xE0\x1C\x80c\xDB\xD05\xFF\x14a\0#W[`\0\x80\xFD[a\x006a\x0016`\x04a\0\x81V[a\0LV[`@Qa\0C\x91\x90a\0\xF6V[`@Q\x80\x91\x03\x90\xF3[``` `\0R\x81` R\x81`\x05\x1B`@\x01`@\x84[\x81\x83\x14a\0zW\x805T\x82R` \x91\x82\x01\x91\x01a\0bV[PP\x80`\0\xF3[`\0\x80` \x83\x85\x03\x12\x15a\0\x94W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\0\xACW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\0\xC0W`\0\x80\xFD[\x815\x81\x81\x11\x15a\0\xCFW`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a\0\xE4W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x01.W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x01\x12V[P\x90\x96\x95PPPPPPV\xFE\xA1dsolcC\0\x08\x16\0\n";
    /// The bytecode of the contract.
    pub static EPHEMERALSTORAGELENS_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\x1EW`\x005`\xE0\x1C\x80c\xDB\xD05\xFF\x14a\0#W[`\0\x80\xFD[a\x006a\x0016`\x04a\0\x81V[a\0LV[`@Qa\0C\x91\x90a\0\xF6V[`@Q\x80\x91\x03\x90\xF3[``` `\0R\x81` R\x81`\x05\x1B`@\x01`@\x84[\x81\x83\x14a\0zW\x805T\x82R` \x91\x82\x01\x91\x01a\0bV[PP\x80`\0\xF3[`\0\x80` \x83\x85\x03\x12\x15a\0\x94W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\0\xACW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\0\xC0W`\0\x80\xFD[\x815\x81\x81\x11\x15a\0\xCFW`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a\0\xE4W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x01.W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x01\x12V[P\x90\x96\x95PPPPPPV\xFE\xA1dsolcC\0\x08\x16\0\n";
    /// The deployed bytecode of the contract.
    pub static EPHEMERALSTORAGELENS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct EphemeralStorageLens<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for EphemeralStorageLens<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for EphemeralStorageLens<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for EphemeralStorageLens<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for EphemeralStorageLens<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(EphemeralStorageLens))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> EphemeralStorageLens<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                EPHEMERALSTORAGELENS_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                EPHEMERALSTORAGELENS_ABI.clone(),
                EPHEMERALSTORAGELENS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `extsload` (0xdbd035ff) function
        pub fn extsload(
            &self,
            slots: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([219, 208, 53, 255], slots)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for EphemeralStorageLens<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `extsload` function with signature `extsload(bytes32[])` and selector `0xdbd035ff`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "extsload", abi = "extsload(bytes32[])")]
    pub struct ExtsloadCall {
        pub slots: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all return fields from the `extsload` function with signature `extsload(bytes32[])` and selector `0xdbd035ff`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ExtsloadReturn(pub ::std::vec::Vec<[u8; 32]>);
}
