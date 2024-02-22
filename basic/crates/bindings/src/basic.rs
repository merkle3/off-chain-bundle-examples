pub use basic::*;
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
pub mod basic {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("merkleOffChainBundleSigner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "merkleOffChainBundleSigner",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("search"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("search"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_fees"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                (
                    ::std::borrow::ToOwned::to_owned("settle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("settle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_fees"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("OnChainEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OnChainEvent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fees"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static BASIC_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[Pa\x03H\x80a\0\x1D_9_\xF3\xFE`\x80`@R`\x046\x10a\x003W_5`\xE0\x1C\x80c+\xCC\xC5O\x14a\x007W\x80c9\xC2\xEB\xB9\x14a\0gW\x80c\xDD\xB1\xD7\xF0\x14a\0|W[_\x80\xFD[4\x80\x15a\0BW_\x80\xFD[Pa\0J_\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0za\0u6`\x04a\x01\xD1V[a\0\xA8V[\0[4\x80\x15a\0\x87W_\x80\xFD[Pa\0\x9Ba\0\x966`\x04a\x01\xD1V[a\x01\x87V[`@Qa\0^\x91\x90a\x02\xC9V[3\x15a\x01 W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FCaller must be a verified Merkle`D\x82\x01R\x7F Off-Chain Bundle Signer\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x81\x7F\xBF\x1E+\xF5\xAF>\x9B\xDF\x14(\xE37\xFFM\xF0!\xC1\x15\xDA\xB0\xC0\x15x\xEF}\x82\x8F1\xCA\\]\x94\x82`@Qa\x01P\x91\x90a\x02\xC9V[`@Q\x80\x91\x03\x90\xA2`@QA\x90\x83\x15a\x08\xFC\x02\x90\x84\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x01\x82W=_\x80>=_\xFD[PPPV[`@Q``\x90a\x01\xA6\x90c9\xC2\xEB\xB9`\xE0\x1B\x90\x85\x90\x85\x90` \x01a\x02\xE2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x80`@\x83\x85\x03\x12\x15a\x01\xE2W_\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\0W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x02\x13W_\x80\xFD[\x815\x81\x81\x11\x15a\x02%Wa\x02%a\x01\xBDV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x02MWa\x02Ma\x01\xBDV[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x02eW_\x80\xFD[\x82` \x86\x01` \x83\x017_` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[_\x81Q\x80\x84R_[\x81\x81\x10\x15a\x02\xAAW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x02\x8EV[P_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x02\xDB` \x83\x01\x84a\x02\x86V[\x93\x92PPPV[c\xFF\xFF\xFF\xFF`\xE0\x1B\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R_a\x03\t``\x83\x01\x84a\x02\x86V[\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \x89\x0FhwS\xCB\x19\xBEh\xE1\xB2\x8A\xC4\x8B\x1C\x11m^\x84S\x9C8\x86\x16Dz\x07\xC7\x14\xBE\x05\xA3dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static BASIC_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x003W_5`\xE0\x1C\x80c+\xCC\xC5O\x14a\x007W\x80c9\xC2\xEB\xB9\x14a\0gW\x80c\xDD\xB1\xD7\xF0\x14a\0|W[_\x80\xFD[4\x80\x15a\0BW_\x80\xFD[Pa\0J_\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0za\0u6`\x04a\x01\xD1V[a\0\xA8V[\0[4\x80\x15a\0\x87W_\x80\xFD[Pa\0\x9Ba\0\x966`\x04a\x01\xD1V[a\x01\x87V[`@Qa\0^\x91\x90a\x02\xC9V[3\x15a\x01 W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FCaller must be a verified Merkle`D\x82\x01R\x7F Off-Chain Bundle Signer\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x81\x7F\xBF\x1E+\xF5\xAF>\x9B\xDF\x14(\xE37\xFFM\xF0!\xC1\x15\xDA\xB0\xC0\x15x\xEF}\x82\x8F1\xCA\\]\x94\x82`@Qa\x01P\x91\x90a\x02\xC9V[`@Q\x80\x91\x03\x90\xA2`@QA\x90\x83\x15a\x08\xFC\x02\x90\x84\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x01\x82W=_\x80>=_\xFD[PPPV[`@Q``\x90a\x01\xA6\x90c9\xC2\xEB\xB9`\xE0\x1B\x90\x85\x90\x85\x90` \x01a\x02\xE2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x80`@\x83\x85\x03\x12\x15a\x01\xE2W_\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\0W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x02\x13W_\x80\xFD[\x815\x81\x81\x11\x15a\x02%Wa\x02%a\x01\xBDV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x02MWa\x02Ma\x01\xBDV[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x02eW_\x80\xFD[\x82` \x86\x01` \x83\x017_` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[_\x81Q\x80\x84R_[\x81\x81\x10\x15a\x02\xAAW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x02\x8EV[P_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x02\xDB` \x83\x01\x84a\x02\x86V[\x93\x92PPPV[c\xFF\xFF\xFF\xFF`\xE0\x1B\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R_a\x03\t``\x83\x01\x84a\x02\x86V[\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \x89\x0FhwS\xCB\x19\xBEh\xE1\xB2\x8A\xC4\x8B\x1C\x11m^\x84S\x9C8\x86\x16Dz\x07\xC7\x14\xBE\x05\xA3dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static BASIC_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Basic<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Basic<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Basic<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Basic<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Basic<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Basic)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Basic<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BASIC_ABI.clone(),
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
                BASIC_ABI.clone(),
                BASIC_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `merkleOffChainBundleSigner` (0x2bccc54f) function
        pub fn merkle_off_chain_bundle_signer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([43, 204, 197, 79], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `search` (0xddb1d7f0) function
        pub fn search(
            &self,
            fees: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([221, 177, 215, 240], (fees, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `settle` (0x39c2ebb9) function
        pub fn settle(
            &self,
            fees: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 194, 235, 185], (fees, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `OnChainEvent` event
        pub fn on_chain_event_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OnChainEventFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OnChainEventFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Basic<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "OnChainEvent", abi = "OnChainEvent(uint256,bytes)")]
    pub struct OnChainEventFilter {
        #[ethevent(indexed)]
        pub fees: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `merkleOffChainBundleSigner` function with signature `merkleOffChainBundleSigner()` and selector `0x2bccc54f`
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
        Hash
    )]
    #[ethcall(name = "merkleOffChainBundleSigner", abi = "merkleOffChainBundleSigner()")]
    pub struct MerkleOffChainBundleSignerCall;
    ///Container type for all input parameters for the `search` function with signature `search(uint256,bytes)` and selector `0xddb1d7f0`
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
        Hash
    )]
    #[ethcall(name = "search", abi = "search(uint256,bytes)")]
    pub struct SearchCall {
        pub fees: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `settle` function with signature `settle(uint256,bytes)` and selector `0x39c2ebb9`
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
        Hash
    )]
    #[ethcall(name = "settle", abi = "settle(uint256,bytes)")]
    pub struct SettleCall {
        pub fees: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum BasicCalls {
        MerkleOffChainBundleSigner(MerkleOffChainBundleSignerCall),
        Search(SearchCall),
        Settle(SettleCall),
    }
    impl ::ethers::core::abi::AbiDecode for BasicCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <MerkleOffChainBundleSignerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MerkleOffChainBundleSigner(decoded));
            }
            if let Ok(decoded) = <SearchCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Search(decoded));
            }
            if let Ok(decoded) = <SettleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Settle(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BasicCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MerkleOffChainBundleSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Search(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Settle(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for BasicCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MerkleOffChainBundleSigner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Search(element) => ::core::fmt::Display::fmt(element, f),
                Self::Settle(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MerkleOffChainBundleSignerCall> for BasicCalls {
        fn from(value: MerkleOffChainBundleSignerCall) -> Self {
            Self::MerkleOffChainBundleSigner(value)
        }
    }
    impl ::core::convert::From<SearchCall> for BasicCalls {
        fn from(value: SearchCall) -> Self {
            Self::Search(value)
        }
    }
    impl ::core::convert::From<SettleCall> for BasicCalls {
        fn from(value: SettleCall) -> Self {
            Self::Settle(value)
        }
    }
    ///Container type for all return fields from the `merkleOffChainBundleSigner` function with signature `merkleOffChainBundleSigner()` and selector `0x2bccc54f`
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
        Hash
    )]
    pub struct MerkleOffChainBundleSignerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `search` function with signature `search(uint256,bytes)` and selector `0xddb1d7f0`
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
        Hash
    )]
    pub struct SearchReturn {
        pub settle_data: ::ethers::core::types::Bytes,
    }
}
