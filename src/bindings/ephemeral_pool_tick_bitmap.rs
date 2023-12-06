pub use ephemeral_pool_tick_bitmap::*;
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
pub mod ephemeral_pool_tick_bitmap {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("pool"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("V3PoolCallee"),),
                },],
            }),
            functions: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("getTickBitmap"),
                ::std::vec![::ethers::core::abi::ethabi::Function {
                    name: ::std::borrow::ToOwned::to_owned("getTickBitmap"),
                    inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("pool"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("V3PoolCallee"),),
                    },],
                    outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("slots"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                            ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                        ),),
                        internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                            "struct PoolUtils.Slot[]"
                        ),),
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
    pub static EPHEMERALPOOLTICKBITMAP_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`@Qa\x02\xF18\x03\x80a\x02\xF1\x839\x81\x01`@\x81\x90Ra\0\"\x91a\x02\"V[`\0a\0-\x82a\0[V[\x90P`\0\x81`@Q` \x01a\0B\x91\x90a\x02RV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80Q` \x82\x01\xFD[```\0\x82`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xC1\x91\x90a\x02\xA1V[\x90P`\0\x80b\r\x89\xE7\x19\x83\x81\x07\x82\x13\x90\x84\x90\x05\x03`\x02\x90\x81\x0B`\x08\x90\x81\x1D\x91b\r\x89\xE8\x86\x81\x07\x85\x13\x90\x87\x90\x05\x03\x90\x0B\x90\x1D\x91P\x91P\x81\x81\x03`\x01\x01a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\x17Wa\x01\x17a\x02\xC4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\\W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x015W\x90P[P\x93P\x81[\x81`\x01\x0B\x81`\x01\x0B\x13a\x01\xD1W`\0\x81\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80\x83R\x91\x90\x81\x01a\x01\xA2`\x01`\x01`\xA0\x1B\x03\x8A\x16\x85a\x01\xDAV[\x81RP\x86\x85\x84\x03a\xFF\xFF\x16\x81Q\x81\x10a\x01\xBDWa\x01\xBDa\x02\xDAV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x01aV[PPPP\x91\x90PV[`\0`\x01\x82\x90\x0Ba\x01\xF6\x84c)\x9C\xE1K`\xE1\x1B\x83\x85` a\x02\x01V[PP`\0Q\x92\x91PPV[\x83`\0R\x82`\x04R\x80\x82`$`\0\x88Z\xFAa\x02\x1BW`\0\x80\xFD[PPPPPV[`\0` \x82\x84\x03\x12\x15a\x024W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02KW`\0\x80\xFD[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x02\x94W\x81Q\x80Q\x85R\x86\x01Q\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x02oV[P\x91\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x02\xB3W`\0\x80\xFD[\x81Q\x80`\x02\x0B\x81\x14a\x02KW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE";
    /// The bytecode of the contract.
    pub static EPHEMERALPOOLTICKBITMAP_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\x1EW`\x005`\xE0\x1C\x80cfk]\xA3\x14a\0#W[`\0\x80\xFD[a\x006a\x0016`\x04a\x02nV[a\0LV[`@Qa\0C\x91\x90a\x02\xABV[`@Q\x80\x91\x03\x90\xF3[```\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xBF\x91\x90a\x02\xFAV[\x90P`\0\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18\x83\x81\x07\x82\x13\x90\x84\x90\x05\x03`\x02\x90\x81\x0B`\x08\x90\x81\x1D\x91b\r\x89\xE8\x86\x81\x07\x85\x13\x90\x87\x90\x05\x03\x90\x0B\x90\x1D\x91P\x91P\x81\x81\x03`\x01\x01a\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x012Wa\x012a\x03\x1DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01wW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01PW\x90P[P\x93P\x81[\x81`\x01\x0B\x81`\x01\x0B\x13a\x02\x04W`\0\x81`\0R`\x06` R`@`\0 \x90P`@Q\x80`@\x01`@R\x80\x82\x81R` \x01a\x01\xD5\x84\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x02\r\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81RP\x86\x85\x84\x03a\xFF\xFF\x16\x81Q\x81\x10a\x01\xF0Wa\x01\xF0a\x03LV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x01|V[PPPP\x91\x90PV[`\0`\x01\x82\x90\x0Ba\x02B\x84\x7FS9\xC2\x96\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x85` a\x02MV[PP`\0Q\x92\x91PPV[\x83`\0R\x82`\x04R\x80\x82`$`\0\x88Z\xFAa\x02gW`\0\x80\xFD[PPPPPV[`\0` \x82\x84\x03\x12\x15a\x02\x80W`\0\x80\xFD[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02\xA4W`\0\x80\xFD[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x02\xEDW\x81Q\x80Q\x85R\x86\x01Q\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x02\xC8V[P\x91\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x03\x0CW`\0\x80\xFD[\x81Q\x80`\x02\x0B\x81\x14a\x02\xA4W`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x16\0\n";
    /// The deployed bytecode of the contract.
    pub static EPHEMERALPOOLTICKBITMAP_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct EphemeralPoolTickBitmap<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for EphemeralPoolTickBitmap<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for EphemeralPoolTickBitmap<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for EphemeralPoolTickBitmap<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for EphemeralPoolTickBitmap<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(EphemeralPoolTickBitmap))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> EphemeralPoolTickBitmap<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                EPHEMERALPOOLTICKBITMAP_ABI.clone(),
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
                EPHEMERALPOOLTICKBITMAP_ABI.clone(),
                EPHEMERALPOOLTICKBITMAP_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getTickBitmap` (0x666b5da3) function
        pub fn get_tick_bitmap(
            &self,
            pool: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Slot>> {
            self.0
                .method_hash([102, 107, 93, 163], pool)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for EphemeralPoolTickBitmap<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getTickBitmap` function with signature `getTickBitmap(address)` and selector `0x666b5da3`
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
    #[ethcall(name = "getTickBitmap", abi = "getTickBitmap(address)")]
    pub struct GetTickBitmapCall {
        pub pool: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getTickBitmap` function with signature `getTickBitmap(address)` and selector `0x666b5da3`
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
    pub struct GetTickBitmapReturn {
        pub slots: ::std::vec::Vec<Slot>,
    }
}
