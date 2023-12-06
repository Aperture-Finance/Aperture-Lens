pub use ephemeral_get_populated_ticks_in_range::*;
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
pub mod ephemeral_get_populated_ticks_in_range {
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
                        name: ::std::borrow::ToOwned::to_owned("tickLower"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                        internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("int24"),),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("tickUpper"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                        internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("int24"),),
                    },
                ],
            }),
            functions: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("getPopulatedTicksInRange"),
                ::std::vec![::ethers::core::abi::ethabi::Function {
                    name: ::std::borrow::ToOwned::to_owned("getPopulatedTicksInRange",),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("pool"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "V3PoolCallee"
                            ),),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("tickLower"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("int24"),),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("tickUpper"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("int24"),),
                        },
                    ],
                    outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("populatedTicks"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                            ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                        ),),
                        internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                            "struct PoolUtils.PopulatedTick[]",
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
    pub static EPHEMERALGETPOPULATEDTICKSINRANGE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`@Qb\0\x06\xF08\x03\x80b\0\x06\xF0\x839\x81\x01`@\x81\x90Rb\0\0&\x91b\0\x05'V[`\0b\0\x005\x84\x84\x84b\0\0eV[\x90P`\0\x81`@Q` \x01b\0\0L\x91\x90b\0\x05~V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80Q` \x82\x01\xFD[``\x81`\x02\x0B\x83`\x02\x0B\x13\x15b\0\0{W`\0\x80\xFD[`\0\x84`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\0\xBCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\0\xE2\x91\x90b\0\x05\xFDV[\x90P\x80\x84\x07`\0\x90\x81\x13\x82\x86\x05\x03`\x02\x90\x81\x0B`\x08\x90\x81\x1D\x92\x84\x87\x07\x81\x13\x85\x88\x05\x03\x90\x92\x0B\x90\x1D\x90\x80b\0\x01\x18\x89\x85\x85b\0\x01\xF8V[\x91P\x91P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x017Wb\0\x017b\0\x06\"V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15b\0\x01\x92W\x81` \x01[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81b\0\x01VW\x90P[P\x95P`\0\x84[\x84`\x01\x0B\x81`\x01\x0B\x13b\0\x01\xEAWb\0\x01\xDF\x8B\x82\x89\x87\x8A\x86\x03a\xFF\xFF\x16\x81Q\x81\x10b\0\x01\xC9Wb\0\x01\xC9b\0\x068V[` \x02` \x01\x01Q\x8C\x87b\0\x03\x90` \x1B` \x1CV[\x91P`\x01\x01b\0\x01\x99V[PPPPPPP\x93\x92PPPV[```\0b\0\x02\x08\x84\x84b\0\x06dV[b\0\x02\x15\x90`\x01b\0\x06\x90V[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x023Wb\0\x023b\0\x06\"V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15b\0\x02]W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x83[\x83`\x01\x0B\x81`\x01\x0B\x13b\0\x03\x87W`\0b\0\x02\x87`\x01`\x01`\xA0\x1B\x03\x88\x16\x83b\0\x04\x03V[\x90P\x80\x84b\0\x02\x97\x88\x85b\0\x06dV[a\xFF\xFF\x16\x81Q\x81\x10b\0\x02\xAEWb\0\x02\xAEb\0\x068V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x7F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x7FUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU`\x01\x83\x90\x1C\x16\x82\x03`\x02\x81\x90\x1C\x7F33333333333333333333333333333333\x90\x81\x16\x91\x16\x01`\x04\x81\x90\x1C\x01\x16\x7F\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x02`\xF8\x1C`\0\x19\x82\x14`\x08\x1B\x17b\0\x03p\x90\x84b\0\x06\xB6V[\x92PP\x80b\0\x03\x7F\x90b\0\x06\xCCV[\x90Pb\0\x02bV[P\x93P\x93\x91PPV[`\0\x80[a\x01\0\x81\x10\x15b\0\x03\xF7W`\x01\x81\x1B\x85\x16\x15b\0\x03\xEEW`\0\x81\x88`\x08\x1B\x01\x87\x02\x90Pb\0\x03\xEC\x89\x82\x87\x87\x80`\x01\x01\x98P\x81Q\x81\x10b\0\x03\xD8Wb\0\x03\xD8b\0\x068V[` \x02` \x01\x01Qb\0\x04,` \x1B` \x1CV[P[`\x01\x01b\0\x03\x94V[P\x90\x96\x95PPPPPPV[`\0`\x01\x82\x90\x0Bb\0\x04!\x84c)\x9C\xE1K`\xE1\x1B\x83\x85` b\0\x04\x86V[PP`\0Q\x92\x91PPV[`\0b\0\x04C`\x01`\x01`\xA0\x1B\x03\x85\x16\x84b\0\x04\xA8V[`\x02\x93\x90\x93\x0B\x82RP` \x80\x83\x01Q`\x0F\x0B\x90\x82\x01R\x81Q`\x01`\x01`\x80\x1B\x03\x16`@\x80\x83\x01\x91\x90\x91R\x82\x01Q``\x80\x83\x01\x91\x90\x91R\x90\x91\x01Q`\x80\x90\x91\x01RPV[\x83`\0R\x82`\x04R\x80\x82`$`\0\x88Z\xFAb\0\x04\xA1W`\0\x80\xFD[PPPPPV[`@\x80Qa\x01\0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R`\x02\x82\x90\x0B\x81b\0\x05\x07\x85c\xF3\r\xBA\x93`\xE0\x1B\x84\x84a\x01\0b\0\x04\x86V[PP\x92\x91PPV[\x80Q`\x02\x81\x90\x0B\x81\x14b\0\x05\"W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x05=W`\0\x80\xFD[\x83Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x05UW`\0\x80\xFD[\x92Pb\0\x05e` \x85\x01b\0\x05\x0FV[\x91Pb\0\x05u`@\x85\x01b\0\x05\x0FV[\x90P\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15b\0\x05\xF0W\x81Q\x80Q`\x02\x0B\x85R\x86\x81\x01Q`\x0F\x0B\x87\x86\x01R\x85\x81\x01Q`\x01`\x01`\x80\x1B\x03\x16\x86\x86\x01R``\x80\x82\x01Q\x90\x86\x01R`\x80\x90\x81\x01Q\x90\x85\x01R`\xA0\x90\x93\x01\x92\x90\x85\x01\x90`\x01\x01b\0\x05\x9BV[P\x91\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15b\0\x06\x10W`\0\x80\xFD[b\0\x06\x1B\x82b\0\x05\x0FV[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01\x82\x81\x0B\x90\x82\x90\x0B\x03a\x7F\xFF\x19\x81\x12a\x7F\xFF\x82\x13\x17\x15b\0\x06\x8AWb\0\x06\x8Ab\0\x06NV[\x92\x91PPV[`\x01\x81\x81\x0B\x90\x83\x90\x0B\x01a\x7F\xFF\x81\x13a\x7F\xFF\x19\x82\x12\x17\x15b\0\x06\x8AWb\0\x06\x8Ab\0\x06NV[\x80\x82\x01\x80\x82\x11\x15b\0\x06\x8AWb\0\x06\x8Ab\0\x06NV[`\0\x81`\x01\x0Ba\x7F\xFF\x81\x03b\0\x06\xE6Wb\0\x06\xE6b\0\x06NV[`\x01\x01\x92\x91PPV\xFE";
    /// The bytecode of the contract.
    pub static EPHEMERALGETPOPULATEDTICKSINRANGE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\x1EW`\x005`\xE0\x1C\x80c\xF2\xBB\0\x8B\x14a\0#W[`\0\x80\xFD[a\x006a\x0016`\x04a\x05fV[a\0LV[`@Qa\0C\x91\x90a\x05\xCAV[`@Q\x80\x91\x03\x90\xF3[``\x81`\x02\x0B\x83`\x02\x0B\x13\x15a\0aW`\0\x80\xFD[`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xAEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xD2\x91\x90a\x06PV[\x90P\x80\x84\x07`\0\x90\x81\x13\x82\x86\x05\x03`\x02\x90\x81\x0B`\x08\x90\x81\x1D\x92\x84\x87\x07\x81\x13\x85\x88\x05\x03\x90\x92\x0B\x90\x1D\x90\x80a\x01\x06\x89\x85\x85a\x01\xF3V[\x91P\x91P\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01#Wa\x01#a\x06tV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\x9AW\x81` \x01[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x01AW\x90P[P\x95P`\0\x84[\x84`\x01\x0B\x81`\x01\x0B\x13a\x01\xE5Wa\x01\xDB\x8B\x82\x89\x87\x8A\x86\x03a\xFF\xFF\x16\x81Q\x81\x10a\x01\xCCWa\x01\xCCa\x06\xA3V[` \x02` \x01\x01Q\x8C\x87a\x03\xA2V[\x91P`\x01\x01a\x01\xA1V[PPPPPPP\x93\x92PPPV[```\0a\x02\x01\x84\x84a\x07\x01V[a\x02\x0C\x90`\x01a\x07GV[a\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02(Wa\x02(a\x06tV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02QW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x83[\x83`\x01\x0B\x81`\x01\x0B\x13a\x03\x99W`\0a\x02\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x83a\x04\x07V[\x90P\x80\x84a\x02\x93\x88\x85a\x07\x01V[a\xFF\xFF\x16\x81Q\x81\x10a\x02\xA7Wa\x02\xA7a\x06\xA3V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x7F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x7FUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU`\x01\x83\x90\x1C\x16\x82\x03`\x02\x81\x90\x1C\x7F33333333333333333333333333333333\x90\x81\x16\x91\x16\x01`\x04\x81\x90\x1C\x01\x16\x7F\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x02`\xF8\x1C\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x14`\x08\x1B\x17a\x03\x85\x90\x84a\x07\x87V[\x92PP\x80a\x03\x92\x90a\x07\x9AV[\x90Pa\x02VV[P\x93P\x93\x91PPV[`\0\x80[a\x01\0\x81\x10\x15a\x03\xFBW`\x01\x81\x1B\x85\x16\x15a\x03\xF3W`\0\x81\x88`\x08\x1B\x01\x87\x02\x90Pa\x03\xF1\x89\x82\x87\x87\x80`\x01\x01\x98P\x81Q\x81\x10a\x03\xE4Wa\x03\xE4a\x06\xA3V[` \x02` \x01\x01Qa\x04GV[P[`\x01\x01a\x03\xA6V[P\x90\x96\x95PPPPPPV[`\0`\x01\x82\x90\x0Ba\x04<\x84\x7FS9\xC2\x96\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x85` a\x04\xB5V[PP`\0Q\x92\x91PPV[`\0a\x04is\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84a\x04\xD6V[`\x02\x93\x90\x93\x0B\x82RP` \x80\x83\x01Q`\x0F\x0B\x90\x82\x01R\x81Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x80\x83\x01\x91\x90\x91R\x82\x01Q``\x80\x83\x01\x91\x90\x91R\x90\x91\x01Q`\x80\x90\x91\x01RPV[\x83`\0R\x82`\x04R\x80\x82`$`\0\x88Z\xFAa\x04\xCFW`\0\x80\xFD[PPPPPV[`@\x80Qa\x01\0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R`\x02\x82\x90\x0B\x81a\x05L\x85\x7F\xF3\r\xBA\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84a\x01\0a\x04\xB5V[PP\x92\x91PPV[\x80`\x02\x0B\x81\x14a\x05cW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x05{W`\0\x80\xFD[\x835s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\x9FW`\0\x80\xFD[\x92P` \x84\x015a\x05\xAF\x81a\x05TV[\x91P`@\x84\x015a\x05\xBF\x81a\x05TV[\x80\x91PP\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x06CW\x81Q\x80Q`\x02\x0B\x85R\x86\x81\x01Q`\x0F\x0B\x87\x86\x01R\x85\x81\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86\x86\x01R``\x80\x82\x01Q\x90\x86\x01R`\x80\x90\x81\x01Q\x90\x85\x01R`\xA0\x90\x93\x01\x92\x90\x85\x01\x90`\x01\x01a\x05\xE7V[P\x91\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x06bW`\0\x80\xFD[\x81Qa\x06m\x81a\x05TV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\x01\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\x81\x12a\x7F\xFF\x82\x13\x17\x15a\x07AWa\x07Aa\x06\xD2V[\x92\x91PPV[`\x01\x81\x81\x0B\x90\x83\x90\x0B\x01a\x7F\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\x82\x12\x17\x15a\x07AWa\x07Aa\x06\xD2V[\x80\x82\x01\x80\x82\x11\x15a\x07AWa\x07Aa\x06\xD2V[`\0\x81`\x01\x0Ba\x7F\xFF\x81\x03a\x07\xB1Wa\x07\xB1a\x06\xD2V[`\x01\x01\x92\x91PPV\xFE\xA1dsolcC\0\x08\x16\0\n";
    /// The deployed bytecode of the contract.
    pub static EPHEMERALGETPOPULATEDTICKSINRANGE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct EphemeralGetPopulatedTicksInRange<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for EphemeralGetPopulatedTicksInRange<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for EphemeralGetPopulatedTicksInRange<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for EphemeralGetPopulatedTicksInRange<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for EphemeralGetPopulatedTicksInRange<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(EphemeralGetPopulatedTicksInRange))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> EphemeralGetPopulatedTicksInRange<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                EPHEMERALGETPOPULATEDTICKSINRANGE_ABI.clone(),
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
                EPHEMERALGETPOPULATEDTICKSINRANGE_ABI.clone(),
                EPHEMERALGETPOPULATEDTICKSINRANGE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getPopulatedTicksInRange` (0xf2bb008b) function
        pub fn get_populated_ticks_in_range(
            &self,
            pool: ::ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<PopulatedTick>> {
            self.0
                .method_hash([242, 187, 0, 139], (pool, tick_lower, tick_upper))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for EphemeralGetPopulatedTicksInRange<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getPopulatedTicksInRange` function with signature `getPopulatedTicksInRange(address,int24,int24)` and selector `0xf2bb008b`
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
    #[ethcall(
        name = "getPopulatedTicksInRange",
        abi = "getPopulatedTicksInRange(address,int24,int24)"
    )]
    pub struct GetPopulatedTicksInRangeCall {
        pub pool: ::ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    ///Container type for all return fields from the `getPopulatedTicksInRange` function with signature `getPopulatedTicksInRange(address,int24,int24)` and selector `0xf2bb008b`
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
    pub struct GetPopulatedTicksInRangeReturn {
        pub populated_ticks: ::std::vec::Vec<PopulatedTick>,
    }
    ///`PopulatedTick(int24,int128,uint128,uint256,uint256)`
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
    pub struct PopulatedTick {
        pub tick: i32,
        pub liquidity_net: i128,
        pub liquidity_gross: u128,
        pub fee_growth_outside_0x128: ::ethers::core::types::U256,
        pub fee_growth_outside_1x128: ::ethers::core::types::U256,
    }
}
