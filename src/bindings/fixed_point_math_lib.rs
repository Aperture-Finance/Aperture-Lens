pub use fixed_point_math_lib::*;
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
pub mod fixed_point_math_lib {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::std::collections::BTreeMap::new(),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DivFailed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("DivFailed"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DivWadFailed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("DivWadFailed"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExpOverflow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ExpOverflow"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FactorialOverflow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FactorialOverflow"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FullMulDivFailed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FullMulDivFailed"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LnWadUndefined"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("LnWadUndefined"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MantissaOverflow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("MantissaOverflow"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MulDivFailed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("MulDivFailed"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MulWadFailed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("MulWadFailed"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RPowOverflow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("RPowOverflow"),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static FIXEDPOINTMATHLIB_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`-`7`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14`*WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\0\x80\xFD\xFE\xA1dsolcC\0\x08\x16\0\n";
    /// The bytecode of the contract.
    pub static FIXEDPOINTMATHLIB_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\0\x80\xFD\xFE\xA1dsolcC\0\x08\x16\0\n";
    /// The deployed bytecode of the contract.
    pub static FIXEDPOINTMATHLIB_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct FixedPointMathLib<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FixedPointMathLib<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FixedPointMathLib<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FixedPointMathLib<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FixedPointMathLib<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(FixedPointMathLib))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FixedPointMathLib<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                FIXEDPOINTMATHLIB_ABI.clone(),
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
                FIXEDPOINTMATHLIB_ABI.clone(),
                FIXEDPOINTMATHLIB_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for FixedPointMathLib<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `DivFailed` with signature `DivFailed()` and selector `0x65244e4e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "DivFailed", abi = "DivFailed()")]
    pub struct DivFailed;
    ///Custom Error type `DivWadFailed` with signature `DivWadFailed()` and selector `0x7c5f487d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "DivWadFailed", abi = "DivWadFailed()")]
    pub struct DivWadFailed;
    ///Custom Error type `ExpOverflow` with signature `ExpOverflow()` and selector `0xa37bfec9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ExpOverflow", abi = "ExpOverflow()")]
    pub struct ExpOverflow;
    ///Custom Error type `FactorialOverflow` with signature `FactorialOverflow()` and selector `0xaba0f2a2`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "FactorialOverflow", abi = "FactorialOverflow()")]
    pub struct FactorialOverflow;
    ///Custom Error type `FullMulDivFailed` with signature `FullMulDivFailed()` and selector `0xae47f702`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "FullMulDivFailed", abi = "FullMulDivFailed()")]
    pub struct FullMulDivFailed;
    ///Custom Error type `LnWadUndefined` with signature `LnWadUndefined()` and selector `0x1615e638`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "LnWadUndefined", abi = "LnWadUndefined()")]
    pub struct LnWadUndefined;
    ///Custom Error type `MantissaOverflow` with signature `MantissaOverflow()` and selector `0xce30380c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "MantissaOverflow", abi = "MantissaOverflow()")]
    pub struct MantissaOverflow;
    ///Custom Error type `MulDivFailed` with signature `MulDivFailed()` and selector `0xad251c27`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "MulDivFailed", abi = "MulDivFailed()")]
    pub struct MulDivFailed;
    ///Custom Error type `MulWadFailed` with signature `MulWadFailed()` and selector `0xbac65e5b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "MulWadFailed", abi = "MulWadFailed()")]
    pub struct MulWadFailed;
    ///Custom Error type `RPowOverflow` with signature `RPowOverflow()` and selector `0x49f7642b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "RPowOverflow", abi = "RPowOverflow()")]
    pub struct RPowOverflow;
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone, ::ethers::contract::EthAbiType, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash,
    )]
    pub enum FixedPointMathLibErrors {
        DivFailed(DivFailed),
        DivWadFailed(DivWadFailed),
        ExpOverflow(ExpOverflow),
        FactorialOverflow(FactorialOverflow),
        FullMulDivFailed(FullMulDivFailed),
        LnWadUndefined(LnWadUndefined),
        MantissaOverflow(MantissaOverflow),
        MulDivFailed(MulDivFailed),
        MulWadFailed(MulWadFailed),
        RPowOverflow(RPowOverflow),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for FixedPointMathLibErrors {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <DivFailed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DivFailed(decoded));
            }
            if let Ok(decoded) = <DivWadFailed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DivWadFailed(decoded));
            }
            if let Ok(decoded) = <ExpOverflow as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExpOverflow(decoded));
            }
            if let Ok(decoded) = <FactorialOverflow as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FactorialOverflow(decoded));
            }
            if let Ok(decoded) = <FullMulDivFailed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FullMulDivFailed(decoded));
            }
            if let Ok(decoded) = <LnWadUndefined as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LnWadUndefined(decoded));
            }
            if let Ok(decoded) = <MantissaOverflow as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MantissaOverflow(decoded));
            }
            if let Ok(decoded) = <MulDivFailed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MulDivFailed(decoded));
            }
            if let Ok(decoded) = <MulWadFailed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MulWadFailed(decoded));
            }
            if let Ok(decoded) = <RPowOverflow as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RPowOverflow(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FixedPointMathLibErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DivFailed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DivWadFailed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExpOverflow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FactorialOverflow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FullMulDivFailed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LnWadUndefined(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MantissaOverflow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MulDivFailed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MulWadFailed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RPowOverflow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for FixedPointMathLibErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <DivFailed as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <DivWadFailed as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ExpOverflow as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <FactorialOverflow as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <FullMulDivFailed as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <LnWadUndefined as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <MantissaOverflow as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <MulDivFailed as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <MulWadFailed as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <RPowOverflow as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for FixedPointMathLibErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DivFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::DivWadFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpOverflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::FactorialOverflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::FullMulDivFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::LnWadUndefined(element) => ::core::fmt::Display::fmt(element, f),
                Self::MantissaOverflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::MulDivFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::MulWadFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::RPowOverflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for FixedPointMathLibErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DivFailed> for FixedPointMathLibErrors {
        fn from(value: DivFailed) -> Self {
            Self::DivFailed(value)
        }
    }
    impl ::core::convert::From<DivWadFailed> for FixedPointMathLibErrors {
        fn from(value: DivWadFailed) -> Self {
            Self::DivWadFailed(value)
        }
    }
    impl ::core::convert::From<ExpOverflow> for FixedPointMathLibErrors {
        fn from(value: ExpOverflow) -> Self {
            Self::ExpOverflow(value)
        }
    }
    impl ::core::convert::From<FactorialOverflow> for FixedPointMathLibErrors {
        fn from(value: FactorialOverflow) -> Self {
            Self::FactorialOverflow(value)
        }
    }
    impl ::core::convert::From<FullMulDivFailed> for FixedPointMathLibErrors {
        fn from(value: FullMulDivFailed) -> Self {
            Self::FullMulDivFailed(value)
        }
    }
    impl ::core::convert::From<LnWadUndefined> for FixedPointMathLibErrors {
        fn from(value: LnWadUndefined) -> Self {
            Self::LnWadUndefined(value)
        }
    }
    impl ::core::convert::From<MantissaOverflow> for FixedPointMathLibErrors {
        fn from(value: MantissaOverflow) -> Self {
            Self::MantissaOverflow(value)
        }
    }
    impl ::core::convert::From<MulDivFailed> for FixedPointMathLibErrors {
        fn from(value: MulDivFailed) -> Self {
            Self::MulDivFailed(value)
        }
    }
    impl ::core::convert::From<MulWadFailed> for FixedPointMathLibErrors {
        fn from(value: MulWadFailed) -> Self {
            Self::MulWadFailed(value)
        }
    }
    impl ::core::convert::From<RPowOverflow> for FixedPointMathLibErrors {
        fn from(value: RPowOverflow) -> Self {
            Self::RPowOverflow(value)
        }
    }
}
