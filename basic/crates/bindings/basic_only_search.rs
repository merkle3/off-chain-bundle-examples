pub use basic_only_search::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod basic_only_search {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("search"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("search"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("settleData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static BASICONLYSEARCH_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[Pa\x02\xE9\x80a\0\x1D_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80c\xFF\xE4vb\x14a\0-W[_\x80\xFD[a\0@a\0;6`\x04a\x01\tV[a\0VV[`@Qa\0M\x91\x90a\x01\xD0V[`@Q\x80\x91\x03\x90\xF3[``_\x80\x83\x80` \x01\x90Q\x81\x01\x90a\0n\x91\x90a\x01\xE9V[\x91P\x91P\x81\x81`@Q` \x01a\0\x85\x92\x91\x90a\x02gV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\0\xDAWa\0\xDAa\0\x9DV[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\0\xFBWa\0\xFBa\0\x9DV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_` \x82\x84\x03\x12\x15a\x01\x19W_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01/W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x01?W_\x80\xFD[\x805a\x01Ra\x01M\x82a\0\xE2V[a\0\xB1V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x01fW_\x80\xFD[\x81` \x84\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_[\x83\x81\x10\x15a\x01\x9DW\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\x85V[PP_\x91\x01RV[_\x81Q\x80\x84Ra\x01\xBC\x81` \x86\x01` \x86\x01a\x01\x83V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a\x01\xE2` \x83\x01\x84a\x01\xA5V[\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a\x01\xFAW_\x80\xFD[\x82Q\x91P` \x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\x17W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x02'W_\x80\xFD[\x80Qa\x025a\x01M\x82a\0\xE2V[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15a\x02IW_\x80\xFD[a\x02Z\x82` \x83\x01` \x86\x01a\x01\x83V[\x80\x93PPPP\x92P\x92\x90PV[``\x81R`\x15``\x82\x01Rtsettle(uint256,bytes)`X\x1B`\x80\x82\x01R\x82` \x82\x01R`\xA0`@\x82\x01R_a\x02\xAB`\xA0\x83\x01\x84a\x01\xA5V[\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 `\x85\xAB\xCEY\xFDc@:~:\x11\r\xEB\xFA\xADU\x14\x9B\x87\x05\x94+\xFE\xA7\x0E\x11\x04\xD30\xAA\xA2dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static BASICONLYSEARCH_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80c\xFF\xE4vb\x14a\0-W[_\x80\xFD[a\0@a\0;6`\x04a\x01\tV[a\0VV[`@Qa\0M\x91\x90a\x01\xD0V[`@Q\x80\x91\x03\x90\xF3[``_\x80\x83\x80` \x01\x90Q\x81\x01\x90a\0n\x91\x90a\x01\xE9V[\x91P\x91P\x81\x81`@Q` \x01a\0\x85\x92\x91\x90a\x02gV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\0\xDAWa\0\xDAa\0\x9DV[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\0\xFBWa\0\xFBa\0\x9DV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_` \x82\x84\x03\x12\x15a\x01\x19W_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01/W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x01?W_\x80\xFD[\x805a\x01Ra\x01M\x82a\0\xE2V[a\0\xB1V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x01fW_\x80\xFD[\x81` \x84\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_[\x83\x81\x10\x15a\x01\x9DW\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\x85V[PP_\x91\x01RV[_\x81Q\x80\x84Ra\x01\xBC\x81` \x86\x01` \x86\x01a\x01\x83V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a\x01\xE2` \x83\x01\x84a\x01\xA5V[\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a\x01\xFAW_\x80\xFD[\x82Q\x91P` \x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\x17W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x02'W_\x80\xFD[\x80Qa\x025a\x01M\x82a\0\xE2V[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15a\x02IW_\x80\xFD[a\x02Z\x82` \x83\x01` \x86\x01a\x01\x83V[\x80\x93PPPP\x92P\x92\x90PV[``\x81R`\x15``\x82\x01Rtsettle(uint256,bytes)`X\x1B`\x80\x82\x01R\x82` \x82\x01R`\xA0`@\x82\x01R_a\x02\xAB`\xA0\x83\x01\x84a\x01\xA5V[\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 `\x85\xAB\xCEY\xFDc@:~:\x11\r\xEB\xFA\xADU\x14\x9B\x87\x05\x94+\xFE\xA7\x0E\x11\x04\xD30\xAA\xA2dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static BASICONLYSEARCH_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct BasicOnlySearch<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BasicOnlySearch<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BasicOnlySearch<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BasicOnlySearch<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BasicOnlySearch<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BasicOnlySearch))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BasicOnlySearch<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BASICONLYSEARCH_ABI.clone(),
                    client,
                ),
            )
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
                BASICONLYSEARCH_ABI.clone(),
                BASICONLYSEARCH_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `search` (0xffe47662) function
        pub fn search(
            &self,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([255, 228, 118, 98], data)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for BasicOnlySearch<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `search` function with signature `search(bytes)` and selector `0xffe47662`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "search", abi = "search(bytes)")]
    pub struct SearchCall {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `search` function with signature `search(bytes)` and selector `0xffe47662`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SearchReturn {
        pub settle_data: ::ethers::core::types::Bytes,
    }
}
