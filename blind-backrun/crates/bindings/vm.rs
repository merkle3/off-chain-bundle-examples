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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reads"),
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
                                    name: ::std::borrow::ToOwned::to_owned("writes"),
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
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("allowCheatcodes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowCheatcodes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("createFork"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createFork"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createFork"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createFork"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createSelectFork"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createSelectFork"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deriveKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                    ::std::borrow::ToOwned::to_owned("difficulty"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("difficulty"),
                            inputs: ::std::vec![
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
                                    name: ::std::string::String::new(),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envBool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envBytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envBytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envInt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("envUint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("expectCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("expectCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("expectEmit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("expectEmit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("fee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fee"),
                            inputs: ::std::vec![
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("isPersistent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isPersistent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("makePersistent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("parseBool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseBool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("parseBytes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseBytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("parseBytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseBytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("parseInt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseInt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("parseJson"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseJson"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseJson"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("parseUint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseUint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("prank"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("prank"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("projectRoot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("projectRoot"),
                            inputs: ::std::vec![],
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
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("revertTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revertTo"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokePersistent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                                    name: ::std::borrow::ToOwned::to_owned("transaction"),
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
                    ::std::borrow::ToOwned::to_owned("rpcUrl"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rpcUrl"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeBool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeBytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeBytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeInt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serializeUint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("snapshot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("snapshot"),
                            inputs: ::std::vec![],
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
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("startPrank"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("startPrank"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            name: ::std::borrow::ToOwned::to_owned("toString"),
                            inputs: ::std::vec![
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
                            name: ::std::borrow::ToOwned::to_owned("toString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("toString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                            name: ::std::borrow::ToOwned::to_owned("toString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                            name: ::std::borrow::ToOwned::to_owned("toString"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("warp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("warp"),
                            inputs: ::std::vec![
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<[u8; 32]>, ::std::vec::Vec<[u8; 32]>),
        > {
            self.0
                .method_hash([101, 188, 148, 129], p0)
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
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([255, 161, 134, 73], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowCheatcodes` (0xea060291) function
        pub fn allow_cheatcodes(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 6, 2, 145], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assume` (0x4c63e562) function
        pub fn assume(
            &self,
            p0: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 99, 229, 98], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `broadcast` (0xafc98040) function
        pub fn broadcast_0(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([175, 201, 128, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `broadcast` (0xe6962cdb) function
        pub fn broadcast_1(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 150, 44, 219], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `broadcast` (0xf67a965b) function
        pub fn broadcast_2(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 122, 150, 91], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `chainId` (0x4049ddd2) function
        pub fn chain_id(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 73, 221, 210], p0)
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
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 195, 36, 31], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `coinbase` (0xff483c54) function
        pub fn coinbase(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([255, 72, 60, 84], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createFork` (0x31ba3498) function
        pub fn create_fork_0(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([49, 186, 52, 152], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createFork` (0x6ba3ba2b) function
        pub fn create_fork_1(
            &self,
            p0: ::std::string::String,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([107, 163, 186, 43], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createFork` (0x7ca29682) function
        pub fn create_fork_2(
            &self,
            p0: ::std::string::String,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([124, 162, 150, 130], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createSelectFork` (0x71ee464d) function
        pub fn create_select_fork_1(
            &self,
            p0: ::std::string::String,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([113, 238, 70, 77], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createSelectFork` (0x84d52b7a) function
        pub fn create_select_fork_2(
            &self,
            p0: ::std::string::String,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([132, 213, 43, 122], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createSelectFork` (0x98680034) function
        pub fn create_select_fork_0(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([152, 104, 0, 52], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deal` (0xc88a5e6d) function
        pub fn deal(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([200, 138, 94, 109], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deriveKey` (0x6229498b) function
        pub fn derive_key_0(
            &self,
            p0: ::std::string::String,
            p1: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([98, 41, 73, 139], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deriveKey` (0x6bcb2c1b) function
        pub fn derive_key_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([107, 203, 44, 27], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `difficulty` (0x46cc92d9) function
        pub fn difficulty(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 204, 146, 217], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envAddress` (0x350d56bf) function
        pub fn env_address_0(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([53, 13, 86, 191], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envAddress` (0xad31b9fa) function
        pub fn env_address_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([173, 49, 185, 250], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envBool` (0x7ed1ec7d) function
        pub fn env_bool_0(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([126, 209, 236, 125], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envBool` (0xaaaddeaf) function
        pub fn env_bool_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<bool>> {
            self.0
                .method_hash([170, 173, 222, 175], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envBytes` (0x4d7baf06) function
        pub fn env_bytes_0(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([77, 123, 175, 6], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envBytes` (0xddc2651b) function
        pub fn env_bytes_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([221, 194, 101, 27], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envBytes32` (0x5af231c1) function
        pub fn env_bytes_321(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([90, 242, 49, 193], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envBytes32` (0x97949042) function
        pub fn env_bytes_320(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([151, 148, 144, 66], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envInt` (0x42181150) function
        pub fn env_int_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::I256>,
        > {
            self.0
                .method_hash([66, 24, 17, 80], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envInt` (0x892a0c61) function
        pub fn env_int_0(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([137, 42, 12, 97], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envString` (0x14b02bc9) function
        pub fn env_string_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([20, 176, 43, 201], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envString` (0xf877cb19) function
        pub fn env_string_0(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([248, 119, 203, 25], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envUint` (0xc1978d1f) function
        pub fn env_uint_0(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([193, 151, 141, 31], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envUint` (0xf3dec099) function
        pub fn env_uint_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([243, 222, 192, 153], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `etch` (0xb4d6c782) function
        pub fn etch(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([180, 214, 199, 130], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectCall` (0xbd6af434) function
        pub fn expect_call_0(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([189, 106, 244, 52], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectCall` (0xf30c7ba3) function
        pub fn expect_call_1(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
            p2: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 12, 123, 163], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectEmit` (0x491cc7c2) function
        pub fn expect_emit_0(
            &self,
            p0: bool,
            p1: bool,
            p2: bool,
            p3: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 28, 199, 194], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectEmit` (0x81bad6f3) function
        pub fn expect_emit_1(
            &self,
            p0: bool,
            p1: bool,
            p2: bool,
            p3: bool,
            p4: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 186, 214, 243], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectRevert` (0xc31eb0e0) function
        pub fn expect_revert_1(
            &self,
            p0: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 30, 176, 224], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectRevert` (0xf28dceb3) function
        pub fn expect_revert_2(
            &self,
            p0: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 141, 206, 179], p0)
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
        ///Calls the contract's `fee` (0x39b37ab0) function
        pub fn fee(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 179, 122, 176], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ffi` (0x89160467) function
        pub fn ffi(
            &self,
            p0: ::std::vec::Vec<::std::string::String>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([137, 22, 4, 103], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCode` (0x8d1cc925) function
        pub fn get_code(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([141, 28, 201, 37], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDeployedCode` (0x3ebf73b4) function
        pub fn get_deployed_code(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([62, 191, 115, 180], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNonce` (0x2d0335ab) function
        pub fn get_nonce(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([45, 3, 53, 171], p0)
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
        ///Calls the contract's `isPersistent` (0xd92d8efd) function
        pub fn is_persistent(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([217, 45, 142, 253], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `label` (0xc657c718) function
        pub fn label(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 87, 199, 24], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `load` (0x667f9d70) function
        pub fn load(
            &self,
            p0: ::ethers::core::types::Address,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([102, 127, 157, 112], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `makePersistent` (0x1d9e269e) function
        pub fn make_persistent_0(
            &self,
            p0: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 158, 38, 158], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `makePersistent` (0x4074e0a8) function
        pub fn make_persistent_2(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 116, 224, 168], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `makePersistent` (0x57e22dde) function
        pub fn make_persistent_1(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 226, 45, 222], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `makePersistent` (0xefb77a75) function
        pub fn make_persistent_3(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 183, 122, 117], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mockCall` (0x81409b91) function
        pub fn mock_call_1(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
            p2: ::ethers::core::types::Bytes,
            p3: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 64, 155, 145], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mockCall` (0xb96213e4) function
        pub fn mock_call_0(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Bytes,
            p2: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 98, 19, 228], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseAddress` (0xc6ce059d) function
        pub fn parse_address(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([198, 206, 5, 157], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseBool` (0x974ef924) function
        pub fn parse_bool(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([151, 78, 249, 36], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseBytes` (0x8f5d232d) function
        pub fn parse_bytes(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([143, 93, 35, 45], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseBytes32` (0x087e6e81) function
        pub fn parse_bytes_32(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([8, 126, 110, 129], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseInt` (0x42346c5e) function
        pub fn parse_int(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([66, 52, 108, 94], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJson` (0x6a82600a) function
        pub fn parse_json_0(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([106, 130, 96, 10], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJson` (0x85940ef1) function
        pub fn parse_json_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([133, 148, 14, 241], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseUint` (0xfa91454d) function
        pub fn parse_uint(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([250, 145, 69, 77], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prank` (0x47e50cce) function
        pub fn prank_1(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 229, 12, 206], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prank` (0xca669fa7) function
        pub fn prank_0(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 102, 159, 167], p0)
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
        ///Calls the contract's `readFile` (0x60f9bb11) function
        pub fn read_file(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([96, 249, 187, 17], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `readFileBinary` (0x16ed7bc4) function
        pub fn read_file_binary(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([22, 237, 123, 196], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `readLine` (0x70f55728) function
        pub fn read_line(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([112, 245, 87, 40], p0)
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
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([34, 16, 0, 100], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeFile` (0xf1afe04d) function
        pub fn remove_file(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 175, 224, 77], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revertTo` (0x44d7f0a4) function
        pub fn revert_to(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([68, 215, 240, 164], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokePersistent` (0x3ce969e6) function
        pub fn revoke_persistent_0(
            &self,
            p0: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 233, 105, 230], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokePersistent` (0x997a0222) function
        pub fn revoke_persistent_1(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 122, 2, 34], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `roll` (0x1f7b4f30) function
        pub fn roll(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 123, 79, 48], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rollFork` (0x0f29772b) function
        pub fn roll_fork_0(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 41, 119, 43], p0)
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
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([217, 187, 243, 161], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rollFork` (0xf2830f7b) function
        pub fn roll_fork_3(
            &self,
            fork_id: ::ethers::core::types::U256,
            transaction: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 131, 15, 123], (fork_id, transaction))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rpcUrl` (0x975a6ce9) function
        pub fn rpc_url(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([151, 90, 108, 233], p0)
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
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([158, 191, 104, 39], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeAddress` (0x1e356e1a) function
        pub fn serialize_address_0(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([30, 53, 110, 26], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeAddress` (0x972c6062) function
        pub fn serialize_address_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([151, 44, 96, 98], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeBool` (0x92925aa1) function
        pub fn serialize_bool_0(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::vec::Vec<bool>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([146, 146, 90, 161], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeBool` (0xac22e971) function
        pub fn serialize_bool_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([172, 34, 233, 113], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeBytes` (0x9884b232) function
        pub fn serialize_bytes_0(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([152, 132, 178, 50], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeBytes` (0xf21d52c7) function
        pub fn serialize_bytes_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([242, 29, 82, 199], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeBytes32` (0x201e43e2) function
        pub fn serialize_bytes_320(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([32, 30, 67, 226], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeBytes32` (0x2d812b44) function
        pub fn serialize_bytes_321(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([45, 129, 43, 68], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeInt` (0x3f33db60) function
        pub fn serialize_int_0(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([63, 51, 219, 96], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeInt` (0x7676e127) function
        pub fn serialize_int_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::vec::Vec<::ethers::core::types::I256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([118, 118, 225, 39], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeString` (0x561cd6f3) function
        pub fn serialize_string_0(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::vec::Vec<::std::string::String>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([86, 28, 214, 243], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeString` (0x88da6d35) function
        pub fn serialize_string_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([136, 218, 109, 53], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeUint` (0x129e9002) function
        pub fn serialize_uint_0(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([18, 158, 144, 2], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeUint` (0xfee9a469) function
        pub fn serialize_uint_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([254, 233, 164, 105], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEnv` (0x3d5923ee) function
        pub fn set_env(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([61, 89, 35, 238], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setNonce` (0xf8e18b57) function
        pub fn set_nonce(
            &self,
            p0: ::ethers::core::types::Address,
            p1: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 225, 139, 87], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sign` (0xe341eaa4) function
        pub fn sign(
            &self,
            p0: ::ethers::core::types::U256,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (u8, [u8; 32], [u8; 32])> {
            self.0
                .method_hash([227, 65, 234, 164], (p0, p1))
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
        pub fn start_broadcast_0(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 181, 41, 127], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startBroadcast` (0x7fec2a8d) function
        pub fn start_broadcast_1(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 236, 42, 141], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startBroadcast` (0xce817d47) function
        pub fn start_broadcast_2(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 129, 125, 71], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startPrank` (0x06447d56) function
        pub fn start_prank_0(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([6, 68, 125, 86], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startPrank` (0x45b56078) function
        pub fn start_prank_1(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([69, 181, 96, 120], (p0, p1))
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
        ///Calls the contract's `stopPrank` (0x90c5013b) function
        pub fn stop_prank(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([144, 197, 1, 59], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `store` (0x70ca10bb) function
        pub fn store(
            &self,
            p0: ::ethers::core::types::Address,
            p1: [u8; 32],
            p2: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 202, 16, 187], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0x56ca623e) function
        pub fn to_string_0(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([86, 202, 98, 62], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0x6900a3ae) function
        pub fn to_string_1(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([105, 0, 163, 174], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0x71aad10d) function
        pub fn to_string_2(
            &self,
            p0: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([113, 170, 209, 13], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0x71dce7da) function
        pub fn to_string_3(
            &self,
            p0: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([113, 220, 231, 218], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0xa322c40e) function
        pub fn to_string_4(
            &self,
            p0: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([163, 34, 196, 14], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0xb11a19e8) function
        pub fn to_string_5(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([177, 26, 25, 232], p0)
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
        ///Calls the contract's `warp` (0xe5d6bf02) function
        pub fn warp(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([229, 214, 191, 2], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `writeFile` (0x897e0a97) function
        pub fn write_file(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 126, 10, 151], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `writeFileBinary` (0x1f21fc80) function
        pub fn write_file_binary(
            &self,
            p0: ::std::string::String,
            p1: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 33, 252, 128], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `writeJson` (0x35d6ad46) function
        pub fn write_json_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 214, 173, 70], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `writeJson` (0xe23cd19f) function
        pub fn write_json_0(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 60, 209, 159], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `writeLine` (0x619d897f) function
        pub fn write_line(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([97, 157, 137, 127], (p0, p1))
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
    pub struct AccessesCall(pub ::ethers::core::types::Address);
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
    pub struct AddrCall(pub ::ethers::core::types::U256);
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
    pub struct AllowCheatcodesCall(pub ::ethers::core::types::Address);
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
    pub struct AssumeCall(pub bool);
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
    pub struct Broadcast0Call;
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
    pub struct Broadcast1Call(pub ::ethers::core::types::Address);
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
    pub struct Broadcast2Call(pub ::ethers::core::types::U256);
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
    pub struct ChainIdCall(pub ::ethers::core::types::U256);
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
    pub struct CloseFileCall(pub ::std::string::String);
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
    pub struct CoinbaseCall(pub ::ethers::core::types::Address);
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
    pub struct CreateFork0Call(pub ::std::string::String);
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
    pub struct CreateFork1Call(
        pub ::std::string::String,
        pub ::ethers::core::types::U256,
    );
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
    pub struct CreateFork2Call(pub ::std::string::String, pub [u8; 32]);
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
    pub struct CreateSelectFork1Call(
        pub ::std::string::String,
        pub ::ethers::core::types::U256,
    );
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
    pub struct CreateSelectFork2Call(pub ::std::string::String, pub [u8; 32]);
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
    pub struct CreateSelectFork0Call(pub ::std::string::String);
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
    pub struct DealCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
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
    pub struct DeriveKey0Call(pub ::std::string::String, pub u32);
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
    pub struct DeriveKey1Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub u32,
    );
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
    pub struct DifficultyCall(pub ::ethers::core::types::U256);
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
    pub struct EnvAddress0Call(pub ::std::string::String);
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
    pub struct EnvAddress1Call(pub ::std::string::String, pub ::std::string::String);
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
    pub struct EnvBool0Call(pub ::std::string::String);
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
    pub struct EnvBool1Call(pub ::std::string::String, pub ::std::string::String);
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
    pub struct EnvBytes0Call(pub ::std::string::String);
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
    pub struct EnvBytes1Call(pub ::std::string::String, pub ::std::string::String);
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
    pub struct EnvBytes321Call(pub ::std::string::String, pub ::std::string::String);
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
    pub struct EnvBytes320Call(pub ::std::string::String);
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
    pub struct EnvInt1Call(pub ::std::string::String, pub ::std::string::String);
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
    pub struct EnvInt0Call(pub ::std::string::String);
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
    pub struct EnvString1Call(pub ::std::string::String, pub ::std::string::String);
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
    pub struct EnvString0Call(pub ::std::string::String);
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
    pub struct EnvUint0Call(pub ::std::string::String);
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
    pub struct EnvUint1Call(pub ::std::string::String, pub ::std::string::String);
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
    pub struct EtchCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Bytes,
    );
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
    pub struct ExpectCall0Call(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Bytes,
    );
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
    pub struct ExpectCall1Call(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
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
    pub struct ExpectEmit0Call(pub bool, pub bool, pub bool, pub bool);
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
    pub struct ExpectEmit1Call(
        pub bool,
        pub bool,
        pub bool,
        pub bool,
        pub ::ethers::core::types::Address,
    );
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
    pub struct ExpectRevert1Call(pub [u8; 4]);
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
    pub struct ExpectRevert2Call(pub ::ethers::core::types::Bytes);
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
    pub struct FeeCall(pub ::ethers::core::types::U256);
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
    pub struct FfiCall(pub ::std::vec::Vec<::std::string::String>);
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
    pub struct GetCodeCall(pub ::std::string::String);
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
    pub struct GetDeployedCodeCall(pub ::std::string::String);
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
    pub struct GetNonceCall(pub ::ethers::core::types::Address);
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
    pub struct IsPersistentCall(pub ::ethers::core::types::Address);
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
    pub struct LabelCall(pub ::ethers::core::types::Address, pub ::std::string::String);
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
    pub struct LoadCall(pub ::ethers::core::types::Address, pub [u8; 32]);
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
    pub struct MakePersistent0Call(pub ::std::vec::Vec<::ethers::core::types::Address>);
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
    pub struct MakePersistent2Call(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
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
    pub struct MakePersistent1Call(pub ::ethers::core::types::Address);
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
    pub struct MakePersistent3Call(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
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
    pub struct MockCall1Call(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
        pub ::ethers::core::types::Bytes,
    );
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
    pub struct MockCall0Call(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Bytes,
        pub ::ethers::core::types::Bytes,
    );
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
    pub struct ParseAddressCall(pub ::std::string::String);
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
    pub struct ParseBoolCall(pub ::std::string::String);
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
    pub struct ParseBytesCall(pub ::std::string::String);
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
    pub struct ParseBytes32Call(pub ::std::string::String);
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
    pub struct ParseIntCall(pub ::std::string::String);
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
    pub struct ParseJson0Call(pub ::std::string::String);
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
    pub struct ParseJson1Call(pub ::std::string::String, pub ::std::string::String);
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
    pub struct ParseUintCall(pub ::std::string::String);
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
    pub struct Prank1Call(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
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
    pub struct Prank0Call(pub ::ethers::core::types::Address);
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
    pub struct ReadFileCall(pub ::std::string::String);
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
    pub struct ReadFileBinaryCall(pub ::std::string::String);
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
    pub struct ReadLineCall(pub ::std::string::String);
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
    pub struct RememberKeyCall(pub ::ethers::core::types::U256);
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
    pub struct RemoveFileCall(pub ::std::string::String);
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
    pub struct RevertToCall(pub ::ethers::core::types::U256);
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
    pub struct RevokePersistent0Call(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
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
    pub struct RevokePersistent1Call(pub ::ethers::core::types::Address);
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
    pub struct RollCall(pub ::ethers::core::types::U256);
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
    pub struct RollFork0Call(pub [u8; 32]);
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
    pub struct RollFork1Call(pub ::ethers::core::types::U256);
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
        pub transaction: [u8; 32],
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
    pub struct RpcUrlCall(pub ::std::string::String);
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
    pub struct SelectForkCall(pub ::ethers::core::types::U256);
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
    pub struct SerializeAddress0Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
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
    pub struct SerializeAddress1Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::ethers::core::types::Address,
    );
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
    pub struct SerializeBool0Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::vec::Vec<bool>,
    );
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
    pub struct SerializeBool1Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub bool,
    );
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
    pub struct SerializeBytes0Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::vec::Vec<::ethers::core::types::Bytes>,
    );
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
    pub struct SerializeBytes1Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::ethers::core::types::Bytes,
    );
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
    pub struct SerializeBytes320Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::vec::Vec<[u8; 32]>,
    );
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
    pub struct SerializeBytes321Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub [u8; 32],
    );
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
    pub struct SerializeInt0Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::ethers::core::types::I256,
    );
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
    pub struct SerializeInt1Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::vec::Vec<::ethers::core::types::I256>,
    );
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
    pub struct SerializeString0Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::vec::Vec<::std::string::String>,
    );
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
    pub struct SerializeString1Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::string::String,
    );
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
    pub struct SerializeUint0Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::ethers::core::types::U256,
    );
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
    pub struct SerializeUint1Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
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
    pub struct SetEnvCall(pub ::std::string::String, pub ::std::string::String);
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
    pub struct SetNonceCall(pub ::ethers::core::types::Address, pub u64);
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
    pub struct SignCall(pub ::ethers::core::types::U256, pub [u8; 32]);
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
    pub struct StartBroadcast0Call;
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
    pub struct StartBroadcast1Call(pub ::ethers::core::types::Address);
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
    pub struct StartBroadcast2Call(pub ::ethers::core::types::U256);
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
    pub struct StartPrank0Call(pub ::ethers::core::types::Address);
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
    pub struct StartPrank1Call(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
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
    pub struct StoreCall(pub ::ethers::core::types::Address, pub [u8; 32], pub [u8; 32]);
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
    pub struct ToString0Call(pub ::ethers::core::types::Address);
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
    pub struct ToString1Call(pub ::ethers::core::types::U256);
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
    pub struct ToString2Call(pub ::ethers::core::types::Bytes);
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
    pub struct ToString3Call(pub bool);
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
    pub struct ToString4Call(pub ::ethers::core::types::I256);
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
    pub struct ToString5Call(pub [u8; 32]);
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
    pub struct WarpCall(pub ::ethers::core::types::U256);
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
    pub struct WriteFileCall(pub ::std::string::String, pub ::std::string::String);
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
    pub struct WriteFileBinaryCall(
        pub ::std::string::String,
        pub ::ethers::core::types::Bytes,
    );
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
    pub struct WriteJson1Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::string::String,
    );
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
    pub struct WriteJson0Call(pub ::std::string::String, pub ::std::string::String);
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
    pub struct WriteLineCall(pub ::std::string::String, pub ::std::string::String);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VmCalls {
        Accesses(AccessesCall),
        ActiveFork(ActiveForkCall),
        Addr(AddrCall),
        AllowCheatcodes(AllowCheatcodesCall),
        Assume(AssumeCall),
        Broadcast0(Broadcast0Call),
        Broadcast1(Broadcast1Call),
        Broadcast2(Broadcast2Call),
        ChainId(ChainIdCall),
        ClearMockedCalls(ClearMockedCallsCall),
        CloseFile(CloseFileCall),
        Coinbase(CoinbaseCall),
        CreateFork0(CreateFork0Call),
        CreateFork1(CreateFork1Call),
        CreateFork2(CreateFork2Call),
        CreateSelectFork1(CreateSelectFork1Call),
        CreateSelectFork2(CreateSelectFork2Call),
        CreateSelectFork0(CreateSelectFork0Call),
        Deal(DealCall),
        DeriveKey0(DeriveKey0Call),
        DeriveKey1(DeriveKey1Call),
        Difficulty(DifficultyCall),
        EnvAddress0(EnvAddress0Call),
        EnvAddress1(EnvAddress1Call),
        EnvBool0(EnvBool0Call),
        EnvBool1(EnvBool1Call),
        EnvBytes0(EnvBytes0Call),
        EnvBytes1(EnvBytes1Call),
        EnvBytes321(EnvBytes321Call),
        EnvBytes320(EnvBytes320Call),
        EnvInt1(EnvInt1Call),
        EnvInt0(EnvInt0Call),
        EnvString1(EnvString1Call),
        EnvString0(EnvString0Call),
        EnvUint0(EnvUint0Call),
        EnvUint1(EnvUint1Call),
        Etch(EtchCall),
        ExpectCall0(ExpectCall0Call),
        ExpectCall1(ExpectCall1Call),
        ExpectEmit0(ExpectEmit0Call),
        ExpectEmit1(ExpectEmit1Call),
        ExpectRevert1(ExpectRevert1Call),
        ExpectRevert2(ExpectRevert2Call),
        ExpectRevert0(ExpectRevert0Call),
        Fee(FeeCall),
        Ffi(FfiCall),
        GetCode(GetCodeCall),
        GetDeployedCode(GetDeployedCodeCall),
        GetNonce(GetNonceCall),
        GetRecordedLogs(GetRecordedLogsCall),
        IsPersistent(IsPersistentCall),
        Label(LabelCall),
        Load(LoadCall),
        MakePersistent0(MakePersistent0Call),
        MakePersistent2(MakePersistent2Call),
        MakePersistent1(MakePersistent1Call),
        MakePersistent3(MakePersistent3Call),
        MockCall1(MockCall1Call),
        MockCall0(MockCall0Call),
        ParseAddress(ParseAddressCall),
        ParseBool(ParseBoolCall),
        ParseBytes(ParseBytesCall),
        ParseBytes32(ParseBytes32Call),
        ParseInt(ParseIntCall),
        ParseJson0(ParseJson0Call),
        ParseJson1(ParseJson1Call),
        ParseUint(ParseUintCall),
        Prank1(Prank1Call),
        Prank0(Prank0Call),
        ProjectRoot(ProjectRootCall),
        ReadFile(ReadFileCall),
        ReadFileBinary(ReadFileBinaryCall),
        ReadLine(ReadLineCall),
        Record(RecordCall),
        RecordLogs(RecordLogsCall),
        RememberKey(RememberKeyCall),
        RemoveFile(RemoveFileCall),
        RevertTo(RevertToCall),
        RevokePersistent0(RevokePersistent0Call),
        RevokePersistent1(RevokePersistent1Call),
        Roll(RollCall),
        RollFork0(RollFork0Call),
        RollFork2(RollFork2Call),
        RollFork1(RollFork1Call),
        RollFork3(RollFork3Call),
        RpcUrl(RpcUrlCall),
        RpcUrlStructs(RpcUrlStructsCall),
        RpcUrls(RpcUrlsCall),
        SelectFork(SelectForkCall),
        SerializeAddress0(SerializeAddress0Call),
        SerializeAddress1(SerializeAddress1Call),
        SerializeBool0(SerializeBool0Call),
        SerializeBool1(SerializeBool1Call),
        SerializeBytes0(SerializeBytes0Call),
        SerializeBytes1(SerializeBytes1Call),
        SerializeBytes320(SerializeBytes320Call),
        SerializeBytes321(SerializeBytes321Call),
        SerializeInt0(SerializeInt0Call),
        SerializeInt1(SerializeInt1Call),
        SerializeString0(SerializeString0Call),
        SerializeString1(SerializeString1Call),
        SerializeUint0(SerializeUint0Call),
        SerializeUint1(SerializeUint1Call),
        SetEnv(SetEnvCall),
        SetNonce(SetNonceCall),
        Sign(SignCall),
        Snapshot(SnapshotCall),
        StartBroadcast0(StartBroadcast0Call),
        StartBroadcast1(StartBroadcast1Call),
        StartBroadcast2(StartBroadcast2Call),
        StartPrank0(StartPrank0Call),
        StartPrank1(StartPrank1Call),
        StopBroadcast(StopBroadcastCall),
        StopPrank(StopPrankCall),
        Store(StoreCall),
        ToString0(ToString0Call),
        ToString1(ToString1Call),
        ToString2(ToString2Call),
        ToString3(ToString3Call),
        ToString4(ToString4Call),
        ToString5(ToString5Call),
        TransactWithForkId(TransactWithForkIdCall),
        Transact(TransactCall),
        Warp(WarpCall),
        WriteFile(WriteFileCall),
        WriteFileBinary(WriteFileBinaryCall),
        WriteJson1(WriteJson1Call),
        WriteJson0(WriteJson0Call),
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
            if let Ok(decoded) = <Broadcast0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Broadcast0(decoded));
            }
            if let Ok(decoded) = <Broadcast1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Broadcast1(decoded));
            }
            if let Ok(decoded) = <Broadcast2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Broadcast2(decoded));
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
            if let Ok(decoded) = <CreateFork0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateFork0(decoded));
            }
            if let Ok(decoded) = <CreateFork1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateFork1(decoded));
            }
            if let Ok(decoded) = <CreateFork2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateFork2(decoded));
            }
            if let Ok(decoded) = <CreateSelectFork1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateSelectFork1(decoded));
            }
            if let Ok(decoded) = <CreateSelectFork2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateSelectFork2(decoded));
            }
            if let Ok(decoded) = <CreateSelectFork0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateSelectFork0(decoded));
            }
            if let Ok(decoded) = <DealCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deal(decoded));
            }
            if let Ok(decoded) = <DeriveKey0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeriveKey0(decoded));
            }
            if let Ok(decoded) = <DeriveKey1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeriveKey1(decoded));
            }
            if let Ok(decoded) = <DifficultyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Difficulty(decoded));
            }
            if let Ok(decoded) = <EnvAddress0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvAddress0(decoded));
            }
            if let Ok(decoded) = <EnvAddress1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvAddress1(decoded));
            }
            if let Ok(decoded) = <EnvBool0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvBool0(decoded));
            }
            if let Ok(decoded) = <EnvBool1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvBool1(decoded));
            }
            if let Ok(decoded) = <EnvBytes0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvBytes0(decoded));
            }
            if let Ok(decoded) = <EnvBytes1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvBytes1(decoded));
            }
            if let Ok(decoded) = <EnvBytes321Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvBytes321(decoded));
            }
            if let Ok(decoded) = <EnvBytes320Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvBytes320(decoded));
            }
            if let Ok(decoded) = <EnvInt1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvInt1(decoded));
            }
            if let Ok(decoded) = <EnvInt0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvInt0(decoded));
            }
            if let Ok(decoded) = <EnvString1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvString1(decoded));
            }
            if let Ok(decoded) = <EnvString0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvString0(decoded));
            }
            if let Ok(decoded) = <EnvUint0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvUint0(decoded));
            }
            if let Ok(decoded) = <EnvUint1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvUint1(decoded));
            }
            if let Ok(decoded) = <EtchCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Etch(decoded));
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
            if let Ok(decoded) = <ExpectEmit0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectEmit0(decoded));
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
            if let Ok(decoded) = <GetNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNonce(decoded));
            }
            if let Ok(decoded) = <GetRecordedLogsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRecordedLogs(decoded));
            }
            if let Ok(decoded) = <IsPersistentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsPersistent(decoded));
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
            if let Ok(decoded) = <MockCall1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MockCall1(decoded));
            }
            if let Ok(decoded) = <MockCall0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MockCall0(decoded));
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
            if let Ok(decoded) = <ParseJson0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJson0(decoded));
            }
            if let Ok(decoded) = <ParseJson1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJson1(decoded));
            }
            if let Ok(decoded) = <ParseUintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseUint(decoded));
            }
            if let Ok(decoded) = <Prank1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Prank1(decoded));
            }
            if let Ok(decoded) = <Prank0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Prank0(decoded));
            }
            if let Ok(decoded) = <ProjectRootCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProjectRoot(decoded));
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
            if let Ok(decoded) = <RemoveFileCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveFile(decoded));
            }
            if let Ok(decoded) = <RevertToCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertTo(decoded));
            }
            if let Ok(decoded) = <RevokePersistent0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokePersistent0(decoded));
            }
            if let Ok(decoded) = <RevokePersistent1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokePersistent1(decoded));
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
            if let Ok(decoded) = <SerializeAddress0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeAddress0(decoded));
            }
            if let Ok(decoded) = <SerializeAddress1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeAddress1(decoded));
            }
            if let Ok(decoded) = <SerializeBool0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeBool0(decoded));
            }
            if let Ok(decoded) = <SerializeBool1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeBool1(decoded));
            }
            if let Ok(decoded) = <SerializeBytes0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeBytes0(decoded));
            }
            if let Ok(decoded) = <SerializeBytes1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeBytes1(decoded));
            }
            if let Ok(decoded) = <SerializeBytes320Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeBytes320(decoded));
            }
            if let Ok(decoded) = <SerializeBytes321Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeBytes321(decoded));
            }
            if let Ok(decoded) = <SerializeInt0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeInt0(decoded));
            }
            if let Ok(decoded) = <SerializeInt1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeInt1(decoded));
            }
            if let Ok(decoded) = <SerializeString0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeString0(decoded));
            }
            if let Ok(decoded) = <SerializeString1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeString1(decoded));
            }
            if let Ok(decoded) = <SerializeUint0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeUint0(decoded));
            }
            if let Ok(decoded) = <SerializeUint1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeUint1(decoded));
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
            if let Ok(decoded) = <SignCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Sign(decoded));
            }
            if let Ok(decoded) = <SnapshotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Snapshot(decoded));
            }
            if let Ok(decoded) = <StartBroadcast0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartBroadcast0(decoded));
            }
            if let Ok(decoded) = <StartBroadcast1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartBroadcast1(decoded));
            }
            if let Ok(decoded) = <StartBroadcast2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartBroadcast2(decoded));
            }
            if let Ok(decoded) = <StartPrank0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartPrank0(decoded));
            }
            if let Ok(decoded) = <StartPrank1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartPrank1(decoded));
            }
            if let Ok(decoded) = <StopBroadcastCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StopBroadcast(decoded));
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
            if let Ok(decoded) = <WriteJson1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WriteJson1(decoded));
            }
            if let Ok(decoded) = <WriteJson0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WriteJson0(decoded));
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
                Self::Broadcast0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Broadcast1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Broadcast2(element) => {
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
                Self::CreateFork0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateFork1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateFork2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateSelectFork1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateSelectFork2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateSelectFork0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deal(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeriveKey0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeriveKey1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Difficulty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvAddress0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvAddress1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvBool0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvBool1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvBytes0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvBytes1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvBytes321(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvBytes320(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvInt1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnvInt0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnvString1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvString0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvUint0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnvUint1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Etch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExpectCall0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectCall1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectEmit0(element) => {
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
                Self::Fee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Ffi(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDeployedCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRecordedLogs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsPersistent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Label(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Load(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::MockCall1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MockCall0(element) => {
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
                Self::ParseJson0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseJson1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseUint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Prank1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Prank0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProjectRoot(element) => {
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
                Self::Record(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RecordLogs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RememberKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveFile(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokePersistent0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokePersistent1(element) => {
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
                Self::RpcUrl(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RpcUrlStructs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RpcUrls(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SelectFork(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeAddress0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeAddress1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeBool0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeBool1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeBytes0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeBytes1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeBytes320(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeBytes321(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeInt0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeInt1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeString0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeString1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeUint0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SerializeUint1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEnv(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Sign(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Snapshot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartBroadcast0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartBroadcast1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartBroadcast2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartPrank0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartPrank1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StopBroadcast(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StopPrank(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Store(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::Warp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WriteFile(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WriteFileBinary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WriteJson1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WriteJson0(element) => {
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
                Self::Broadcast0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Broadcast1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Broadcast2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClearMockedCalls(element) => ::core::fmt::Display::fmt(element, f),
                Self::CloseFile(element) => ::core::fmt::Display::fmt(element, f),
                Self::Coinbase(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateFork0(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateFork1(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateFork2(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateSelectFork1(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateSelectFork2(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateSelectFork0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deal(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeriveKey0(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeriveKey1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Difficulty(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvAddress0(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvAddress1(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvBool0(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvBool1(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvBytes0(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvBytes1(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvBytes321(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvBytes320(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvInt1(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvInt0(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvString1(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvString0(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvUint0(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvUint1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Etch(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectCall0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectCall1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectEmit0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectEmit1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectRevert1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectRevert2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectRevert0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Fee(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ffi(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDeployedCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRecordedLogs(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPersistent(element) => ::core::fmt::Display::fmt(element, f),
                Self::Label(element) => ::core::fmt::Display::fmt(element, f),
                Self::Load(element) => ::core::fmt::Display::fmt(element, f),
                Self::MakePersistent0(element) => ::core::fmt::Display::fmt(element, f),
                Self::MakePersistent2(element) => ::core::fmt::Display::fmt(element, f),
                Self::MakePersistent1(element) => ::core::fmt::Display::fmt(element, f),
                Self::MakePersistent3(element) => ::core::fmt::Display::fmt(element, f),
                Self::MockCall1(element) => ::core::fmt::Display::fmt(element, f),
                Self::MockCall0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseBool(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseBytes(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseInt(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJson0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJson1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Prank1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Prank0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProjectRoot(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadFile(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadFileBinary(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadLine(element) => ::core::fmt::Display::fmt(element, f),
                Self::Record(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecordLogs(element) => ::core::fmt::Display::fmt(element, f),
                Self::RememberKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveFile(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokePersistent0(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokePersistent1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Roll(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollFork0(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollFork2(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollFork1(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollFork3(element) => ::core::fmt::Display::fmt(element, f),
                Self::RpcUrl(element) => ::core::fmt::Display::fmt(element, f),
                Self::RpcUrlStructs(element) => ::core::fmt::Display::fmt(element, f),
                Self::RpcUrls(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelectFork(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeAddress0(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeAddress1(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeBool0(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeBool1(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeBytes0(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeBytes1(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeBytes320(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeBytes321(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeInt0(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeInt1(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeString0(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeString1(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeUint0(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeUint1(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEnv(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sign(element) => ::core::fmt::Display::fmt(element, f),
                Self::Snapshot(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartBroadcast0(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartBroadcast1(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartBroadcast2(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartPrank0(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartPrank1(element) => ::core::fmt::Display::fmt(element, f),
                Self::StopBroadcast(element) => ::core::fmt::Display::fmt(element, f),
                Self::StopPrank(element) => ::core::fmt::Display::fmt(element, f),
                Self::Store(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::Warp(element) => ::core::fmt::Display::fmt(element, f),
                Self::WriteFile(element) => ::core::fmt::Display::fmt(element, f),
                Self::WriteFileBinary(element) => ::core::fmt::Display::fmt(element, f),
                Self::WriteJson1(element) => ::core::fmt::Display::fmt(element, f),
                Self::WriteJson0(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<Broadcast0Call> for VmCalls {
        fn from(value: Broadcast0Call) -> Self {
            Self::Broadcast0(value)
        }
    }
    impl ::core::convert::From<Broadcast1Call> for VmCalls {
        fn from(value: Broadcast1Call) -> Self {
            Self::Broadcast1(value)
        }
    }
    impl ::core::convert::From<Broadcast2Call> for VmCalls {
        fn from(value: Broadcast2Call) -> Self {
            Self::Broadcast2(value)
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
    impl ::core::convert::From<CreateFork0Call> for VmCalls {
        fn from(value: CreateFork0Call) -> Self {
            Self::CreateFork0(value)
        }
    }
    impl ::core::convert::From<CreateFork1Call> for VmCalls {
        fn from(value: CreateFork1Call) -> Self {
            Self::CreateFork1(value)
        }
    }
    impl ::core::convert::From<CreateFork2Call> for VmCalls {
        fn from(value: CreateFork2Call) -> Self {
            Self::CreateFork2(value)
        }
    }
    impl ::core::convert::From<CreateSelectFork1Call> for VmCalls {
        fn from(value: CreateSelectFork1Call) -> Self {
            Self::CreateSelectFork1(value)
        }
    }
    impl ::core::convert::From<CreateSelectFork2Call> for VmCalls {
        fn from(value: CreateSelectFork2Call) -> Self {
            Self::CreateSelectFork2(value)
        }
    }
    impl ::core::convert::From<CreateSelectFork0Call> for VmCalls {
        fn from(value: CreateSelectFork0Call) -> Self {
            Self::CreateSelectFork0(value)
        }
    }
    impl ::core::convert::From<DealCall> for VmCalls {
        fn from(value: DealCall) -> Self {
            Self::Deal(value)
        }
    }
    impl ::core::convert::From<DeriveKey0Call> for VmCalls {
        fn from(value: DeriveKey0Call) -> Self {
            Self::DeriveKey0(value)
        }
    }
    impl ::core::convert::From<DeriveKey1Call> for VmCalls {
        fn from(value: DeriveKey1Call) -> Self {
            Self::DeriveKey1(value)
        }
    }
    impl ::core::convert::From<DifficultyCall> for VmCalls {
        fn from(value: DifficultyCall) -> Self {
            Self::Difficulty(value)
        }
    }
    impl ::core::convert::From<EnvAddress0Call> for VmCalls {
        fn from(value: EnvAddress0Call) -> Self {
            Self::EnvAddress0(value)
        }
    }
    impl ::core::convert::From<EnvAddress1Call> for VmCalls {
        fn from(value: EnvAddress1Call) -> Self {
            Self::EnvAddress1(value)
        }
    }
    impl ::core::convert::From<EnvBool0Call> for VmCalls {
        fn from(value: EnvBool0Call) -> Self {
            Self::EnvBool0(value)
        }
    }
    impl ::core::convert::From<EnvBool1Call> for VmCalls {
        fn from(value: EnvBool1Call) -> Self {
            Self::EnvBool1(value)
        }
    }
    impl ::core::convert::From<EnvBytes0Call> for VmCalls {
        fn from(value: EnvBytes0Call) -> Self {
            Self::EnvBytes0(value)
        }
    }
    impl ::core::convert::From<EnvBytes1Call> for VmCalls {
        fn from(value: EnvBytes1Call) -> Self {
            Self::EnvBytes1(value)
        }
    }
    impl ::core::convert::From<EnvBytes321Call> for VmCalls {
        fn from(value: EnvBytes321Call) -> Self {
            Self::EnvBytes321(value)
        }
    }
    impl ::core::convert::From<EnvBytes320Call> for VmCalls {
        fn from(value: EnvBytes320Call) -> Self {
            Self::EnvBytes320(value)
        }
    }
    impl ::core::convert::From<EnvInt1Call> for VmCalls {
        fn from(value: EnvInt1Call) -> Self {
            Self::EnvInt1(value)
        }
    }
    impl ::core::convert::From<EnvInt0Call> for VmCalls {
        fn from(value: EnvInt0Call) -> Self {
            Self::EnvInt0(value)
        }
    }
    impl ::core::convert::From<EnvString1Call> for VmCalls {
        fn from(value: EnvString1Call) -> Self {
            Self::EnvString1(value)
        }
    }
    impl ::core::convert::From<EnvString0Call> for VmCalls {
        fn from(value: EnvString0Call) -> Self {
            Self::EnvString0(value)
        }
    }
    impl ::core::convert::From<EnvUint0Call> for VmCalls {
        fn from(value: EnvUint0Call) -> Self {
            Self::EnvUint0(value)
        }
    }
    impl ::core::convert::From<EnvUint1Call> for VmCalls {
        fn from(value: EnvUint1Call) -> Self {
            Self::EnvUint1(value)
        }
    }
    impl ::core::convert::From<EtchCall> for VmCalls {
        fn from(value: EtchCall) -> Self {
            Self::Etch(value)
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
    impl ::core::convert::From<ExpectEmit0Call> for VmCalls {
        fn from(value: ExpectEmit0Call) -> Self {
            Self::ExpectEmit0(value)
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
    impl ::core::convert::From<GetNonceCall> for VmCalls {
        fn from(value: GetNonceCall) -> Self {
            Self::GetNonce(value)
        }
    }
    impl ::core::convert::From<GetRecordedLogsCall> for VmCalls {
        fn from(value: GetRecordedLogsCall) -> Self {
            Self::GetRecordedLogs(value)
        }
    }
    impl ::core::convert::From<IsPersistentCall> for VmCalls {
        fn from(value: IsPersistentCall) -> Self {
            Self::IsPersistent(value)
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
    impl ::core::convert::From<MockCall1Call> for VmCalls {
        fn from(value: MockCall1Call) -> Self {
            Self::MockCall1(value)
        }
    }
    impl ::core::convert::From<MockCall0Call> for VmCalls {
        fn from(value: MockCall0Call) -> Self {
            Self::MockCall0(value)
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
    impl ::core::convert::From<ParseJson0Call> for VmCalls {
        fn from(value: ParseJson0Call) -> Self {
            Self::ParseJson0(value)
        }
    }
    impl ::core::convert::From<ParseJson1Call> for VmCalls {
        fn from(value: ParseJson1Call) -> Self {
            Self::ParseJson1(value)
        }
    }
    impl ::core::convert::From<ParseUintCall> for VmCalls {
        fn from(value: ParseUintCall) -> Self {
            Self::ParseUint(value)
        }
    }
    impl ::core::convert::From<Prank1Call> for VmCalls {
        fn from(value: Prank1Call) -> Self {
            Self::Prank1(value)
        }
    }
    impl ::core::convert::From<Prank0Call> for VmCalls {
        fn from(value: Prank0Call) -> Self {
            Self::Prank0(value)
        }
    }
    impl ::core::convert::From<ProjectRootCall> for VmCalls {
        fn from(value: ProjectRootCall) -> Self {
            Self::ProjectRoot(value)
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
    impl ::core::convert::From<RemoveFileCall> for VmCalls {
        fn from(value: RemoveFileCall) -> Self {
            Self::RemoveFile(value)
        }
    }
    impl ::core::convert::From<RevertToCall> for VmCalls {
        fn from(value: RevertToCall) -> Self {
            Self::RevertTo(value)
        }
    }
    impl ::core::convert::From<RevokePersistent0Call> for VmCalls {
        fn from(value: RevokePersistent0Call) -> Self {
            Self::RevokePersistent0(value)
        }
    }
    impl ::core::convert::From<RevokePersistent1Call> for VmCalls {
        fn from(value: RevokePersistent1Call) -> Self {
            Self::RevokePersistent1(value)
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
    impl ::core::convert::From<SerializeAddress0Call> for VmCalls {
        fn from(value: SerializeAddress0Call) -> Self {
            Self::SerializeAddress0(value)
        }
    }
    impl ::core::convert::From<SerializeAddress1Call> for VmCalls {
        fn from(value: SerializeAddress1Call) -> Self {
            Self::SerializeAddress1(value)
        }
    }
    impl ::core::convert::From<SerializeBool0Call> for VmCalls {
        fn from(value: SerializeBool0Call) -> Self {
            Self::SerializeBool0(value)
        }
    }
    impl ::core::convert::From<SerializeBool1Call> for VmCalls {
        fn from(value: SerializeBool1Call) -> Self {
            Self::SerializeBool1(value)
        }
    }
    impl ::core::convert::From<SerializeBytes0Call> for VmCalls {
        fn from(value: SerializeBytes0Call) -> Self {
            Self::SerializeBytes0(value)
        }
    }
    impl ::core::convert::From<SerializeBytes1Call> for VmCalls {
        fn from(value: SerializeBytes1Call) -> Self {
            Self::SerializeBytes1(value)
        }
    }
    impl ::core::convert::From<SerializeBytes320Call> for VmCalls {
        fn from(value: SerializeBytes320Call) -> Self {
            Self::SerializeBytes320(value)
        }
    }
    impl ::core::convert::From<SerializeBytes321Call> for VmCalls {
        fn from(value: SerializeBytes321Call) -> Self {
            Self::SerializeBytes321(value)
        }
    }
    impl ::core::convert::From<SerializeInt0Call> for VmCalls {
        fn from(value: SerializeInt0Call) -> Self {
            Self::SerializeInt0(value)
        }
    }
    impl ::core::convert::From<SerializeInt1Call> for VmCalls {
        fn from(value: SerializeInt1Call) -> Self {
            Self::SerializeInt1(value)
        }
    }
    impl ::core::convert::From<SerializeString0Call> for VmCalls {
        fn from(value: SerializeString0Call) -> Self {
            Self::SerializeString0(value)
        }
    }
    impl ::core::convert::From<SerializeString1Call> for VmCalls {
        fn from(value: SerializeString1Call) -> Self {
            Self::SerializeString1(value)
        }
    }
    impl ::core::convert::From<SerializeUint0Call> for VmCalls {
        fn from(value: SerializeUint0Call) -> Self {
            Self::SerializeUint0(value)
        }
    }
    impl ::core::convert::From<SerializeUint1Call> for VmCalls {
        fn from(value: SerializeUint1Call) -> Self {
            Self::SerializeUint1(value)
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
    impl ::core::convert::From<SignCall> for VmCalls {
        fn from(value: SignCall) -> Self {
            Self::Sign(value)
        }
    }
    impl ::core::convert::From<SnapshotCall> for VmCalls {
        fn from(value: SnapshotCall) -> Self {
            Self::Snapshot(value)
        }
    }
    impl ::core::convert::From<StartBroadcast0Call> for VmCalls {
        fn from(value: StartBroadcast0Call) -> Self {
            Self::StartBroadcast0(value)
        }
    }
    impl ::core::convert::From<StartBroadcast1Call> for VmCalls {
        fn from(value: StartBroadcast1Call) -> Self {
            Self::StartBroadcast1(value)
        }
    }
    impl ::core::convert::From<StartBroadcast2Call> for VmCalls {
        fn from(value: StartBroadcast2Call) -> Self {
            Self::StartBroadcast2(value)
        }
    }
    impl ::core::convert::From<StartPrank0Call> for VmCalls {
        fn from(value: StartPrank0Call) -> Self {
            Self::StartPrank0(value)
        }
    }
    impl ::core::convert::From<StartPrank1Call> for VmCalls {
        fn from(value: StartPrank1Call) -> Self {
            Self::StartPrank1(value)
        }
    }
    impl ::core::convert::From<StopBroadcastCall> for VmCalls {
        fn from(value: StopBroadcastCall) -> Self {
            Self::StopBroadcast(value)
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
    impl ::core::convert::From<WriteJson1Call> for VmCalls {
        fn from(value: WriteJson1Call) -> Self {
            Self::WriteJson1(value)
        }
    }
    impl ::core::convert::From<WriteJson0Call> for VmCalls {
        fn from(value: WriteJson0Call) -> Self {
            Self::WriteJson0(value)
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
        pub reads: ::std::vec::Vec<[u8; 32]>,
        pub writes: ::std::vec::Vec<[u8; 32]>,
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
    pub struct ActiveForkReturn(pub ::ethers::core::types::U256);
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
    pub struct AddrReturn(pub ::ethers::core::types::Address);
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
    pub struct CreateFork0Return(pub ::ethers::core::types::U256);
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
    pub struct CreateFork1Return(pub ::ethers::core::types::U256);
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
    pub struct CreateFork2Return(pub ::ethers::core::types::U256);
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
    pub struct CreateSelectFork1Return(pub ::ethers::core::types::U256);
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
    pub struct CreateSelectFork2Return(pub ::ethers::core::types::U256);
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
    pub struct CreateSelectFork0Return(pub ::ethers::core::types::U256);
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
    pub struct DeriveKey0Return(pub ::ethers::core::types::U256);
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
    pub struct DeriveKey1Return(pub ::ethers::core::types::U256);
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
    pub struct EnvAddress0Return(pub ::ethers::core::types::Address);
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
    pub struct EnvAddress1Return(pub ::std::vec::Vec<::ethers::core::types::Address>);
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
    pub struct EnvBool0Return(pub bool);
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
    pub struct EnvBool1Return(pub ::std::vec::Vec<bool>);
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
    pub struct EnvBytes0Return(pub ::ethers::core::types::Bytes);
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
    pub struct EnvBytes1Return(pub ::std::vec::Vec<::ethers::core::types::Bytes>);
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
    pub struct EnvBytes321Return(pub ::std::vec::Vec<[u8; 32]>);
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
    pub struct EnvBytes320Return(pub [u8; 32]);
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
    pub struct EnvInt1Return(pub ::std::vec::Vec<::ethers::core::types::I256>);
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
    pub struct EnvInt0Return(pub ::ethers::core::types::I256);
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
    pub struct EnvString1Return(pub ::std::vec::Vec<::std::string::String>);
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
    pub struct EnvString0Return(pub ::std::string::String);
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
    pub struct EnvUint0Return(pub ::ethers::core::types::U256);
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
    pub struct EnvUint1Return(pub ::std::vec::Vec<::ethers::core::types::U256>);
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
    pub struct FfiReturn(pub ::ethers::core::types::Bytes);
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
    pub struct GetCodeReturn(pub ::ethers::core::types::Bytes);
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
    pub struct GetDeployedCodeReturn(pub ::ethers::core::types::Bytes);
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
    pub struct GetNonceReturn(pub u64);
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
    pub struct GetRecordedLogsReturn(pub ::std::vec::Vec<Log>);
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
    pub struct IsPersistentReturn(pub bool);
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
    pub struct LoadReturn(pub [u8; 32]);
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
    pub struct ParseAddressReturn(pub ::ethers::core::types::Address);
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
    pub struct ParseBoolReturn(pub bool);
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
    pub struct ParseBytesReturn(pub ::ethers::core::types::Bytes);
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
    pub struct ParseBytes32Return(pub [u8; 32]);
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
    pub struct ParseIntReturn(pub ::ethers::core::types::I256);
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
    pub struct ParseJson0Return(pub ::ethers::core::types::Bytes);
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
    pub struct ParseJson1Return(pub ::ethers::core::types::Bytes);
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
    pub struct ParseUintReturn(pub ::ethers::core::types::U256);
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
    pub struct ProjectRootReturn(pub ::std::string::String);
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
    pub struct ReadFileReturn(pub ::std::string::String);
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
    pub struct ReadFileBinaryReturn(pub ::ethers::core::types::Bytes);
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
    pub struct ReadLineReturn(pub ::std::string::String);
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
    pub struct RememberKeyReturn(pub ::ethers::core::types::Address);
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
    pub struct RevertToReturn(pub bool);
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
    pub struct RpcUrlReturn(pub ::std::string::String);
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
    pub struct RpcUrlStructsReturn(pub ::std::vec::Vec<Rpc>);
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
    pub struct RpcUrlsReturn(pub ::std::vec::Vec<[::std::string::String; 2]>);
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
    pub struct SerializeAddress0Return(pub ::std::string::String);
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
    pub struct SerializeAddress1Return(pub ::std::string::String);
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
    pub struct SerializeBool0Return(pub ::std::string::String);
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
    pub struct SerializeBool1Return(pub ::std::string::String);
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
    pub struct SerializeBytes0Return(pub ::std::string::String);
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
    pub struct SerializeBytes1Return(pub ::std::string::String);
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
    pub struct SerializeBytes320Return(pub ::std::string::String);
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
    pub struct SerializeBytes321Return(pub ::std::string::String);
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
    pub struct SerializeInt0Return(pub ::std::string::String);
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
    pub struct SerializeInt1Return(pub ::std::string::String);
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
    pub struct SerializeString0Return(pub ::std::string::String);
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
    pub struct SerializeString1Return(pub ::std::string::String);
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
    pub struct SerializeUint0Return(pub ::std::string::String);
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
    pub struct SerializeUint1Return(pub ::std::string::String);
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
    pub struct SignReturn(pub u8, pub [u8; 32], pub [u8; 32]);
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
    pub struct SnapshotReturn(pub ::ethers::core::types::U256);
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
    pub struct ToString0Return(pub ::std::string::String);
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
    pub struct ToString1Return(pub ::std::string::String);
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
    pub struct ToString2Return(pub ::std::string::String);
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
    pub struct ToString3Return(pub ::std::string::String);
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
    pub struct ToString4Return(pub ::std::string::String);
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
    pub struct ToString5Return(pub ::std::string::String);
}
