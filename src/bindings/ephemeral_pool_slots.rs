pub use ephemeral_pool_slots::*;
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
pub mod ephemeral_pool_slots {
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
                ::std::borrow::ToOwned::to_owned("getSlots"),
                ::std::vec![::ethers::core::abi::ethabi::Function {
                    name: ::std::borrow::ToOwned::to_owned("getSlots"),
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
    pub static EPHEMERALPOOLSLOTS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`@Qb\0\x05\xA98\x03\x80b\0\x05\xA9\x839\x81\x01`@\x81\x90Rb\0\0&\x91b\0\x05\0V[`\0b\0\x003\x82b\0\0cV[\x90P`\0\x81`@Q` \x01b\0\0J\x91\x90b\0\x05+V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80Q` \x82\x01\xFD[```\0\x80\x80\x80\x80\x80\x80\x80b\0\0\x82`\x01`\x01`\xA0\x1B\x03\x8B\x16b\0\x03mV[a\xFF\xFF\x84\x16\x9EP\x95\x9CP\x93\x9AP\x91\x98P\x96P\x94P\x92P\x90Pb\xFF\xFF\xFF`\xA0\x1B`\xA0\x87\x90\x1B\x16`\xB8\x86\x90\x1B`\xC8\x86\x90\x1B`\xD8\x86\x90\x1B`\xE8\x86\x90\x1B`\xF0\x86\x90\x1B\x17\x17\x17\x17\x17\x87\x17`\x05\x89\x01`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\0\xE5Wb\0\0\xE5b\0\x05|V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15b\0\x01,W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81b\0\x01\x04W\x90P[P\x99P`@Q\x80`@\x01`@R\x80`\0\x81R` \x01\x82\x81RP\x8A`\0\x81Q\x81\x10b\0\x01[Wb\0\x01[b\0\x05\x92V[` \x02` \x01\x01\x81\x90RPPPPPPPPP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01b\0\x01\x9A\x85`\x01`\x01`\xA0\x1B\x03\x16b\0\x03\xD6` \x1B` \x1CV[\x81RP\x82`\x01\x81Q\x81\x10b\0\x01\xB3Wb\0\x01\xB3b\0\x05\x92V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01b\0\x01\xEA\x85`\x01`\x01`\xA0\x1B\x03\x16b\0\x03\xF1` \x1B` \x1CV[\x81RP\x82`\x02\x81Q\x81\x10b\0\x02\x03Wb\0\x02\x03b\0\x05\x92V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\0\x80b\0\x02%`\x01`\x01`\xA0\x1B\x03\x86\x16b\0\x04\x06V[\x91P\x91P`\0\x82\x82`\x80\x1B\x17\x90P`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01\x82\x81RP\x85`\x03\x81Q\x81\x10b\0\x02_Wb\0\x02_b\0\x05\x92V[` \x02` \x01\x01\x81\x90RPPPP`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01b\0\x02\x99\x85`\x01`\x01`\xA0\x1B\x03\x16b\0\x04*` \x1B` \x1CV[`\x01`\x01`\x80\x1B\x03\x16\x81RP\x82`\x04\x81Q\x81\x10b\0\x02\xBBWb\0\x02\xBBb\0\x05\x92V[` \x02` \x01\x01\x81\x90RP`\0[\x81\x81\x10\x15b\0\x03fW`\0\x80\x80\x80b\0\x02\xEC`\x01`\x01`\xA0\x1B\x03\x89\x16\x86b\0\x04GV[\x93P\x93P\x93P\x93P`\0\x81`\xF8\x1B\x90P\x80\x83`X\x1B\x17\x90P\x80\x84f\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x1B\x17\x90P\x80\x85\x17\x90P`@Q\x80`@\x01`@R\x80`\x08\x88\x01\x81R` \x01\x82\x81RP\x88\x87`\x05\x01\x81Q\x81\x10b\0\x03JWb\0\x03Jb\0\x05\x92V[` \x02` \x01\x01\x81\x90RPPPPPP\x80`\x01\x01\x90Pb\0\x02\xC9V[PP\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80c8P\xC7\xBD`\xE0\x1B\x90P`@Q\x81`\0R`\xE0\x81`\x04`\0\x8DZ\xFAb\0\x03\x9EW`\0\x80\xFD[\x80Q` \x82\x01Q`@\x83\x01Q``\x84\x01Q`\x80\x85\x01Q`\xA0\x86\x01Q`\xC0\x90\x96\x01Q\x94\x9F\x93\x9EP\x91\x9CP\x9AP\x98P\x91\x96P\x94P\x92PPPV[`\0b\0\x03\xEB\x82c\xF3\x05\x83\x99`\xE0\x1Bb\0\x04\x8CV[\x92\x91PPV[`\0b\0\x03\xEB\x82cF\x14\x13\x19`\xE0\x1Bb\0\x04\x8CV[`\0\x80\x80\x80b\0\x04\x1E\x85c\x1A\xD8\xB0;`\xE0\x1Bb\0\x04\xB1V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80b\0\x04@\x83c\r42\x81`\xE1\x1Bb\0\x04\x8CV[\x93\x92PPPV[`@Q`\0\x90\x81\x90\x81\x90\x81\x90b\0\x04j\x87c%,\t\xD7`\xE0\x1B\x88\x84`\x80b\0\x04\xDEV[\x80Q` \x82\x01Q`@\x83\x01Q``\x90\x93\x01Q\x91\x99\x90\x98P\x91\x96P\x94P\x92PPPV[`\0\x81`\0R` `\0`\x04`\0\x86Z\xFAb\0\x04\xA7W`\0\x80\xFD[PP`\0Q\x91\x90PV[`\0\x80\x82`\0R`@`\0`\x04`\0\x87Z\xFAb\0\x04\xCDW`\0\x80\xFD[`\0Q\x91P` Q\x90P\x92P\x92\x90PV[\x83`\0R\x82`\x04R\x80\x82`$`\0\x88Z\xFAb\0\x04\xF9W`\0\x80\xFD[PPPPPV[`\0` \x82\x84\x03\x12\x15b\0\x05\x13W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x04@W`\0\x80\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15b\0\x05oW\x81Q\x80Q\x85R\x86\x01Q\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01b\0\x05HV[P\x91\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE";
    /// The bytecode of the contract.
    pub static EPHEMERALPOOLSLOTS_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\x1EW`\x005`\xE0\x1C\x80c\x1D6\xF0\xF0\x14a\0#W[`\0\x80\xFD[a\x006a\x0016`\x04a\x05\x8DV[a\0LV[`@Qa\0C\x91\x90a\x05\xC3V[`@Q\x80\x91\x03\x90\xF3[```\0\x80`\0\x80`\0\x80`\0\x80a\0y\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\x8BV[a\xFF\xFF\x84\x16\x9EP\x95\x9CP\x93\x9AP\x91\x98P\x96P\x94P\x92P\x90Pv\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xA0\x87\x90\x1B\x16`\xB8\x86\x90\x1B`\xC8\x86\x90\x1B`\xD8\x86\x90\x1B`\xE8\x86\x90\x1B`\xF0\x86\x90\x1B\x17\x17\x17\x17\x17\x87\x17`\x05\x89\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xEBWa\0\xEBa\x06\x12V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x010W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01\tW\x90P[P\x99P`@Q\x80`@\x01`@R\x80`\0\x81R` \x01\x82\x81RP\x8A`\0\x81Q\x81\x10a\x01\\Wa\x01\\a\x06AV[` \x02` \x01\x01\x81\x90RPPPPPPPPP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01a\x01\xA0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\xF3V[\x81RP\x82`\x01\x81Q\x81\x10a\x01\xB6Wa\x01\xB6a\x06AV[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x01\xF2\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04%V[\x81RP\x82`\x02\x81Q\x81\x10a\x02\x08Wa\x02\x08a\x06AV[` \x02` \x01\x01\x81\x90RP`\0\x80a\x025\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04QV[\x91P\x91P`\0\x82\x82`\x80\x1B\x17\x90P`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01\x82\x81RP\x85`\x03\x81Q\x81\x10a\x02lWa\x02la\x06AV[` \x02` \x01\x01\x81\x90RPPPP`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01a\x02\xAB\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04\x8CV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x82`\x04\x81Q\x81\x10a\x02\xD3Wa\x02\xD3a\x06AV[` \x02` \x01\x01\x81\x90RP`\0[\x81\x81\x10\x15a\x03\x84W`\0\x80\x80\x80a\x03\x0Es\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x86a\x04\xC0V[\x93P\x93P\x93P\x93P`\0\x81`\xF8\x1B\x90P\x80\x83`X\x1B\x17\x90P\x80\x84f\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x1B\x17\x90P\x80\x85\x17\x90P`@Q\x80`@\x01`@R\x80`\x08\x88\x01\x81R` \x01\x82\x81RP\x88\x87`\x05\x01\x81Q\x81\x10a\x03iWa\x03ia\x06AV[` \x02` \x01\x01\x81\x90RPPPPPP\x80`\x01\x01\x90Pa\x02\xE1V[PP\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80c8P\xC7\xBD`\xE0\x1B\x90P`@Q\x81`\0R`\xE0\x81`\x04`\0\x8DZ\xFAa\x03\xBBW`\0\x80\xFD[\x80Q` \x82\x01Q`@\x83\x01Q``\x84\x01Q`\x80\x85\x01Q`\xA0\x86\x01Q`\xC0\x90\x96\x01Q\x94\x9F\x93\x9EP\x91\x9CP\x9AP\x98P\x91\x96P\x94P\x92PPPV[`\0a\x04\x1F\x82\x7F\xF3\x05\x83\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05\x1CV[\x92\x91PPV[`\0a\x04\x1F\x82\x7FF\x14\x13\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05\x1CV[`\0\x80\x80\x80a\x04\x80\x85\x7F\x1A\xD8\xB0;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05@V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80a\x04\xB9\x83\x7F\x1Ahe\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05\x1CV[\x93\x92PPPV[`@Q`\0\x90\x81\x90\x81\x90\x81\x90a\x04\xFA\x87\x7F%,\t\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x84`\x80a\x05lV[\x80Q` \x82\x01Q`@\x83\x01Q``\x90\x93\x01Q\x91\x99\x90\x98P\x91\x96P\x94P\x92PPPV[`\0\x81`\0R` `\0`\x04`\0\x86Z\xFAa\x056W`\0\x80\xFD[PP`\0Q\x91\x90PV[`\0\x80\x82`\0R`@`\0`\x04`\0\x87Z\xFAa\x05[W`\0\x80\xFD[`\0Q\x91P` Q\x90P\x92P\x92\x90PV[\x83`\0R\x82`\x04R\x80\x82`$`\0\x88Z\xFAa\x05\x86W`\0\x80\xFD[PPPPPV[`\0` \x82\x84\x03\x12\x15a\x05\x9FW`\0\x80\xFD[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04\xB9W`\0\x80\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x06\x05W\x81Q\x80Q\x85R\x86\x01Q\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x05\xE0V[P\x91\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA1dsolcC\0\x08\x16\0\n";
    /// The deployed bytecode of the contract.
    pub static EPHEMERALPOOLSLOTS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct EphemeralPoolSlots<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for EphemeralPoolSlots<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for EphemeralPoolSlots<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for EphemeralPoolSlots<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for EphemeralPoolSlots<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(EphemeralPoolSlots))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> EphemeralPoolSlots<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                EPHEMERALPOOLSLOTS_ABI.clone(),
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
                EPHEMERALPOOLSLOTS_ABI.clone(),
                EPHEMERALPOOLSLOTS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getSlots` (0x1d36f0f0) function
        pub fn get_slots(
            &self,
            pool: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Slot>> {
            self.0
                .method_hash([29, 54, 240, 240], pool)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for EphemeralPoolSlots<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getSlots` function with signature `getSlots(address)` and selector `0x1d36f0f0`
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
    #[ethcall(name = "getSlots", abi = "getSlots(address)")]
    pub struct GetSlotsCall {
        pub pool: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getSlots` function with signature `getSlots(address)` and selector `0x1d36f0f0`
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
    pub struct GetSlotsReturn {
        pub slots: ::std::vec::Vec<Slot>,
    }
}
