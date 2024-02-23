pub use vm::*;
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
pub mod vm {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("accesses"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("accesses"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("readSlots"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("writeSlots"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("activeFork"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("activeFork"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("forkId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addr"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addr"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("privateKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("keyAddr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allowCheatcodes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowCheatcodes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("assume"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("assume"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("condition"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("breakpoint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("breakpoint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("char"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("breakpoint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("char"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("broadcast"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("broadcast"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("broadcast"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("broadcast"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("privateKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("chainId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("chainId"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("clearMockedCalls"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clearMockedCalls"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("closeFile"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("closeFile"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("coinbase"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("coinbase"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newCoinbase"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("computeCreate2Address"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeCreate2Address",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initCodeHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeCreate2Address",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initCodeHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deployer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("computeCreateAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeCreateAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deployer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("copyFile"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("copyFile"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("copied"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createDir"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createDir"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recursive"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createFork"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createFork"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("urlOrAlias"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("forkId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createFork"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("urlOrAlias"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("forkId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createFork"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("urlOrAlias"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("forkId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createSelectFork"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createSelectFork"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("urlOrAlias"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("forkId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createSelectFork"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("urlOrAlias"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("forkId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createSelectFork"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("urlOrAlias"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("forkId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createWallet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createWallet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("walletLabel"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("wallet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct VmSafe.Wallet"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createWallet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("privateKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("wallet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct VmSafe.Wallet"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createWallet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("privateKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("walletLabel"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("wallet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct VmSafe.Wallet"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newBalance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deleteSnapshot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deleteSnapshot"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("snapshotId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deleteSnapshots"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deleteSnapshots"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deriveKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deriveKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mnemonic"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("derivationPath"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("language"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("privateKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deriveKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mnemonic"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("language"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("privateKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deriveKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mnemonic"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("privateKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deriveKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mnemonic"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("derivationPath"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("privateKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("difficulty"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("difficulty"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newDifficulty"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dumpState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dumpState"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pathToStateJson"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("envAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("envBool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envBool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envBool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("envBytes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envBytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envBytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("envBytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envBytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envBytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("envInt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envInt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envInt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("envOr"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envOr"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("defaultValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envOr"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("defaultValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envOr"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("defaultValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envOr"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("defaultValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envOr"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("defaultValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envOr"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("defaultValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envOr"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("defaultValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envOr"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("defaultValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envOr"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("defaultValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envOr"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("defaultValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envOr"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("defaultValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envOr"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("defaultValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envOr"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("defaultValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envOr"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("defaultValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("envString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("envUint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envUint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envUint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delim"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("etch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("etch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newRuntimeBytecode",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("eth_getLogs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("eth_getLogs"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fromBlock"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("toBlock"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("topics"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("logs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct VmSafe.EthGetLogs[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("exists"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exists"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("expectCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("expectCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("expectCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("count"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("expectCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("count"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("expectCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("expectCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("count"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("expectCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("expectCallMinGas"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("expectCallMinGas"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("expectCallMinGas"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("count"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("expectEmit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("expectEmit"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("expectEmit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("checkTopic1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("checkTopic2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("checkTopic3"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("checkData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("expectEmit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("checkTopic1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("checkTopic2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("checkTopic3"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("checkData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("emitter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("expectEmit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("emitter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("expectRevert"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("expectRevert"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("revertData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("expectRevert"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("revertData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("expectRevert"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("expectSafeMemory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("expectSafeMemory"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("min"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("max"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("expectSafeMemoryCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "expectSafeMemoryCall",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("min"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("max"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("fee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newBasefee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ffi"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ffi"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("commandInput"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("fsMetadata"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fsMetadata"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("metadata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct VmSafe.FsMetadata"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBlockNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBlockNumber"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("height"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBlockTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBlockTimestamp"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getCode"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("artifactPath"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("creationBytecode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDeployedCode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getDeployedCode"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("artifactPath"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("runtimeBytecode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLabel"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getLabel"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("currentLabel"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMappingKeyAndParentOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getMappingKeyAndParentOf",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("elementSlot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("found"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("parent"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMappingLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMappingLength"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mappingSlot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("length"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMappingSlotAt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMappingSlotAt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mappingSlot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("idx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNonce"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNonce"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("wallet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct VmSafe.Wallet"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRecordedLogs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRecordedLogs"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("logs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct VmSafe.Log[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isDir"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isDir"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isFile"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isFile"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isPersistent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isPersistent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("persistent"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("keyExists"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("keyExists"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("label"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("label"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newLabel"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("load"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("load"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("slot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("loadAllocs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("loadAllocs"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pathToAllocsJson"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("makePersistent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("makePersistent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("accounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("makePersistent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("makePersistent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("makePersistent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mockCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mockCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("returnData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mockCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("returnData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mockCallRevert"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mockCallRevert"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("revertData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mockCallRevert"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("revertData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("parseAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stringifiedValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("parsedValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("parseBool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseBool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stringifiedValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("parsedValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("parseBytes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseBytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stringifiedValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("parsedValue"),
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
                    ::std::borrow::ToOwned::to_owned("parseBytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseBytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stringifiedValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("parsedValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("parseInt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseInt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stringifiedValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("parsedValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("parseJson"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseJson"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("abiEncodedData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseJson"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("abiEncodedData"),
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
                    ::std::borrow::ToOwned::to_owned("parseJsonAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseJsonAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("parseJsonAddressArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "parseJsonAddressArray",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("parseJsonBool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseJsonBool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("parseJsonBoolArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseJsonBoolArray"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("parseJsonBytes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseJsonBytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("parseJsonBytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseJsonBytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("parseJsonBytes32Array"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "parseJsonBytes32Array",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("parseJsonBytesArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "parseJsonBytesArray",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("parseJsonInt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseJsonInt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("parseJsonIntArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseJsonIntArray"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("parseJsonKeys"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseJsonKeys"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("keys"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("parseJsonString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseJsonString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("parseJsonStringArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "parseJsonStringArray",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("parseJsonUint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseJsonUint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("parseJsonUintArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseJsonUintArray"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("parseUint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseUint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stringifiedValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("parsedValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pauseGasMetering"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pauseGasMetering"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("prank"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("prank"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgSender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txOrigin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("prank"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgSender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("prevrandao"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("prevrandao"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPrevrandao"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("projectRoot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("projectRoot"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("readCallers"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("readCallers"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callerMode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum VmSafe.CallerMode"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgSender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txOrigin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("readDir"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("readDir"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxDepth"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("entries"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct VmSafe.DirEntry[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("readDir"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxDepth"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("followLinks"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("entries"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct VmSafe.DirEntry[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("readDir"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("entries"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct VmSafe.DirEntry[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("readFile"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("readFile"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("readFileBinary"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("readFileBinary"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("readLine"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("readLine"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("line"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("readLink"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("readLink"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("linkPath"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetPath"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("record"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("record"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("recordLogs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("recordLogs"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rememberKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rememberKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("privateKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("keyAddr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeDir"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeDir"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recursive"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeFile"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeFile"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("resetNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("resetNonce"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("resumeGasMetering"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("resumeGasMetering"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("revertTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revertTo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("snapshotId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("revertToAndDelete"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revertToAndDelete"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("snapshotId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("revokePersistent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokePersistent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("accounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokePersistent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("roll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("roll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newHeight"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rollFork"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rollFork"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rollFork"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("forkId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rollFork"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rollFork"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("forkId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rpc"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rpc"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("method"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rpcUrl"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rpcUrl"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rpcAlias"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rpcUrlStructs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rpcUrlStructs"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("urls"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct VmSafe.Rpc[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rpcUrls"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rpcUrls"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("urls"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                ),
                                                2usize,
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[2][]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("selectFork"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("selectFork"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("forkId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("serializeAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("objectKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valueKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("objectKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valueKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("serializeBool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeBool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("objectKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valueKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeBool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("objectKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valueKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("serializeBytes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeBytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("objectKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valueKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeBytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("objectKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valueKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("serializeBytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeBytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("objectKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valueKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeBytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("objectKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valueKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("serializeInt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeInt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("objectKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valueKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeInt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("objectKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valueKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("serializeJson"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeJson"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("objectKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("serializeString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("objectKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valueKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("objectKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valueKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("serializeUint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeUint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("objectKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valueKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeUint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("objectKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valueKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setEnv"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setEnv"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setNonce"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setNonceUnsafe"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setNonceUnsafe"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sign"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sign"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("wallet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct VmSafe.Wallet"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("digest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sign"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("privateKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("digest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signP256"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("signP256"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("privateKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("digest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("skip"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("skip"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("skipTest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sleep"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sleep"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("duration"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("snapshot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("snapshot"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("snapshotId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("startBroadcast"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("startBroadcast"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("startBroadcast"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("startBroadcast"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("privateKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("startMappingRecording"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "startMappingRecording",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("startPrank"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("startPrank"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgSender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("startPrank"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgSender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txOrigin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("startStateDiffRecording"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "startStateDiffRecording",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stopAndReturnStateDiff"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "stopAndReturnStateDiff",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("accountAccesses"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct VmSafe.AccountAccess[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stopBroadcast"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stopBroadcast"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stopMappingRecording"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "stopMappingRecording",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stopPrank"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stopPrank"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("store"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("store"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("slot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("toBase64"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("toBase64"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("toBase64"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("toBase64URL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("toBase64URL"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("toBase64URL"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("toString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("toString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stringifiedValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("toString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stringifiedValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("toString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stringifiedValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("toString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stringifiedValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("toString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stringifiedValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("toString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stringifiedValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transact"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transact"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("forkId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transact"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tryFfi"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tryFfi"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("commandInput"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct VmSafe.FfiResult"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("txGasPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("txGasPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newGasPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unixTime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unixTime"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("milliseconds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("warp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("warp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("writeFile"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("writeFile"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("writeFileBinary"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("writeFileBinary"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("writeJson"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("writeJson"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valueKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("writeJson"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("json"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("writeLine"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("writeLine"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
    pub static VM_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct Vm<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Vm<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Vm<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Vm<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Vm<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Vm)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Vm<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(address.into(), VM_ABI.clone(), client),
            )
        }
        ///Calls the contract's `accesses` (0x65bc9481) function
        pub fn accesses(
            &self,
            target: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<[u8; 32]>, ::std::vec::Vec<[u8; 32]>),
        > {
            self.0
                .method_hash([101, 188, 148, 129], target)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `activeFork` (0x2f103f22) function
        pub fn active_fork(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([47, 16, 63, 34], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addr` (0xffa18649) function
        pub fn addr(
            &self,
            private_key: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([255, 161, 134, 73], private_key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowCheatcodes` (0xea060291) function
        pub fn allow_cheatcodes(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 6, 2, 145], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assume` (0x4c63e562) function
        pub fn assume(
            &self,
            condition: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 99, 229, 98], condition)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `breakpoint` (0xf0259e92) function
        pub fn breakpoint(
            &self,
            char: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 37, 158, 146], char)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `breakpoint` (0xf7d39a8d) function
        pub fn breakpoint_with_value(
            &self,
            char: ::std::string::String,
            value: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([247, 211, 154, 141], (char, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `broadcast` (0xafc98040) function
        pub fn broadcast(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([175, 201, 128, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `broadcast` (0xe6962cdb) function
        pub fn broadcast_with_signer(
            &self,
            signer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 150, 44, 219], signer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `broadcast` (0xf67a965b) function
        pub fn broadcast_with_private_key(
            &self,
            private_key: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 122, 150, 91], private_key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `chainId` (0x4049ddd2) function
        pub fn chain_id(
            &self,
            new_chain_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 73, 221, 210], new_chain_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clearMockedCalls` (0x3fdf4e15) function
        pub fn clear_mocked_calls(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 223, 78, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `closeFile` (0x48c3241f) function
        pub fn close_file(
            &self,
            path: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 195, 36, 31], path)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `coinbase` (0xff483c54) function
        pub fn coinbase(
            &self,
            new_coinbase: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([255, 72, 60, 84], new_coinbase)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeCreate2Address` (0x890c283b) function
        pub fn compute_create_2_address(
            &self,
            salt: [u8; 32],
            init_code_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([137, 12, 40, 59], (salt, init_code_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeCreate2Address` (0xd323826a) function
        pub fn compute_create_2_address_with_salt_and_init_code_hash(
            &self,
            salt: [u8; 32],
            init_code_hash: [u8; 32],
            deployer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([211, 35, 130, 106], (salt, init_code_hash, deployer))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeCreateAddress` (0x74637a7a) function
        pub fn compute_create_address(
            &self,
            deployer: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([116, 99, 122, 122], (deployer, nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `copyFile` (0xa54a87d8) function
        pub fn copy_file(
            &self,
            from: ::std::string::String,
            to: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([165, 74, 135, 216], (from, to))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createDir` (0x168b64d3) function
        pub fn create_dir(
            &self,
            path: ::std::string::String,
            recursive: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 139, 100, 211], (path, recursive))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createFork` (0x31ba3498) function
        pub fn create_fork(
            &self,
            url_or_alias: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([49, 186, 52, 152], url_or_alias)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createFork` (0x6ba3ba2b) function
        pub fn create_fork_with_block_number(
            &self,
            url_or_alias: ::std::string::String,
            block_number: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([107, 163, 186, 43], (url_or_alias, block_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createFork` (0x7ca29682) function
        pub fn create_fork_with_tx_hash(
            &self,
            url_or_alias: ::std::string::String,
            tx_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([124, 162, 150, 130], (url_or_alias, tx_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createSelectFork` (0x71ee464d) function
        pub fn create_select_fork_with_block_number(
            &self,
            url_or_alias: ::std::string::String,
            block_number: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([113, 238, 70, 77], (url_or_alias, block_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createSelectFork` (0x84d52b7a) function
        pub fn create_select_fork_with_tx_hash(
            &self,
            url_or_alias: ::std::string::String,
            tx_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([132, 213, 43, 122], (url_or_alias, tx_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createSelectFork` (0x98680034) function
        pub fn create_select_fork(
            &self,
            url_or_alias: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([152, 104, 0, 52], url_or_alias)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createWallet` (0x7404f1d2) function
        pub fn create_wallet_0(
            &self,
            wallet_label: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, Wallet> {
            self.0
                .method_hash([116, 4, 241, 210], wallet_label)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createWallet` (0x7a675bb6) function
        pub fn create_wallet_1(
            &self,
            private_key: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Wallet> {
            self.0
                .method_hash([122, 103, 91, 182], private_key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createWallet` (0xed7c5462) function
        pub fn create_wallet_2(
            &self,
            private_key: ::ethers::core::types::U256,
            wallet_label: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, Wallet> {
            self.0
                .method_hash([237, 124, 84, 98], (private_key, wallet_label))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deal` (0xc88a5e6d) function
        pub fn deal(
            &self,
            account: ::ethers::core::types::Address,
            new_balance: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([200, 138, 94, 109], (account, new_balance))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deleteSnapshot` (0xa6368557) function
        pub fn delete_snapshot(
            &self,
            snapshot_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([166, 54, 133, 87], snapshot_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deleteSnapshots` (0x421ae469) function
        pub fn delete_snapshots(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 26, 228, 105], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deriveKey` (0x29233b1f) function
        pub fn derive_key_3(
            &self,
            mnemonic: ::std::string::String,
            derivation_path: ::std::string::String,
            index: u32,
            language: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [41, 35, 59, 31],
                    (mnemonic, derivation_path, index, language),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deriveKey` (0x32c8176d) function
        pub fn derive_key_1(
            &self,
            mnemonic: ::std::string::String,
            index: u32,
            language: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([50, 200, 23, 109], (mnemonic, index, language))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deriveKey` (0x6229498b) function
        pub fn derive_key_0(
            &self,
            mnemonic: ::std::string::String,
            index: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([98, 41, 73, 139], (mnemonic, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deriveKey` (0x6bcb2c1b) function
        pub fn derive_key_2(
            &self,
            mnemonic: ::std::string::String,
            derivation_path: ::std::string::String,
            index: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([107, 203, 44, 27], (mnemonic, derivation_path, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `difficulty` (0x46cc92d9) function
        pub fn difficulty(
            &self,
            new_difficulty: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 204, 146, 217], new_difficulty)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dumpState` (0x709ecd3f) function
        pub fn dump_state(
            &self,
            path_to_state_json: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 158, 205, 63], path_to_state_json)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envAddress` (0x350d56bf) function
        pub fn env_address(
            &self,
            name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([53, 13, 86, 191], name)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envAddress` (0xad31b9fa) function
        pub fn env_address_with_delim(
            &self,
            name: ::std::string::String,
            delim: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([173, 49, 185, 250], (name, delim))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envBool` (0x7ed1ec7d) function
        pub fn env_bool(
            &self,
            name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([126, 209, 236, 125], name)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envBool` (0xaaaddeaf) function
        pub fn env_bool_with_delim(
            &self,
            name: ::std::string::String,
            delim: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<bool>> {
            self.0
                .method_hash([170, 173, 222, 175], (name, delim))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envBytes` (0x4d7baf06) function
        pub fn env_bytes(
            &self,
            name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([77, 123, 175, 6], name)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envBytes` (0xddc2651b) function
        pub fn env_bytes_with_delim(
            &self,
            name: ::std::string::String,
            delim: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([221, 194, 101, 27], (name, delim))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envBytes32` (0x5af231c1) function
        pub fn env_bytes_32_with_delim(
            &self,
            name: ::std::string::String,
            delim: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([90, 242, 49, 193], (name, delim))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envBytes32` (0x97949042) function
        pub fn env_bytes_32(
            &self,
            name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([151, 148, 144, 66], name)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envInt` (0x42181150) function
        pub fn env_int_with_delim(
            &self,
            name: ::std::string::String,
            delim: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::I256>,
        > {
            self.0
                .method_hash([66, 24, 17, 80], (name, delim))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envInt` (0x892a0c61) function
        pub fn env_int(
            &self,
            name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([137, 42, 12, 97], name)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0x2281f367) function
        pub fn env_or_7(
            &self,
            name: ::std::string::String,
            delim: ::std::string::String,
            default_value: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([34, 129, 243, 103], (name, delim, default_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0x4700d74b) function
        pub fn env_or_8(
            &self,
            name: ::std::string::String,
            delim: ::std::string::String,
            default_value: ::std::vec::Vec<::ethers::core::types::I256>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::I256>,
        > {
            self.0
                .method_hash([71, 0, 215, 75], (name, delim, default_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0x4777f3cf) function
        pub fn env_or_0(
            &self,
            name: ::std::string::String,
            default_value: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([71, 119, 243, 207], (name, default_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0x561fe540) function
        pub fn env_or_1(
            &self,
            name: ::std::string::String,
            default_value: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([86, 31, 229, 64], (name, default_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0x5e97348f) function
        pub fn env_or_2(
            &self,
            name: ::std::string::String,
            default_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([94, 151, 52, 143], (name, default_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0x64bc3e64) function
        pub fn env_or_9(
            &self,
            name: ::std::string::String,
            delim: ::std::string::String,
            default_value: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([100, 188, 62, 100], (name, delim, default_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0x74318528) function
        pub fn env_or_10(
            &self,
            name: ::std::string::String,
            delim: ::std::string::String,
            default_value: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([116, 49, 133, 40], (name, delim, default_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0x859216bc) function
        pub fn env_or_11(
            &self,
            name: ::std::string::String,
            delim: ::std::string::String,
            default_value: ::std::vec::Vec<::std::string::String>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([133, 146, 22, 188], (name, delim, default_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0xb3e47705) function
        pub fn env_or_3(
            &self,
            name: ::std::string::String,
            default_value: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([179, 228, 119, 5], (name, default_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0xb4a85892) function
        pub fn env_or_4(
            &self,
            name: ::std::string::String,
            default_value: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([180, 168, 88, 146], (name, default_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0xbbcb713e) function
        pub fn env_or_5(
            &self,
            name: ::std::string::String,
            default_value: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([187, 203, 113, 62], (name, default_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0xc74e9deb) function
        pub fn env_or_12(
            &self,
            name: ::std::string::String,
            delim: ::std::string::String,
            default_value: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([199, 78, 157, 235], (name, delim, default_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0xd145736c) function
        pub fn env_or_6(
            &self,
            name: ::std::string::String,
            default_value: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([209, 69, 115, 108], (name, default_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0xeb85e83b) function
        pub fn env_or_13(
            &self,
            name: ::std::string::String,
            delim: ::std::string::String,
            default_value: ::std::vec::Vec<bool>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<bool>> {
            self.0
                .method_hash([235, 133, 232, 59], (name, delim, default_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envString` (0x14b02bc9) function
        pub fn env_string_with_delim(
            &self,
            name: ::std::string::String,
            delim: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([20, 176, 43, 201], (name, delim))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envString` (0xf877cb19) function
        pub fn env_string(
            &self,
            name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([248, 119, 203, 25], name)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envUint` (0xc1978d1f) function
        pub fn env_uint(
            &self,
            name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([193, 151, 141, 31], name)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envUint` (0xf3dec099) function
        pub fn env_uint_with_delim(
            &self,
            name: ::std::string::String,
            delim: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([243, 222, 192, 153], (name, delim))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `etch` (0xb4d6c782) function
        pub fn etch(
            &self,
            target: ::ethers::core::types::Address,
            new_runtime_bytecode: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([180, 214, 199, 130], (target, new_runtime_bytecode))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eth_getLogs` (0x35e1349b) function
        pub fn eth_get_logs(
            &self,
            from_block: ::ethers::core::types::U256,
            to_block: ::ethers::core::types::U256,
            target: ::ethers::core::types::Address,
            topics: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<EthGetLogs>> {
            self.0
                .method_hash([53, 225, 52, 155], (from_block, to_block, target, topics))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exists` (0x261a323e) function
        pub fn exists(
            &self,
            path: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([38, 26, 50, 62], path)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectCall` (0x23361207) function
        pub fn expect_call_3(
            &self,
            callee: ::ethers::core::types::Address,
            msg_value: ::ethers::core::types::U256,
            gas: u64,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 54, 18, 7], (callee, msg_value, gas, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectCall` (0x65b7b7cc) function
        pub fn expect_call_5(
            &self,
            callee: ::ethers::core::types::Address,
            msg_value: ::ethers::core::types::U256,
            gas: u64,
            data: ::ethers::core::types::Bytes,
            count: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([101, 183, 183, 204], (callee, msg_value, gas, data, count))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectCall` (0xa2b1a1ae) function
        pub fn expect_call_4(
            &self,
            callee: ::ethers::core::types::Address,
            msg_value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            count: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 177, 161, 174], (callee, msg_value, data, count))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectCall` (0xbd6af434) function
        pub fn expect_call_0(
            &self,
            callee: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([189, 106, 244, 52], (callee, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectCall` (0xc1adbbff) function
        pub fn expect_call_1(
            &self,
            callee: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
            count: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([193, 173, 187, 255], (callee, data, count))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectCall` (0xf30c7ba3) function
        pub fn expect_call_2(
            &self,
            callee: ::ethers::core::types::Address,
            msg_value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 12, 123, 163], (callee, msg_value, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectCallMinGas` (0x08e4e116) function
        pub fn expect_call_min_gas(
            &self,
            callee: ::ethers::core::types::Address,
            msg_value: ::ethers::core::types::U256,
            min_gas: u64,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([8, 228, 225, 22], (callee, msg_value, min_gas, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectCallMinGas` (0xe13a1834) function
        pub fn expect_call_min_gas_with_callee_and_msg_value_and_min_gas_and_data(
            &self,
            callee: ::ethers::core::types::Address,
            msg_value: ::ethers::core::types::U256,
            min_gas: u64,
            data: ::ethers::core::types::Bytes,
            count: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [225, 58, 24, 52],
                    (callee, msg_value, min_gas, data, count),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectEmit` (0x440ed10d) function
        pub fn expect_emit_0(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 14, 209, 13], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectEmit` (0x491cc7c2) function
        pub fn expect_emit_2(
            &self,
            check_topic_1: bool,
            check_topic_2: bool,
            check_topic_3: bool,
            check_data: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [73, 28, 199, 194],
                    (check_topic_1, check_topic_2, check_topic_3, check_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectEmit` (0x81bad6f3) function
        pub fn expect_emit_3(
            &self,
            check_topic_1: bool,
            check_topic_2: bool,
            check_topic_3: bool,
            check_data: bool,
            emitter: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [129, 186, 214, 243],
                    (check_topic_1, check_topic_2, check_topic_3, check_data, emitter),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectEmit` (0x86b9620d) function
        pub fn expect_emit_1(
            &self,
            emitter: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([134, 185, 98, 13], emitter)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectRevert` (0xc31eb0e0) function
        pub fn expect_revert_1(
            &self,
            revert_data: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 30, 176, 224], revert_data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectRevert` (0xf28dceb3) function
        pub fn expect_revert_2(
            &self,
            revert_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 141, 206, 179], revert_data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectRevert` (0xf4844814) function
        pub fn expect_revert_0(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([244, 132, 72, 20], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectSafeMemory` (0x6d016688) function
        pub fn expect_safe_memory(
            &self,
            min: u64,
            max: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 1, 102, 136], (min, max))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectSafeMemoryCall` (0x05838bf4) function
        pub fn expect_safe_memory_call(
            &self,
            min: u64,
            max: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 131, 139, 244], (min, max))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fee` (0x39b37ab0) function
        pub fn fee(
            &self,
            new_basefee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 179, 122, 176], new_basefee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ffi` (0x89160467) function
        pub fn ffi(
            &self,
            command_input: ::std::vec::Vec<::std::string::String>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([137, 22, 4, 103], command_input)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fsMetadata` (0xaf368a08) function
        pub fn fs_metadata(
            &self,
            path: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, FsMetadata> {
            self.0
                .method_hash([175, 54, 138, 8], path)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBlockNumber` (0x42cbb15c) function
        pub fn get_block_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([66, 203, 177, 92], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBlockTimestamp` (0x796b89b9) function
        pub fn get_block_timestamp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([121, 107, 137, 185], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCode` (0x8d1cc925) function
        pub fn get_code(
            &self,
            artifact_path: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([141, 28, 201, 37], artifact_path)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDeployedCode` (0x3ebf73b4) function
        pub fn get_deployed_code(
            &self,
            artifact_path: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([62, 191, 115, 180], artifact_path)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLabel` (0x28a249b0) function
        pub fn get_label(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([40, 162, 73, 176], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMappingKeyAndParentOf` (0x876e24e6) function
        pub fn get_mapping_key_and_parent_of(
            &self,
            target: ::ethers::core::types::Address,
            element_slot: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, [u8; 32], [u8; 32])> {
            self.0
                .method_hash([135, 110, 36, 230], (target, element_slot))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMappingLength` (0x2f2fd63f) function
        pub fn get_mapping_length(
            &self,
            target: ::ethers::core::types::Address,
            mapping_slot: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([47, 47, 214, 63], (target, mapping_slot))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMappingSlotAt` (0xebc73ab4) function
        pub fn get_mapping_slot_at(
            &self,
            target: ::ethers::core::types::Address,
            mapping_slot: [u8; 32],
            idx: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([235, 199, 58, 180], (target, mapping_slot, idx))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNonce` (0x2d0335ab) function
        pub fn get_nonce(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([45, 3, 53, 171], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNonce` (0xa5748aad) function
        pub fn get_nonce_with_wallet(
            &self,
            wallet: Wallet,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([165, 116, 138, 173], (wallet,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRecordedLogs` (0x191553a4) function
        pub fn get_recorded_logs(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Log>> {
            self.0
                .method_hash([25, 21, 83, 164], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isDir` (0x7d15d019) function
        pub fn is_dir(
            &self,
            path: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([125, 21, 208, 25], path)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isFile` (0xe0eb04d4) function
        pub fn is_file(
            &self,
            path: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([224, 235, 4, 212], path)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPersistent` (0xd92d8efd) function
        pub fn is_persistent(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([217, 45, 142, 253], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `keyExists` (0x528a683c) function
        pub fn key_exists(
            &self,
            json: ::std::string::String,
            key: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([82, 138, 104, 60], (json, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `label` (0xc657c718) function
        pub fn label(
            &self,
            account: ::ethers::core::types::Address,
            new_label: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 87, 199, 24], (account, new_label))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `load` (0x667f9d70) function
        pub fn load(
            &self,
            target: ::ethers::core::types::Address,
            slot: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([102, 127, 157, 112], (target, slot))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `loadAllocs` (0xb3a056d7) function
        pub fn load_allocs(
            &self,
            path_to_allocs_json: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([179, 160, 86, 215], path_to_allocs_json)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `makePersistent` (0x1d9e269e) function
        pub fn make_persistent_0(
            &self,
            accounts: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 158, 38, 158], accounts)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `makePersistent` (0x4074e0a8) function
        pub fn make_persistent_2(
            &self,
            account_0: ::ethers::core::types::Address,
            account_1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 116, 224, 168], (account_0, account_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `makePersistent` (0x57e22dde) function
        pub fn make_persistent_1(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 226, 45, 222], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `makePersistent` (0xefb77a75) function
        pub fn make_persistent_3(
            &self,
            account_0: ::ethers::core::types::Address,
            account_1: ::ethers::core::types::Address,
            account_2: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 183, 122, 117], (account_0, account_1, account_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mockCall` (0x81409b91) function
        pub fn mock_call_with_callee_and_msg_value_and_data(
            &self,
            callee: ::ethers::core::types::Address,
            msg_value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            return_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 64, 155, 145], (callee, msg_value, data, return_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mockCall` (0xb96213e4) function
        pub fn mock_call(
            &self,
            callee: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
            return_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 98, 19, 228], (callee, data, return_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mockCallRevert` (0xd23cd037) function
        pub fn mock_call_revert_with_callee_and_msg_value_and_data(
            &self,
            callee: ::ethers::core::types::Address,
            msg_value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            revert_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 60, 208, 55], (callee, msg_value, data, revert_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mockCallRevert` (0xdbaad147) function
        pub fn mock_call_revert(
            &self,
            callee: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
            revert_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 170, 209, 71], (callee, data, revert_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseAddress` (0xc6ce059d) function
        pub fn parse_address(
            &self,
            stringified_value: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([198, 206, 5, 157], stringified_value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseBool` (0x974ef924) function
        pub fn parse_bool(
            &self,
            stringified_value: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([151, 78, 249, 36], stringified_value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseBytes` (0x8f5d232d) function
        pub fn parse_bytes(
            &self,
            stringified_value: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([143, 93, 35, 45], stringified_value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseBytes32` (0x087e6e81) function
        pub fn parse_bytes_32(
            &self,
            stringified_value: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([8, 126, 110, 129], stringified_value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseInt` (0x42346c5e) function
        pub fn parse_int(
            &self,
            stringified_value: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([66, 52, 108, 94], stringified_value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJson` (0x6a82600a) function
        pub fn parse_json(
            &self,
            json: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([106, 130, 96, 10], json)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJson` (0x85940ef1) function
        pub fn parse_json_with_key(
            &self,
            json: ::std::string::String,
            key: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([133, 148, 14, 241], (json, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonAddress` (0x1e19e657) function
        pub fn parse_json_address(
            &self,
            json: ::std::string::String,
            key: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([30, 25, 230, 87], (json, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonAddressArray` (0x2fce7883) function
        pub fn parse_json_address_array(
            &self,
            json: ::std::string::String,
            key: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([47, 206, 120, 131], (json, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonBool` (0x9f86dc91) function
        pub fn parse_json_bool(
            &self,
            json: ::std::string::String,
            key: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([159, 134, 220, 145], (json, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonBoolArray` (0x91f3b94f) function
        pub fn parse_json_bool_array(
            &self,
            json: ::std::string::String,
            key: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<bool>> {
            self.0
                .method_hash([145, 243, 185, 79], (json, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonBytes` (0xfd921be8) function
        pub fn parse_json_bytes(
            &self,
            json: ::std::string::String,
            key: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([253, 146, 27, 232], (json, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonBytes32` (0x1777e59d) function
        pub fn parse_json_bytes_32(
            &self,
            json: ::std::string::String,
            key: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([23, 119, 229, 157], (json, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonBytes32Array` (0x91c75bc3) function
        pub fn parse_json_bytes_32_array(
            &self,
            json: ::std::string::String,
            key: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([145, 199, 91, 195], (json, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonBytesArray` (0x6631aa99) function
        pub fn parse_json_bytes_array(
            &self,
            json: ::std::string::String,
            key: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([102, 49, 170, 153], (json, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonInt` (0x7b048ccd) function
        pub fn parse_json_int(
            &self,
            json: ::std::string::String,
            key: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([123, 4, 140, 205], (json, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonIntArray` (0x9983c28a) function
        pub fn parse_json_int_array(
            &self,
            json: ::std::string::String,
            key: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::I256>,
        > {
            self.0
                .method_hash([153, 131, 194, 138], (json, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonKeys` (0x213e4198) function
        pub fn parse_json_keys(
            &self,
            json: ::std::string::String,
            key: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([33, 62, 65, 152], (json, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonString` (0x49c4fac8) function
        pub fn parse_json_string(
            &self,
            json: ::std::string::String,
            key: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([73, 196, 250, 200], (json, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonStringArray` (0x498fdcf4) function
        pub fn parse_json_string_array(
            &self,
            json: ::std::string::String,
            key: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([73, 143, 220, 244], (json, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonUint` (0xaddde2b6) function
        pub fn parse_json_uint(
            &self,
            json: ::std::string::String,
            key: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([173, 221, 226, 182], (json, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonUintArray` (0x522074ab) function
        pub fn parse_json_uint_array(
            &self,
            json: ::std::string::String,
            key: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([82, 32, 116, 171], (json, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseUint` (0xfa91454d) function
        pub fn parse_uint(
            &self,
            stringified_value: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([250, 145, 69, 77], stringified_value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pauseGasMetering` (0xd1a5b36f) function
        pub fn pause_gas_metering(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([209, 165, 179, 111], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prank` (0x47e50cce) function
        pub fn prank_with_tx_origin(
            &self,
            msg_sender: ::ethers::core::types::Address,
            tx_origin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 229, 12, 206], (msg_sender, tx_origin))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prank` (0xca669fa7) function
        pub fn prank(
            &self,
            msg_sender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 102, 159, 167], msg_sender)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prevrandao` (0x3b925549) function
        pub fn prevrandao(
            &self,
            new_prevrandao: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 146, 85, 73], new_prevrandao)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `projectRoot` (0xd930a0e6) function
        pub fn project_root(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([217, 48, 160, 230], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `readCallers` (0x4ad0bac9) function
        pub fn read_callers(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u8, ::ethers::core::types::Address, ::ethers::core::types::Address),
        > {
            self.0
                .method_hash([74, 208, 186, 201], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `readDir` (0x1497876c) function
        pub fn read_dir_1(
            &self,
            path: ::std::string::String,
            max_depth: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<DirEntry>> {
            self.0
                .method_hash([20, 151, 135, 108], (path, max_depth))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `readDir` (0x8102d70d) function
        pub fn read_dir_2(
            &self,
            path: ::std::string::String,
            max_depth: u64,
            follow_links: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<DirEntry>> {
            self.0
                .method_hash([129, 2, 215, 13], (path, max_depth, follow_links))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `readDir` (0xc4bc59e0) function
        pub fn read_dir_0(
            &self,
            path: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<DirEntry>> {
            self.0
                .method_hash([196, 188, 89, 224], path)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `readFile` (0x60f9bb11) function
        pub fn read_file(
            &self,
            path: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([96, 249, 187, 17], path)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `readFileBinary` (0x16ed7bc4) function
        pub fn read_file_binary(
            &self,
            path: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([22, 237, 123, 196], path)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `readLine` (0x70f55728) function
        pub fn read_line(
            &self,
            path: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([112, 245, 87, 40], path)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `readLink` (0x9f5684a2) function
        pub fn read_link(
            &self,
            link_path: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([159, 86, 132, 162], link_path)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `record` (0x266cf109) function
        pub fn record(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 108, 241, 9], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recordLogs` (0x41af2f52) function
        pub fn record_logs(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([65, 175, 47, 82], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rememberKey` (0x22100064) function
        pub fn remember_key(
            &self,
            private_key: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([34, 16, 0, 100], private_key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeDir` (0x45c62011) function
        pub fn remove_dir(
            &self,
            path: ::std::string::String,
            recursive: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([69, 198, 32, 17], (path, recursive))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeFile` (0xf1afe04d) function
        pub fn remove_file(
            &self,
            path: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 175, 224, 77], path)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `resetNonce` (0x1c72346d) function
        pub fn reset_nonce(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 114, 52, 109], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `resumeGasMetering` (0x2bcd50e0) function
        pub fn resume_gas_metering(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([43, 205, 80, 224], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revertTo` (0x44d7f0a4) function
        pub fn revert_to(
            &self,
            snapshot_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([68, 215, 240, 164], snapshot_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revertToAndDelete` (0x03e0aca9) function
        pub fn revert_to_and_delete(
            &self,
            snapshot_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([3, 224, 172, 169], snapshot_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokePersistent` (0x3ce969e6) function
        pub fn revoke_persistent(
            &self,
            accounts: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 233, 105, 230], accounts)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokePersistent` (0x997a0222) function
        pub fn revoke_persistent_with_account(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 122, 2, 34], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `roll` (0x1f7b4f30) function
        pub fn roll(
            &self,
            new_height: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 123, 79, 48], new_height)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rollFork` (0x0f29772b) function
        pub fn roll_fork_0(
            &self,
            tx_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 41, 119, 43], tx_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rollFork` (0xd74c83a4) function
        pub fn roll_fork_2(
            &self,
            fork_id: ::ethers::core::types::U256,
            block_number: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([215, 76, 131, 164], (fork_id, block_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rollFork` (0xd9bbf3a1) function
        pub fn roll_fork_1(
            &self,
            block_number: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([217, 187, 243, 161], block_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rollFork` (0xf2830f7b) function
        pub fn roll_fork_3(
            &self,
            fork_id: ::ethers::core::types::U256,
            tx_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 131, 15, 123], (fork_id, tx_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rpc` (0x1206c8a8) function
        pub fn rpc(
            &self,
            method: ::std::string::String,
            params: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([18, 6, 200, 168], (method, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rpcUrl` (0x975a6ce9) function
        pub fn rpc_url(
            &self,
            rpc_alias: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([151, 90, 108, 233], rpc_alias)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rpcUrlStructs` (0x9d2ad72a) function
        pub fn rpc_url_structs(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Rpc>> {
            self.0
                .method_hash([157, 42, 215, 42], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rpcUrls` (0xa85a8418) function
        pub fn rpc_urls(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<[::std::string::String; 2]>,
        > {
            self.0
                .method_hash([168, 90, 132, 24], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selectFork` (0x9ebf6827) function
        pub fn select_fork(
            &self,
            fork_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([158, 191, 104, 39], fork_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeAddress` (0x1e356e1a) function
        pub fn serialize_address(
            &self,
            object_key: ::std::string::String,
            value_key: ::std::string::String,
            values: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([30, 53, 110, 26], (object_key, value_key, values))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeAddress` (0x972c6062) function
        pub fn serialize_address_with_object_key_and_value_key_and_value(
            &self,
            object_key: ::std::string::String,
            value_key: ::std::string::String,
            value: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([151, 44, 96, 98], (object_key, value_key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeBool` (0x92925aa1) function
        pub fn serialize_bool(
            &self,
            object_key: ::std::string::String,
            value_key: ::std::string::String,
            values: ::std::vec::Vec<bool>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([146, 146, 90, 161], (object_key, value_key, values))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeBool` (0xac22e971) function
        pub fn serialize_bool_with_object_key_and_value_key_and_value(
            &self,
            object_key: ::std::string::String,
            value_key: ::std::string::String,
            value: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([172, 34, 233, 113], (object_key, value_key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeBytes` (0x9884b232) function
        pub fn serialize_bytes(
            &self,
            object_key: ::std::string::String,
            value_key: ::std::string::String,
            values: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([152, 132, 178, 50], (object_key, value_key, values))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeBytes` (0xf21d52c7) function
        pub fn serialize_bytes_with_object_key_and_value_key_and_value(
            &self,
            object_key: ::std::string::String,
            value_key: ::std::string::String,
            value: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([242, 29, 82, 199], (object_key, value_key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeBytes32` (0x201e43e2) function
        pub fn serialize_bytes_32(
            &self,
            object_key: ::std::string::String,
            value_key: ::std::string::String,
            values: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([32, 30, 67, 226], (object_key, value_key, values))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeBytes32` (0x2d812b44) function
        pub fn serialize_bytes_32_with_object_key_and_value_key_and_value(
            &self,
            object_key: ::std::string::String,
            value_key: ::std::string::String,
            value: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([45, 129, 43, 68], (object_key, value_key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeInt` (0x3f33db60) function
        pub fn serialize_int(
            &self,
            object_key: ::std::string::String,
            value_key: ::std::string::String,
            value: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([63, 51, 219, 96], (object_key, value_key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeInt` (0x7676e127) function
        pub fn serialize_int_with_object_key_and_value_key_and_values(
            &self,
            object_key: ::std::string::String,
            value_key: ::std::string::String,
            values: ::std::vec::Vec<::ethers::core::types::I256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([118, 118, 225, 39], (object_key, value_key, values))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeJson` (0x9b3358b0) function
        pub fn serialize_json(
            &self,
            object_key: ::std::string::String,
            value: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([155, 51, 88, 176], (object_key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeString` (0x561cd6f3) function
        pub fn serialize_string(
            &self,
            object_key: ::std::string::String,
            value_key: ::std::string::String,
            values: ::std::vec::Vec<::std::string::String>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([86, 28, 214, 243], (object_key, value_key, values))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeString` (0x88da6d35) function
        pub fn serialize_string_with_object_key_and_value_key_and_value(
            &self,
            object_key: ::std::string::String,
            value_key: ::std::string::String,
            value: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([136, 218, 109, 53], (object_key, value_key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeUint` (0x129e9002) function
        pub fn serialize_uint(
            &self,
            object_key: ::std::string::String,
            value_key: ::std::string::String,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([18, 158, 144, 2], (object_key, value_key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeUint` (0xfee9a469) function
        pub fn serialize_uint_with_object_key_and_value_key_and_values(
            &self,
            object_key: ::std::string::String,
            value_key: ::std::string::String,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([254, 233, 164, 105], (object_key, value_key, values))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEnv` (0x3d5923ee) function
        pub fn set_env(
            &self,
            name: ::std::string::String,
            value: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([61, 89, 35, 238], (name, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setNonce` (0xf8e18b57) function
        pub fn set_nonce(
            &self,
            account: ::ethers::core::types::Address,
            new_nonce: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 225, 139, 87], (account, new_nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setNonceUnsafe` (0x9b67b21c) function
        pub fn set_nonce_unsafe(
            &self,
            account: ::ethers::core::types::Address,
            new_nonce: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 103, 178, 28], (account, new_nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sign` (0xb25c5a25) function
        pub fn sign(
            &self,
            wallet: Wallet,
            digest: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (u8, [u8; 32], [u8; 32])> {
            self.0
                .method_hash([178, 92, 90, 37], (wallet, digest))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sign` (0xe341eaa4) function
        pub fn sign_with_private_key(
            &self,
            private_key: ::ethers::core::types::U256,
            digest: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (u8, [u8; 32], [u8; 32])> {
            self.0
                .method_hash([227, 65, 234, 164], (private_key, digest))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signP256` (0x83211b40) function
        pub fn sign_p256(
            &self,
            private_key: ::ethers::core::types::U256,
            digest: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ([u8; 32], [u8; 32])> {
            self.0
                .method_hash([131, 33, 27, 64], (private_key, digest))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `skip` (0xdd82d13e) function
        pub fn skip(
            &self,
            skip_test: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 130, 209, 62], skip_test)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sleep` (0xfa9d8713) function
        pub fn sleep(
            &self,
            duration: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 157, 135, 19], duration)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `snapshot` (0x9711715a) function
        pub fn snapshot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([151, 17, 113, 90], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startBroadcast` (0x7fb5297f) function
        pub fn start_broadcast(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 181, 41, 127], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startBroadcast` (0x7fec2a8d) function
        pub fn start_broadcast_with_signer(
            &self,
            signer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 236, 42, 141], signer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startBroadcast` (0xce817d47) function
        pub fn start_broadcast_with_private_key(
            &self,
            private_key: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 129, 125, 71], private_key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startMappingRecording` (0x3e9705c0) function
        pub fn start_mapping_recording(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([62, 151, 5, 192], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startPrank` (0x06447d56) function
        pub fn start_prank(
            &self,
            msg_sender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([6, 68, 125, 86], msg_sender)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startPrank` (0x45b56078) function
        pub fn start_prank_with_tx_origin(
            &self,
            msg_sender: ::ethers::core::types::Address,
            tx_origin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([69, 181, 96, 120], (msg_sender, tx_origin))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startStateDiffRecording` (0xcf22e3c9) function
        pub fn start_state_diff_recording(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 34, 227, 201], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stopAndReturnStateDiff` (0xaa5cf90e) function
        pub fn stop_and_return_state_diff(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<AccountAccess>,
        > {
            self.0
                .method_hash([170, 92, 249, 14], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stopBroadcast` (0x76eadd36) function
        pub fn stop_broadcast(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([118, 234, 221, 54], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stopMappingRecording` (0x0d4aae9b) function
        pub fn stop_mapping_recording(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 74, 174, 155], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stopPrank` (0x90c5013b) function
        pub fn stop_prank(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([144, 197, 1, 59], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `store` (0x70ca10bb) function
        pub fn store(
            &self,
            target: ::ethers::core::types::Address,
            slot: [u8; 32],
            value: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 202, 16, 187], (target, slot, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toBase64` (0x3f8be2c8) function
        pub fn to_base_64(
            &self,
            data: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([63, 139, 226, 200], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toBase64` (0xa5cbfe65) function
        pub fn to_base_64_with_data(
            &self,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([165, 203, 254, 101], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toBase64URL` (0xae3165b3) function
        pub fn to_base_64url(
            &self,
            data: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([174, 49, 101, 179], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toBase64URL` (0xc8bd0e4a) function
        pub fn to_base_6_4url_with_data(
            &self,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([200, 189, 14, 74], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0x56ca623e) function
        pub fn to_string_0(
            &self,
            value: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([86, 202, 98, 62], value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0x6900a3ae) function
        pub fn to_string_1(
            &self,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([105, 0, 163, 174], value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0x71aad10d) function
        pub fn to_string_2(
            &self,
            value: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([113, 170, 209, 13], value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0x71dce7da) function
        pub fn to_string_3(
            &self,
            value: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([113, 220, 231, 218], value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0xa322c40e) function
        pub fn to_string_4(
            &self,
            value: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([163, 34, 196, 14], value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0xb11a19e8) function
        pub fn to_string_5(
            &self,
            value: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([177, 26, 25, 232], value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transact` (0x4d8abc4b) function
        pub fn transact_with_fork_id(
            &self,
            fork_id: ::ethers::core::types::U256,
            tx_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([77, 138, 188, 75], (fork_id, tx_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transact` (0xbe646da1) function
        pub fn transact(
            &self,
            tx_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([190, 100, 109, 161], tx_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tryFfi` (0xf45c1ce7) function
        pub fn try_ffi(
            &self,
            command_input: ::std::vec::Vec<::std::string::String>,
        ) -> ::ethers::contract::builders::ContractCall<M, FfiResult> {
            self.0
                .method_hash([244, 92, 28, 231], command_input)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `txGasPrice` (0x48f50c0f) function
        pub fn tx_gas_price(
            &self,
            new_gas_price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 245, 12, 15], new_gas_price)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unixTime` (0x625387dc) function
        pub fn unix_time(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([98, 83, 135, 220], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `warp` (0xe5d6bf02) function
        pub fn warp(
            &self,
            new_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([229, 214, 191, 2], new_timestamp)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `writeFile` (0x897e0a97) function
        pub fn write_file(
            &self,
            path: ::std::string::String,
            data: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 126, 10, 151], (path, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `writeFileBinary` (0x1f21fc80) function
        pub fn write_file_binary(
            &self,
            path: ::std::string::String,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 33, 252, 128], (path, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `writeJson` (0x35d6ad46) function
        pub fn write_json_with_json_and_path(
            &self,
            json: ::std::string::String,
            path: ::std::string::String,
            value_key: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 214, 173, 70], (json, path, value_key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `writeJson` (0xe23cd19f) function
        pub fn write_json(
            &self,
            json: ::std::string::String,
            path: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 60, 209, 159], (json, path))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `writeLine` (0x619d897f) function
        pub fn write_line(
            &self,
            path: ::std::string::String,
            data: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([97, 157, 137, 127], (path, data))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Vm<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `accesses` function with signature `accesses(address)` and selector `0x65bc9481`
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
    #[ethcall(name = "accesses", abi = "accesses(address)")]
    pub struct AccessesCall {
        pub target: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `activeFork` function with signature `activeFork()` and selector `0x2f103f22`
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
    #[ethcall(name = "activeFork", abi = "activeFork()")]
    pub struct ActiveForkCall;
    ///Container type for all input parameters for the `addr` function with signature `addr(uint256)` and selector `0xffa18649`
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
    #[ethcall(name = "addr", abi = "addr(uint256)")]
    pub struct AddrCall {
        pub private_key: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `allowCheatcodes` function with signature `allowCheatcodes(address)` and selector `0xea060291`
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
    #[ethcall(name = "allowCheatcodes", abi = "allowCheatcodes(address)")]
    pub struct AllowCheatcodesCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `assume` function with signature `assume(bool)` and selector `0x4c63e562`
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
    #[ethcall(name = "assume", abi = "assume(bool)")]
    pub struct AssumeCall {
        pub condition: bool,
    }
    ///Container type for all input parameters for the `breakpoint` function with signature `breakpoint(string)` and selector `0xf0259e92`
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
    #[ethcall(name = "breakpoint", abi = "breakpoint(string)")]
    pub struct BreakpointCall {
        pub char: ::std::string::String,
    }
    ///Container type for all input parameters for the `breakpoint` function with signature `breakpoint(string,bool)` and selector `0xf7d39a8d`
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
    #[ethcall(name = "breakpoint", abi = "breakpoint(string,bool)")]
    pub struct BreakpointWithValueCall {
        pub char: ::std::string::String,
        pub value: bool,
    }
    ///Container type for all input parameters for the `broadcast` function with signature `broadcast()` and selector `0xafc98040`
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
    #[ethcall(name = "broadcast", abi = "broadcast()")]
    pub struct BroadcastCall;
    ///Container type for all input parameters for the `broadcast` function with signature `broadcast(address)` and selector `0xe6962cdb`
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
    #[ethcall(name = "broadcast", abi = "broadcast(address)")]
    pub struct BroadcastWithSignerCall {
        pub signer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `broadcast` function with signature `broadcast(uint256)` and selector `0xf67a965b`
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
    #[ethcall(name = "broadcast", abi = "broadcast(uint256)")]
    pub struct BroadcastWithPrivateKeyCall {
        pub private_key: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `chainId` function with signature `chainId(uint256)` and selector `0x4049ddd2`
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
    #[ethcall(name = "chainId", abi = "chainId(uint256)")]
    pub struct ChainIdCall {
        pub new_chain_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `clearMockedCalls` function with signature `clearMockedCalls()` and selector `0x3fdf4e15`
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
    #[ethcall(name = "clearMockedCalls", abi = "clearMockedCalls()")]
    pub struct ClearMockedCallsCall;
    ///Container type for all input parameters for the `closeFile` function with signature `closeFile(string)` and selector `0x48c3241f`
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
    #[ethcall(name = "closeFile", abi = "closeFile(string)")]
    pub struct CloseFileCall {
        pub path: ::std::string::String,
    }
    ///Container type for all input parameters for the `coinbase` function with signature `coinbase(address)` and selector `0xff483c54`
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
    #[ethcall(name = "coinbase", abi = "coinbase(address)")]
    pub struct CoinbaseCall {
        pub new_coinbase: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `computeCreate2Address` function with signature `computeCreate2Address(bytes32,bytes32)` and selector `0x890c283b`
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
    #[ethcall(
        name = "computeCreate2Address",
        abi = "computeCreate2Address(bytes32,bytes32)"
    )]
    pub struct ComputeCreate2AddressCall {
        pub salt: [u8; 32],
        pub init_code_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `computeCreate2Address` function with signature `computeCreate2Address(bytes32,bytes32,address)` and selector `0xd323826a`
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
    #[ethcall(
        name = "computeCreate2Address",
        abi = "computeCreate2Address(bytes32,bytes32,address)"
    )]
    pub struct ComputeCreate2AddressWithSaltAndInitCodeHashCall {
        pub salt: [u8; 32],
        pub init_code_hash: [u8; 32],
        pub deployer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `computeCreateAddress` function with signature `computeCreateAddress(address,uint256)` and selector `0x74637a7a`
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
    #[ethcall(
        name = "computeCreateAddress",
        abi = "computeCreateAddress(address,uint256)"
    )]
    pub struct ComputeCreateAddressCall {
        pub deployer: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `copyFile` function with signature `copyFile(string,string)` and selector `0xa54a87d8`
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
    #[ethcall(name = "copyFile", abi = "copyFile(string,string)")]
    pub struct CopyFileCall {
        pub from: ::std::string::String,
        pub to: ::std::string::String,
    }
    ///Container type for all input parameters for the `createDir` function with signature `createDir(string,bool)` and selector `0x168b64d3`
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
    #[ethcall(name = "createDir", abi = "createDir(string,bool)")]
    pub struct CreateDirCall {
        pub path: ::std::string::String,
        pub recursive: bool,
    }
    ///Container type for all input parameters for the `createFork` function with signature `createFork(string)` and selector `0x31ba3498`
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
    #[ethcall(name = "createFork", abi = "createFork(string)")]
    pub struct CreateForkCall {
        pub url_or_alias: ::std::string::String,
    }
    ///Container type for all input parameters for the `createFork` function with signature `createFork(string,uint256)` and selector `0x6ba3ba2b`
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
    #[ethcall(name = "createFork", abi = "createFork(string,uint256)")]
    pub struct CreateForkWithBlockNumberCall {
        pub url_or_alias: ::std::string::String,
        pub block_number: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `createFork` function with signature `createFork(string,bytes32)` and selector `0x7ca29682`
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
    #[ethcall(name = "createFork", abi = "createFork(string,bytes32)")]
    pub struct CreateForkWithTxHashCall {
        pub url_or_alias: ::std::string::String,
        pub tx_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `createSelectFork` function with signature `createSelectFork(string,uint256)` and selector `0x71ee464d`
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
    #[ethcall(name = "createSelectFork", abi = "createSelectFork(string,uint256)")]
    pub struct CreateSelectForkWithBlockNumberCall {
        pub url_or_alias: ::std::string::String,
        pub block_number: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `createSelectFork` function with signature `createSelectFork(string,bytes32)` and selector `0x84d52b7a`
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
    #[ethcall(name = "createSelectFork", abi = "createSelectFork(string,bytes32)")]
    pub struct CreateSelectForkWithTxHashCall {
        pub url_or_alias: ::std::string::String,
        pub tx_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `createSelectFork` function with signature `createSelectFork(string)` and selector `0x98680034`
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
    #[ethcall(name = "createSelectFork", abi = "createSelectFork(string)")]
    pub struct CreateSelectForkCall {
        pub url_or_alias: ::std::string::String,
    }
    ///Container type for all input parameters for the `createWallet` function with signature `createWallet(string)` and selector `0x7404f1d2`
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
    #[ethcall(name = "createWallet", abi = "createWallet(string)")]
    pub struct CreateWallet0Call {
        pub wallet_label: ::std::string::String,
    }
    ///Container type for all input parameters for the `createWallet` function with signature `createWallet(uint256)` and selector `0x7a675bb6`
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
    #[ethcall(name = "createWallet", abi = "createWallet(uint256)")]
    pub struct CreateWallet1Call {
        pub private_key: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `createWallet` function with signature `createWallet(uint256,string)` and selector `0xed7c5462`
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
    #[ethcall(name = "createWallet", abi = "createWallet(uint256,string)")]
    pub struct CreateWallet2Call {
        pub private_key: ::ethers::core::types::U256,
        pub wallet_label: ::std::string::String,
    }
    ///Container type for all input parameters for the `deal` function with signature `deal(address,uint256)` and selector `0xc88a5e6d`
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
    #[ethcall(name = "deal", abi = "deal(address,uint256)")]
    pub struct DealCall {
        pub account: ::ethers::core::types::Address,
        pub new_balance: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deleteSnapshot` function with signature `deleteSnapshot(uint256)` and selector `0xa6368557`
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
    #[ethcall(name = "deleteSnapshot", abi = "deleteSnapshot(uint256)")]
    pub struct DeleteSnapshotCall {
        pub snapshot_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deleteSnapshots` function with signature `deleteSnapshots()` and selector `0x421ae469`
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
    #[ethcall(name = "deleteSnapshots", abi = "deleteSnapshots()")]
    pub struct DeleteSnapshotsCall;
    ///Container type for all input parameters for the `deriveKey` function with signature `deriveKey(string,string,uint32,string)` and selector `0x29233b1f`
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
    #[ethcall(name = "deriveKey", abi = "deriveKey(string,string,uint32,string)")]
    pub struct DeriveKey3Call {
        pub mnemonic: ::std::string::String,
        pub derivation_path: ::std::string::String,
        pub index: u32,
        pub language: ::std::string::String,
    }
    ///Container type for all input parameters for the `deriveKey` function with signature `deriveKey(string,uint32,string)` and selector `0x32c8176d`
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
    #[ethcall(name = "deriveKey", abi = "deriveKey(string,uint32,string)")]
    pub struct DeriveKey1Call {
        pub mnemonic: ::std::string::String,
        pub index: u32,
        pub language: ::std::string::String,
    }
    ///Container type for all input parameters for the `deriveKey` function with signature `deriveKey(string,uint32)` and selector `0x6229498b`
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
    #[ethcall(name = "deriveKey", abi = "deriveKey(string,uint32)")]
    pub struct DeriveKey0Call {
        pub mnemonic: ::std::string::String,
        pub index: u32,
    }
    ///Container type for all input parameters for the `deriveKey` function with signature `deriveKey(string,string,uint32)` and selector `0x6bcb2c1b`
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
    #[ethcall(name = "deriveKey", abi = "deriveKey(string,string,uint32)")]
    pub struct DeriveKey2Call {
        pub mnemonic: ::std::string::String,
        pub derivation_path: ::std::string::String,
        pub index: u32,
    }
    ///Container type for all input parameters for the `difficulty` function with signature `difficulty(uint256)` and selector `0x46cc92d9`
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
    #[ethcall(name = "difficulty", abi = "difficulty(uint256)")]
    pub struct DifficultyCall {
        pub new_difficulty: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `dumpState` function with signature `dumpState(string)` and selector `0x709ecd3f`
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
    #[ethcall(name = "dumpState", abi = "dumpState(string)")]
    pub struct DumpStateCall {
        pub path_to_state_json: ::std::string::String,
    }
    ///Container type for all input parameters for the `envAddress` function with signature `envAddress(string)` and selector `0x350d56bf`
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
    #[ethcall(name = "envAddress", abi = "envAddress(string)")]
    pub struct EnvAddressCall {
        pub name: ::std::string::String,
    }
    ///Container type for all input parameters for the `envAddress` function with signature `envAddress(string,string)` and selector `0xad31b9fa`
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
    #[ethcall(name = "envAddress", abi = "envAddress(string,string)")]
    pub struct EnvAddressWithDelimCall {
        pub name: ::std::string::String,
        pub delim: ::std::string::String,
    }
    ///Container type for all input parameters for the `envBool` function with signature `envBool(string)` and selector `0x7ed1ec7d`
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
    #[ethcall(name = "envBool", abi = "envBool(string)")]
    pub struct EnvBoolCall {
        pub name: ::std::string::String,
    }
    ///Container type for all input parameters for the `envBool` function with signature `envBool(string,string)` and selector `0xaaaddeaf`
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
    #[ethcall(name = "envBool", abi = "envBool(string,string)")]
    pub struct EnvBoolWithDelimCall {
        pub name: ::std::string::String,
        pub delim: ::std::string::String,
    }
    ///Container type for all input parameters for the `envBytes` function with signature `envBytes(string)` and selector `0x4d7baf06`
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
    #[ethcall(name = "envBytes", abi = "envBytes(string)")]
    pub struct EnvBytesCall {
        pub name: ::std::string::String,
    }
    ///Container type for all input parameters for the `envBytes` function with signature `envBytes(string,string)` and selector `0xddc2651b`
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
    #[ethcall(name = "envBytes", abi = "envBytes(string,string)")]
    pub struct EnvBytesWithDelimCall {
        pub name: ::std::string::String,
        pub delim: ::std::string::String,
    }
    ///Container type for all input parameters for the `envBytes32` function with signature `envBytes32(string,string)` and selector `0x5af231c1`
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
    #[ethcall(name = "envBytes32", abi = "envBytes32(string,string)")]
    pub struct EnvBytes32WithDelimCall {
        pub name: ::std::string::String,
        pub delim: ::std::string::String,
    }
    ///Container type for all input parameters for the `envBytes32` function with signature `envBytes32(string)` and selector `0x97949042`
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
    #[ethcall(name = "envBytes32", abi = "envBytes32(string)")]
    pub struct EnvBytes32Call {
        pub name: ::std::string::String,
    }
    ///Container type for all input parameters for the `envInt` function with signature `envInt(string,string)` and selector `0x42181150`
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
    #[ethcall(name = "envInt", abi = "envInt(string,string)")]
    pub struct EnvIntWithDelimCall {
        pub name: ::std::string::String,
        pub delim: ::std::string::String,
    }
    ///Container type for all input parameters for the `envInt` function with signature `envInt(string)` and selector `0x892a0c61`
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
    #[ethcall(name = "envInt", abi = "envInt(string)")]
    pub struct EnvIntCall {
        pub name: ::std::string::String,
    }
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,string,bytes32[])` and selector `0x2281f367`
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
    #[ethcall(name = "envOr", abi = "envOr(string,string,bytes32[])")]
    pub struct EnvOr7Call {
        pub name: ::std::string::String,
        pub delim: ::std::string::String,
        pub default_value: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,string,int256[])` and selector `0x4700d74b`
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
    #[ethcall(name = "envOr", abi = "envOr(string,string,int256[])")]
    pub struct EnvOr8Call {
        pub name: ::std::string::String,
        pub delim: ::std::string::String,
        pub default_value: ::std::vec::Vec<::ethers::core::types::I256>,
    }
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,bool)` and selector `0x4777f3cf`
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
    #[ethcall(name = "envOr", abi = "envOr(string,bool)")]
    pub struct EnvOr0Call {
        pub name: ::std::string::String,
        pub default_value: bool,
    }
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,address)` and selector `0x561fe540`
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
    #[ethcall(name = "envOr", abi = "envOr(string,address)")]
    pub struct EnvOr1Call {
        pub name: ::std::string::String,
        pub default_value: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,uint256)` and selector `0x5e97348f`
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
    #[ethcall(name = "envOr", abi = "envOr(string,uint256)")]
    pub struct EnvOr2Call {
        pub name: ::std::string::String,
        pub default_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,string,bytes[])` and selector `0x64bc3e64`
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
    #[ethcall(name = "envOr", abi = "envOr(string,string,bytes[])")]
    pub struct EnvOr9Call {
        pub name: ::std::string::String,
        pub delim: ::std::string::String,
        pub default_value: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,string,uint256[])` and selector `0x74318528`
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
    #[ethcall(name = "envOr", abi = "envOr(string,string,uint256[])")]
    pub struct EnvOr10Call {
        pub name: ::std::string::String,
        pub delim: ::std::string::String,
        pub default_value: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,string,string[])` and selector `0x859216bc`
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
    #[ethcall(name = "envOr", abi = "envOr(string,string,string[])")]
    pub struct EnvOr11Call {
        pub name: ::std::string::String,
        pub delim: ::std::string::String,
        pub default_value: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,bytes)` and selector `0xb3e47705`
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
    #[ethcall(name = "envOr", abi = "envOr(string,bytes)")]
    pub struct EnvOr3Call {
        pub name: ::std::string::String,
        pub default_value: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,bytes32)` and selector `0xb4a85892`
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
    #[ethcall(name = "envOr", abi = "envOr(string,bytes32)")]
    pub struct EnvOr4Call {
        pub name: ::std::string::String,
        pub default_value: [u8; 32],
    }
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,int256)` and selector `0xbbcb713e`
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
    #[ethcall(name = "envOr", abi = "envOr(string,int256)")]
    pub struct EnvOr5Call {
        pub name: ::std::string::String,
        pub default_value: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,string,address[])` and selector `0xc74e9deb`
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
    #[ethcall(name = "envOr", abi = "envOr(string,string,address[])")]
    pub struct EnvOr12Call {
        pub name: ::std::string::String,
        pub delim: ::std::string::String,
        pub default_value: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,string)` and selector `0xd145736c`
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
    #[ethcall(name = "envOr", abi = "envOr(string,string)")]
    pub struct EnvOr6Call {
        pub name: ::std::string::String,
        pub default_value: ::std::string::String,
    }
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,string,bool[])` and selector `0xeb85e83b`
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
    #[ethcall(name = "envOr", abi = "envOr(string,string,bool[])")]
    pub struct EnvOr13Call {
        pub name: ::std::string::String,
        pub delim: ::std::string::String,
        pub default_value: ::std::vec::Vec<bool>,
    }
    ///Container type for all input parameters for the `envString` function with signature `envString(string,string)` and selector `0x14b02bc9`
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
    #[ethcall(name = "envString", abi = "envString(string,string)")]
    pub struct EnvStringWithDelimCall {
        pub name: ::std::string::String,
        pub delim: ::std::string::String,
    }
    ///Container type for all input parameters for the `envString` function with signature `envString(string)` and selector `0xf877cb19`
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
    #[ethcall(name = "envString", abi = "envString(string)")]
    pub struct EnvStringCall {
        pub name: ::std::string::String,
    }
    ///Container type for all input parameters for the `envUint` function with signature `envUint(string)` and selector `0xc1978d1f`
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
    #[ethcall(name = "envUint", abi = "envUint(string)")]
    pub struct EnvUintCall {
        pub name: ::std::string::String,
    }
    ///Container type for all input parameters for the `envUint` function with signature `envUint(string,string)` and selector `0xf3dec099`
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
    #[ethcall(name = "envUint", abi = "envUint(string,string)")]
    pub struct EnvUintWithDelimCall {
        pub name: ::std::string::String,
        pub delim: ::std::string::String,
    }
    ///Container type for all input parameters for the `etch` function with signature `etch(address,bytes)` and selector `0xb4d6c782`
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
    #[ethcall(name = "etch", abi = "etch(address,bytes)")]
    pub struct EtchCall {
        pub target: ::ethers::core::types::Address,
        pub new_runtime_bytecode: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `eth_getLogs` function with signature `eth_getLogs(uint256,uint256,address,bytes32[])` and selector `0x35e1349b`
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
    #[ethcall(
        name = "eth_getLogs",
        abi = "eth_getLogs(uint256,uint256,address,bytes32[])"
    )]
    pub struct EthGetLogsCall {
        pub from_block: ::ethers::core::types::U256,
        pub to_block: ::ethers::core::types::U256,
        pub target: ::ethers::core::types::Address,
        pub topics: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `exists` function with signature `exists(string)` and selector `0x261a323e`
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
    #[ethcall(name = "exists", abi = "exists(string)")]
    pub struct ExistsCall {
        pub path: ::std::string::String,
    }
    ///Container type for all input parameters for the `expectCall` function with signature `expectCall(address,uint256,uint64,bytes)` and selector `0x23361207`
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
    #[ethcall(name = "expectCall", abi = "expectCall(address,uint256,uint64,bytes)")]
    pub struct ExpectCall3Call {
        pub callee: ::ethers::core::types::Address,
        pub msg_value: ::ethers::core::types::U256,
        pub gas: u64,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `expectCall` function with signature `expectCall(address,uint256,uint64,bytes,uint64)` and selector `0x65b7b7cc`
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
    #[ethcall(
        name = "expectCall",
        abi = "expectCall(address,uint256,uint64,bytes,uint64)"
    )]
    pub struct ExpectCall5Call {
        pub callee: ::ethers::core::types::Address,
        pub msg_value: ::ethers::core::types::U256,
        pub gas: u64,
        pub data: ::ethers::core::types::Bytes,
        pub count: u64,
    }
    ///Container type for all input parameters for the `expectCall` function with signature `expectCall(address,uint256,bytes,uint64)` and selector `0xa2b1a1ae`
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
    #[ethcall(name = "expectCall", abi = "expectCall(address,uint256,bytes,uint64)")]
    pub struct ExpectCall4Call {
        pub callee: ::ethers::core::types::Address,
        pub msg_value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub count: u64,
    }
    ///Container type for all input parameters for the `expectCall` function with signature `expectCall(address,bytes)` and selector `0xbd6af434`
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
    #[ethcall(name = "expectCall", abi = "expectCall(address,bytes)")]
    pub struct ExpectCall0Call {
        pub callee: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `expectCall` function with signature `expectCall(address,bytes,uint64)` and selector `0xc1adbbff`
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
    #[ethcall(name = "expectCall", abi = "expectCall(address,bytes,uint64)")]
    pub struct ExpectCall1Call {
        pub callee: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
        pub count: u64,
    }
    ///Container type for all input parameters for the `expectCall` function with signature `expectCall(address,uint256,bytes)` and selector `0xf30c7ba3`
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
    #[ethcall(name = "expectCall", abi = "expectCall(address,uint256,bytes)")]
    pub struct ExpectCall2Call {
        pub callee: ::ethers::core::types::Address,
        pub msg_value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `expectCallMinGas` function with signature `expectCallMinGas(address,uint256,uint64,bytes)` and selector `0x08e4e116`
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
    #[ethcall(
        name = "expectCallMinGas",
        abi = "expectCallMinGas(address,uint256,uint64,bytes)"
    )]
    pub struct ExpectCallMinGasCall {
        pub callee: ::ethers::core::types::Address,
        pub msg_value: ::ethers::core::types::U256,
        pub min_gas: u64,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `expectCallMinGas` function with signature `expectCallMinGas(address,uint256,uint64,bytes,uint64)` and selector `0xe13a1834`
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
    #[ethcall(
        name = "expectCallMinGas",
        abi = "expectCallMinGas(address,uint256,uint64,bytes,uint64)"
    )]
    pub struct ExpectCallMinGasWithCalleeAndMsgValueAndMinGasAndDataCall {
        pub callee: ::ethers::core::types::Address,
        pub msg_value: ::ethers::core::types::U256,
        pub min_gas: u64,
        pub data: ::ethers::core::types::Bytes,
        pub count: u64,
    }
    ///Container type for all input parameters for the `expectEmit` function with signature `expectEmit()` and selector `0x440ed10d`
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
    #[ethcall(name = "expectEmit", abi = "expectEmit()")]
    pub struct ExpectEmit0Call;
    ///Container type for all input parameters for the `expectEmit` function with signature `expectEmit(bool,bool,bool,bool)` and selector `0x491cc7c2`
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
    #[ethcall(name = "expectEmit", abi = "expectEmit(bool,bool,bool,bool)")]
    pub struct ExpectEmit2Call {
        pub check_topic_1: bool,
        pub check_topic_2: bool,
        pub check_topic_3: bool,
        pub check_data: bool,
    }
    ///Container type for all input parameters for the `expectEmit` function with signature `expectEmit(bool,bool,bool,bool,address)` and selector `0x81bad6f3`
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
    #[ethcall(name = "expectEmit", abi = "expectEmit(bool,bool,bool,bool,address)")]
    pub struct ExpectEmit3Call {
        pub check_topic_1: bool,
        pub check_topic_2: bool,
        pub check_topic_3: bool,
        pub check_data: bool,
        pub emitter: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `expectEmit` function with signature `expectEmit(address)` and selector `0x86b9620d`
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
    #[ethcall(name = "expectEmit", abi = "expectEmit(address)")]
    pub struct ExpectEmit1Call {
        pub emitter: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `expectRevert` function with signature `expectRevert(bytes4)` and selector `0xc31eb0e0`
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
    #[ethcall(name = "expectRevert", abi = "expectRevert(bytes4)")]
    pub struct ExpectRevert1Call {
        pub revert_data: [u8; 4],
    }
    ///Container type for all input parameters for the `expectRevert` function with signature `expectRevert(bytes)` and selector `0xf28dceb3`
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
    #[ethcall(name = "expectRevert", abi = "expectRevert(bytes)")]
    pub struct ExpectRevert2Call {
        pub revert_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `expectRevert` function with signature `expectRevert()` and selector `0xf4844814`
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
    #[ethcall(name = "expectRevert", abi = "expectRevert()")]
    pub struct ExpectRevert0Call;
    ///Container type for all input parameters for the `expectSafeMemory` function with signature `expectSafeMemory(uint64,uint64)` and selector `0x6d016688`
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
    #[ethcall(name = "expectSafeMemory", abi = "expectSafeMemory(uint64,uint64)")]
    pub struct ExpectSafeMemoryCall {
        pub min: u64,
        pub max: u64,
    }
    ///Container type for all input parameters for the `expectSafeMemoryCall` function with signature `expectSafeMemoryCall(uint64,uint64)` and selector `0x05838bf4`
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
    #[ethcall(
        name = "expectSafeMemoryCall",
        abi = "expectSafeMemoryCall(uint64,uint64)"
    )]
    pub struct ExpectSafeMemoryCallCall {
        pub min: u64,
        pub max: u64,
    }
    ///Container type for all input parameters for the `fee` function with signature `fee(uint256)` and selector `0x39b37ab0`
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
    #[ethcall(name = "fee", abi = "fee(uint256)")]
    pub struct FeeCall {
        pub new_basefee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `ffi` function with signature `ffi(string[])` and selector `0x89160467`
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
    #[ethcall(name = "ffi", abi = "ffi(string[])")]
    pub struct FfiCall {
        pub command_input: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all input parameters for the `fsMetadata` function with signature `fsMetadata(string)` and selector `0xaf368a08`
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
    #[ethcall(name = "fsMetadata", abi = "fsMetadata(string)")]
    pub struct FsMetadataCall {
        pub path: ::std::string::String,
    }
    ///Container type for all input parameters for the `getBlockNumber` function with signature `getBlockNumber()` and selector `0x42cbb15c`
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
    #[ethcall(name = "getBlockNumber", abi = "getBlockNumber()")]
    pub struct GetBlockNumberCall;
    ///Container type for all input parameters for the `getBlockTimestamp` function with signature `getBlockTimestamp()` and selector `0x796b89b9`
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
    #[ethcall(name = "getBlockTimestamp", abi = "getBlockTimestamp()")]
    pub struct GetBlockTimestampCall;
    ///Container type for all input parameters for the `getCode` function with signature `getCode(string)` and selector `0x8d1cc925`
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
    #[ethcall(name = "getCode", abi = "getCode(string)")]
    pub struct GetCodeCall {
        pub artifact_path: ::std::string::String,
    }
    ///Container type for all input parameters for the `getDeployedCode` function with signature `getDeployedCode(string)` and selector `0x3ebf73b4`
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
    #[ethcall(name = "getDeployedCode", abi = "getDeployedCode(string)")]
    pub struct GetDeployedCodeCall {
        pub artifact_path: ::std::string::String,
    }
    ///Container type for all input parameters for the `getLabel` function with signature `getLabel(address)` and selector `0x28a249b0`
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
    #[ethcall(name = "getLabel", abi = "getLabel(address)")]
    pub struct GetLabelCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getMappingKeyAndParentOf` function with signature `getMappingKeyAndParentOf(address,bytes32)` and selector `0x876e24e6`
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
    #[ethcall(
        name = "getMappingKeyAndParentOf",
        abi = "getMappingKeyAndParentOf(address,bytes32)"
    )]
    pub struct GetMappingKeyAndParentOfCall {
        pub target: ::ethers::core::types::Address,
        pub element_slot: [u8; 32],
    }
    ///Container type for all input parameters for the `getMappingLength` function with signature `getMappingLength(address,bytes32)` and selector `0x2f2fd63f`
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
    #[ethcall(name = "getMappingLength", abi = "getMappingLength(address,bytes32)")]
    pub struct GetMappingLengthCall {
        pub target: ::ethers::core::types::Address,
        pub mapping_slot: [u8; 32],
    }
    ///Container type for all input parameters for the `getMappingSlotAt` function with signature `getMappingSlotAt(address,bytes32,uint256)` and selector `0xebc73ab4`
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
    #[ethcall(
        name = "getMappingSlotAt",
        abi = "getMappingSlotAt(address,bytes32,uint256)"
    )]
    pub struct GetMappingSlotAtCall {
        pub target: ::ethers::core::types::Address,
        pub mapping_slot: [u8; 32],
        pub idx: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getNonce` function with signature `getNonce(address)` and selector `0x2d0335ab`
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
    #[ethcall(name = "getNonce", abi = "getNonce(address)")]
    pub struct GetNonceCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getNonce` function with signature `getNonce((address,uint256,uint256,uint256))` and selector `0xa5748aad`
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
    #[ethcall(name = "getNonce", abi = "getNonce((address,uint256,uint256,uint256))")]
    pub struct GetNonceWithWalletCall {
        pub wallet: Wallet,
    }
    ///Container type for all input parameters for the `getRecordedLogs` function with signature `getRecordedLogs()` and selector `0x191553a4`
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
    #[ethcall(name = "getRecordedLogs", abi = "getRecordedLogs()")]
    pub struct GetRecordedLogsCall;
    ///Container type for all input parameters for the `isDir` function with signature `isDir(string)` and selector `0x7d15d019`
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
    #[ethcall(name = "isDir", abi = "isDir(string)")]
    pub struct IsDirCall {
        pub path: ::std::string::String,
    }
    ///Container type for all input parameters for the `isFile` function with signature `isFile(string)` and selector `0xe0eb04d4`
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
    #[ethcall(name = "isFile", abi = "isFile(string)")]
    pub struct IsFileCall {
        pub path: ::std::string::String,
    }
    ///Container type for all input parameters for the `isPersistent` function with signature `isPersistent(address)` and selector `0xd92d8efd`
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
    #[ethcall(name = "isPersistent", abi = "isPersistent(address)")]
    pub struct IsPersistentCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `keyExists` function with signature `keyExists(string,string)` and selector `0x528a683c`
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
    #[ethcall(name = "keyExists", abi = "keyExists(string,string)")]
    pub struct KeyExistsCall {
        pub json: ::std::string::String,
        pub key: ::std::string::String,
    }
    ///Container type for all input parameters for the `label` function with signature `label(address,string)` and selector `0xc657c718`
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
    #[ethcall(name = "label", abi = "label(address,string)")]
    pub struct LabelCall {
        pub account: ::ethers::core::types::Address,
        pub new_label: ::std::string::String,
    }
    ///Container type for all input parameters for the `load` function with signature `load(address,bytes32)` and selector `0x667f9d70`
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
    #[ethcall(name = "load", abi = "load(address,bytes32)")]
    pub struct LoadCall {
        pub target: ::ethers::core::types::Address,
        pub slot: [u8; 32],
    }
    ///Container type for all input parameters for the `loadAllocs` function with signature `loadAllocs(string)` and selector `0xb3a056d7`
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
    #[ethcall(name = "loadAllocs", abi = "loadAllocs(string)")]
    pub struct LoadAllocsCall {
        pub path_to_allocs_json: ::std::string::String,
    }
    ///Container type for all input parameters for the `makePersistent` function with signature `makePersistent(address[])` and selector `0x1d9e269e`
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
    #[ethcall(name = "makePersistent", abi = "makePersistent(address[])")]
    pub struct MakePersistent0Call {
        pub accounts: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `makePersistent` function with signature `makePersistent(address,address)` and selector `0x4074e0a8`
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
    #[ethcall(name = "makePersistent", abi = "makePersistent(address,address)")]
    pub struct MakePersistent2Call {
        pub account_0: ::ethers::core::types::Address,
        pub account_1: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `makePersistent` function with signature `makePersistent(address)` and selector `0x57e22dde`
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
    #[ethcall(name = "makePersistent", abi = "makePersistent(address)")]
    pub struct MakePersistent1Call {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `makePersistent` function with signature `makePersistent(address,address,address)` and selector `0xefb77a75`
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
    #[ethcall(name = "makePersistent", abi = "makePersistent(address,address,address)")]
    pub struct MakePersistent3Call {
        pub account_0: ::ethers::core::types::Address,
        pub account_1: ::ethers::core::types::Address,
        pub account_2: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `mockCall` function with signature `mockCall(address,uint256,bytes,bytes)` and selector `0x81409b91`
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
    #[ethcall(name = "mockCall", abi = "mockCall(address,uint256,bytes,bytes)")]
    pub struct MockCallWithCalleeAndMsgValueAndDataCall {
        pub callee: ::ethers::core::types::Address,
        pub msg_value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub return_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `mockCall` function with signature `mockCall(address,bytes,bytes)` and selector `0xb96213e4`
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
    #[ethcall(name = "mockCall", abi = "mockCall(address,bytes,bytes)")]
    pub struct MockCallCall {
        pub callee: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
        pub return_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `mockCallRevert` function with signature `mockCallRevert(address,uint256,bytes,bytes)` and selector `0xd23cd037`
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
    #[ethcall(
        name = "mockCallRevert",
        abi = "mockCallRevert(address,uint256,bytes,bytes)"
    )]
    pub struct MockCallRevertWithCalleeAndMsgValueAndDataCall {
        pub callee: ::ethers::core::types::Address,
        pub msg_value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub revert_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `mockCallRevert` function with signature `mockCallRevert(address,bytes,bytes)` and selector `0xdbaad147`
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
    #[ethcall(name = "mockCallRevert", abi = "mockCallRevert(address,bytes,bytes)")]
    pub struct MockCallRevertCall {
        pub callee: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
        pub revert_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `parseAddress` function with signature `parseAddress(string)` and selector `0xc6ce059d`
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
    #[ethcall(name = "parseAddress", abi = "parseAddress(string)")]
    pub struct ParseAddressCall {
        pub stringified_value: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseBool` function with signature `parseBool(string)` and selector `0x974ef924`
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
    #[ethcall(name = "parseBool", abi = "parseBool(string)")]
    pub struct ParseBoolCall {
        pub stringified_value: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseBytes` function with signature `parseBytes(string)` and selector `0x8f5d232d`
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
    #[ethcall(name = "parseBytes", abi = "parseBytes(string)")]
    pub struct ParseBytesCall {
        pub stringified_value: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseBytes32` function with signature `parseBytes32(string)` and selector `0x087e6e81`
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
    #[ethcall(name = "parseBytes32", abi = "parseBytes32(string)")]
    pub struct ParseBytes32Call {
        pub stringified_value: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseInt` function with signature `parseInt(string)` and selector `0x42346c5e`
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
    #[ethcall(name = "parseInt", abi = "parseInt(string)")]
    pub struct ParseIntCall {
        pub stringified_value: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseJson` function with signature `parseJson(string)` and selector `0x6a82600a`
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
    #[ethcall(name = "parseJson", abi = "parseJson(string)")]
    pub struct ParseJsonCall {
        pub json: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseJson` function with signature `parseJson(string,string)` and selector `0x85940ef1`
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
    #[ethcall(name = "parseJson", abi = "parseJson(string,string)")]
    pub struct ParseJsonWithKeyCall {
        pub json: ::std::string::String,
        pub key: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseJsonAddress` function with signature `parseJsonAddress(string,string)` and selector `0x1e19e657`
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
    #[ethcall(name = "parseJsonAddress", abi = "parseJsonAddress(string,string)")]
    pub struct ParseJsonAddressCall {
        pub json: ::std::string::String,
        pub key: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseJsonAddressArray` function with signature `parseJsonAddressArray(string,string)` and selector `0x2fce7883`
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
    #[ethcall(
        name = "parseJsonAddressArray",
        abi = "parseJsonAddressArray(string,string)"
    )]
    pub struct ParseJsonAddressArrayCall {
        pub json: ::std::string::String,
        pub key: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseJsonBool` function with signature `parseJsonBool(string,string)` and selector `0x9f86dc91`
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
    #[ethcall(name = "parseJsonBool", abi = "parseJsonBool(string,string)")]
    pub struct ParseJsonBoolCall {
        pub json: ::std::string::String,
        pub key: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseJsonBoolArray` function with signature `parseJsonBoolArray(string,string)` and selector `0x91f3b94f`
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
    #[ethcall(name = "parseJsonBoolArray", abi = "parseJsonBoolArray(string,string)")]
    pub struct ParseJsonBoolArrayCall {
        pub json: ::std::string::String,
        pub key: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseJsonBytes` function with signature `parseJsonBytes(string,string)` and selector `0xfd921be8`
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
    #[ethcall(name = "parseJsonBytes", abi = "parseJsonBytes(string,string)")]
    pub struct ParseJsonBytesCall {
        pub json: ::std::string::String,
        pub key: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseJsonBytes32` function with signature `parseJsonBytes32(string,string)` and selector `0x1777e59d`
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
    #[ethcall(name = "parseJsonBytes32", abi = "parseJsonBytes32(string,string)")]
    pub struct ParseJsonBytes32Call {
        pub json: ::std::string::String,
        pub key: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseJsonBytes32Array` function with signature `parseJsonBytes32Array(string,string)` and selector `0x91c75bc3`
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
    #[ethcall(
        name = "parseJsonBytes32Array",
        abi = "parseJsonBytes32Array(string,string)"
    )]
    pub struct ParseJsonBytes32ArrayCall {
        pub json: ::std::string::String,
        pub key: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseJsonBytesArray` function with signature `parseJsonBytesArray(string,string)` and selector `0x6631aa99`
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
    #[ethcall(name = "parseJsonBytesArray", abi = "parseJsonBytesArray(string,string)")]
    pub struct ParseJsonBytesArrayCall {
        pub json: ::std::string::String,
        pub key: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseJsonInt` function with signature `parseJsonInt(string,string)` and selector `0x7b048ccd`
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
    #[ethcall(name = "parseJsonInt", abi = "parseJsonInt(string,string)")]
    pub struct ParseJsonIntCall {
        pub json: ::std::string::String,
        pub key: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseJsonIntArray` function with signature `parseJsonIntArray(string,string)` and selector `0x9983c28a`
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
    #[ethcall(name = "parseJsonIntArray", abi = "parseJsonIntArray(string,string)")]
    pub struct ParseJsonIntArrayCall {
        pub json: ::std::string::String,
        pub key: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseJsonKeys` function with signature `parseJsonKeys(string,string)` and selector `0x213e4198`
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
    #[ethcall(name = "parseJsonKeys", abi = "parseJsonKeys(string,string)")]
    pub struct ParseJsonKeysCall {
        pub json: ::std::string::String,
        pub key: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseJsonString` function with signature `parseJsonString(string,string)` and selector `0x49c4fac8`
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
    #[ethcall(name = "parseJsonString", abi = "parseJsonString(string,string)")]
    pub struct ParseJsonStringCall {
        pub json: ::std::string::String,
        pub key: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseJsonStringArray` function with signature `parseJsonStringArray(string,string)` and selector `0x498fdcf4`
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
    #[ethcall(
        name = "parseJsonStringArray",
        abi = "parseJsonStringArray(string,string)"
    )]
    pub struct ParseJsonStringArrayCall {
        pub json: ::std::string::String,
        pub key: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseJsonUint` function with signature `parseJsonUint(string,string)` and selector `0xaddde2b6`
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
    #[ethcall(name = "parseJsonUint", abi = "parseJsonUint(string,string)")]
    pub struct ParseJsonUintCall {
        pub json: ::std::string::String,
        pub key: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseJsonUintArray` function with signature `parseJsonUintArray(string,string)` and selector `0x522074ab`
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
    #[ethcall(name = "parseJsonUintArray", abi = "parseJsonUintArray(string,string)")]
    pub struct ParseJsonUintArrayCall {
        pub json: ::std::string::String,
        pub key: ::std::string::String,
    }
    ///Container type for all input parameters for the `parseUint` function with signature `parseUint(string)` and selector `0xfa91454d`
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
    #[ethcall(name = "parseUint", abi = "parseUint(string)")]
    pub struct ParseUintCall {
        pub stringified_value: ::std::string::String,
    }
    ///Container type for all input parameters for the `pauseGasMetering` function with signature `pauseGasMetering()` and selector `0xd1a5b36f`
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
    #[ethcall(name = "pauseGasMetering", abi = "pauseGasMetering()")]
    pub struct PauseGasMeteringCall;
    ///Container type for all input parameters for the `prank` function with signature `prank(address,address)` and selector `0x47e50cce`
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
    #[ethcall(name = "prank", abi = "prank(address,address)")]
    pub struct PrankWithTxOriginCall {
        pub msg_sender: ::ethers::core::types::Address,
        pub tx_origin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `prank` function with signature `prank(address)` and selector `0xca669fa7`
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
    #[ethcall(name = "prank", abi = "prank(address)")]
    pub struct PrankCall {
        pub msg_sender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `prevrandao` function with signature `prevrandao(bytes32)` and selector `0x3b925549`
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
    #[ethcall(name = "prevrandao", abi = "prevrandao(bytes32)")]
    pub struct PrevrandaoCall {
        pub new_prevrandao: [u8; 32],
    }
    ///Container type for all input parameters for the `projectRoot` function with signature `projectRoot()` and selector `0xd930a0e6`
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
    #[ethcall(name = "projectRoot", abi = "projectRoot()")]
    pub struct ProjectRootCall;
    ///Container type for all input parameters for the `readCallers` function with signature `readCallers()` and selector `0x4ad0bac9`
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
    #[ethcall(name = "readCallers", abi = "readCallers()")]
    pub struct ReadCallersCall;
    ///Container type for all input parameters for the `readDir` function with signature `readDir(string,uint64)` and selector `0x1497876c`
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
    #[ethcall(name = "readDir", abi = "readDir(string,uint64)")]
    pub struct ReadDir1Call {
        pub path: ::std::string::String,
        pub max_depth: u64,
    }
    ///Container type for all input parameters for the `readDir` function with signature `readDir(string,uint64,bool)` and selector `0x8102d70d`
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
    #[ethcall(name = "readDir", abi = "readDir(string,uint64,bool)")]
    pub struct ReadDir2Call {
        pub path: ::std::string::String,
        pub max_depth: u64,
        pub follow_links: bool,
    }
    ///Container type for all input parameters for the `readDir` function with signature `readDir(string)` and selector `0xc4bc59e0`
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
    #[ethcall(name = "readDir", abi = "readDir(string)")]
    pub struct ReadDir0Call {
        pub path: ::std::string::String,
    }
    ///Container type for all input parameters for the `readFile` function with signature `readFile(string)` and selector `0x60f9bb11`
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
    #[ethcall(name = "readFile", abi = "readFile(string)")]
    pub struct ReadFileCall {
        pub path: ::std::string::String,
    }
    ///Container type for all input parameters for the `readFileBinary` function with signature `readFileBinary(string)` and selector `0x16ed7bc4`
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
    #[ethcall(name = "readFileBinary", abi = "readFileBinary(string)")]
    pub struct ReadFileBinaryCall {
        pub path: ::std::string::String,
    }
    ///Container type for all input parameters for the `readLine` function with signature `readLine(string)` and selector `0x70f55728`
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
    #[ethcall(name = "readLine", abi = "readLine(string)")]
    pub struct ReadLineCall {
        pub path: ::std::string::String,
    }
    ///Container type for all input parameters for the `readLink` function with signature `readLink(string)` and selector `0x9f5684a2`
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
    #[ethcall(name = "readLink", abi = "readLink(string)")]
    pub struct ReadLinkCall {
        pub link_path: ::std::string::String,
    }
    ///Container type for all input parameters for the `record` function with signature `record()` and selector `0x266cf109`
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
    #[ethcall(name = "record", abi = "record()")]
    pub struct RecordCall;
    ///Container type for all input parameters for the `recordLogs` function with signature `recordLogs()` and selector `0x41af2f52`
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
    #[ethcall(name = "recordLogs", abi = "recordLogs()")]
    pub struct RecordLogsCall;
    ///Container type for all input parameters for the `rememberKey` function with signature `rememberKey(uint256)` and selector `0x22100064`
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
    #[ethcall(name = "rememberKey", abi = "rememberKey(uint256)")]
    pub struct RememberKeyCall {
        pub private_key: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeDir` function with signature `removeDir(string,bool)` and selector `0x45c62011`
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
    #[ethcall(name = "removeDir", abi = "removeDir(string,bool)")]
    pub struct RemoveDirCall {
        pub path: ::std::string::String,
        pub recursive: bool,
    }
    ///Container type for all input parameters for the `removeFile` function with signature `removeFile(string)` and selector `0xf1afe04d`
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
    #[ethcall(name = "removeFile", abi = "removeFile(string)")]
    pub struct RemoveFileCall {
        pub path: ::std::string::String,
    }
    ///Container type for all input parameters for the `resetNonce` function with signature `resetNonce(address)` and selector `0x1c72346d`
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
    #[ethcall(name = "resetNonce", abi = "resetNonce(address)")]
    pub struct ResetNonceCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `resumeGasMetering` function with signature `resumeGasMetering()` and selector `0x2bcd50e0`
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
    #[ethcall(name = "resumeGasMetering", abi = "resumeGasMetering()")]
    pub struct ResumeGasMeteringCall;
    ///Container type for all input parameters for the `revertTo` function with signature `revertTo(uint256)` and selector `0x44d7f0a4`
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
    #[ethcall(name = "revertTo", abi = "revertTo(uint256)")]
    pub struct RevertToCall {
        pub snapshot_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `revertToAndDelete` function with signature `revertToAndDelete(uint256)` and selector `0x03e0aca9`
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
    #[ethcall(name = "revertToAndDelete", abi = "revertToAndDelete(uint256)")]
    pub struct RevertToAndDeleteCall {
        pub snapshot_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `revokePersistent` function with signature `revokePersistent(address[])` and selector `0x3ce969e6`
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
    #[ethcall(name = "revokePersistent", abi = "revokePersistent(address[])")]
    pub struct RevokePersistentCall {
        pub accounts: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `revokePersistent` function with signature `revokePersistent(address)` and selector `0x997a0222`
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
    #[ethcall(name = "revokePersistent", abi = "revokePersistent(address)")]
    pub struct RevokePersistentWithAccountCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `roll` function with signature `roll(uint256)` and selector `0x1f7b4f30`
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
    #[ethcall(name = "roll", abi = "roll(uint256)")]
    pub struct RollCall {
        pub new_height: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `rollFork` function with signature `rollFork(bytes32)` and selector `0x0f29772b`
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
    #[ethcall(name = "rollFork", abi = "rollFork(bytes32)")]
    pub struct RollFork0Call {
        pub tx_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `rollFork` function with signature `rollFork(uint256,uint256)` and selector `0xd74c83a4`
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
    #[ethcall(name = "rollFork", abi = "rollFork(uint256,uint256)")]
    pub struct RollFork2Call {
        pub fork_id: ::ethers::core::types::U256,
        pub block_number: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `rollFork` function with signature `rollFork(uint256)` and selector `0xd9bbf3a1`
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
    #[ethcall(name = "rollFork", abi = "rollFork(uint256)")]
    pub struct RollFork1Call {
        pub block_number: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `rollFork` function with signature `rollFork(uint256,bytes32)` and selector `0xf2830f7b`
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
    #[ethcall(name = "rollFork", abi = "rollFork(uint256,bytes32)")]
    pub struct RollFork3Call {
        pub fork_id: ::ethers::core::types::U256,
        pub tx_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `rpc` function with signature `rpc(string,string)` and selector `0x1206c8a8`
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
    #[ethcall(name = "rpc", abi = "rpc(string,string)")]
    pub struct RpcCall {
        pub method: ::std::string::String,
        pub params: ::std::string::String,
    }
    ///Container type for all input parameters for the `rpcUrl` function with signature `rpcUrl(string)` and selector `0x975a6ce9`
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
    #[ethcall(name = "rpcUrl", abi = "rpcUrl(string)")]
    pub struct RpcUrlCall {
        pub rpc_alias: ::std::string::String,
    }
    ///Container type for all input parameters for the `rpcUrlStructs` function with signature `rpcUrlStructs()` and selector `0x9d2ad72a`
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
    #[ethcall(name = "rpcUrlStructs", abi = "rpcUrlStructs()")]
    pub struct RpcUrlStructsCall;
    ///Container type for all input parameters for the `rpcUrls` function with signature `rpcUrls()` and selector `0xa85a8418`
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
    #[ethcall(name = "rpcUrls", abi = "rpcUrls()")]
    pub struct RpcUrlsCall;
    ///Container type for all input parameters for the `selectFork` function with signature `selectFork(uint256)` and selector `0x9ebf6827`
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
    #[ethcall(name = "selectFork", abi = "selectFork(uint256)")]
    pub struct SelectForkCall {
        pub fork_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `serializeAddress` function with signature `serializeAddress(string,string,address[])` and selector `0x1e356e1a`
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
    #[ethcall(
        name = "serializeAddress",
        abi = "serializeAddress(string,string,address[])"
    )]
    pub struct SerializeAddressCall {
        pub object_key: ::std::string::String,
        pub value_key: ::std::string::String,
        pub values: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `serializeAddress` function with signature `serializeAddress(string,string,address)` and selector `0x972c6062`
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
    #[ethcall(
        name = "serializeAddress",
        abi = "serializeAddress(string,string,address)"
    )]
    pub struct SerializeAddressWithObjectKeyAndValueKeyAndValueCall {
        pub object_key: ::std::string::String,
        pub value_key: ::std::string::String,
        pub value: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `serializeBool` function with signature `serializeBool(string,string,bool[])` and selector `0x92925aa1`
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
    #[ethcall(name = "serializeBool", abi = "serializeBool(string,string,bool[])")]
    pub struct SerializeBoolCall {
        pub object_key: ::std::string::String,
        pub value_key: ::std::string::String,
        pub values: ::std::vec::Vec<bool>,
    }
    ///Container type for all input parameters for the `serializeBool` function with signature `serializeBool(string,string,bool)` and selector `0xac22e971`
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
    #[ethcall(name = "serializeBool", abi = "serializeBool(string,string,bool)")]
    pub struct SerializeBoolWithObjectKeyAndValueKeyAndValueCall {
        pub object_key: ::std::string::String,
        pub value_key: ::std::string::String,
        pub value: bool,
    }
    ///Container type for all input parameters for the `serializeBytes` function with signature `serializeBytes(string,string,bytes[])` and selector `0x9884b232`
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
    #[ethcall(name = "serializeBytes", abi = "serializeBytes(string,string,bytes[])")]
    pub struct SerializeBytesCall {
        pub object_key: ::std::string::String,
        pub value_key: ::std::string::String,
        pub values: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `serializeBytes` function with signature `serializeBytes(string,string,bytes)` and selector `0xf21d52c7`
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
    #[ethcall(name = "serializeBytes", abi = "serializeBytes(string,string,bytes)")]
    pub struct SerializeBytesWithObjectKeyAndValueKeyAndValueCall {
        pub object_key: ::std::string::String,
        pub value_key: ::std::string::String,
        pub value: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `serializeBytes32` function with signature `serializeBytes32(string,string,bytes32[])` and selector `0x201e43e2`
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
    #[ethcall(
        name = "serializeBytes32",
        abi = "serializeBytes32(string,string,bytes32[])"
    )]
    pub struct SerializeBytes32Call {
        pub object_key: ::std::string::String,
        pub value_key: ::std::string::String,
        pub values: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `serializeBytes32` function with signature `serializeBytes32(string,string,bytes32)` and selector `0x2d812b44`
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
    #[ethcall(
        name = "serializeBytes32",
        abi = "serializeBytes32(string,string,bytes32)"
    )]
    pub struct SerializeBytes32WithObjectKeyAndValueKeyAndValueCall {
        pub object_key: ::std::string::String,
        pub value_key: ::std::string::String,
        pub value: [u8; 32],
    }
    ///Container type for all input parameters for the `serializeInt` function with signature `serializeInt(string,string,int256)` and selector `0x3f33db60`
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
    #[ethcall(name = "serializeInt", abi = "serializeInt(string,string,int256)")]
    pub struct SerializeIntCall {
        pub object_key: ::std::string::String,
        pub value_key: ::std::string::String,
        pub value: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `serializeInt` function with signature `serializeInt(string,string,int256[])` and selector `0x7676e127`
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
    #[ethcall(name = "serializeInt", abi = "serializeInt(string,string,int256[])")]
    pub struct SerializeIntWithObjectKeyAndValueKeyAndValuesCall {
        pub object_key: ::std::string::String,
        pub value_key: ::std::string::String,
        pub values: ::std::vec::Vec<::ethers::core::types::I256>,
    }
    ///Container type for all input parameters for the `serializeJson` function with signature `serializeJson(string,string)` and selector `0x9b3358b0`
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
    #[ethcall(name = "serializeJson", abi = "serializeJson(string,string)")]
    pub struct SerializeJsonCall {
        pub object_key: ::std::string::String,
        pub value: ::std::string::String,
    }
    ///Container type for all input parameters for the `serializeString` function with signature `serializeString(string,string,string[])` and selector `0x561cd6f3`
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
    #[ethcall(name = "serializeString", abi = "serializeString(string,string,string[])")]
    pub struct SerializeStringCall {
        pub object_key: ::std::string::String,
        pub value_key: ::std::string::String,
        pub values: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all input parameters for the `serializeString` function with signature `serializeString(string,string,string)` and selector `0x88da6d35`
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
    #[ethcall(name = "serializeString", abi = "serializeString(string,string,string)")]
    pub struct SerializeStringWithObjectKeyAndValueKeyAndValueCall {
        pub object_key: ::std::string::String,
        pub value_key: ::std::string::String,
        pub value: ::std::string::String,
    }
    ///Container type for all input parameters for the `serializeUint` function with signature `serializeUint(string,string,uint256)` and selector `0x129e9002`
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
    #[ethcall(name = "serializeUint", abi = "serializeUint(string,string,uint256)")]
    pub struct SerializeUintCall {
        pub object_key: ::std::string::String,
        pub value_key: ::std::string::String,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `serializeUint` function with signature `serializeUint(string,string,uint256[])` and selector `0xfee9a469`
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
    #[ethcall(name = "serializeUint", abi = "serializeUint(string,string,uint256[])")]
    pub struct SerializeUintWithObjectKeyAndValueKeyAndValuesCall {
        pub object_key: ::std::string::String,
        pub value_key: ::std::string::String,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `setEnv` function with signature `setEnv(string,string)` and selector `0x3d5923ee`
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
    #[ethcall(name = "setEnv", abi = "setEnv(string,string)")]
    pub struct SetEnvCall {
        pub name: ::std::string::String,
        pub value: ::std::string::String,
    }
    ///Container type for all input parameters for the `setNonce` function with signature `setNonce(address,uint64)` and selector `0xf8e18b57`
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
    #[ethcall(name = "setNonce", abi = "setNonce(address,uint64)")]
    pub struct SetNonceCall {
        pub account: ::ethers::core::types::Address,
        pub new_nonce: u64,
    }
    ///Container type for all input parameters for the `setNonceUnsafe` function with signature `setNonceUnsafe(address,uint64)` and selector `0x9b67b21c`
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
    #[ethcall(name = "setNonceUnsafe", abi = "setNonceUnsafe(address,uint64)")]
    pub struct SetNonceUnsafeCall {
        pub account: ::ethers::core::types::Address,
        pub new_nonce: u64,
    }
    ///Container type for all input parameters for the `sign` function with signature `sign((address,uint256,uint256,uint256),bytes32)` and selector `0xb25c5a25`
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
    #[ethcall(name = "sign", abi = "sign((address,uint256,uint256,uint256),bytes32)")]
    pub struct SignCall {
        pub wallet: Wallet,
        pub digest: [u8; 32],
    }
    ///Container type for all input parameters for the `sign` function with signature `sign(uint256,bytes32)` and selector `0xe341eaa4`
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
    #[ethcall(name = "sign", abi = "sign(uint256,bytes32)")]
    pub struct SignWithPrivateKeyCall {
        pub private_key: ::ethers::core::types::U256,
        pub digest: [u8; 32],
    }
    ///Container type for all input parameters for the `signP256` function with signature `signP256(uint256,bytes32)` and selector `0x83211b40`
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
    #[ethcall(name = "signP256", abi = "signP256(uint256,bytes32)")]
    pub struct SignP256Call {
        pub private_key: ::ethers::core::types::U256,
        pub digest: [u8; 32],
    }
    ///Container type for all input parameters for the `skip` function with signature `skip(bool)` and selector `0xdd82d13e`
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
    #[ethcall(name = "skip", abi = "skip(bool)")]
    pub struct SkipCall {
        pub skip_test: bool,
    }
    ///Container type for all input parameters for the `sleep` function with signature `sleep(uint256)` and selector `0xfa9d8713`
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
    #[ethcall(name = "sleep", abi = "sleep(uint256)")]
    pub struct SleepCall {
        pub duration: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `snapshot` function with signature `snapshot()` and selector `0x9711715a`
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
    #[ethcall(name = "snapshot", abi = "snapshot()")]
    pub struct SnapshotCall;
    ///Container type for all input parameters for the `startBroadcast` function with signature `startBroadcast()` and selector `0x7fb5297f`
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
    #[ethcall(name = "startBroadcast", abi = "startBroadcast()")]
    pub struct StartBroadcastCall;
    ///Container type for all input parameters for the `startBroadcast` function with signature `startBroadcast(address)` and selector `0x7fec2a8d`
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
    #[ethcall(name = "startBroadcast", abi = "startBroadcast(address)")]
    pub struct StartBroadcastWithSignerCall {
        pub signer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `startBroadcast` function with signature `startBroadcast(uint256)` and selector `0xce817d47`
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
    #[ethcall(name = "startBroadcast", abi = "startBroadcast(uint256)")]
    pub struct StartBroadcastWithPrivateKeyCall {
        pub private_key: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `startMappingRecording` function with signature `startMappingRecording()` and selector `0x3e9705c0`
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
    #[ethcall(name = "startMappingRecording", abi = "startMappingRecording()")]
    pub struct StartMappingRecordingCall;
    ///Container type for all input parameters for the `startPrank` function with signature `startPrank(address)` and selector `0x06447d56`
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
    #[ethcall(name = "startPrank", abi = "startPrank(address)")]
    pub struct StartPrankCall {
        pub msg_sender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `startPrank` function with signature `startPrank(address,address)` and selector `0x45b56078`
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
    #[ethcall(name = "startPrank", abi = "startPrank(address,address)")]
    pub struct StartPrankWithTxOriginCall {
        pub msg_sender: ::ethers::core::types::Address,
        pub tx_origin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `startStateDiffRecording` function with signature `startStateDiffRecording()` and selector `0xcf22e3c9`
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
    #[ethcall(name = "startStateDiffRecording", abi = "startStateDiffRecording()")]
    pub struct StartStateDiffRecordingCall;
    ///Container type for all input parameters for the `stopAndReturnStateDiff` function with signature `stopAndReturnStateDiff()` and selector `0xaa5cf90e`
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
    #[ethcall(name = "stopAndReturnStateDiff", abi = "stopAndReturnStateDiff()")]
    pub struct StopAndReturnStateDiffCall;
    ///Container type for all input parameters for the `stopBroadcast` function with signature `stopBroadcast()` and selector `0x76eadd36`
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
    #[ethcall(name = "stopBroadcast", abi = "stopBroadcast()")]
    pub struct StopBroadcastCall;
    ///Container type for all input parameters for the `stopMappingRecording` function with signature `stopMappingRecording()` and selector `0x0d4aae9b`
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
    #[ethcall(name = "stopMappingRecording", abi = "stopMappingRecording()")]
    pub struct StopMappingRecordingCall;
    ///Container type for all input parameters for the `stopPrank` function with signature `stopPrank()` and selector `0x90c5013b`
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
    #[ethcall(name = "stopPrank", abi = "stopPrank()")]
    pub struct StopPrankCall;
    ///Container type for all input parameters for the `store` function with signature `store(address,bytes32,bytes32)` and selector `0x70ca10bb`
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
    #[ethcall(name = "store", abi = "store(address,bytes32,bytes32)")]
    pub struct StoreCall {
        pub target: ::ethers::core::types::Address,
        pub slot: [u8; 32],
        pub value: [u8; 32],
    }
    ///Container type for all input parameters for the `toBase64` function with signature `toBase64(string)` and selector `0x3f8be2c8`
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
    #[ethcall(name = "toBase64", abi = "toBase64(string)")]
    pub struct ToBase64Call {
        pub data: ::std::string::String,
    }
    ///Container type for all input parameters for the `toBase64` function with signature `toBase64(bytes)` and selector `0xa5cbfe65`
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
    #[ethcall(name = "toBase64", abi = "toBase64(bytes)")]
    pub struct ToBase64WithDataCall {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `toBase64URL` function with signature `toBase64URL(string)` and selector `0xae3165b3`
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
    #[ethcall(name = "toBase64URL", abi = "toBase64URL(string)")]
    pub struct ToBase64URLCall {
        pub data: ::std::string::String,
    }
    ///Container type for all input parameters for the `toBase64URL` function with signature `toBase64URL(bytes)` and selector `0xc8bd0e4a`
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
    #[ethcall(name = "toBase64URL", abi = "toBase64URL(bytes)")]
    pub struct ToBase64UrlWithDataCall {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `toString` function with signature `toString(address)` and selector `0x56ca623e`
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
    #[ethcall(name = "toString", abi = "toString(address)")]
    pub struct ToString0Call {
        pub value: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `toString` function with signature `toString(uint256)` and selector `0x6900a3ae`
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
    #[ethcall(name = "toString", abi = "toString(uint256)")]
    pub struct ToString1Call {
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `toString` function with signature `toString(bytes)` and selector `0x71aad10d`
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
    #[ethcall(name = "toString", abi = "toString(bytes)")]
    pub struct ToString2Call {
        pub value: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `toString` function with signature `toString(bool)` and selector `0x71dce7da`
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
    #[ethcall(name = "toString", abi = "toString(bool)")]
    pub struct ToString3Call {
        pub value: bool,
    }
    ///Container type for all input parameters for the `toString` function with signature `toString(int256)` and selector `0xa322c40e`
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
    #[ethcall(name = "toString", abi = "toString(int256)")]
    pub struct ToString4Call {
        pub value: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `toString` function with signature `toString(bytes32)` and selector `0xb11a19e8`
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
    #[ethcall(name = "toString", abi = "toString(bytes32)")]
    pub struct ToString5Call {
        pub value: [u8; 32],
    }
    ///Container type for all input parameters for the `transact` function with signature `transact(uint256,bytes32)` and selector `0x4d8abc4b`
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
    #[ethcall(name = "transact", abi = "transact(uint256,bytes32)")]
    pub struct TransactWithForkIdCall {
        pub fork_id: ::ethers::core::types::U256,
        pub tx_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `transact` function with signature `transact(bytes32)` and selector `0xbe646da1`
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
    #[ethcall(name = "transact", abi = "transact(bytes32)")]
    pub struct TransactCall {
        pub tx_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `tryFfi` function with signature `tryFfi(string[])` and selector `0xf45c1ce7`
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
    #[ethcall(name = "tryFfi", abi = "tryFfi(string[])")]
    pub struct TryFfiCall {
        pub command_input: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all input parameters for the `txGasPrice` function with signature `txGasPrice(uint256)` and selector `0x48f50c0f`
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
    #[ethcall(name = "txGasPrice", abi = "txGasPrice(uint256)")]
    pub struct TxGasPriceCall {
        pub new_gas_price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `unixTime` function with signature `unixTime()` and selector `0x625387dc`
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
    #[ethcall(name = "unixTime", abi = "unixTime()")]
    pub struct UnixTimeCall;
    ///Container type for all input parameters for the `warp` function with signature `warp(uint256)` and selector `0xe5d6bf02`
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
    #[ethcall(name = "warp", abi = "warp(uint256)")]
    pub struct WarpCall {
        pub new_timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `writeFile` function with signature `writeFile(string,string)` and selector `0x897e0a97`
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
    #[ethcall(name = "writeFile", abi = "writeFile(string,string)")]
    pub struct WriteFileCall {
        pub path: ::std::string::String,
        pub data: ::std::string::String,
    }
    ///Container type for all input parameters for the `writeFileBinary` function with signature `writeFileBinary(string,bytes)` and selector `0x1f21fc80`
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
    #[ethcall(name = "writeFileBinary", abi = "writeFileBinary(string,bytes)")]
    pub struct WriteFileBinaryCall {
        pub path: ::std::string::String,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `writeJson` function with signature `writeJson(string,string,string)` and selector `0x35d6ad46`
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
    #[ethcall(name = "writeJson", abi = "writeJson(string,string,string)")]
    pub struct WriteJsonWithJsonAndPathCall {
        pub json: ::std::string::String,
        pub path: ::std::string::String,
        pub value_key: ::std::string::String,
    }
    ///Container type for all input parameters for the `writeJson` function with signature `writeJson(string,string)` and selector `0xe23cd19f`
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
    #[ethcall(name = "writeJson", abi = "writeJson(string,string)")]
    pub struct WriteJsonCall {
        pub json: ::std::string::String,
        pub path: ::std::string::String,
    }
    ///Container type for all input parameters for the `writeLine` function with signature `writeLine(string,string)` and selector `0x619d897f`
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
    #[ethcall(name = "writeLine", abi = "writeLine(string,string)")]
    pub struct WriteLineCall {
        pub path: ::std::string::String,
        pub data: ::std::string::String,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VmCalls {
        Accesses(AccessesCall),
        ActiveFork(ActiveForkCall),
        Addr(AddrCall),
        AllowCheatcodes(AllowCheatcodesCall),
        Assume(AssumeCall),
        Breakpoint(BreakpointCall),
        BreakpointWithValue(BreakpointWithValueCall),
        Broadcast(BroadcastCall),
        BroadcastWithSigner(BroadcastWithSignerCall),
        BroadcastWithPrivateKey(BroadcastWithPrivateKeyCall),
        ChainId(ChainIdCall),
        ClearMockedCalls(ClearMockedCallsCall),
        CloseFile(CloseFileCall),
        Coinbase(CoinbaseCall),
        ComputeCreate2Address(ComputeCreate2AddressCall),
        ComputeCreate2AddressWithSaltAndInitCodeHash(
            ComputeCreate2AddressWithSaltAndInitCodeHashCall,
        ),
        ComputeCreateAddress(ComputeCreateAddressCall),
        CopyFile(CopyFileCall),
        CreateDir(CreateDirCall),
        CreateFork(CreateForkCall),
        CreateForkWithBlockNumber(CreateForkWithBlockNumberCall),
        CreateForkWithTxHash(CreateForkWithTxHashCall),
        CreateSelectForkWithBlockNumber(CreateSelectForkWithBlockNumberCall),
        CreateSelectForkWithTxHash(CreateSelectForkWithTxHashCall),
        CreateSelectFork(CreateSelectForkCall),
        CreateWallet0(CreateWallet0Call),
        CreateWallet1(CreateWallet1Call),
        CreateWallet2(CreateWallet2Call),
        Deal(DealCall),
        DeleteSnapshot(DeleteSnapshotCall),
        DeleteSnapshots(DeleteSnapshotsCall),
        DeriveKey3(DeriveKey3Call),
        DeriveKey1(DeriveKey1Call),
        DeriveKey0(DeriveKey0Call),
        DeriveKey2(DeriveKey2Call),
        Difficulty(DifficultyCall),
        DumpState(DumpStateCall),
        EnvAddress(EnvAddressCall),
        EnvAddressWithDelim(EnvAddressWithDelimCall),
        EnvBool(EnvBoolCall),
        EnvBoolWithDelim(EnvBoolWithDelimCall),
        EnvBytes(EnvBytesCall),
        EnvBytesWithDelim(EnvBytesWithDelimCall),
        EnvBytes32WithDelim(EnvBytes32WithDelimCall),
        EnvBytes32(EnvBytes32Call),
        EnvIntWithDelim(EnvIntWithDelimCall),
        EnvInt(EnvIntCall),
        EnvOr7(EnvOr7Call),
        EnvOr8(EnvOr8Call),
        EnvOr0(EnvOr0Call),
        EnvOr1(EnvOr1Call),
        EnvOr2(EnvOr2Call),
        EnvOr9(EnvOr9Call),
        EnvOr10(EnvOr10Call),
        EnvOr11(EnvOr11Call),
        EnvOr3(EnvOr3Call),
        EnvOr4(EnvOr4Call),
        EnvOr5(EnvOr5Call),
        EnvOr12(EnvOr12Call),
        EnvOr6(EnvOr6Call),
        EnvOr13(EnvOr13Call),
        EnvStringWithDelim(EnvStringWithDelimCall),
        EnvString(EnvStringCall),
        EnvUint(EnvUintCall),
        EnvUintWithDelim(EnvUintWithDelimCall),
        Etch(EtchCall),
        EthGetLogs(EthGetLogsCall),
        Exists(ExistsCall),
        ExpectCall3(ExpectCall3Call),
        ExpectCall5(ExpectCall5Call),
        ExpectCall4(ExpectCall4Call),
        ExpectCall0(ExpectCall0Call),
        ExpectCall1(ExpectCall1Call),
        ExpectCall2(ExpectCall2Call),
        ExpectCallMinGas(ExpectCallMinGasCall),
        ExpectCallMinGasWithCalleeAndMsgValueAndMinGasAndData(
            ExpectCallMinGasWithCalleeAndMsgValueAndMinGasAndDataCall,
        ),
        ExpectEmit0(ExpectEmit0Call),
        ExpectEmit2(ExpectEmit2Call),
        ExpectEmit3(ExpectEmit3Call),
        ExpectEmit1(ExpectEmit1Call),
        ExpectRevert1(ExpectRevert1Call),
        ExpectRevert2(ExpectRevert2Call),
        ExpectRevert0(ExpectRevert0Call),
        ExpectSafeMemory(ExpectSafeMemoryCall),
        ExpectSafeMemoryCall(ExpectSafeMemoryCallCall),
        Fee(FeeCall),
        Ffi(FfiCall),
        FsMetadata(FsMetadataCall),
        GetBlockNumber(GetBlockNumberCall),
        GetBlockTimestamp(GetBlockTimestampCall),
        GetCode(GetCodeCall),
        GetDeployedCode(GetDeployedCodeCall),
        GetLabel(GetLabelCall),
        GetMappingKeyAndParentOf(GetMappingKeyAndParentOfCall),
        GetMappingLength(GetMappingLengthCall),
        GetMappingSlotAt(GetMappingSlotAtCall),
        GetNonce(GetNonceCall),
        GetNonceWithWallet(GetNonceWithWalletCall),
        GetRecordedLogs(GetRecordedLogsCall),
        IsDir(IsDirCall),
        IsFile(IsFileCall),
        IsPersistent(IsPersistentCall),
        KeyExists(KeyExistsCall),
        Label(LabelCall),
        Load(LoadCall),
        LoadAllocs(LoadAllocsCall),
        MakePersistent0(MakePersistent0Call),
        MakePersistent2(MakePersistent2Call),
        MakePersistent1(MakePersistent1Call),
        MakePersistent3(MakePersistent3Call),
        MockCallWithCalleeAndMsgValueAndData(MockCallWithCalleeAndMsgValueAndDataCall),
        MockCall(MockCallCall),
        MockCallRevertWithCalleeAndMsgValueAndData(
            MockCallRevertWithCalleeAndMsgValueAndDataCall,
        ),
        MockCallRevert(MockCallRevertCall),
        ParseAddress(ParseAddressCall),
        ParseBool(ParseBoolCall),
        ParseBytes(ParseBytesCall),
        ParseBytes32(ParseBytes32Call),
        ParseInt(ParseIntCall),
        ParseJson(ParseJsonCall),
        ParseJsonWithKey(ParseJsonWithKeyCall),
        ParseJsonAddress(ParseJsonAddressCall),
        ParseJsonAddressArray(ParseJsonAddressArrayCall),
        ParseJsonBool(ParseJsonBoolCall),
        ParseJsonBoolArray(ParseJsonBoolArrayCall),
        ParseJsonBytes(ParseJsonBytesCall),
        ParseJsonBytes32(ParseJsonBytes32Call),
        ParseJsonBytes32Array(ParseJsonBytes32ArrayCall),
        ParseJsonBytesArray(ParseJsonBytesArrayCall),
        ParseJsonInt(ParseJsonIntCall),
        ParseJsonIntArray(ParseJsonIntArrayCall),
        ParseJsonKeys(ParseJsonKeysCall),
        ParseJsonString(ParseJsonStringCall),
        ParseJsonStringArray(ParseJsonStringArrayCall),
        ParseJsonUint(ParseJsonUintCall),
        ParseJsonUintArray(ParseJsonUintArrayCall),
        ParseUint(ParseUintCall),
        PauseGasMetering(PauseGasMeteringCall),
        PrankWithTxOrigin(PrankWithTxOriginCall),
        Prank(PrankCall),
        Prevrandao(PrevrandaoCall),
        ProjectRoot(ProjectRootCall),
        ReadCallers(ReadCallersCall),
        ReadDir1(ReadDir1Call),
        ReadDir2(ReadDir2Call),
        ReadDir0(ReadDir0Call),
        ReadFile(ReadFileCall),
        ReadFileBinary(ReadFileBinaryCall),
        ReadLine(ReadLineCall),
        ReadLink(ReadLinkCall),
        Record(RecordCall),
        RecordLogs(RecordLogsCall),
        RememberKey(RememberKeyCall),
        RemoveDir(RemoveDirCall),
        RemoveFile(RemoveFileCall),
        ResetNonce(ResetNonceCall),
        ResumeGasMetering(ResumeGasMeteringCall),
        RevertTo(RevertToCall),
        RevertToAndDelete(RevertToAndDeleteCall),
        RevokePersistent(RevokePersistentCall),
        RevokePersistentWithAccount(RevokePersistentWithAccountCall),
        Roll(RollCall),
        RollFork0(RollFork0Call),
        RollFork2(RollFork2Call),
        RollFork1(RollFork1Call),
        RollFork3(RollFork3Call),
        Rpc(RpcCall),
        RpcUrl(RpcUrlCall),
        RpcUrlStructs(RpcUrlStructsCall),
        RpcUrls(RpcUrlsCall),
        SelectFork(SelectForkCall),
        SerializeAddress(SerializeAddressCall),
        SerializeAddressWithObjectKeyAndValueKeyAndValue(
            SerializeAddressWithObjectKeyAndValueKeyAndValueCall,
        ),
        SerializeBool(SerializeBoolCall),
        SerializeBoolWithObjectKeyAndValueKeyAndValue(
            SerializeBoolWithObjectKeyAndValueKeyAndValueCall,
        ),
        SerializeBytes(SerializeBytesCall),
        SerializeBytesWithObjectKeyAndValueKeyAndValue(
            SerializeBytesWithObjectKeyAndValueKeyAndValueCall,
        ),
        SerializeBytes32(SerializeBytes32Call),
        SerializeBytes32WithObjectKeyAndValueKeyAndValue(
            SerializeBytes32WithObjectKeyAndValueKeyAndValueCall,
        ),
        SerializeInt(SerializeIntCall),
        SerializeIntWithObjectKeyAndValueKeyAndValues(
            SerializeIntWithObjectKeyAndValueKeyAndValuesCall,
        ),
        SerializeJson(SerializeJsonCall),
        SerializeString(SerializeStringCall),
        SerializeStringWithObjectKeyAndValueKeyAndValue(
            SerializeStringWithObjectKeyAndValueKeyAndValueCall,
        ),
        SerializeUint(SerializeUintCall),
        SerializeUintWithObjectKeyAndValueKeyAndValues(
            SerializeUintWithObjectKeyAndValueKeyAndValuesCall,
        ),
        SetEnv(SetEnvCall),
        SetNonce(SetNonceCall),
        SetNonceUnsafe(SetNonceUnsafeCall),
        Sign(SignCall),
        SignWithPrivateKey(SignWithPrivateKeyCall),
        SignP256(SignP256Call),
        Skip(SkipCall),
        Sleep(SleepCall),
        Snapshot(SnapshotCall),
        StartBroadcast(StartBroadcastCall),
        StartBroadcastWithSigner(StartBroadcastWithSignerCall),
        StartBroadcastWithPrivateKey(StartBroadcastWithPrivateKeyCall),
        StartMappingRecording(StartMappingRecordingCall),
        StartPrank(StartPrankCall),
        StartPrankWithTxOrigin(StartPrankWithTxOriginCall),
        StartStateDiffRecording(StartStateDiffRecordingCall),
        StopAndReturnStateDiff(StopAndReturnStateDiffCall),
        StopBroadcast(StopBroadcastCall),
        StopMappingRecording(StopMappingRecordingCall),
        StopPrank(StopPrankCall),
        Store(StoreCall),
        ToBase64(ToBase64Call),
        ToBase64WithData(ToBase64WithDataCall),
        ToBase64URL(ToBase64URLCall),
        ToBase64UrlWithData(ToBase64UrlWithDataCall),
        ToString0(ToString0Call),
        ToString1(ToString1Call),
        ToString2(ToString2Call),
        ToString3(ToString3Call),
        ToString4(ToString4Call),
        ToString5(ToString5Call),
        TransactWithForkId(TransactWithForkIdCall),
        Transact(TransactCall),
        TryFfi(TryFfiCall),
        TxGasPrice(TxGasPriceCall),
        UnixTime(UnixTimeCall),
        Warp(WarpCall),
        WriteFile(WriteFileCall),
        WriteFileBinary(WriteFileBinaryCall),
        WriteJsonWithJsonAndPath(WriteJsonWithJsonAndPathCall),
        WriteJson(WriteJsonCall),
        WriteLine(WriteLineCall),
    }
    impl ::ethers::core::abi::AbiDecode for VmCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AccessesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Accesses(decoded));
            }
            if let Ok(decoded) = <ActiveForkCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ActiveFork(decoded));
            }
            if let Ok(decoded) = <AddrCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Addr(decoded));
            }
            if let Ok(decoded) = <AllowCheatcodesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllowCheatcodes(decoded));
            }
            if let Ok(decoded) = <AssumeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Assume(decoded));
            }
            if let Ok(decoded) = <BreakpointCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Breakpoint(decoded));
            }
            if let Ok(decoded) = <BreakpointWithValueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BreakpointWithValue(decoded));
            }
            if let Ok(decoded) = <BroadcastCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Broadcast(decoded));
            }
            if let Ok(decoded) = <BroadcastWithSignerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BroadcastWithSigner(decoded));
            }
            if let Ok(decoded) = <BroadcastWithPrivateKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BroadcastWithPrivateKey(decoded));
            }
            if let Ok(decoded) = <ChainIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChainId(decoded));
            }
            if let Ok(decoded) = <ClearMockedCallsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClearMockedCalls(decoded));
            }
            if let Ok(decoded) = <CloseFileCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CloseFile(decoded));
            }
            if let Ok(decoded) = <CoinbaseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Coinbase(decoded));
            }
            if let Ok(decoded) = <ComputeCreate2AddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeCreate2Address(decoded));
            }
            if let Ok(decoded) = <ComputeCreate2AddressWithSaltAndInitCodeHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeCreate2AddressWithSaltAndInitCodeHash(decoded));
            }
            if let Ok(decoded) = <ComputeCreateAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeCreateAddress(decoded));
            }
            if let Ok(decoded) = <CopyFileCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CopyFile(decoded));
            }
            if let Ok(decoded) = <CreateDirCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateDir(decoded));
            }
            if let Ok(decoded) = <CreateForkCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateFork(decoded));
            }
            if let Ok(decoded) = <CreateForkWithBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateForkWithBlockNumber(decoded));
            }
            if let Ok(decoded) = <CreateForkWithTxHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateForkWithTxHash(decoded));
            }
            if let Ok(decoded) = <CreateSelectForkWithBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateSelectForkWithBlockNumber(decoded));
            }
            if let Ok(decoded) = <CreateSelectForkWithTxHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateSelectForkWithTxHash(decoded));
            }
            if let Ok(decoded) = <CreateSelectForkCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateSelectFork(decoded));
            }
            if let Ok(decoded) = <CreateWallet0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateWallet0(decoded));
            }
            if let Ok(decoded) = <CreateWallet1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateWallet1(decoded));
            }
            if let Ok(decoded) = <CreateWallet2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateWallet2(decoded));
            }
            if let Ok(decoded) = <DealCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deal(decoded));
            }
            if let Ok(decoded) = <DeleteSnapshotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeleteSnapshot(decoded));
            }
            if let Ok(decoded) = <DeleteSnapshotsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeleteSnapshots(decoded));
            }
            if let Ok(decoded) = <DeriveKey3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeriveKey3(decoded));
            }
            if let Ok(decoded) = <DeriveKey1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeriveKey1(decoded));
            }
            if let Ok(decoded) = <DeriveKey0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeriveKey0(decoded));
            }
            if let Ok(decoded) = <DeriveKey2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeriveKey2(decoded));
            }
            if let Ok(decoded) = <DifficultyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Difficulty(decoded));
            }
            if let Ok(decoded) = <DumpStateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DumpState(decoded));
            }
            if let Ok(decoded) = <EnvAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvAddress(decoded));
            }
            if let Ok(decoded) = <EnvAddressWithDelimCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvAddressWithDelim(decoded));
            }
            if let Ok(decoded) = <EnvBoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvBool(decoded));
            }
            if let Ok(decoded) = <EnvBoolWithDelimCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvBoolWithDelim(decoded));
            }
            if let Ok(decoded) = <EnvBytesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvBytes(decoded));
            }
            if let Ok(decoded) = <EnvBytesWithDelimCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvBytesWithDelim(decoded));
            }
            if let Ok(decoded) = <EnvBytes32WithDelimCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvBytes32WithDelim(decoded));
            }
            if let Ok(decoded) = <EnvBytes32Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvBytes32(decoded));
            }
            if let Ok(decoded) = <EnvIntWithDelimCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvIntWithDelim(decoded));
            }
            if let Ok(decoded) = <EnvIntCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvInt(decoded));
            }
            if let Ok(decoded) = <EnvOr7Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr7(decoded));
            }
            if let Ok(decoded) = <EnvOr8Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr8(decoded));
            }
            if let Ok(decoded) = <EnvOr0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr0(decoded));
            }
            if let Ok(decoded) = <EnvOr1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr1(decoded));
            }
            if let Ok(decoded) = <EnvOr2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr2(decoded));
            }
            if let Ok(decoded) = <EnvOr9Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr9(decoded));
            }
            if let Ok(decoded) = <EnvOr10Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr10(decoded));
            }
            if let Ok(decoded) = <EnvOr11Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr11(decoded));
            }
            if let Ok(decoded) = <EnvOr3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr3(decoded));
            }
            if let Ok(decoded) = <EnvOr4Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr4(decoded));
            }
            if let Ok(decoded) = <EnvOr5Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr5(decoded));
            }
            if let Ok(decoded) = <EnvOr12Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr12(decoded));
            }
            if let Ok(decoded) = <EnvOr6Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr6(decoded));
            }
            if let Ok(decoded) = <EnvOr13Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr13(decoded));
            }
            if let Ok(decoded) = <EnvStringWithDelimCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvStringWithDelim(decoded));
            }
            if let Ok(decoded) = <EnvStringCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvString(decoded));
            }
            if let Ok(decoded) = <EnvUintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvUint(decoded));
            }
            if let Ok(decoded) = <EnvUintWithDelimCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvUintWithDelim(decoded));
            }
            if let Ok(decoded) = <EtchCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Etch(decoded));
            }
            if let Ok(decoded) = <EthGetLogsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EthGetLogs(decoded));
            }
            if let Ok(decoded) = <ExistsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Exists(decoded));
            }
            if let Ok(decoded) = <ExpectCall3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectCall3(decoded));
            }
            if let Ok(decoded) = <ExpectCall5Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectCall5(decoded));
            }
            if let Ok(decoded) = <ExpectCall4Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectCall4(decoded));
            }
            if let Ok(decoded) = <ExpectCall0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectCall0(decoded));
            }
            if let Ok(decoded) = <ExpectCall1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectCall1(decoded));
            }
            if let Ok(decoded) = <ExpectCall2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectCall2(decoded));
            }
            if let Ok(decoded) = <ExpectCallMinGasCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectCallMinGas(decoded));
            }
            if let Ok(decoded) = <ExpectCallMinGasWithCalleeAndMsgValueAndMinGasAndDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::ExpectCallMinGasWithCalleeAndMsgValueAndMinGasAndData(decoded),
                );
            }
            if let Ok(decoded) = <ExpectEmit0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectEmit0(decoded));
            }
            if let Ok(decoded) = <ExpectEmit2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectEmit2(decoded));
            }
            if let Ok(decoded) = <ExpectEmit3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectEmit3(decoded));
            }
            if let Ok(decoded) = <ExpectEmit1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectEmit1(decoded));
            }
            if let Ok(decoded) = <ExpectRevert1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectRevert1(decoded));
            }
            if let Ok(decoded) = <ExpectRevert2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectRevert2(decoded));
            }
            if let Ok(decoded) = <ExpectRevert0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectRevert0(decoded));
            }
            if let Ok(decoded) = <ExpectSafeMemoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectSafeMemory(decoded));
            }
            if let Ok(decoded) = <ExpectSafeMemoryCallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectSafeMemoryCall(decoded));
            }
            if let Ok(decoded) = <FeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Fee(decoded));
            }
            if let Ok(decoded) = <FfiCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ffi(decoded));
            }
            if let Ok(decoded) = <FsMetadataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FsMetadata(decoded));
            }
            if let Ok(decoded) = <GetBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBlockNumber(decoded));
            }
            if let Ok(decoded) = <GetBlockTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBlockTimestamp(decoded));
            }
            if let Ok(decoded) = <GetCodeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCode(decoded));
            }
            if let Ok(decoded) = <GetDeployedCodeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDeployedCode(decoded));
            }
            if let Ok(decoded) = <GetLabelCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLabel(decoded));
            }
            if let Ok(decoded) = <GetMappingKeyAndParentOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMappingKeyAndParentOf(decoded));
            }
            if let Ok(decoded) = <GetMappingLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMappingLength(decoded));
            }
            if let Ok(decoded) = <GetMappingSlotAtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMappingSlotAt(decoded));
            }
            if let Ok(decoded) = <GetNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNonce(decoded));
            }
            if let Ok(decoded) = <GetNonceWithWalletCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNonceWithWallet(decoded));
            }
            if let Ok(decoded) = <GetRecordedLogsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRecordedLogs(decoded));
            }
            if let Ok(decoded) = <IsDirCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsDir(decoded));
            }
            if let Ok(decoded) = <IsFileCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsFile(decoded));
            }
            if let Ok(decoded) = <IsPersistentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsPersistent(decoded));
            }
            if let Ok(decoded) = <KeyExistsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::KeyExists(decoded));
            }
            if let Ok(decoded) = <LabelCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Label(decoded));
            }
            if let Ok(decoded) = <LoadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Load(decoded));
            }
            if let Ok(decoded) = <LoadAllocsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LoadAllocs(decoded));
            }
            if let Ok(decoded) = <MakePersistent0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MakePersistent0(decoded));
            }
            if let Ok(decoded) = <MakePersistent2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MakePersistent2(decoded));
            }
            if let Ok(decoded) = <MakePersistent1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MakePersistent1(decoded));
            }
            if let Ok(decoded) = <MakePersistent3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MakePersistent3(decoded));
            }
            if let Ok(decoded) = <MockCallWithCalleeAndMsgValueAndDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MockCallWithCalleeAndMsgValueAndData(decoded));
            }
            if let Ok(decoded) = <MockCallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MockCall(decoded));
            }
            if let Ok(decoded) = <MockCallRevertWithCalleeAndMsgValueAndDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MockCallRevertWithCalleeAndMsgValueAndData(decoded));
            }
            if let Ok(decoded) = <MockCallRevertCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MockCallRevert(decoded));
            }
            if let Ok(decoded) = <ParseAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseAddress(decoded));
            }
            if let Ok(decoded) = <ParseBoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseBool(decoded));
            }
            if let Ok(decoded) = <ParseBytesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseBytes(decoded));
            }
            if let Ok(decoded) = <ParseBytes32Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseBytes32(decoded));
            }
            if let Ok(decoded) = <ParseIntCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseInt(decoded));
            }
            if let Ok(decoded) = <ParseJsonCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJson(decoded));
            }
            if let Ok(decoded) = <ParseJsonWithKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonWithKey(decoded));
            }
            if let Ok(decoded) = <ParseJsonAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonAddress(decoded));
            }
            if let Ok(decoded) = <ParseJsonAddressArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonAddressArray(decoded));
            }
            if let Ok(decoded) = <ParseJsonBoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonBool(decoded));
            }
            if let Ok(decoded) = <ParseJsonBoolArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonBoolArray(decoded));
            }
            if let Ok(decoded) = <ParseJsonBytesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonBytes(decoded));
            }
            if let Ok(decoded) = <ParseJsonBytes32Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonBytes32(decoded));
            }
            if let Ok(decoded) = <ParseJsonBytes32ArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonBytes32Array(decoded));
            }
            if let Ok(decoded) = <ParseJsonBytesArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonBytesArray(decoded));
            }
            if let Ok(decoded) = <ParseJsonIntCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonInt(decoded));
            }
            if let Ok(decoded) = <ParseJsonIntArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonIntArray(decoded));
            }
            if let Ok(decoded) = <ParseJsonKeysCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonKeys(decoded));
            }
            if let Ok(decoded) = <ParseJsonStringCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonString(decoded));
            }
            if let Ok(decoded) = <ParseJsonStringArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonStringArray(decoded));
            }
            if let Ok(decoded) = <ParseJsonUintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonUint(decoded));
            }
            if let Ok(decoded) = <ParseJsonUintArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonUintArray(decoded));
            }
            if let Ok(decoded) = <ParseUintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseUint(decoded));
            }
            if let Ok(decoded) = <PauseGasMeteringCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PauseGasMetering(decoded));
            }
            if let Ok(decoded) = <PrankWithTxOriginCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PrankWithTxOrigin(decoded));
            }
            if let Ok(decoded) = <PrankCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Prank(decoded));
            }
            if let Ok(decoded) = <PrevrandaoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Prevrandao(decoded));
            }
            if let Ok(decoded) = <ProjectRootCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProjectRoot(decoded));
            }
            if let Ok(decoded) = <ReadCallersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReadCallers(decoded));
            }
            if let Ok(decoded) = <ReadDir1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReadDir1(decoded));
            }
            if let Ok(decoded) = <ReadDir2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReadDir2(decoded));
            }
            if let Ok(decoded) = <ReadDir0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReadDir0(decoded));
            }
            if let Ok(decoded) = <ReadFileCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReadFile(decoded));
            }
            if let Ok(decoded) = <ReadFileBinaryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReadFileBinary(decoded));
            }
            if let Ok(decoded) = <ReadLineCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReadLine(decoded));
            }
            if let Ok(decoded) = <ReadLinkCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReadLink(decoded));
            }
            if let Ok(decoded) = <RecordCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Record(decoded));
            }
            if let Ok(decoded) = <RecordLogsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RecordLogs(decoded));
            }
            if let Ok(decoded) = <RememberKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RememberKey(decoded));
            }
            if let Ok(decoded) = <RemoveDirCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveDir(decoded));
            }
            if let Ok(decoded) = <RemoveFileCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveFile(decoded));
            }
            if let Ok(decoded) = <ResetNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ResetNonce(decoded));
            }
            if let Ok(decoded) = <ResumeGasMeteringCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ResumeGasMetering(decoded));
            }
            if let Ok(decoded) = <RevertToCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertTo(decoded));
            }
            if let Ok(decoded) = <RevertToAndDeleteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertToAndDelete(decoded));
            }
            if let Ok(decoded) = <RevokePersistentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokePersistent(decoded));
            }
            if let Ok(decoded) = <RevokePersistentWithAccountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokePersistentWithAccount(decoded));
            }
            if let Ok(decoded) = <RollCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Roll(decoded));
            }
            if let Ok(decoded) = <RollFork0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RollFork0(decoded));
            }
            if let Ok(decoded) = <RollFork2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RollFork2(decoded));
            }
            if let Ok(decoded) = <RollFork1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RollFork1(decoded));
            }
            if let Ok(decoded) = <RollFork3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RollFork3(decoded));
            }
            if let Ok(decoded) = <RpcCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Rpc(decoded));
            }
            if let Ok(decoded) = <RpcUrlCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RpcUrl(decoded));
            }
            if let Ok(decoded) = <RpcUrlStructsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RpcUrlStructs(decoded));
            }
            if let Ok(decoded) = <RpcUrlsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RpcUrls(decoded));
            }
            if let Ok(decoded) = <SelectForkCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SelectFork(decoded));
            }
            if let Ok(decoded) = <SerializeAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeAddress(decoded));
            }
            if let Ok(decoded) = <SerializeAddressWithObjectKeyAndValueKeyAndValueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::SerializeAddressWithObjectKeyAndValueKeyAndValue(decoded),
                );
            }
            if let Ok(decoded) = <SerializeBoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeBool(decoded));
            }
            if let Ok(decoded) = <SerializeBoolWithObjectKeyAndValueKeyAndValueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeBoolWithObjectKeyAndValueKeyAndValue(decoded));
            }
            if let Ok(decoded) = <SerializeBytesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeBytes(decoded));
            }
            if let Ok(decoded) = <SerializeBytesWithObjectKeyAndValueKeyAndValueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeBytesWithObjectKeyAndValueKeyAndValue(decoded));
            }
            if let Ok(decoded) = <SerializeBytes32Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeBytes32(decoded));
            }
            if let Ok(decoded) = <SerializeBytes32WithObjectKeyAndValueKeyAndValueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::SerializeBytes32WithObjectKeyAndValueKeyAndValue(decoded),
                );
            }
            if let Ok(decoded) = <SerializeIntCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeInt(decoded));
            }
            if let Ok(decoded) = <SerializeIntWithObjectKeyAndValueKeyAndValuesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeIntWithObjectKeyAndValueKeyAndValues(decoded));
            }
            if let Ok(decoded) = <SerializeJsonCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeJson(decoded));
            }
            if let Ok(decoded) = <SerializeStringCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeString(decoded));
            }
            if let Ok(decoded) = <SerializeStringWithObjectKeyAndValueKeyAndValueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::SerializeStringWithObjectKeyAndValueKeyAndValue(decoded),
                );
            }
            if let Ok(decoded) = <SerializeUintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeUint(decoded));
            }
            if let Ok(decoded) = <SerializeUintWithObjectKeyAndValueKeyAndValuesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeUintWithObjectKeyAndValueKeyAndValues(decoded));
            }
            if let Ok(decoded) = <SetEnvCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetEnv(decoded));
            }
            if let Ok(decoded) = <SetNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetNonce(decoded));
            }
            if let Ok(decoded) = <SetNonceUnsafeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetNonceUnsafe(decoded));
            }
            if let Ok(decoded) = <SignCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Sign(decoded));
            }
            if let Ok(decoded) = <SignWithPrivateKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SignWithPrivateKey(decoded));
            }
            if let Ok(decoded) = <SignP256Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SignP256(decoded));
            }
            if let Ok(decoded) = <SkipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Skip(decoded));
            }
            if let Ok(decoded) = <SleepCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Sleep(decoded));
            }
            if let Ok(decoded) = <SnapshotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Snapshot(decoded));
            }
            if let Ok(decoded) = <StartBroadcastCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartBroadcast(decoded));
            }
            if let Ok(decoded) = <StartBroadcastWithSignerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartBroadcastWithSigner(decoded));
            }
            if let Ok(decoded) = <StartBroadcastWithPrivateKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartBroadcastWithPrivateKey(decoded));
            }
            if let Ok(decoded) = <StartMappingRecordingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartMappingRecording(decoded));
            }
            if let Ok(decoded) = <StartPrankCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartPrank(decoded));
            }
            if let Ok(decoded) = <StartPrankWithTxOriginCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartPrankWithTxOrigin(decoded));
            }
            if let Ok(decoded) = <StartStateDiffRecordingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartStateDiffRecording(decoded));
            }
            if let Ok(decoded) = <StopAndReturnStateDiffCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StopAndReturnStateDiff(decoded));
            }
            if let Ok(decoded) = <StopBroadcastCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StopBroadcast(decoded));
            }
            if let Ok(decoded) = <StopMappingRecordingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StopMappingRecording(decoded));
            }
            if let Ok(decoded) = <StopPrankCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StopPrank(decoded));
            }
            if let Ok(decoded) = <StoreCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Store(decoded));
            }
            if let Ok(decoded) = <ToBase64Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToBase64(decoded));
            }
            if let Ok(decoded) = <ToBase64WithDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToBase64WithData(decoded));
            }
            if let Ok(decoded) = <ToBase64URLCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToBase64URL(decoded));
            }
            if let Ok(decoded) = <ToBase64UrlWithDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToBase64UrlWithData(decoded));
            }
            if let Ok(decoded) = <ToString0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToString0(decoded));
            }
            if let Ok(decoded) = <ToString1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToString1(decoded));
            }
            if let Ok(decoded) = <ToString2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToString2(decoded));
            }
            if let Ok(decoded) = <ToString3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToString3(decoded));
            }
            if let Ok(decoded) = <ToString4Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToString4(decoded));
            }
            if let Ok(decoded) = <ToString5Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToString5(decoded));
            }
            if let Ok(decoded) = <TransactWithForkIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransactWithForkId(decoded));
            }
            if let Ok(decoded) = <TransactCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Transact(decoded));
            }
            if let Ok(decoded) = <TryFfiCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TryFfi(decoded));
            }
            if let Ok(decoded) = <TxGasPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TxGasPrice(decoded));
            }
            if let Ok(decoded) = <UnixTimeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnixTime(decoded));
            }
            if let Ok(decoded) = <WarpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Warp(decoded));
            }
            if let Ok(decoded) = <WriteFileCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WriteFile(decoded));
            }
            if let Ok(decoded) = <WriteFileBinaryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WriteFileBinary(decoded));
            }
            if let Ok(decoded) = <WriteJsonWithJsonAndPathCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WriteJsonWithJsonAndPath(decoded));
            }
            if let Ok(decoded) = <WriteJsonCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WriteJson(decoded));
            }
            if let Ok(decoded) = <WriteLineCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WriteLine(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for VmCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Accesses(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ActiveFork(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Addr(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AllowCheatcodes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Assume(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Breakpoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BreakpointWithValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Broadcast(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BroadcastWithSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BroadcastWithPrivateKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChainId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ClearMockedCalls(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CloseFile(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Coinbase(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeCreate2Address(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeCreate2AddressWithSaltAndInitCodeHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeCreateAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CopyFile(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateDir(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateFork(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateForkWithBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateForkWithTxHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateSelectForkWithBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateSelectForkWithTxHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateSelectFork(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateWallet0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateWallet1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateWallet2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deal(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeleteSnapshot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeleteSnapshots(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeriveKey3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeriveKey1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeriveKey0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeriveKey2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Difficulty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DumpState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvAddressWithDelim(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvBool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnvBoolWithDelim(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvBytes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvBytesWithDelim(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvBytes32WithDelim(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvBytes32(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvIntWithDelim(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvInt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnvOr7(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnvOr8(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnvOr0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnvOr1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnvOr2(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnvOr9(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnvOr10(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnvOr11(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnvOr3(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnvOr4(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnvOr5(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnvOr12(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnvOr6(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnvOr13(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnvStringWithDelim(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvString(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvUint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnvUintWithDelim(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Etch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EthGetLogs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Exists(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExpectCall3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectCall5(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectCall4(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectCall0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectCall1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectCall2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectCallMinGas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectCallMinGasWithCalleeAndMsgValueAndMinGasAndData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectEmit0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectEmit2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectEmit3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectEmit1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectRevert1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectRevert2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectRevert0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectSafeMemory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectSafeMemoryCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Fee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Ffi(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FsMetadata(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBlockTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDeployedCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLabel(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMappingKeyAndParentOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMappingLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMappingSlotAt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNonceWithWallet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRecordedLogs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsDir(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsFile(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsPersistent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::KeyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Label(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Load(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LoadAllocs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MakePersistent0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MakePersistent2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MakePersistent1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MakePersistent3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MockCallWithCalleeAndMsgValueAndData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MockCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MockCallRevertWithCalleeAndMsgValueAndData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MockCallRevert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseBool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseBytes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseBytes32(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseInt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseJson(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonWithKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonAddressArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonBool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonBoolArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonBytes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonBytes32(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonBytes32Array(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonBytesArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonInt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonIntArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonKeys(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonString(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonStringArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonUint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonUintArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseUint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PauseGasMetering(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrankWithTxOrigin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Prank(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Prevrandao(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProjectRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReadCallers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReadDir1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReadDir2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReadDir0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReadFile(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReadFileBinary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReadLine(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReadLink(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Record(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RecordLogs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RememberKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveDir(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveFile(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ResetNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ResumeGasMetering(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertToAndDelete(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokePersistent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokePersistentWithAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Roll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RollFork0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RollFork2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RollFork1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RollFork3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Rpc(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RpcUrl(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RpcUrlStructs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RpcUrls(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SelectFork(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeAddressWithObjectKeyAndValueKeyAndValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeBool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeBoolWithObjectKeyAndValueKeyAndValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeBytes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeBytesWithObjectKeyAndValueKeyAndValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeBytes32(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeBytes32WithObjectKeyAndValueKeyAndValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeInt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeIntWithObjectKeyAndValueKeyAndValues(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeJson(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeString(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeStringWithObjectKeyAndValueKeyAndValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeUint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeUintWithObjectKeyAndValueKeyAndValues(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEnv(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetNonceUnsafe(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Sign(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SignWithPrivateKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SignP256(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Skip(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Sleep(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Snapshot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartBroadcast(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartBroadcastWithSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartBroadcastWithPrivateKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartMappingRecording(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartPrank(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartPrankWithTxOrigin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartStateDiffRecording(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StopAndReturnStateDiff(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StopBroadcast(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StopMappingRecording(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StopPrank(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Store(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ToBase64(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ToBase64WithData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ToBase64URL(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ToBase64UrlWithData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ToString0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ToString1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ToString2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ToString3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ToString4(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ToString5(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransactWithForkId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transact(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TryFfi(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TxGasPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnixTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Warp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WriteFile(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WriteFileBinary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WriteJsonWithJsonAndPath(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WriteJson(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WriteLine(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for VmCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Accesses(element) => ::core::fmt::Display::fmt(element, f),
                Self::ActiveFork(element) => ::core::fmt::Display::fmt(element, f),
                Self::Addr(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllowCheatcodes(element) => ::core::fmt::Display::fmt(element, f),
                Self::Assume(element) => ::core::fmt::Display::fmt(element, f),
                Self::Breakpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::BreakpointWithValue(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Broadcast(element) => ::core::fmt::Display::fmt(element, f),
                Self::BroadcastWithSigner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BroadcastWithPrivateKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClearMockedCalls(element) => ::core::fmt::Display::fmt(element, f),
                Self::CloseFile(element) => ::core::fmt::Display::fmt(element, f),
                Self::Coinbase(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeCreate2Address(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ComputeCreate2AddressWithSaltAndInitCodeHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ComputeCreateAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CopyFile(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateDir(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateFork(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateForkWithBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateForkWithTxHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateSelectForkWithBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateSelectForkWithTxHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateSelectFork(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateWallet0(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateWallet1(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateWallet2(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deal(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeleteSnapshot(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeleteSnapshots(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeriveKey3(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeriveKey1(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeriveKey0(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeriveKey2(element) => ::core::fmt::Display::fmt(element, f),
                Self::Difficulty(element) => ::core::fmt::Display::fmt(element, f),
                Self::DumpState(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvAddressWithDelim(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnvBool(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvBoolWithDelim(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvBytes(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvBytesWithDelim(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvBytes32WithDelim(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnvBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvIntWithDelim(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvInt(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr7(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr8(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr0(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr1(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr2(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr9(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr10(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr11(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr3(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr4(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr5(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr12(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr6(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr13(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvStringWithDelim(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnvString(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvUintWithDelim(element) => ::core::fmt::Display::fmt(element, f),
                Self::Etch(element) => ::core::fmt::Display::fmt(element, f),
                Self::EthGetLogs(element) => ::core::fmt::Display::fmt(element, f),
                Self::Exists(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectCall3(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectCall5(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectCall4(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectCall0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectCall1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectCall2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectCallMinGas(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectCallMinGasWithCalleeAndMsgValueAndMinGasAndData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExpectEmit0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectEmit2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectEmit3(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectEmit1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectRevert1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectRevert2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectRevert0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectSafeMemory(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectSafeMemoryCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Fee(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ffi(element) => ::core::fmt::Display::fmt(element, f),
                Self::FsMetadata(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBlockNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBlockTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDeployedCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLabel(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMappingKeyAndParentOf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMappingLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMappingSlotAt(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNonceWithWallet(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRecordedLogs(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDir(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsFile(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPersistent(element) => ::core::fmt::Display::fmt(element, f),
                Self::KeyExists(element) => ::core::fmt::Display::fmt(element, f),
                Self::Label(element) => ::core::fmt::Display::fmt(element, f),
                Self::Load(element) => ::core::fmt::Display::fmt(element, f),
                Self::LoadAllocs(element) => ::core::fmt::Display::fmt(element, f),
                Self::MakePersistent0(element) => ::core::fmt::Display::fmt(element, f),
                Self::MakePersistent2(element) => ::core::fmt::Display::fmt(element, f),
                Self::MakePersistent1(element) => ::core::fmt::Display::fmt(element, f),
                Self::MakePersistent3(element) => ::core::fmt::Display::fmt(element, f),
                Self::MockCallWithCalleeAndMsgValueAndData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MockCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::MockCallRevertWithCalleeAndMsgValueAndData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MockCallRevert(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseBool(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseBytes(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseInt(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJson(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJsonWithKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJsonAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJsonAddressArray(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ParseJsonBool(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJsonBoolArray(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ParseJsonBytes(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJsonBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJsonBytes32Array(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ParseJsonBytesArray(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ParseJsonInt(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJsonIntArray(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJsonKeys(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJsonString(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJsonStringArray(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ParseJsonUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJsonUintArray(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ParseUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseGasMetering(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrankWithTxOrigin(element) => ::core::fmt::Display::fmt(element, f),
                Self::Prank(element) => ::core::fmt::Display::fmt(element, f),
                Self::Prevrandao(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProjectRoot(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadCallers(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadDir1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadDir2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadDir0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadFile(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadFileBinary(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadLine(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadLink(element) => ::core::fmt::Display::fmt(element, f),
                Self::Record(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecordLogs(element) => ::core::fmt::Display::fmt(element, f),
                Self::RememberKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveDir(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveFile(element) => ::core::fmt::Display::fmt(element, f),
                Self::ResetNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::ResumeGasMetering(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertToAndDelete(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokePersistent(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokePersistentWithAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Roll(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollFork0(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollFork2(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollFork1(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollFork3(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rpc(element) => ::core::fmt::Display::fmt(element, f),
                Self::RpcUrl(element) => ::core::fmt::Display::fmt(element, f),
                Self::RpcUrlStructs(element) => ::core::fmt::Display::fmt(element, f),
                Self::RpcUrls(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelectFork(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeAddressWithObjectKeyAndValueKeyAndValue(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SerializeBool(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeBoolWithObjectKeyAndValueKeyAndValue(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SerializeBytes(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeBytesWithObjectKeyAndValueKeyAndValue(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SerializeBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeBytes32WithObjectKeyAndValueKeyAndValue(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SerializeInt(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeIntWithObjectKeyAndValueKeyAndValues(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SerializeJson(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeString(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeStringWithObjectKeyAndValueKeyAndValue(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SerializeUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeUintWithObjectKeyAndValueKeyAndValues(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetEnv(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetNonceUnsafe(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sign(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignWithPrivateKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SignP256(element) => ::core::fmt::Display::fmt(element, f),
                Self::Skip(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sleep(element) => ::core::fmt::Display::fmt(element, f),
                Self::Snapshot(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartBroadcast(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartBroadcastWithSigner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StartBroadcastWithPrivateKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StartMappingRecording(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StartPrank(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartPrankWithTxOrigin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StartStateDiffRecording(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StopAndReturnStateDiff(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StopBroadcast(element) => ::core::fmt::Display::fmt(element, f),
                Self::StopMappingRecording(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StopPrank(element) => ::core::fmt::Display::fmt(element, f),
                Self::Store(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToBase64(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToBase64WithData(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToBase64URL(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToBase64UrlWithData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ToString0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToString1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToString2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToString3(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToString4(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToString5(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransactWithForkId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Transact(element) => ::core::fmt::Display::fmt(element, f),
                Self::TryFfi(element) => ::core::fmt::Display::fmt(element, f),
                Self::TxGasPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnixTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::Warp(element) => ::core::fmt::Display::fmt(element, f),
                Self::WriteFile(element) => ::core::fmt::Display::fmt(element, f),
                Self::WriteFileBinary(element) => ::core::fmt::Display::fmt(element, f),
                Self::WriteJsonWithJsonAndPath(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WriteJson(element) => ::core::fmt::Display::fmt(element, f),
                Self::WriteLine(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AccessesCall> for VmCalls {
        fn from(value: AccessesCall) -> Self {
            Self::Accesses(value)
        }
    }
    impl ::core::convert::From<ActiveForkCall> for VmCalls {
        fn from(value: ActiveForkCall) -> Self {
            Self::ActiveFork(value)
        }
    }
    impl ::core::convert::From<AddrCall> for VmCalls {
        fn from(value: AddrCall) -> Self {
            Self::Addr(value)
        }
    }
    impl ::core::convert::From<AllowCheatcodesCall> for VmCalls {
        fn from(value: AllowCheatcodesCall) -> Self {
            Self::AllowCheatcodes(value)
        }
    }
    impl ::core::convert::From<AssumeCall> for VmCalls {
        fn from(value: AssumeCall) -> Self {
            Self::Assume(value)
        }
    }
    impl ::core::convert::From<BreakpointCall> for VmCalls {
        fn from(value: BreakpointCall) -> Self {
            Self::Breakpoint(value)
        }
    }
    impl ::core::convert::From<BreakpointWithValueCall> for VmCalls {
        fn from(value: BreakpointWithValueCall) -> Self {
            Self::BreakpointWithValue(value)
        }
    }
    impl ::core::convert::From<BroadcastCall> for VmCalls {
        fn from(value: BroadcastCall) -> Self {
            Self::Broadcast(value)
        }
    }
    impl ::core::convert::From<BroadcastWithSignerCall> for VmCalls {
        fn from(value: BroadcastWithSignerCall) -> Self {
            Self::BroadcastWithSigner(value)
        }
    }
    impl ::core::convert::From<BroadcastWithPrivateKeyCall> for VmCalls {
        fn from(value: BroadcastWithPrivateKeyCall) -> Self {
            Self::BroadcastWithPrivateKey(value)
        }
    }
    impl ::core::convert::From<ChainIdCall> for VmCalls {
        fn from(value: ChainIdCall) -> Self {
            Self::ChainId(value)
        }
    }
    impl ::core::convert::From<ClearMockedCallsCall> for VmCalls {
        fn from(value: ClearMockedCallsCall) -> Self {
            Self::ClearMockedCalls(value)
        }
    }
    impl ::core::convert::From<CloseFileCall> for VmCalls {
        fn from(value: CloseFileCall) -> Self {
            Self::CloseFile(value)
        }
    }
    impl ::core::convert::From<CoinbaseCall> for VmCalls {
        fn from(value: CoinbaseCall) -> Self {
            Self::Coinbase(value)
        }
    }
    impl ::core::convert::From<ComputeCreate2AddressCall> for VmCalls {
        fn from(value: ComputeCreate2AddressCall) -> Self {
            Self::ComputeCreate2Address(value)
        }
    }
    impl ::core::convert::From<ComputeCreate2AddressWithSaltAndInitCodeHashCall>
    for VmCalls {
        fn from(value: ComputeCreate2AddressWithSaltAndInitCodeHashCall) -> Self {
            Self::ComputeCreate2AddressWithSaltAndInitCodeHash(value)
        }
    }
    impl ::core::convert::From<ComputeCreateAddressCall> for VmCalls {
        fn from(value: ComputeCreateAddressCall) -> Self {
            Self::ComputeCreateAddress(value)
        }
    }
    impl ::core::convert::From<CopyFileCall> for VmCalls {
        fn from(value: CopyFileCall) -> Self {
            Self::CopyFile(value)
        }
    }
    impl ::core::convert::From<CreateDirCall> for VmCalls {
        fn from(value: CreateDirCall) -> Self {
            Self::CreateDir(value)
        }
    }
    impl ::core::convert::From<CreateForkCall> for VmCalls {
        fn from(value: CreateForkCall) -> Self {
            Self::CreateFork(value)
        }
    }
    impl ::core::convert::From<CreateForkWithBlockNumberCall> for VmCalls {
        fn from(value: CreateForkWithBlockNumberCall) -> Self {
            Self::CreateForkWithBlockNumber(value)
        }
    }
    impl ::core::convert::From<CreateForkWithTxHashCall> for VmCalls {
        fn from(value: CreateForkWithTxHashCall) -> Self {
            Self::CreateForkWithTxHash(value)
        }
    }
    impl ::core::convert::From<CreateSelectForkWithBlockNumberCall> for VmCalls {
        fn from(value: CreateSelectForkWithBlockNumberCall) -> Self {
            Self::CreateSelectForkWithBlockNumber(value)
        }
    }
    impl ::core::convert::From<CreateSelectForkWithTxHashCall> for VmCalls {
        fn from(value: CreateSelectForkWithTxHashCall) -> Self {
            Self::CreateSelectForkWithTxHash(value)
        }
    }
    impl ::core::convert::From<CreateSelectForkCall> for VmCalls {
        fn from(value: CreateSelectForkCall) -> Self {
            Self::CreateSelectFork(value)
        }
    }
    impl ::core::convert::From<CreateWallet0Call> for VmCalls {
        fn from(value: CreateWallet0Call) -> Self {
            Self::CreateWallet0(value)
        }
    }
    impl ::core::convert::From<CreateWallet1Call> for VmCalls {
        fn from(value: CreateWallet1Call) -> Self {
            Self::CreateWallet1(value)
        }
    }
    impl ::core::convert::From<CreateWallet2Call> for VmCalls {
        fn from(value: CreateWallet2Call) -> Self {
            Self::CreateWallet2(value)
        }
    }
    impl ::core::convert::From<DealCall> for VmCalls {
        fn from(value: DealCall) -> Self {
            Self::Deal(value)
        }
    }
    impl ::core::convert::From<DeleteSnapshotCall> for VmCalls {
        fn from(value: DeleteSnapshotCall) -> Self {
            Self::DeleteSnapshot(value)
        }
    }
    impl ::core::convert::From<DeleteSnapshotsCall> for VmCalls {
        fn from(value: DeleteSnapshotsCall) -> Self {
            Self::DeleteSnapshots(value)
        }
    }
    impl ::core::convert::From<DeriveKey3Call> for VmCalls {
        fn from(value: DeriveKey3Call) -> Self {
            Self::DeriveKey3(value)
        }
    }
    impl ::core::convert::From<DeriveKey1Call> for VmCalls {
        fn from(value: DeriveKey1Call) -> Self {
            Self::DeriveKey1(value)
        }
    }
    impl ::core::convert::From<DeriveKey0Call> for VmCalls {
        fn from(value: DeriveKey0Call) -> Self {
            Self::DeriveKey0(value)
        }
    }
    impl ::core::convert::From<DeriveKey2Call> for VmCalls {
        fn from(value: DeriveKey2Call) -> Self {
            Self::DeriveKey2(value)
        }
    }
    impl ::core::convert::From<DifficultyCall> for VmCalls {
        fn from(value: DifficultyCall) -> Self {
            Self::Difficulty(value)
        }
    }
    impl ::core::convert::From<DumpStateCall> for VmCalls {
        fn from(value: DumpStateCall) -> Self {
            Self::DumpState(value)
        }
    }
    impl ::core::convert::From<EnvAddressCall> for VmCalls {
        fn from(value: EnvAddressCall) -> Self {
            Self::EnvAddress(value)
        }
    }
    impl ::core::convert::From<EnvAddressWithDelimCall> for VmCalls {
        fn from(value: EnvAddressWithDelimCall) -> Self {
            Self::EnvAddressWithDelim(value)
        }
    }
    impl ::core::convert::From<EnvBoolCall> for VmCalls {
        fn from(value: EnvBoolCall) -> Self {
            Self::EnvBool(value)
        }
    }
    impl ::core::convert::From<EnvBoolWithDelimCall> for VmCalls {
        fn from(value: EnvBoolWithDelimCall) -> Self {
            Self::EnvBoolWithDelim(value)
        }
    }
    impl ::core::convert::From<EnvBytesCall> for VmCalls {
        fn from(value: EnvBytesCall) -> Self {
            Self::EnvBytes(value)
        }
    }
    impl ::core::convert::From<EnvBytesWithDelimCall> for VmCalls {
        fn from(value: EnvBytesWithDelimCall) -> Self {
            Self::EnvBytesWithDelim(value)
        }
    }
    impl ::core::convert::From<EnvBytes32WithDelimCall> for VmCalls {
        fn from(value: EnvBytes32WithDelimCall) -> Self {
            Self::EnvBytes32WithDelim(value)
        }
    }
    impl ::core::convert::From<EnvBytes32Call> for VmCalls {
        fn from(value: EnvBytes32Call) -> Self {
            Self::EnvBytes32(value)
        }
    }
    impl ::core::convert::From<EnvIntWithDelimCall> for VmCalls {
        fn from(value: EnvIntWithDelimCall) -> Self {
            Self::EnvIntWithDelim(value)
        }
    }
    impl ::core::convert::From<EnvIntCall> for VmCalls {
        fn from(value: EnvIntCall) -> Self {
            Self::EnvInt(value)
        }
    }
    impl ::core::convert::From<EnvOr7Call> for VmCalls {
        fn from(value: EnvOr7Call) -> Self {
            Self::EnvOr7(value)
        }
    }
    impl ::core::convert::From<EnvOr8Call> for VmCalls {
        fn from(value: EnvOr8Call) -> Self {
            Self::EnvOr8(value)
        }
    }
    impl ::core::convert::From<EnvOr0Call> for VmCalls {
        fn from(value: EnvOr0Call) -> Self {
            Self::EnvOr0(value)
        }
    }
    impl ::core::convert::From<EnvOr1Call> for VmCalls {
        fn from(value: EnvOr1Call) -> Self {
            Self::EnvOr1(value)
        }
    }
    impl ::core::convert::From<EnvOr2Call> for VmCalls {
        fn from(value: EnvOr2Call) -> Self {
            Self::EnvOr2(value)
        }
    }
    impl ::core::convert::From<EnvOr9Call> for VmCalls {
        fn from(value: EnvOr9Call) -> Self {
            Self::EnvOr9(value)
        }
    }
    impl ::core::convert::From<EnvOr10Call> for VmCalls {
        fn from(value: EnvOr10Call) -> Self {
            Self::EnvOr10(value)
        }
    }
    impl ::core::convert::From<EnvOr11Call> for VmCalls {
        fn from(value: EnvOr11Call) -> Self {
            Self::EnvOr11(value)
        }
    }
    impl ::core::convert::From<EnvOr3Call> for VmCalls {
        fn from(value: EnvOr3Call) -> Self {
            Self::EnvOr3(value)
        }
    }
    impl ::core::convert::From<EnvOr4Call> for VmCalls {
        fn from(value: EnvOr4Call) -> Self {
            Self::EnvOr4(value)
        }
    }
    impl ::core::convert::From<EnvOr5Call> for VmCalls {
        fn from(value: EnvOr5Call) -> Self {
            Self::EnvOr5(value)
        }
    }
    impl ::core::convert::From<EnvOr12Call> for VmCalls {
        fn from(value: EnvOr12Call) -> Self {
            Self::EnvOr12(value)
        }
    }
    impl ::core::convert::From<EnvOr6Call> for VmCalls {
        fn from(value: EnvOr6Call) -> Self {
            Self::EnvOr6(value)
        }
    }
    impl ::core::convert::From<EnvOr13Call> for VmCalls {
        fn from(value: EnvOr13Call) -> Self {
            Self::EnvOr13(value)
        }
    }
    impl ::core::convert::From<EnvStringWithDelimCall> for VmCalls {
        fn from(value: EnvStringWithDelimCall) -> Self {
            Self::EnvStringWithDelim(value)
        }
    }
    impl ::core::convert::From<EnvStringCall> for VmCalls {
        fn from(value: EnvStringCall) -> Self {
            Self::EnvString(value)
        }
    }
    impl ::core::convert::From<EnvUintCall> for VmCalls {
        fn from(value: EnvUintCall) -> Self {
            Self::EnvUint(value)
        }
    }
    impl ::core::convert::From<EnvUintWithDelimCall> for VmCalls {
        fn from(value: EnvUintWithDelimCall) -> Self {
            Self::EnvUintWithDelim(value)
        }
    }
    impl ::core::convert::From<EtchCall> for VmCalls {
        fn from(value: EtchCall) -> Self {
            Self::Etch(value)
        }
    }
    impl ::core::convert::From<EthGetLogsCall> for VmCalls {
        fn from(value: EthGetLogsCall) -> Self {
            Self::EthGetLogs(value)
        }
    }
    impl ::core::convert::From<ExistsCall> for VmCalls {
        fn from(value: ExistsCall) -> Self {
            Self::Exists(value)
        }
    }
    impl ::core::convert::From<ExpectCall3Call> for VmCalls {
        fn from(value: ExpectCall3Call) -> Self {
            Self::ExpectCall3(value)
        }
    }
    impl ::core::convert::From<ExpectCall5Call> for VmCalls {
        fn from(value: ExpectCall5Call) -> Self {
            Self::ExpectCall5(value)
        }
    }
    impl ::core::convert::From<ExpectCall4Call> for VmCalls {
        fn from(value: ExpectCall4Call) -> Self {
            Self::ExpectCall4(value)
        }
    }
    impl ::core::convert::From<ExpectCall0Call> for VmCalls {
        fn from(value: ExpectCall0Call) -> Self {
            Self::ExpectCall0(value)
        }
    }
    impl ::core::convert::From<ExpectCall1Call> for VmCalls {
        fn from(value: ExpectCall1Call) -> Self {
            Self::ExpectCall1(value)
        }
    }
    impl ::core::convert::From<ExpectCall2Call> for VmCalls {
        fn from(value: ExpectCall2Call) -> Self {
            Self::ExpectCall2(value)
        }
    }
    impl ::core::convert::From<ExpectCallMinGasCall> for VmCalls {
        fn from(value: ExpectCallMinGasCall) -> Self {
            Self::ExpectCallMinGas(value)
        }
    }
    impl ::core::convert::From<ExpectCallMinGasWithCalleeAndMsgValueAndMinGasAndDataCall>
    for VmCalls {
        fn from(
            value: ExpectCallMinGasWithCalleeAndMsgValueAndMinGasAndDataCall,
        ) -> Self {
            Self::ExpectCallMinGasWithCalleeAndMsgValueAndMinGasAndData(value)
        }
    }
    impl ::core::convert::From<ExpectEmit0Call> for VmCalls {
        fn from(value: ExpectEmit0Call) -> Self {
            Self::ExpectEmit0(value)
        }
    }
    impl ::core::convert::From<ExpectEmit2Call> for VmCalls {
        fn from(value: ExpectEmit2Call) -> Self {
            Self::ExpectEmit2(value)
        }
    }
    impl ::core::convert::From<ExpectEmit3Call> for VmCalls {
        fn from(value: ExpectEmit3Call) -> Self {
            Self::ExpectEmit3(value)
        }
    }
    impl ::core::convert::From<ExpectEmit1Call> for VmCalls {
        fn from(value: ExpectEmit1Call) -> Self {
            Self::ExpectEmit1(value)
        }
    }
    impl ::core::convert::From<ExpectRevert1Call> for VmCalls {
        fn from(value: ExpectRevert1Call) -> Self {
            Self::ExpectRevert1(value)
        }
    }
    impl ::core::convert::From<ExpectRevert2Call> for VmCalls {
        fn from(value: ExpectRevert2Call) -> Self {
            Self::ExpectRevert2(value)
        }
    }
    impl ::core::convert::From<ExpectRevert0Call> for VmCalls {
        fn from(value: ExpectRevert0Call) -> Self {
            Self::ExpectRevert0(value)
        }
    }
    impl ::core::convert::From<ExpectSafeMemoryCall> for VmCalls {
        fn from(value: ExpectSafeMemoryCall) -> Self {
            Self::ExpectSafeMemory(value)
        }
    }
    impl ::core::convert::From<ExpectSafeMemoryCallCall> for VmCalls {
        fn from(value: ExpectSafeMemoryCallCall) -> Self {
            Self::ExpectSafeMemoryCall(value)
        }
    }
    impl ::core::convert::From<FeeCall> for VmCalls {
        fn from(value: FeeCall) -> Self {
            Self::Fee(value)
        }
    }
    impl ::core::convert::From<FfiCall> for VmCalls {
        fn from(value: FfiCall) -> Self {
            Self::Ffi(value)
        }
    }
    impl ::core::convert::From<FsMetadataCall> for VmCalls {
        fn from(value: FsMetadataCall) -> Self {
            Self::FsMetadata(value)
        }
    }
    impl ::core::convert::From<GetBlockNumberCall> for VmCalls {
        fn from(value: GetBlockNumberCall) -> Self {
            Self::GetBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetBlockTimestampCall> for VmCalls {
        fn from(value: GetBlockTimestampCall) -> Self {
            Self::GetBlockTimestamp(value)
        }
    }
    impl ::core::convert::From<GetCodeCall> for VmCalls {
        fn from(value: GetCodeCall) -> Self {
            Self::GetCode(value)
        }
    }
    impl ::core::convert::From<GetDeployedCodeCall> for VmCalls {
        fn from(value: GetDeployedCodeCall) -> Self {
            Self::GetDeployedCode(value)
        }
    }
    impl ::core::convert::From<GetLabelCall> for VmCalls {
        fn from(value: GetLabelCall) -> Self {
            Self::GetLabel(value)
        }
    }
    impl ::core::convert::From<GetMappingKeyAndParentOfCall> for VmCalls {
        fn from(value: GetMappingKeyAndParentOfCall) -> Self {
            Self::GetMappingKeyAndParentOf(value)
        }
    }
    impl ::core::convert::From<GetMappingLengthCall> for VmCalls {
        fn from(value: GetMappingLengthCall) -> Self {
            Self::GetMappingLength(value)
        }
    }
    impl ::core::convert::From<GetMappingSlotAtCall> for VmCalls {
        fn from(value: GetMappingSlotAtCall) -> Self {
            Self::GetMappingSlotAt(value)
        }
    }
    impl ::core::convert::From<GetNonceCall> for VmCalls {
        fn from(value: GetNonceCall) -> Self {
            Self::GetNonce(value)
        }
    }
    impl ::core::convert::From<GetNonceWithWalletCall> for VmCalls {
        fn from(value: GetNonceWithWalletCall) -> Self {
            Self::GetNonceWithWallet(value)
        }
    }
    impl ::core::convert::From<GetRecordedLogsCall> for VmCalls {
        fn from(value: GetRecordedLogsCall) -> Self {
            Self::GetRecordedLogs(value)
        }
    }
    impl ::core::convert::From<IsDirCall> for VmCalls {
        fn from(value: IsDirCall) -> Self {
            Self::IsDir(value)
        }
    }
    impl ::core::convert::From<IsFileCall> for VmCalls {
        fn from(value: IsFileCall) -> Self {
            Self::IsFile(value)
        }
    }
    impl ::core::convert::From<IsPersistentCall> for VmCalls {
        fn from(value: IsPersistentCall) -> Self {
            Self::IsPersistent(value)
        }
    }
    impl ::core::convert::From<KeyExistsCall> for VmCalls {
        fn from(value: KeyExistsCall) -> Self {
            Self::KeyExists(value)
        }
    }
    impl ::core::convert::From<LabelCall> for VmCalls {
        fn from(value: LabelCall) -> Self {
            Self::Label(value)
        }
    }
    impl ::core::convert::From<LoadCall> for VmCalls {
        fn from(value: LoadCall) -> Self {
            Self::Load(value)
        }
    }
    impl ::core::convert::From<LoadAllocsCall> for VmCalls {
        fn from(value: LoadAllocsCall) -> Self {
            Self::LoadAllocs(value)
        }
    }
    impl ::core::convert::From<MakePersistent0Call> for VmCalls {
        fn from(value: MakePersistent0Call) -> Self {
            Self::MakePersistent0(value)
        }
    }
    impl ::core::convert::From<MakePersistent2Call> for VmCalls {
        fn from(value: MakePersistent2Call) -> Self {
            Self::MakePersistent2(value)
        }
    }
    impl ::core::convert::From<MakePersistent1Call> for VmCalls {
        fn from(value: MakePersistent1Call) -> Self {
            Self::MakePersistent1(value)
        }
    }
    impl ::core::convert::From<MakePersistent3Call> for VmCalls {
        fn from(value: MakePersistent3Call) -> Self {
            Self::MakePersistent3(value)
        }
    }
    impl ::core::convert::From<MockCallWithCalleeAndMsgValueAndDataCall> for VmCalls {
        fn from(value: MockCallWithCalleeAndMsgValueAndDataCall) -> Self {
            Self::MockCallWithCalleeAndMsgValueAndData(value)
        }
    }
    impl ::core::convert::From<MockCallCall> for VmCalls {
        fn from(value: MockCallCall) -> Self {
            Self::MockCall(value)
        }
    }
    impl ::core::convert::From<MockCallRevertWithCalleeAndMsgValueAndDataCall>
    for VmCalls {
        fn from(value: MockCallRevertWithCalleeAndMsgValueAndDataCall) -> Self {
            Self::MockCallRevertWithCalleeAndMsgValueAndData(value)
        }
    }
    impl ::core::convert::From<MockCallRevertCall> for VmCalls {
        fn from(value: MockCallRevertCall) -> Self {
            Self::MockCallRevert(value)
        }
    }
    impl ::core::convert::From<ParseAddressCall> for VmCalls {
        fn from(value: ParseAddressCall) -> Self {
            Self::ParseAddress(value)
        }
    }
    impl ::core::convert::From<ParseBoolCall> for VmCalls {
        fn from(value: ParseBoolCall) -> Self {
            Self::ParseBool(value)
        }
    }
    impl ::core::convert::From<ParseBytesCall> for VmCalls {
        fn from(value: ParseBytesCall) -> Self {
            Self::ParseBytes(value)
        }
    }
    impl ::core::convert::From<ParseBytes32Call> for VmCalls {
        fn from(value: ParseBytes32Call) -> Self {
            Self::ParseBytes32(value)
        }
    }
    impl ::core::convert::From<ParseIntCall> for VmCalls {
        fn from(value: ParseIntCall) -> Self {
            Self::ParseInt(value)
        }
    }
    impl ::core::convert::From<ParseJsonCall> for VmCalls {
        fn from(value: ParseJsonCall) -> Self {
            Self::ParseJson(value)
        }
    }
    impl ::core::convert::From<ParseJsonWithKeyCall> for VmCalls {
        fn from(value: ParseJsonWithKeyCall) -> Self {
            Self::ParseJsonWithKey(value)
        }
    }
    impl ::core::convert::From<ParseJsonAddressCall> for VmCalls {
        fn from(value: ParseJsonAddressCall) -> Self {
            Self::ParseJsonAddress(value)
        }
    }
    impl ::core::convert::From<ParseJsonAddressArrayCall> for VmCalls {
        fn from(value: ParseJsonAddressArrayCall) -> Self {
            Self::ParseJsonAddressArray(value)
        }
    }
    impl ::core::convert::From<ParseJsonBoolCall> for VmCalls {
        fn from(value: ParseJsonBoolCall) -> Self {
            Self::ParseJsonBool(value)
        }
    }
    impl ::core::convert::From<ParseJsonBoolArrayCall> for VmCalls {
        fn from(value: ParseJsonBoolArrayCall) -> Self {
            Self::ParseJsonBoolArray(value)
        }
    }
    impl ::core::convert::From<ParseJsonBytesCall> for VmCalls {
        fn from(value: ParseJsonBytesCall) -> Self {
            Self::ParseJsonBytes(value)
        }
    }
    impl ::core::convert::From<ParseJsonBytes32Call> for VmCalls {
        fn from(value: ParseJsonBytes32Call) -> Self {
            Self::ParseJsonBytes32(value)
        }
    }
    impl ::core::convert::From<ParseJsonBytes32ArrayCall> for VmCalls {
        fn from(value: ParseJsonBytes32ArrayCall) -> Self {
            Self::ParseJsonBytes32Array(value)
        }
    }
    impl ::core::convert::From<ParseJsonBytesArrayCall> for VmCalls {
        fn from(value: ParseJsonBytesArrayCall) -> Self {
            Self::ParseJsonBytesArray(value)
        }
    }
    impl ::core::convert::From<ParseJsonIntCall> for VmCalls {
        fn from(value: ParseJsonIntCall) -> Self {
            Self::ParseJsonInt(value)
        }
    }
    impl ::core::convert::From<ParseJsonIntArrayCall> for VmCalls {
        fn from(value: ParseJsonIntArrayCall) -> Self {
            Self::ParseJsonIntArray(value)
        }
    }
    impl ::core::convert::From<ParseJsonKeysCall> for VmCalls {
        fn from(value: ParseJsonKeysCall) -> Self {
            Self::ParseJsonKeys(value)
        }
    }
    impl ::core::convert::From<ParseJsonStringCall> for VmCalls {
        fn from(value: ParseJsonStringCall) -> Self {
            Self::ParseJsonString(value)
        }
    }
    impl ::core::convert::From<ParseJsonStringArrayCall> for VmCalls {
        fn from(value: ParseJsonStringArrayCall) -> Self {
            Self::ParseJsonStringArray(value)
        }
    }
    impl ::core::convert::From<ParseJsonUintCall> for VmCalls {
        fn from(value: ParseJsonUintCall) -> Self {
            Self::ParseJsonUint(value)
        }
    }
    impl ::core::convert::From<ParseJsonUintArrayCall> for VmCalls {
        fn from(value: ParseJsonUintArrayCall) -> Self {
            Self::ParseJsonUintArray(value)
        }
    }
    impl ::core::convert::From<ParseUintCall> for VmCalls {
        fn from(value: ParseUintCall) -> Self {
            Self::ParseUint(value)
        }
    }
    impl ::core::convert::From<PauseGasMeteringCall> for VmCalls {
        fn from(value: PauseGasMeteringCall) -> Self {
            Self::PauseGasMetering(value)
        }
    }
    impl ::core::convert::From<PrankWithTxOriginCall> for VmCalls {
        fn from(value: PrankWithTxOriginCall) -> Self {
            Self::PrankWithTxOrigin(value)
        }
    }
    impl ::core::convert::From<PrankCall> for VmCalls {
        fn from(value: PrankCall) -> Self {
            Self::Prank(value)
        }
    }
    impl ::core::convert::From<PrevrandaoCall> for VmCalls {
        fn from(value: PrevrandaoCall) -> Self {
            Self::Prevrandao(value)
        }
    }
    impl ::core::convert::From<ProjectRootCall> for VmCalls {
        fn from(value: ProjectRootCall) -> Self {
            Self::ProjectRoot(value)
        }
    }
    impl ::core::convert::From<ReadCallersCall> for VmCalls {
        fn from(value: ReadCallersCall) -> Self {
            Self::ReadCallers(value)
        }
    }
    impl ::core::convert::From<ReadDir1Call> for VmCalls {
        fn from(value: ReadDir1Call) -> Self {
            Self::ReadDir1(value)
        }
    }
    impl ::core::convert::From<ReadDir2Call> for VmCalls {
        fn from(value: ReadDir2Call) -> Self {
            Self::ReadDir2(value)
        }
    }
    impl ::core::convert::From<ReadDir0Call> for VmCalls {
        fn from(value: ReadDir0Call) -> Self {
            Self::ReadDir0(value)
        }
    }
    impl ::core::convert::From<ReadFileCall> for VmCalls {
        fn from(value: ReadFileCall) -> Self {
            Self::ReadFile(value)
        }
    }
    impl ::core::convert::From<ReadFileBinaryCall> for VmCalls {
        fn from(value: ReadFileBinaryCall) -> Self {
            Self::ReadFileBinary(value)
        }
    }
    impl ::core::convert::From<ReadLineCall> for VmCalls {
        fn from(value: ReadLineCall) -> Self {
            Self::ReadLine(value)
        }
    }
    impl ::core::convert::From<ReadLinkCall> for VmCalls {
        fn from(value: ReadLinkCall) -> Self {
            Self::ReadLink(value)
        }
    }
    impl ::core::convert::From<RecordCall> for VmCalls {
        fn from(value: RecordCall) -> Self {
            Self::Record(value)
        }
    }
    impl ::core::convert::From<RecordLogsCall> for VmCalls {
        fn from(value: RecordLogsCall) -> Self {
            Self::RecordLogs(value)
        }
    }
    impl ::core::convert::From<RememberKeyCall> for VmCalls {
        fn from(value: RememberKeyCall) -> Self {
            Self::RememberKey(value)
        }
    }
    impl ::core::convert::From<RemoveDirCall> for VmCalls {
        fn from(value: RemoveDirCall) -> Self {
            Self::RemoveDir(value)
        }
    }
    impl ::core::convert::From<RemoveFileCall> for VmCalls {
        fn from(value: RemoveFileCall) -> Self {
            Self::RemoveFile(value)
        }
    }
    impl ::core::convert::From<ResetNonceCall> for VmCalls {
        fn from(value: ResetNonceCall) -> Self {
            Self::ResetNonce(value)
        }
    }
    impl ::core::convert::From<ResumeGasMeteringCall> for VmCalls {
        fn from(value: ResumeGasMeteringCall) -> Self {
            Self::ResumeGasMetering(value)
        }
    }
    impl ::core::convert::From<RevertToCall> for VmCalls {
        fn from(value: RevertToCall) -> Self {
            Self::RevertTo(value)
        }
    }
    impl ::core::convert::From<RevertToAndDeleteCall> for VmCalls {
        fn from(value: RevertToAndDeleteCall) -> Self {
            Self::RevertToAndDelete(value)
        }
    }
    impl ::core::convert::From<RevokePersistentCall> for VmCalls {
        fn from(value: RevokePersistentCall) -> Self {
            Self::RevokePersistent(value)
        }
    }
    impl ::core::convert::From<RevokePersistentWithAccountCall> for VmCalls {
        fn from(value: RevokePersistentWithAccountCall) -> Self {
            Self::RevokePersistentWithAccount(value)
        }
    }
    impl ::core::convert::From<RollCall> for VmCalls {
        fn from(value: RollCall) -> Self {
            Self::Roll(value)
        }
    }
    impl ::core::convert::From<RollFork0Call> for VmCalls {
        fn from(value: RollFork0Call) -> Self {
            Self::RollFork0(value)
        }
    }
    impl ::core::convert::From<RollFork2Call> for VmCalls {
        fn from(value: RollFork2Call) -> Self {
            Self::RollFork2(value)
        }
    }
    impl ::core::convert::From<RollFork1Call> for VmCalls {
        fn from(value: RollFork1Call) -> Self {
            Self::RollFork1(value)
        }
    }
    impl ::core::convert::From<RollFork3Call> for VmCalls {
        fn from(value: RollFork3Call) -> Self {
            Self::RollFork3(value)
        }
    }
    impl ::core::convert::From<RpcCall> for VmCalls {
        fn from(value: RpcCall) -> Self {
            Self::Rpc(value)
        }
    }
    impl ::core::convert::From<RpcUrlCall> for VmCalls {
        fn from(value: RpcUrlCall) -> Self {
            Self::RpcUrl(value)
        }
    }
    impl ::core::convert::From<RpcUrlStructsCall> for VmCalls {
        fn from(value: RpcUrlStructsCall) -> Self {
            Self::RpcUrlStructs(value)
        }
    }
    impl ::core::convert::From<RpcUrlsCall> for VmCalls {
        fn from(value: RpcUrlsCall) -> Self {
            Self::RpcUrls(value)
        }
    }
    impl ::core::convert::From<SelectForkCall> for VmCalls {
        fn from(value: SelectForkCall) -> Self {
            Self::SelectFork(value)
        }
    }
    impl ::core::convert::From<SerializeAddressCall> for VmCalls {
        fn from(value: SerializeAddressCall) -> Self {
            Self::SerializeAddress(value)
        }
    }
    impl ::core::convert::From<SerializeAddressWithObjectKeyAndValueKeyAndValueCall>
    for VmCalls {
        fn from(value: SerializeAddressWithObjectKeyAndValueKeyAndValueCall) -> Self {
            Self::SerializeAddressWithObjectKeyAndValueKeyAndValue(value)
        }
    }
    impl ::core::convert::From<SerializeBoolCall> for VmCalls {
        fn from(value: SerializeBoolCall) -> Self {
            Self::SerializeBool(value)
        }
    }
    impl ::core::convert::From<SerializeBoolWithObjectKeyAndValueKeyAndValueCall>
    for VmCalls {
        fn from(value: SerializeBoolWithObjectKeyAndValueKeyAndValueCall) -> Self {
            Self::SerializeBoolWithObjectKeyAndValueKeyAndValue(value)
        }
    }
    impl ::core::convert::From<SerializeBytesCall> for VmCalls {
        fn from(value: SerializeBytesCall) -> Self {
            Self::SerializeBytes(value)
        }
    }
    impl ::core::convert::From<SerializeBytesWithObjectKeyAndValueKeyAndValueCall>
    for VmCalls {
        fn from(value: SerializeBytesWithObjectKeyAndValueKeyAndValueCall) -> Self {
            Self::SerializeBytesWithObjectKeyAndValueKeyAndValue(value)
        }
    }
    impl ::core::convert::From<SerializeBytes32Call> for VmCalls {
        fn from(value: SerializeBytes32Call) -> Self {
            Self::SerializeBytes32(value)
        }
    }
    impl ::core::convert::From<SerializeBytes32WithObjectKeyAndValueKeyAndValueCall>
    for VmCalls {
        fn from(value: SerializeBytes32WithObjectKeyAndValueKeyAndValueCall) -> Self {
            Self::SerializeBytes32WithObjectKeyAndValueKeyAndValue(value)
        }
    }
    impl ::core::convert::From<SerializeIntCall> for VmCalls {
        fn from(value: SerializeIntCall) -> Self {
            Self::SerializeInt(value)
        }
    }
    impl ::core::convert::From<SerializeIntWithObjectKeyAndValueKeyAndValuesCall>
    for VmCalls {
        fn from(value: SerializeIntWithObjectKeyAndValueKeyAndValuesCall) -> Self {
            Self::SerializeIntWithObjectKeyAndValueKeyAndValues(value)
        }
    }
    impl ::core::convert::From<SerializeJsonCall> for VmCalls {
        fn from(value: SerializeJsonCall) -> Self {
            Self::SerializeJson(value)
        }
    }
    impl ::core::convert::From<SerializeStringCall> for VmCalls {
        fn from(value: SerializeStringCall) -> Self {
            Self::SerializeString(value)
        }
    }
    impl ::core::convert::From<SerializeStringWithObjectKeyAndValueKeyAndValueCall>
    for VmCalls {
        fn from(value: SerializeStringWithObjectKeyAndValueKeyAndValueCall) -> Self {
            Self::SerializeStringWithObjectKeyAndValueKeyAndValue(value)
        }
    }
    impl ::core::convert::From<SerializeUintCall> for VmCalls {
        fn from(value: SerializeUintCall) -> Self {
            Self::SerializeUint(value)
        }
    }
    impl ::core::convert::From<SerializeUintWithObjectKeyAndValueKeyAndValuesCall>
    for VmCalls {
        fn from(value: SerializeUintWithObjectKeyAndValueKeyAndValuesCall) -> Self {
            Self::SerializeUintWithObjectKeyAndValueKeyAndValues(value)
        }
    }
    impl ::core::convert::From<SetEnvCall> for VmCalls {
        fn from(value: SetEnvCall) -> Self {
            Self::SetEnv(value)
        }
    }
    impl ::core::convert::From<SetNonceCall> for VmCalls {
        fn from(value: SetNonceCall) -> Self {
            Self::SetNonce(value)
        }
    }
    impl ::core::convert::From<SetNonceUnsafeCall> for VmCalls {
        fn from(value: SetNonceUnsafeCall) -> Self {
            Self::SetNonceUnsafe(value)
        }
    }
    impl ::core::convert::From<SignCall> for VmCalls {
        fn from(value: SignCall) -> Self {
            Self::Sign(value)
        }
    }
    impl ::core::convert::From<SignWithPrivateKeyCall> for VmCalls {
        fn from(value: SignWithPrivateKeyCall) -> Self {
            Self::SignWithPrivateKey(value)
        }
    }
    impl ::core::convert::From<SignP256Call> for VmCalls {
        fn from(value: SignP256Call) -> Self {
            Self::SignP256(value)
        }
    }
    impl ::core::convert::From<SkipCall> for VmCalls {
        fn from(value: SkipCall) -> Self {
            Self::Skip(value)
        }
    }
    impl ::core::convert::From<SleepCall> for VmCalls {
        fn from(value: SleepCall) -> Self {
            Self::Sleep(value)
        }
    }
    impl ::core::convert::From<SnapshotCall> for VmCalls {
        fn from(value: SnapshotCall) -> Self {
            Self::Snapshot(value)
        }
    }
    impl ::core::convert::From<StartBroadcastCall> for VmCalls {
        fn from(value: StartBroadcastCall) -> Self {
            Self::StartBroadcast(value)
        }
    }
    impl ::core::convert::From<StartBroadcastWithSignerCall> for VmCalls {
        fn from(value: StartBroadcastWithSignerCall) -> Self {
            Self::StartBroadcastWithSigner(value)
        }
    }
    impl ::core::convert::From<StartBroadcastWithPrivateKeyCall> for VmCalls {
        fn from(value: StartBroadcastWithPrivateKeyCall) -> Self {
            Self::StartBroadcastWithPrivateKey(value)
        }
    }
    impl ::core::convert::From<StartMappingRecordingCall> for VmCalls {
        fn from(value: StartMappingRecordingCall) -> Self {
            Self::StartMappingRecording(value)
        }
    }
    impl ::core::convert::From<StartPrankCall> for VmCalls {
        fn from(value: StartPrankCall) -> Self {
            Self::StartPrank(value)
        }
    }
    impl ::core::convert::From<StartPrankWithTxOriginCall> for VmCalls {
        fn from(value: StartPrankWithTxOriginCall) -> Self {
            Self::StartPrankWithTxOrigin(value)
        }
    }
    impl ::core::convert::From<StartStateDiffRecordingCall> for VmCalls {
        fn from(value: StartStateDiffRecordingCall) -> Self {
            Self::StartStateDiffRecording(value)
        }
    }
    impl ::core::convert::From<StopAndReturnStateDiffCall> for VmCalls {
        fn from(value: StopAndReturnStateDiffCall) -> Self {
            Self::StopAndReturnStateDiff(value)
        }
    }
    impl ::core::convert::From<StopBroadcastCall> for VmCalls {
        fn from(value: StopBroadcastCall) -> Self {
            Self::StopBroadcast(value)
        }
    }
    impl ::core::convert::From<StopMappingRecordingCall> for VmCalls {
        fn from(value: StopMappingRecordingCall) -> Self {
            Self::StopMappingRecording(value)
        }
    }
    impl ::core::convert::From<StopPrankCall> for VmCalls {
        fn from(value: StopPrankCall) -> Self {
            Self::StopPrank(value)
        }
    }
    impl ::core::convert::From<StoreCall> for VmCalls {
        fn from(value: StoreCall) -> Self {
            Self::Store(value)
        }
    }
    impl ::core::convert::From<ToBase64Call> for VmCalls {
        fn from(value: ToBase64Call) -> Self {
            Self::ToBase64(value)
        }
    }
    impl ::core::convert::From<ToBase64WithDataCall> for VmCalls {
        fn from(value: ToBase64WithDataCall) -> Self {
            Self::ToBase64WithData(value)
        }
    }
    impl ::core::convert::From<ToBase64URLCall> for VmCalls {
        fn from(value: ToBase64URLCall) -> Self {
            Self::ToBase64URL(value)
        }
    }
    impl ::core::convert::From<ToBase64UrlWithDataCall> for VmCalls {
        fn from(value: ToBase64UrlWithDataCall) -> Self {
            Self::ToBase64UrlWithData(value)
        }
    }
    impl ::core::convert::From<ToString0Call> for VmCalls {
        fn from(value: ToString0Call) -> Self {
            Self::ToString0(value)
        }
    }
    impl ::core::convert::From<ToString1Call> for VmCalls {
        fn from(value: ToString1Call) -> Self {
            Self::ToString1(value)
        }
    }
    impl ::core::convert::From<ToString2Call> for VmCalls {
        fn from(value: ToString2Call) -> Self {
            Self::ToString2(value)
        }
    }
    impl ::core::convert::From<ToString3Call> for VmCalls {
        fn from(value: ToString3Call) -> Self {
            Self::ToString3(value)
        }
    }
    impl ::core::convert::From<ToString4Call> for VmCalls {
        fn from(value: ToString4Call) -> Self {
            Self::ToString4(value)
        }
    }
    impl ::core::convert::From<ToString5Call> for VmCalls {
        fn from(value: ToString5Call) -> Self {
            Self::ToString5(value)
        }
    }
    impl ::core::convert::From<TransactWithForkIdCall> for VmCalls {
        fn from(value: TransactWithForkIdCall) -> Self {
            Self::TransactWithForkId(value)
        }
    }
    impl ::core::convert::From<TransactCall> for VmCalls {
        fn from(value: TransactCall) -> Self {
            Self::Transact(value)
        }
    }
    impl ::core::convert::From<TryFfiCall> for VmCalls {
        fn from(value: TryFfiCall) -> Self {
            Self::TryFfi(value)
        }
    }
    impl ::core::convert::From<TxGasPriceCall> for VmCalls {
        fn from(value: TxGasPriceCall) -> Self {
            Self::TxGasPrice(value)
        }
    }
    impl ::core::convert::From<UnixTimeCall> for VmCalls {
        fn from(value: UnixTimeCall) -> Self {
            Self::UnixTime(value)
        }
    }
    impl ::core::convert::From<WarpCall> for VmCalls {
        fn from(value: WarpCall) -> Self {
            Self::Warp(value)
        }
    }
    impl ::core::convert::From<WriteFileCall> for VmCalls {
        fn from(value: WriteFileCall) -> Self {
            Self::WriteFile(value)
        }
    }
    impl ::core::convert::From<WriteFileBinaryCall> for VmCalls {
        fn from(value: WriteFileBinaryCall) -> Self {
            Self::WriteFileBinary(value)
        }
    }
    impl ::core::convert::From<WriteJsonWithJsonAndPathCall> for VmCalls {
        fn from(value: WriteJsonWithJsonAndPathCall) -> Self {
            Self::WriteJsonWithJsonAndPath(value)
        }
    }
    impl ::core::convert::From<WriteJsonCall> for VmCalls {
        fn from(value: WriteJsonCall) -> Self {
            Self::WriteJson(value)
        }
    }
    impl ::core::convert::From<WriteLineCall> for VmCalls {
        fn from(value: WriteLineCall) -> Self {
            Self::WriteLine(value)
        }
    }
    ///Container type for all return fields from the `accesses` function with signature `accesses(address)` and selector `0x65bc9481`
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
    pub struct AccessesReturn {
        pub read_slots: ::std::vec::Vec<[u8; 32]>,
        pub write_slots: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all return fields from the `activeFork` function with signature `activeFork()` and selector `0x2f103f22`
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
    pub struct ActiveForkReturn {
        pub fork_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `addr` function with signature `addr(uint256)` and selector `0xffa18649`
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
    pub struct AddrReturn {
        pub key_addr: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `computeCreate2Address` function with signature `computeCreate2Address(bytes32,bytes32)` and selector `0x890c283b`
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
    pub struct ComputeCreate2AddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `computeCreate2Address` function with signature `computeCreate2Address(bytes32,bytes32,address)` and selector `0xd323826a`
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
    pub struct ComputeCreate2AddressWithSaltAndInitCodeHashReturn(
        pub ::ethers::core::types::Address,
    );
    ///Container type for all return fields from the `computeCreateAddress` function with signature `computeCreateAddress(address,uint256)` and selector `0x74637a7a`
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
    pub struct ComputeCreateAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `copyFile` function with signature `copyFile(string,string)` and selector `0xa54a87d8`
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
    pub struct CopyFileReturn {
        pub copied: u64,
    }
    ///Container type for all return fields from the `createFork` function with signature `createFork(string)` and selector `0x31ba3498`
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
    pub struct CreateForkReturn {
        pub fork_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `createFork` function with signature `createFork(string,uint256)` and selector `0x6ba3ba2b`
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
    pub struct CreateForkWithBlockNumberReturn {
        pub fork_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `createFork` function with signature `createFork(string,bytes32)` and selector `0x7ca29682`
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
    pub struct CreateForkWithTxHashReturn {
        pub fork_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `createSelectFork` function with signature `createSelectFork(string,uint256)` and selector `0x71ee464d`
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
    pub struct CreateSelectForkWithBlockNumberReturn {
        pub fork_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `createSelectFork` function with signature `createSelectFork(string,bytes32)` and selector `0x84d52b7a`
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
    pub struct CreateSelectForkWithTxHashReturn {
        pub fork_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `createSelectFork` function with signature `createSelectFork(string)` and selector `0x98680034`
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
    pub struct CreateSelectForkReturn {
        pub fork_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `createWallet` function with signature `createWallet(string)` and selector `0x7404f1d2`
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
    pub struct CreateWallet0Return {
        pub wallet: Wallet,
    }
    ///Container type for all return fields from the `createWallet` function with signature `createWallet(uint256)` and selector `0x7a675bb6`
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
    pub struct CreateWallet1Return {
        pub wallet: Wallet,
    }
    ///Container type for all return fields from the `createWallet` function with signature `createWallet(uint256,string)` and selector `0xed7c5462`
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
    pub struct CreateWallet2Return {
        pub wallet: Wallet,
    }
    ///Container type for all return fields from the `deleteSnapshot` function with signature `deleteSnapshot(uint256)` and selector `0xa6368557`
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
    pub struct DeleteSnapshotReturn {
        pub success: bool,
    }
    ///Container type for all return fields from the `deriveKey` function with signature `deriveKey(string,string,uint32,string)` and selector `0x29233b1f`
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
    pub struct DeriveKey3Return {
        pub private_key: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `deriveKey` function with signature `deriveKey(string,uint32,string)` and selector `0x32c8176d`
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
    pub struct DeriveKey1Return {
        pub private_key: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `deriveKey` function with signature `deriveKey(string,uint32)` and selector `0x6229498b`
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
    pub struct DeriveKey0Return {
        pub private_key: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `deriveKey` function with signature `deriveKey(string,string,uint32)` and selector `0x6bcb2c1b`
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
    pub struct DeriveKey2Return {
        pub private_key: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `envAddress` function with signature `envAddress(string)` and selector `0x350d56bf`
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
    pub struct EnvAddressReturn {
        pub value: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `envAddress` function with signature `envAddress(string,string)` and selector `0xad31b9fa`
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
    pub struct EnvAddressWithDelimReturn {
        pub value: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `envBool` function with signature `envBool(string)` and selector `0x7ed1ec7d`
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
    pub struct EnvBoolReturn {
        pub value: bool,
    }
    ///Container type for all return fields from the `envBool` function with signature `envBool(string,string)` and selector `0xaaaddeaf`
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
    pub struct EnvBoolWithDelimReturn {
        pub value: ::std::vec::Vec<bool>,
    }
    ///Container type for all return fields from the `envBytes` function with signature `envBytes(string)` and selector `0x4d7baf06`
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
    pub struct EnvBytesReturn {
        pub value: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `envBytes` function with signature `envBytes(string,string)` and selector `0xddc2651b`
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
    pub struct EnvBytesWithDelimReturn {
        pub value: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all return fields from the `envBytes32` function with signature `envBytes32(string,string)` and selector `0x5af231c1`
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
    pub struct EnvBytes32WithDelimReturn {
        pub value: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all return fields from the `envBytes32` function with signature `envBytes32(string)` and selector `0x97949042`
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
    pub struct EnvBytes32Return {
        pub value: [u8; 32],
    }
    ///Container type for all return fields from the `envInt` function with signature `envInt(string,string)` and selector `0x42181150`
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
    pub struct EnvIntWithDelimReturn {
        pub value: ::std::vec::Vec<::ethers::core::types::I256>,
    }
    ///Container type for all return fields from the `envInt` function with signature `envInt(string)` and selector `0x892a0c61`
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
    pub struct EnvIntReturn {
        pub value: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,string,bytes32[])` and selector `0x2281f367`
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
    pub struct EnvOr7Return {
        pub value: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,string,int256[])` and selector `0x4700d74b`
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
    pub struct EnvOr8Return {
        pub value: ::std::vec::Vec<::ethers::core::types::I256>,
    }
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,bool)` and selector `0x4777f3cf`
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
    pub struct EnvOr0Return {
        pub value: bool,
    }
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,address)` and selector `0x561fe540`
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
    pub struct EnvOr1Return {
        pub value: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,uint256)` and selector `0x5e97348f`
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
    pub struct EnvOr2Return {
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,string,bytes[])` and selector `0x64bc3e64`
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
    pub struct EnvOr9Return {
        pub value: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,string,uint256[])` and selector `0x74318528`
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
    pub struct EnvOr10Return {
        pub value: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,string,string[])` and selector `0x859216bc`
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
    pub struct EnvOr11Return {
        pub value: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,bytes)` and selector `0xb3e47705`
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
    pub struct EnvOr3Return {
        pub value: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,bytes32)` and selector `0xb4a85892`
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
    pub struct EnvOr4Return {
        pub value: [u8; 32],
    }
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,int256)` and selector `0xbbcb713e`
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
    pub struct EnvOr5Return {
        pub value: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,string,address[])` and selector `0xc74e9deb`
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
    pub struct EnvOr12Return {
        pub value: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,string)` and selector `0xd145736c`
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
    pub struct EnvOr6Return {
        pub value: ::std::string::String,
    }
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,string,bool[])` and selector `0xeb85e83b`
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
    pub struct EnvOr13Return {
        pub value: ::std::vec::Vec<bool>,
    }
    ///Container type for all return fields from the `envString` function with signature `envString(string,string)` and selector `0x14b02bc9`
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
    pub struct EnvStringWithDelimReturn {
        pub value: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all return fields from the `envString` function with signature `envString(string)` and selector `0xf877cb19`
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
    pub struct EnvStringReturn {
        pub value: ::std::string::String,
    }
    ///Container type for all return fields from the `envUint` function with signature `envUint(string)` and selector `0xc1978d1f`
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
    pub struct EnvUintReturn {
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `envUint` function with signature `envUint(string,string)` and selector `0xf3dec099`
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
    pub struct EnvUintWithDelimReturn {
        pub value: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `eth_getLogs` function with signature `eth_getLogs(uint256,uint256,address,bytes32[])` and selector `0x35e1349b`
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
    pub struct EthGetLogsReturn {
        pub logs: ::std::vec::Vec<EthGetLogs>,
    }
    ///Container type for all return fields from the `exists` function with signature `exists(string)` and selector `0x261a323e`
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
    pub struct ExistsReturn {
        pub result: bool,
    }
    ///Container type for all return fields from the `ffi` function with signature `ffi(string[])` and selector `0x89160467`
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
    pub struct FfiReturn {
        pub result: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `fsMetadata` function with signature `fsMetadata(string)` and selector `0xaf368a08`
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
    pub struct FsMetadataReturn {
        pub metadata: FsMetadata,
    }
    ///Container type for all return fields from the `getBlockNumber` function with signature `getBlockNumber()` and selector `0x42cbb15c`
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
    pub struct GetBlockNumberReturn {
        pub height: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getBlockTimestamp` function with signature `getBlockTimestamp()` and selector `0x796b89b9`
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
    pub struct GetBlockTimestampReturn {
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getCode` function with signature `getCode(string)` and selector `0x8d1cc925`
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
    pub struct GetCodeReturn {
        pub creation_bytecode: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `getDeployedCode` function with signature `getDeployedCode(string)` and selector `0x3ebf73b4`
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
    pub struct GetDeployedCodeReturn {
        pub runtime_bytecode: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `getLabel` function with signature `getLabel(address)` and selector `0x28a249b0`
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
    pub struct GetLabelReturn {
        pub current_label: ::std::string::String,
    }
    ///Container type for all return fields from the `getMappingKeyAndParentOf` function with signature `getMappingKeyAndParentOf(address,bytes32)` and selector `0x876e24e6`
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
    pub struct GetMappingKeyAndParentOfReturn {
        pub found: bool,
        pub key: [u8; 32],
        pub parent: [u8; 32],
    }
    ///Container type for all return fields from the `getMappingLength` function with signature `getMappingLength(address,bytes32)` and selector `0x2f2fd63f`
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
    pub struct GetMappingLengthReturn {
        pub length: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getMappingSlotAt` function with signature `getMappingSlotAt(address,bytes32,uint256)` and selector `0xebc73ab4`
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
    pub struct GetMappingSlotAtReturn {
        pub value: [u8; 32],
    }
    ///Container type for all return fields from the `getNonce` function with signature `getNonce(address)` and selector `0x2d0335ab`
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
    pub struct GetNonceReturn {
        pub nonce: u64,
    }
    ///Container type for all return fields from the `getNonce` function with signature `getNonce((address,uint256,uint256,uint256))` and selector `0xa5748aad`
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
    pub struct GetNonceWithWalletReturn {
        pub nonce: u64,
    }
    ///Container type for all return fields from the `getRecordedLogs` function with signature `getRecordedLogs()` and selector `0x191553a4`
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
    pub struct GetRecordedLogsReturn {
        pub logs: ::std::vec::Vec<Log>,
    }
    ///Container type for all return fields from the `isDir` function with signature `isDir(string)` and selector `0x7d15d019`
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
    pub struct IsDirReturn {
        pub result: bool,
    }
    ///Container type for all return fields from the `isFile` function with signature `isFile(string)` and selector `0xe0eb04d4`
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
    pub struct IsFileReturn {
        pub result: bool,
    }
    ///Container type for all return fields from the `isPersistent` function with signature `isPersistent(address)` and selector `0xd92d8efd`
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
    pub struct IsPersistentReturn {
        pub persistent: bool,
    }
    ///Container type for all return fields from the `keyExists` function with signature `keyExists(string,string)` and selector `0x528a683c`
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
    pub struct KeyExistsReturn(pub bool);
    ///Container type for all return fields from the `load` function with signature `load(address,bytes32)` and selector `0x667f9d70`
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
    pub struct LoadReturn {
        pub data: [u8; 32],
    }
    ///Container type for all return fields from the `parseAddress` function with signature `parseAddress(string)` and selector `0xc6ce059d`
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
    pub struct ParseAddressReturn {
        pub parsed_value: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `parseBool` function with signature `parseBool(string)` and selector `0x974ef924`
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
    pub struct ParseBoolReturn {
        pub parsed_value: bool,
    }
    ///Container type for all return fields from the `parseBytes` function with signature `parseBytes(string)` and selector `0x8f5d232d`
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
    pub struct ParseBytesReturn {
        pub parsed_value: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `parseBytes32` function with signature `parseBytes32(string)` and selector `0x087e6e81`
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
    pub struct ParseBytes32Return {
        pub parsed_value: [u8; 32],
    }
    ///Container type for all return fields from the `parseInt` function with signature `parseInt(string)` and selector `0x42346c5e`
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
    pub struct ParseIntReturn {
        pub parsed_value: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `parseJson` function with signature `parseJson(string)` and selector `0x6a82600a`
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
    pub struct ParseJsonReturn {
        pub abi_encoded_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `parseJson` function with signature `parseJson(string,string)` and selector `0x85940ef1`
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
    pub struct ParseJsonWithKeyReturn {
        pub abi_encoded_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `parseJsonAddress` function with signature `parseJsonAddress(string,string)` and selector `0x1e19e657`
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
    pub struct ParseJsonAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `parseJsonAddressArray` function with signature `parseJsonAddressArray(string,string)` and selector `0x2fce7883`
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
    pub struct ParseJsonAddressArrayReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all return fields from the `parseJsonBool` function with signature `parseJsonBool(string,string)` and selector `0x9f86dc91`
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
    pub struct ParseJsonBoolReturn(pub bool);
    ///Container type for all return fields from the `parseJsonBoolArray` function with signature `parseJsonBoolArray(string,string)` and selector `0x91f3b94f`
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
    pub struct ParseJsonBoolArrayReturn(pub ::std::vec::Vec<bool>);
    ///Container type for all return fields from the `parseJsonBytes` function with signature `parseJsonBytes(string,string)` and selector `0xfd921be8`
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
    pub struct ParseJsonBytesReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `parseJsonBytes32` function with signature `parseJsonBytes32(string,string)` and selector `0x1777e59d`
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
    pub struct ParseJsonBytes32Return(pub [u8; 32]);
    ///Container type for all return fields from the `parseJsonBytes32Array` function with signature `parseJsonBytes32Array(string,string)` and selector `0x91c75bc3`
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
    pub struct ParseJsonBytes32ArrayReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `parseJsonBytesArray` function with signature `parseJsonBytesArray(string,string)` and selector `0x6631aa99`
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
    pub struct ParseJsonBytesArrayReturn(
        pub ::std::vec::Vec<::ethers::core::types::Bytes>,
    );
    ///Container type for all return fields from the `parseJsonInt` function with signature `parseJsonInt(string,string)` and selector `0x7b048ccd`
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
    pub struct ParseJsonIntReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `parseJsonIntArray` function with signature `parseJsonIntArray(string,string)` and selector `0x9983c28a`
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
    pub struct ParseJsonIntArrayReturn(pub ::std::vec::Vec<::ethers::core::types::I256>);
    ///Container type for all return fields from the `parseJsonKeys` function with signature `parseJsonKeys(string,string)` and selector `0x213e4198`
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
    pub struct ParseJsonKeysReturn {
        pub keys: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all return fields from the `parseJsonString` function with signature `parseJsonString(string,string)` and selector `0x49c4fac8`
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
    pub struct ParseJsonStringReturn(pub ::std::string::String);
    ///Container type for all return fields from the `parseJsonStringArray` function with signature `parseJsonStringArray(string,string)` and selector `0x498fdcf4`
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
    pub struct ParseJsonStringArrayReturn(pub ::std::vec::Vec<::std::string::String>);
    ///Container type for all return fields from the `parseJsonUint` function with signature `parseJsonUint(string,string)` and selector `0xaddde2b6`
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
    pub struct ParseJsonUintReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `parseJsonUintArray` function with signature `parseJsonUintArray(string,string)` and selector `0x522074ab`
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
    pub struct ParseJsonUintArrayReturn(
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
    ///Container type for all return fields from the `parseUint` function with signature `parseUint(string)` and selector `0xfa91454d`
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
    pub struct ParseUintReturn {
        pub parsed_value: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `projectRoot` function with signature `projectRoot()` and selector `0xd930a0e6`
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
    pub struct ProjectRootReturn {
        pub path: ::std::string::String,
    }
    ///Container type for all return fields from the `readCallers` function with signature `readCallers()` and selector `0x4ad0bac9`
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
    pub struct ReadCallersReturn {
        pub caller_mode: u8,
        pub msg_sender: ::ethers::core::types::Address,
        pub tx_origin: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `readDir` function with signature `readDir(string,uint64)` and selector `0x1497876c`
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
    pub struct ReadDir1Return {
        pub entries: ::std::vec::Vec<DirEntry>,
    }
    ///Container type for all return fields from the `readDir` function with signature `readDir(string,uint64,bool)` and selector `0x8102d70d`
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
    pub struct ReadDir2Return {
        pub entries: ::std::vec::Vec<DirEntry>,
    }
    ///Container type for all return fields from the `readDir` function with signature `readDir(string)` and selector `0xc4bc59e0`
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
    pub struct ReadDir0Return {
        pub entries: ::std::vec::Vec<DirEntry>,
    }
    ///Container type for all return fields from the `readFile` function with signature `readFile(string)` and selector `0x60f9bb11`
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
    pub struct ReadFileReturn {
        pub data: ::std::string::String,
    }
    ///Container type for all return fields from the `readFileBinary` function with signature `readFileBinary(string)` and selector `0x16ed7bc4`
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
    pub struct ReadFileBinaryReturn {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `readLine` function with signature `readLine(string)` and selector `0x70f55728`
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
    pub struct ReadLineReturn {
        pub line: ::std::string::String,
    }
    ///Container type for all return fields from the `readLink` function with signature `readLink(string)` and selector `0x9f5684a2`
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
    pub struct ReadLinkReturn {
        pub target_path: ::std::string::String,
    }
    ///Container type for all return fields from the `rememberKey` function with signature `rememberKey(uint256)` and selector `0x22100064`
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
    pub struct RememberKeyReturn {
        pub key_addr: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `revertTo` function with signature `revertTo(uint256)` and selector `0x44d7f0a4`
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
    pub struct RevertToReturn {
        pub success: bool,
    }
    ///Container type for all return fields from the `revertToAndDelete` function with signature `revertToAndDelete(uint256)` and selector `0x03e0aca9`
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
    pub struct RevertToAndDeleteReturn {
        pub success: bool,
    }
    ///Container type for all return fields from the `rpc` function with signature `rpc(string,string)` and selector `0x1206c8a8`
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
    pub struct RpcReturn {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `rpcUrl` function with signature `rpcUrl(string)` and selector `0x975a6ce9`
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
    pub struct RpcUrlReturn {
        pub json: ::std::string::String,
    }
    ///Container type for all return fields from the `rpcUrlStructs` function with signature `rpcUrlStructs()` and selector `0x9d2ad72a`
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
    pub struct RpcUrlStructsReturn {
        pub urls: ::std::vec::Vec<Rpc>,
    }
    ///Container type for all return fields from the `rpcUrls` function with signature `rpcUrls()` and selector `0xa85a8418`
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
    pub struct RpcUrlsReturn {
        pub urls: ::std::vec::Vec<[::std::string::String; 2]>,
    }
    ///Container type for all return fields from the `serializeAddress` function with signature `serializeAddress(string,string,address[])` and selector `0x1e356e1a`
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
    pub struct SerializeAddressReturn {
        pub json: ::std::string::String,
    }
    ///Container type for all return fields from the `serializeAddress` function with signature `serializeAddress(string,string,address)` and selector `0x972c6062`
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
    pub struct SerializeAddressWithObjectKeyAndValueKeyAndValueReturn {
        pub json: ::std::string::String,
    }
    ///Container type for all return fields from the `serializeBool` function with signature `serializeBool(string,string,bool[])` and selector `0x92925aa1`
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
    pub struct SerializeBoolReturn {
        pub json: ::std::string::String,
    }
    ///Container type for all return fields from the `serializeBool` function with signature `serializeBool(string,string,bool)` and selector `0xac22e971`
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
    pub struct SerializeBoolWithObjectKeyAndValueKeyAndValueReturn {
        pub json: ::std::string::String,
    }
    ///Container type for all return fields from the `serializeBytes` function with signature `serializeBytes(string,string,bytes[])` and selector `0x9884b232`
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
    pub struct SerializeBytesReturn {
        pub json: ::std::string::String,
    }
    ///Container type for all return fields from the `serializeBytes` function with signature `serializeBytes(string,string,bytes)` and selector `0xf21d52c7`
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
    pub struct SerializeBytesWithObjectKeyAndValueKeyAndValueReturn {
        pub json: ::std::string::String,
    }
    ///Container type for all return fields from the `serializeBytes32` function with signature `serializeBytes32(string,string,bytes32[])` and selector `0x201e43e2`
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
    pub struct SerializeBytes32Return {
        pub json: ::std::string::String,
    }
    ///Container type for all return fields from the `serializeBytes32` function with signature `serializeBytes32(string,string,bytes32)` and selector `0x2d812b44`
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
    pub struct SerializeBytes32WithObjectKeyAndValueKeyAndValueReturn {
        pub json: ::std::string::String,
    }
    ///Container type for all return fields from the `serializeInt` function with signature `serializeInt(string,string,int256)` and selector `0x3f33db60`
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
    pub struct SerializeIntReturn {
        pub json: ::std::string::String,
    }
    ///Container type for all return fields from the `serializeInt` function with signature `serializeInt(string,string,int256[])` and selector `0x7676e127`
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
    pub struct SerializeIntWithObjectKeyAndValueKeyAndValuesReturn {
        pub json: ::std::string::String,
    }
    ///Container type for all return fields from the `serializeJson` function with signature `serializeJson(string,string)` and selector `0x9b3358b0`
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
    pub struct SerializeJsonReturn {
        pub json: ::std::string::String,
    }
    ///Container type for all return fields from the `serializeString` function with signature `serializeString(string,string,string[])` and selector `0x561cd6f3`
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
    pub struct SerializeStringReturn {
        pub json: ::std::string::String,
    }
    ///Container type for all return fields from the `serializeString` function with signature `serializeString(string,string,string)` and selector `0x88da6d35`
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
    pub struct SerializeStringWithObjectKeyAndValueKeyAndValueReturn {
        pub json: ::std::string::String,
    }
    ///Container type for all return fields from the `serializeUint` function with signature `serializeUint(string,string,uint256)` and selector `0x129e9002`
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
    pub struct SerializeUintReturn {
        pub json: ::std::string::String,
    }
    ///Container type for all return fields from the `serializeUint` function with signature `serializeUint(string,string,uint256[])` and selector `0xfee9a469`
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
    pub struct SerializeUintWithObjectKeyAndValueKeyAndValuesReturn {
        pub json: ::std::string::String,
    }
    ///Container type for all return fields from the `sign` function with signature `sign((address,uint256,uint256,uint256),bytes32)` and selector `0xb25c5a25`
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
    pub struct SignReturn {
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all return fields from the `sign` function with signature `sign(uint256,bytes32)` and selector `0xe341eaa4`
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
    pub struct SignWithPrivateKeyReturn {
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all return fields from the `signP256` function with signature `signP256(uint256,bytes32)` and selector `0x83211b40`
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
    pub struct SignP256Return {
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all return fields from the `snapshot` function with signature `snapshot()` and selector `0x9711715a`
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
    pub struct SnapshotReturn {
        pub snapshot_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `stopAndReturnStateDiff` function with signature `stopAndReturnStateDiff()` and selector `0xaa5cf90e`
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
    pub struct StopAndReturnStateDiffReturn {
        pub account_accesses: ::std::vec::Vec<AccountAccess>,
    }
    ///Container type for all return fields from the `toBase64` function with signature `toBase64(string)` and selector `0x3f8be2c8`
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
    pub struct ToBase64Return(pub ::std::string::String);
    ///Container type for all return fields from the `toBase64` function with signature `toBase64(bytes)` and selector `0xa5cbfe65`
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
    pub struct ToBase64WithDataReturn(pub ::std::string::String);
    ///Container type for all return fields from the `toBase64URL` function with signature `toBase64URL(string)` and selector `0xae3165b3`
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
    pub struct ToBase64URLReturn(pub ::std::string::String);
    ///Container type for all return fields from the `toBase64URL` function with signature `toBase64URL(bytes)` and selector `0xc8bd0e4a`
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
    pub struct ToBase64UrlWithDataReturn(pub ::std::string::String);
    ///Container type for all return fields from the `toString` function with signature `toString(address)` and selector `0x56ca623e`
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
    pub struct ToString0Return {
        pub stringified_value: ::std::string::String,
    }
    ///Container type for all return fields from the `toString` function with signature `toString(uint256)` and selector `0x6900a3ae`
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
    pub struct ToString1Return {
        pub stringified_value: ::std::string::String,
    }
    ///Container type for all return fields from the `toString` function with signature `toString(bytes)` and selector `0x71aad10d`
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
    pub struct ToString2Return {
        pub stringified_value: ::std::string::String,
    }
    ///Container type for all return fields from the `toString` function with signature `toString(bool)` and selector `0x71dce7da`
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
    pub struct ToString3Return {
        pub stringified_value: ::std::string::String,
    }
    ///Container type for all return fields from the `toString` function with signature `toString(int256)` and selector `0xa322c40e`
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
    pub struct ToString4Return {
        pub stringified_value: ::std::string::String,
    }
    ///Container type for all return fields from the `toString` function with signature `toString(bytes32)` and selector `0xb11a19e8`
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
    pub struct ToString5Return {
        pub stringified_value: ::std::string::String,
    }
    ///Container type for all return fields from the `tryFfi` function with signature `tryFfi(string[])` and selector `0xf45c1ce7`
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
    pub struct TryFfiReturn {
        pub result: FfiResult,
    }
    ///Container type for all return fields from the `unixTime` function with signature `unixTime()` and selector `0x625387dc`
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
    pub struct UnixTimeReturn {
        pub milliseconds: ::ethers::core::types::U256,
    }
}
