pub use vm_safe::*;
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
pub mod vm_safe {
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
    pub static VMSAFE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct VmSafe<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for VmSafe<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for VmSafe<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for VmSafe<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for VmSafe<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(VmSafe)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> VmSafe<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    VMSAFE_ABI.clone(),
                    client,
                ),
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
        ///Calls the contract's `closeFile` (0x48c3241f) function
        pub fn close_file(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 195, 36, 31], p0)
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
        ///Calls the contract's `stopBroadcast` (0x76eadd36) function
        pub fn stop_broadcast(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([118, 234, 221, 54], ())
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
    for VmSafe<M> {
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
    pub enum VmSafeCalls {
        Accesses(AccessesCall),
        Addr(AddrCall),
        Assume(AssumeCall),
        Broadcast0(Broadcast0Call),
        Broadcast1(Broadcast1Call),
        Broadcast2(Broadcast2Call),
        CloseFile(CloseFileCall),
        DeriveKey0(DeriveKey0Call),
        DeriveKey1(DeriveKey1Call),
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
        Ffi(FfiCall),
        GetCode(GetCodeCall),
        GetDeployedCode(GetDeployedCodeCall),
        GetNonce(GetNonceCall),
        GetRecordedLogs(GetRecordedLogsCall),
        Label(LabelCall),
        Load(LoadCall),
        ParseAddress(ParseAddressCall),
        ParseBool(ParseBoolCall),
        ParseBytes(ParseBytesCall),
        ParseBytes32(ParseBytes32Call),
        ParseInt(ParseIntCall),
        ParseJson0(ParseJson0Call),
        ParseJson1(ParseJson1Call),
        ParseUint(ParseUintCall),
        ProjectRoot(ProjectRootCall),
        ReadFile(ReadFileCall),
        ReadFileBinary(ReadFileBinaryCall),
        ReadLine(ReadLineCall),
        Record(RecordCall),
        RecordLogs(RecordLogsCall),
        RememberKey(RememberKeyCall),
        RemoveFile(RemoveFileCall),
        RpcUrl(RpcUrlCall),
        RpcUrlStructs(RpcUrlStructsCall),
        RpcUrls(RpcUrlsCall),
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
        Sign(SignCall),
        StartBroadcast0(StartBroadcast0Call),
        StartBroadcast1(StartBroadcast1Call),
        StartBroadcast2(StartBroadcast2Call),
        StopBroadcast(StopBroadcastCall),
        ToString0(ToString0Call),
        ToString1(ToString1Call),
        ToString2(ToString2Call),
        ToString3(ToString3Call),
        ToString4(ToString4Call),
        ToString5(ToString5Call),
        WriteFile(WriteFileCall),
        WriteFileBinary(WriteFileBinaryCall),
        WriteJson1(WriteJson1Call),
        WriteJson0(WriteJson0Call),
        WriteLine(WriteLineCall),
    }
    impl ::ethers::core::abi::AbiDecode for VmSafeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AccessesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Accesses(decoded));
            }
            if let Ok(decoded) = <AddrCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Addr(decoded));
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
            if let Ok(decoded) = <CloseFileCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CloseFile(decoded));
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
            if let Ok(decoded) = <SignCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Sign(decoded));
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
            if let Ok(decoded) = <StopBroadcastCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StopBroadcast(decoded));
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
    impl ::ethers::core::abi::AbiEncode for VmSafeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Accesses(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Addr(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::CloseFile(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeriveKey0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeriveKey1(element) => {
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
                Self::Label(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Load(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::RpcUrl(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RpcUrlStructs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RpcUrls(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::Sign(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StartBroadcast0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartBroadcast1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartBroadcast2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StopBroadcast(element) => {
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
    impl ::core::fmt::Display for VmSafeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Accesses(element) => ::core::fmt::Display::fmt(element, f),
                Self::Addr(element) => ::core::fmt::Display::fmt(element, f),
                Self::Assume(element) => ::core::fmt::Display::fmt(element, f),
                Self::Broadcast0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Broadcast1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Broadcast2(element) => ::core::fmt::Display::fmt(element, f),
                Self::CloseFile(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeriveKey0(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeriveKey1(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::Ffi(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDeployedCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRecordedLogs(element) => ::core::fmt::Display::fmt(element, f),
                Self::Label(element) => ::core::fmt::Display::fmt(element, f),
                Self::Load(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseBool(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseBytes(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseInt(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJson0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJson1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProjectRoot(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadFile(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadFileBinary(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadLine(element) => ::core::fmt::Display::fmt(element, f),
                Self::Record(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecordLogs(element) => ::core::fmt::Display::fmt(element, f),
                Self::RememberKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveFile(element) => ::core::fmt::Display::fmt(element, f),
                Self::RpcUrl(element) => ::core::fmt::Display::fmt(element, f),
                Self::RpcUrlStructs(element) => ::core::fmt::Display::fmt(element, f),
                Self::RpcUrls(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::Sign(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartBroadcast0(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartBroadcast1(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartBroadcast2(element) => ::core::fmt::Display::fmt(element, f),
                Self::StopBroadcast(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToString0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToString1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToString2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToString3(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToString4(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToString5(element) => ::core::fmt::Display::fmt(element, f),
                Self::WriteFile(element) => ::core::fmt::Display::fmt(element, f),
                Self::WriteFileBinary(element) => ::core::fmt::Display::fmt(element, f),
                Self::WriteJson1(element) => ::core::fmt::Display::fmt(element, f),
                Self::WriteJson0(element) => ::core::fmt::Display::fmt(element, f),
                Self::WriteLine(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AccessesCall> for VmSafeCalls {
        fn from(value: AccessesCall) -> Self {
            Self::Accesses(value)
        }
    }
    impl ::core::convert::From<AddrCall> for VmSafeCalls {
        fn from(value: AddrCall) -> Self {
            Self::Addr(value)
        }
    }
    impl ::core::convert::From<AssumeCall> for VmSafeCalls {
        fn from(value: AssumeCall) -> Self {
            Self::Assume(value)
        }
    }
    impl ::core::convert::From<Broadcast0Call> for VmSafeCalls {
        fn from(value: Broadcast0Call) -> Self {
            Self::Broadcast0(value)
        }
    }
    impl ::core::convert::From<Broadcast1Call> for VmSafeCalls {
        fn from(value: Broadcast1Call) -> Self {
            Self::Broadcast1(value)
        }
    }
    impl ::core::convert::From<Broadcast2Call> for VmSafeCalls {
        fn from(value: Broadcast2Call) -> Self {
            Self::Broadcast2(value)
        }
    }
    impl ::core::convert::From<CloseFileCall> for VmSafeCalls {
        fn from(value: CloseFileCall) -> Self {
            Self::CloseFile(value)
        }
    }
    impl ::core::convert::From<DeriveKey0Call> for VmSafeCalls {
        fn from(value: DeriveKey0Call) -> Self {
            Self::DeriveKey0(value)
        }
    }
    impl ::core::convert::From<DeriveKey1Call> for VmSafeCalls {
        fn from(value: DeriveKey1Call) -> Self {
            Self::DeriveKey1(value)
        }
    }
    impl ::core::convert::From<EnvAddress0Call> for VmSafeCalls {
        fn from(value: EnvAddress0Call) -> Self {
            Self::EnvAddress0(value)
        }
    }
    impl ::core::convert::From<EnvAddress1Call> for VmSafeCalls {
        fn from(value: EnvAddress1Call) -> Self {
            Self::EnvAddress1(value)
        }
    }
    impl ::core::convert::From<EnvBool0Call> for VmSafeCalls {
        fn from(value: EnvBool0Call) -> Self {
            Self::EnvBool0(value)
        }
    }
    impl ::core::convert::From<EnvBool1Call> for VmSafeCalls {
        fn from(value: EnvBool1Call) -> Self {
            Self::EnvBool1(value)
        }
    }
    impl ::core::convert::From<EnvBytes0Call> for VmSafeCalls {
        fn from(value: EnvBytes0Call) -> Self {
            Self::EnvBytes0(value)
        }
    }
    impl ::core::convert::From<EnvBytes1Call> for VmSafeCalls {
        fn from(value: EnvBytes1Call) -> Self {
            Self::EnvBytes1(value)
        }
    }
    impl ::core::convert::From<EnvBytes321Call> for VmSafeCalls {
        fn from(value: EnvBytes321Call) -> Self {
            Self::EnvBytes321(value)
        }
    }
    impl ::core::convert::From<EnvBytes320Call> for VmSafeCalls {
        fn from(value: EnvBytes320Call) -> Self {
            Self::EnvBytes320(value)
        }
    }
    impl ::core::convert::From<EnvInt1Call> for VmSafeCalls {
        fn from(value: EnvInt1Call) -> Self {
            Self::EnvInt1(value)
        }
    }
    impl ::core::convert::From<EnvInt0Call> for VmSafeCalls {
        fn from(value: EnvInt0Call) -> Self {
            Self::EnvInt0(value)
        }
    }
    impl ::core::convert::From<EnvString1Call> for VmSafeCalls {
        fn from(value: EnvString1Call) -> Self {
            Self::EnvString1(value)
        }
    }
    impl ::core::convert::From<EnvString0Call> for VmSafeCalls {
        fn from(value: EnvString0Call) -> Self {
            Self::EnvString0(value)
        }
    }
    impl ::core::convert::From<EnvUint0Call> for VmSafeCalls {
        fn from(value: EnvUint0Call) -> Self {
            Self::EnvUint0(value)
        }
    }
    impl ::core::convert::From<EnvUint1Call> for VmSafeCalls {
        fn from(value: EnvUint1Call) -> Self {
            Self::EnvUint1(value)
        }
    }
    impl ::core::convert::From<FfiCall> for VmSafeCalls {
        fn from(value: FfiCall) -> Self {
            Self::Ffi(value)
        }
    }
    impl ::core::convert::From<GetCodeCall> for VmSafeCalls {
        fn from(value: GetCodeCall) -> Self {
            Self::GetCode(value)
        }
    }
    impl ::core::convert::From<GetDeployedCodeCall> for VmSafeCalls {
        fn from(value: GetDeployedCodeCall) -> Self {
            Self::GetDeployedCode(value)
        }
    }
    impl ::core::convert::From<GetNonceCall> for VmSafeCalls {
        fn from(value: GetNonceCall) -> Self {
            Self::GetNonce(value)
        }
    }
    impl ::core::convert::From<GetRecordedLogsCall> for VmSafeCalls {
        fn from(value: GetRecordedLogsCall) -> Self {
            Self::GetRecordedLogs(value)
        }
    }
    impl ::core::convert::From<LabelCall> for VmSafeCalls {
        fn from(value: LabelCall) -> Self {
            Self::Label(value)
        }
    }
    impl ::core::convert::From<LoadCall> for VmSafeCalls {
        fn from(value: LoadCall) -> Self {
            Self::Load(value)
        }
    }
    impl ::core::convert::From<ParseAddressCall> for VmSafeCalls {
        fn from(value: ParseAddressCall) -> Self {
            Self::ParseAddress(value)
        }
    }
    impl ::core::convert::From<ParseBoolCall> for VmSafeCalls {
        fn from(value: ParseBoolCall) -> Self {
            Self::ParseBool(value)
        }
    }
    impl ::core::convert::From<ParseBytesCall> for VmSafeCalls {
        fn from(value: ParseBytesCall) -> Self {
            Self::ParseBytes(value)
        }
    }
    impl ::core::convert::From<ParseBytes32Call> for VmSafeCalls {
        fn from(value: ParseBytes32Call) -> Self {
            Self::ParseBytes32(value)
        }
    }
    impl ::core::convert::From<ParseIntCall> for VmSafeCalls {
        fn from(value: ParseIntCall) -> Self {
            Self::ParseInt(value)
        }
    }
    impl ::core::convert::From<ParseJson0Call> for VmSafeCalls {
        fn from(value: ParseJson0Call) -> Self {
            Self::ParseJson0(value)
        }
    }
    impl ::core::convert::From<ParseJson1Call> for VmSafeCalls {
        fn from(value: ParseJson1Call) -> Self {
            Self::ParseJson1(value)
        }
    }
    impl ::core::convert::From<ParseUintCall> for VmSafeCalls {
        fn from(value: ParseUintCall) -> Self {
            Self::ParseUint(value)
        }
    }
    impl ::core::convert::From<ProjectRootCall> for VmSafeCalls {
        fn from(value: ProjectRootCall) -> Self {
            Self::ProjectRoot(value)
        }
    }
    impl ::core::convert::From<ReadFileCall> for VmSafeCalls {
        fn from(value: ReadFileCall) -> Self {
            Self::ReadFile(value)
        }
    }
    impl ::core::convert::From<ReadFileBinaryCall> for VmSafeCalls {
        fn from(value: ReadFileBinaryCall) -> Self {
            Self::ReadFileBinary(value)
        }
    }
    impl ::core::convert::From<ReadLineCall> for VmSafeCalls {
        fn from(value: ReadLineCall) -> Self {
            Self::ReadLine(value)
        }
    }
    impl ::core::convert::From<RecordCall> for VmSafeCalls {
        fn from(value: RecordCall) -> Self {
            Self::Record(value)
        }
    }
    impl ::core::convert::From<RecordLogsCall> for VmSafeCalls {
        fn from(value: RecordLogsCall) -> Self {
            Self::RecordLogs(value)
        }
    }
    impl ::core::convert::From<RememberKeyCall> for VmSafeCalls {
        fn from(value: RememberKeyCall) -> Self {
            Self::RememberKey(value)
        }
    }
    impl ::core::convert::From<RemoveFileCall> for VmSafeCalls {
        fn from(value: RemoveFileCall) -> Self {
            Self::RemoveFile(value)
        }
    }
    impl ::core::convert::From<RpcUrlCall> for VmSafeCalls {
        fn from(value: RpcUrlCall) -> Self {
            Self::RpcUrl(value)
        }
    }
    impl ::core::convert::From<RpcUrlStructsCall> for VmSafeCalls {
        fn from(value: RpcUrlStructsCall) -> Self {
            Self::RpcUrlStructs(value)
        }
    }
    impl ::core::convert::From<RpcUrlsCall> for VmSafeCalls {
        fn from(value: RpcUrlsCall) -> Self {
            Self::RpcUrls(value)
        }
    }
    impl ::core::convert::From<SerializeAddress0Call> for VmSafeCalls {
        fn from(value: SerializeAddress0Call) -> Self {
            Self::SerializeAddress0(value)
        }
    }
    impl ::core::convert::From<SerializeAddress1Call> for VmSafeCalls {
        fn from(value: SerializeAddress1Call) -> Self {
            Self::SerializeAddress1(value)
        }
    }
    impl ::core::convert::From<SerializeBool0Call> for VmSafeCalls {
        fn from(value: SerializeBool0Call) -> Self {
            Self::SerializeBool0(value)
        }
    }
    impl ::core::convert::From<SerializeBool1Call> for VmSafeCalls {
        fn from(value: SerializeBool1Call) -> Self {
            Self::SerializeBool1(value)
        }
    }
    impl ::core::convert::From<SerializeBytes0Call> for VmSafeCalls {
        fn from(value: SerializeBytes0Call) -> Self {
            Self::SerializeBytes0(value)
        }
    }
    impl ::core::convert::From<SerializeBytes1Call> for VmSafeCalls {
        fn from(value: SerializeBytes1Call) -> Self {
            Self::SerializeBytes1(value)
        }
    }
    impl ::core::convert::From<SerializeBytes320Call> for VmSafeCalls {
        fn from(value: SerializeBytes320Call) -> Self {
            Self::SerializeBytes320(value)
        }
    }
    impl ::core::convert::From<SerializeBytes321Call> for VmSafeCalls {
        fn from(value: SerializeBytes321Call) -> Self {
            Self::SerializeBytes321(value)
        }
    }
    impl ::core::convert::From<SerializeInt0Call> for VmSafeCalls {
        fn from(value: SerializeInt0Call) -> Self {
            Self::SerializeInt0(value)
        }
    }
    impl ::core::convert::From<SerializeInt1Call> for VmSafeCalls {
        fn from(value: SerializeInt1Call) -> Self {
            Self::SerializeInt1(value)
        }
    }
    impl ::core::convert::From<SerializeString0Call> for VmSafeCalls {
        fn from(value: SerializeString0Call) -> Self {
            Self::SerializeString0(value)
        }
    }
    impl ::core::convert::From<SerializeString1Call> for VmSafeCalls {
        fn from(value: SerializeString1Call) -> Self {
            Self::SerializeString1(value)
        }
    }
    impl ::core::convert::From<SerializeUint0Call> for VmSafeCalls {
        fn from(value: SerializeUint0Call) -> Self {
            Self::SerializeUint0(value)
        }
    }
    impl ::core::convert::From<SerializeUint1Call> for VmSafeCalls {
        fn from(value: SerializeUint1Call) -> Self {
            Self::SerializeUint1(value)
        }
    }
    impl ::core::convert::From<SetEnvCall> for VmSafeCalls {
        fn from(value: SetEnvCall) -> Self {
            Self::SetEnv(value)
        }
    }
    impl ::core::convert::From<SignCall> for VmSafeCalls {
        fn from(value: SignCall) -> Self {
            Self::Sign(value)
        }
    }
    impl ::core::convert::From<StartBroadcast0Call> for VmSafeCalls {
        fn from(value: StartBroadcast0Call) -> Self {
            Self::StartBroadcast0(value)
        }
    }
    impl ::core::convert::From<StartBroadcast1Call> for VmSafeCalls {
        fn from(value: StartBroadcast1Call) -> Self {
            Self::StartBroadcast1(value)
        }
    }
    impl ::core::convert::From<StartBroadcast2Call> for VmSafeCalls {
        fn from(value: StartBroadcast2Call) -> Self {
            Self::StartBroadcast2(value)
        }
    }
    impl ::core::convert::From<StopBroadcastCall> for VmSafeCalls {
        fn from(value: StopBroadcastCall) -> Self {
            Self::StopBroadcast(value)
        }
    }
    impl ::core::convert::From<ToString0Call> for VmSafeCalls {
        fn from(value: ToString0Call) -> Self {
            Self::ToString0(value)
        }
    }
    impl ::core::convert::From<ToString1Call> for VmSafeCalls {
        fn from(value: ToString1Call) -> Self {
            Self::ToString1(value)
        }
    }
    impl ::core::convert::From<ToString2Call> for VmSafeCalls {
        fn from(value: ToString2Call) -> Self {
            Self::ToString2(value)
        }
    }
    impl ::core::convert::From<ToString3Call> for VmSafeCalls {
        fn from(value: ToString3Call) -> Self {
            Self::ToString3(value)
        }
    }
    impl ::core::convert::From<ToString4Call> for VmSafeCalls {
        fn from(value: ToString4Call) -> Self {
            Self::ToString4(value)
        }
    }
    impl ::core::convert::From<ToString5Call> for VmSafeCalls {
        fn from(value: ToString5Call) -> Self {
            Self::ToString5(value)
        }
    }
    impl ::core::convert::From<WriteFileCall> for VmSafeCalls {
        fn from(value: WriteFileCall) -> Self {
            Self::WriteFile(value)
        }
    }
    impl ::core::convert::From<WriteFileBinaryCall> for VmSafeCalls {
        fn from(value: WriteFileBinaryCall) -> Self {
            Self::WriteFileBinary(value)
        }
    }
    impl ::core::convert::From<WriteJson1Call> for VmSafeCalls {
        fn from(value: WriteJson1Call) -> Self {
            Self::WriteJson1(value)
        }
    }
    impl ::core::convert::From<WriteJson0Call> for VmSafeCalls {
        fn from(value: WriteJson0Call) -> Self {
            Self::WriteJson0(value)
        }
    }
    impl ::core::convert::From<WriteLineCall> for VmSafeCalls {
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
