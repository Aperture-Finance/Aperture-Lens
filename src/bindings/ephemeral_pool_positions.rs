pub use ephemeral_pool_positions::*;
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
pub mod ephemeral_pool_positions {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("pool"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("V3PoolCallee"),),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("keys"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                            ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                            ],),
                        ),),
                        internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                            "struct PoolUtils.PositionKey[]",
                        ),),
                    },
                ],
            }),
            functions: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("getPositions"),
                ::std::vec![::ethers::core::abi::ethabi::Function {
                    name: ::std::borrow::ToOwned::to_owned("getPositions"),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("pool"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "V3PoolCallee"
                            ),),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("keys"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                ],),
                            ),),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "struct PoolUtils.PositionKey[]",
                            ),),
                        },
                    ],
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
    pub static EPHEMERALPOOLPOSITIONS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`@Qb\0\x05\x1B8\x03\x80b\0\x05\x1B\x839\x81\x01`@\x81\x90Rb\0\0&\x91b\0\x03\x9CV[`\0b\0\x004\x83\x83b\0\0dV[\x90P`\0\x81`@Q` \x01b\0\0K\x91\x90b\0\x04\xB3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80Q` \x82\x01\xFD[\x80Q``\x90`\x02\x81\x90\x1B`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\0\x88Wb\0\0\x88b\0\x03\x10V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15b\0\0\xCFW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81b\0\0\xA7W\x90P[P\x91P`\0\x80[\x82\x81\x10\x15b\0\x02fW`\0b\0\x01\x0E\x86\x83\x81Q\x81\x10b\0\0\xFAWb\0\0\xFAb\0\x05\x04V[` \x02` \x01\x01Qb\0\x02o` \x1B` \x1CV[`\0\x81\x81R`\x07` R`@\x81 \x91\x92Pb\0\x014`\x01`\x01`\xA0\x1B\x03\x8A\x16\x84b\0\x02\x8BV[`@\x80Q\x80\x82\x01\x90\x91R\x83\x81R\x81Q`\x01`\x01`\x80\x1B\x03\x16` \x82\x01R\x88Q`\x01\x88\x81\x01\x98\x95\x01\x94\x92\x93P\x90\x91\x89\x91\x81\x10b\0\x01tWb\0\x01tb\0\x05\x04V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80\x83\x80`\x01\x01\x94P\x81R` \x01\x82` \x01Q\x81RP\x87\x86\x80`\x01\x01\x97P\x81Q\x81\x10b\0\x01\xB9Wb\0\x01\xB9b\0\x05\x04V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80\x83\x80`\x01\x01\x94P\x81R` \x01\x82`@\x01Q\x81RP\x87\x86\x80`\x01\x01\x97P\x81Q\x81\x10b\0\x01\xFEWb\0\x01\xFEb\0\x05\x04V[` \x02` \x01\x01\x81\x90RP`\0``\x82\x01Q`\x80\x83\x01Q`\x80\x1B\x17\x90P`@Q\x80`@\x01`@R\x80\x84\x81R` \x01\x82\x81RP\x88\x87\x80`\x01\x01\x98P\x81Q\x81\x10b\0\x02KWb\0\x02Kb\0\x05\x04V[` \x02` \x01\x01\x81\x90RPPPPP\x80`\x01\x01\x90Pb\0\0\xD6V[PPP\x92\x91PPV[`@\x81\x01Q`\x06R` \x81\x01Q`\x03RQ`\0R`\x1A`\x0C \x90V[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x80b\0\x02\xCE\x84cQN\xA4\xBF`\xE0\x1B\x85\x84`\xA0b\0\x02\xD5V[P\x92\x91PPV[\x83`\0R\x82`\x04R\x80\x82`$`\0\x88Z\xFAb\0\x02\xF0W`\0\x80\xFD[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x03\rW`\0\x80\xFD[PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x03KWb\0\x03Kb\0\x03\x10V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x03|Wb\0\x03|b\0\x03\x10V[`@R\x91\x90PV[\x80Q`\x02\x81\x90\x0B\x81\x14b\0\x03\x97W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x80\x84\x86\x03\x12\x15b\0\x03\xB1W`\0\x80\xFD[\x83Qb\0\x03\xBE\x81b\0\x02\xF7V[` \x85\x81\x01Q\x91\x94P\x90`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x03\xDEW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12b\0\x03\xF3W`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0\x04\x08Wb\0\x04\x08b\0\x03\x10V[b\0\x04\x18\x84\x82`\x05\x1B\x01b\0\x03QV[\x81\x81R\x84\x81\x01\x92P``\x91\x82\x02\x84\x01\x85\x01\x91\x8A\x83\x11\x15b\0\x048W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15b\0\x04\xA2W\x80\x85\x8C\x03\x12\x15b\0\x04WW`\0\x80\x81\xFD[b\0\x04ab\0\x03&V[\x85Qb\0\x04n\x81b\0\x02\xF7V[\x81Rb\0\x04}\x86\x88\x01b\0\x03\x84V[\x87\x82\x01Rb\0\x04\x8E\x88\x87\x01b\0\x03\x84V[\x81\x89\x01R\x84R\x93\x84\x01\x93\x92\x85\x01\x92b\0\x04=V[P\x80\x96PPPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15b\0\x04\xF7W\x81Q\x80Q\x85R\x86\x01Q\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01b\0\x04\xD0V[P\x91\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE";
    /// The bytecode of the contract.
    pub static EPHEMERALPOOLPOSITIONS_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\x1EW`\x005`\xE0\x1C\x80c\xC2A\xE2z\x14a\0#W[`\0\x80\xFD[a\x006a\x0016`\x04a\x03\xCAV[a\0LV[`@Qa\0C\x91\x90a\x04\xCAV[`@Q\x80\x91\x03\x90\xF3[\x80Q``\x90`\x02\x81\x90\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0nWa\0na\x03\x0CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xB3W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0\x8CW\x90P[P\x91P`\0\x80[\x82\x81\x10\x15a\x02\\W`\0a\0\xFD\x86\x83\x81Q\x81\x10a\0\xD9Wa\0\xD9a\x05\x19V[` \x02` \x01\x01Q`@\x81\x01Q`\x06R` \x81\x01Q`\x03RQ`\0R`\x1A`\x0C \x90V[`\0\x81\x81R`\x07` R`@\x81 \x91\x92Pa\x01.s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16\x84a\x02eV[`@\x80Q\x80\x82\x01\x90\x91R\x83\x81R\x81Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x82\x01R\x88Q`\x01\x88\x81\x01\x98\x95\x01\x94\x92\x93P\x90\x91\x89\x91\x81\x10a\x01tWa\x01ta\x05\x19V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80\x83\x80`\x01\x01\x94P\x81R` \x01\x82` \x01Q\x81RP\x87\x86\x80`\x01\x01\x97P\x81Q\x81\x10a\x01\xB6Wa\x01\xB6a\x05\x19V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80\x83\x80`\x01\x01\x94P\x81R` \x01\x82`@\x01Q\x81RP\x87\x86\x80`\x01\x01\x97P\x81Q\x81\x10a\x01\xF8Wa\x01\xF8a\x05\x19V[` \x02` \x01\x01\x81\x90RP`\0``\x82\x01Q`\x80\x83\x01Q`\x80\x1B\x17\x90P`@Q\x80`@\x01`@R\x80\x84\x81R` \x01\x82\x81RP\x88\x87\x80`\x01\x01\x98P\x81Q\x81\x10a\x02BWa\x02Ba\x05\x19V[` \x02` \x01\x01\x81\x90RPPPPP\x80`\x01\x01\x90Pa\0\xBAV[PPP\x92\x91PPV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x80a\x02\xBF\x84\x7FQN\xA4\xBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x84`\xA0a\x02\xC6V[P\x92\x91PPV[\x83`\0R\x82`\x04R\x80\x82`$`\0\x88Z\xFAa\x02\xE0W`\0\x80\xFD[PPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\tW`\0\x80\xFD[PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03^Wa\x03^a\x03\x0CV[`@R\x90V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\xABWa\x03\xABa\x03\x0CV[`@R\x91\x90PV[\x805`\x02\x81\x90\x0B\x81\x14a\x03\xC5W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x80\x84\x86\x03\x12\x15a\x03\xDEW`\0\x80\xFD[\x835a\x03\xE9\x81a\x02\xE7V[\x92P` \x84\x81\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\x07W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x04\x1BW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x04-Wa\x04-a\x03\x0CV[a\x04;\x84\x82`\x05\x1B\x01a\x03dV[\x81\x81R\x84\x81\x01\x92P``\x91\x82\x02\x84\x01\x85\x01\x91\x8A\x83\x11\x15a\x04ZW`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x04\xB9W\x80\x85\x8C\x03\x12\x15a\x04wW`\0\x80\x81\xFD[a\x04\x7Fa\x03;V[\x855a\x04\x8A\x81a\x02\xE7V[\x81Ra\x04\x97\x86\x88\x01a\x03\xB3V[\x87\x82\x01Ra\x04\xA6\x88\x87\x01a\x03\xB3V[\x81\x89\x01R\x84R\x93\x84\x01\x93\x92\x85\x01\x92a\x04_V[P\x80\x96PPPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x05\x0CW\x81Q\x80Q\x85R\x86\x01Q\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x04\xE7V[P\x91\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x16\0\n";
    /// The deployed bytecode of the contract.
    pub static EPHEMERALPOOLPOSITIONS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct EphemeralPoolPositions<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for EphemeralPoolPositions<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for EphemeralPoolPositions<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for EphemeralPoolPositions<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for EphemeralPoolPositions<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(EphemeralPoolPositions))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> EphemeralPoolPositions<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                EPHEMERALPOOLPOSITIONS_ABI.clone(),
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
                EPHEMERALPOOLPOSITIONS_ABI.clone(),
                EPHEMERALPOOLPOSITIONS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getPositions` (0xc241e27a) function
        pub fn get_positions(
            &self,
            pool: ::ethers::core::types::Address,
            keys: ::std::vec::Vec<PositionKey>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Slot>> {
            self.0
                .method_hash([194, 65, 226, 122], (pool, keys))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for EphemeralPoolPositions<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getPositions` function with signature `getPositions(address,(address,int24,int24)[])` and selector `0xc241e27a`
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
    #[ethcall(name = "getPositions", abi = "getPositions(address,(address,int24,int24)[])")]
    pub struct GetPositionsCall {
        pub pool: ::ethers::core::types::Address,
        pub keys: ::std::vec::Vec<PositionKey>,
    }
    ///Container type for all return fields from the `getPositions` function with signature `getPositions(address,(address,int24,int24)[])` and selector `0xc241e27a`
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
    pub struct GetPositionsReturn {
        pub slots: ::std::vec::Vec<Slot>,
    }
    ///`PositionKey(address,int24,int24)`
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
    pub struct PositionKey {
        pub owner: ::ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
}
