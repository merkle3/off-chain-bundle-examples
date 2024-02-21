pub use counter_test::*;
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
pub mod counter_test {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("IS_TEST"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("IS_TEST"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("counter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("counter"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Counter"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("failed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("failed"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("setUp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setUp"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("testIncrement"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("testIncrement"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("testSetNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("testSetNumber"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("log"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_address"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_address"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_array"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_bytes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_bytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_bytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_bytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_int"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_int"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_address"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_address"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_array"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_bytes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_bytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_bytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_bytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_decimal_int"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "log_named_decimal_int",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("decimals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_decimal_uint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "log_named_decimal_uint",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("decimals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_int"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_int"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_string"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_string"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_uint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_uint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_string"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_string"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_uint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_uint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("logs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("logs"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
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
    pub static COUNTERTEST_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x07\x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\0\x1Bb\0\x001V[`\x08U4\x80\x15b\0\0*W_\x80\xFD[Pb\0\x12\"V[`@\x80Q`\xA0\x81\x01\x82R`\x05``\x82\x01\x81\x81Rd\x10[\x9D\x9A[`\xDA\x1B`\x80\x84\x01R\x82Razi` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x15\x81R\x7Fhttp://127.0.0.1:8545\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x82\x84\x01R\x91Qd\x18[\x9D\x9A[`\xDA\x1B\x81R_\x92`\t\x91\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90b\0\0\xC9\x90\x82b\0\x0E\xE8V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90b\0\0\xEA\x90\x82b\0\x0E\xE8V[PP`@\x80Q`\xA0\x81\x01\x82R`\x07``\x82\x01\x81\x81Rf\x12\x18\\\x99\x1A\x18]`\xCA\x1B`\x80\x84\x01R\x82Razi` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x15\x81R\x7Fhttp://127.0.0.1:8545\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x82\x84\x01R\x91Qf\x1A\x18\\\x99\x1A\x18]`\xCA\x1B\x81R\x90\x92P`\t\x91\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90b\0\x01\x89\x90\x82b\0\x0E\xE8V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90b\0\x01\xAA\x90\x82b\0\x0E\xE8V[P\x90PP`@Q\x80``\x01`@R\x80`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f\x13XZ[\x9B\x99]`\xCA\x1B\x81RP\x81R` \x01`\x01\x81R` \x01`@Q\x80``\x01`@R\x80`=\x81R` \x01b\0\x1A\xFE`=\x919\x90R`@Qf\x1BXZ[\x9B\x99]`\xCA\x1B\x81R`\t\x90`\x07\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90b\0\x026\x90\x82b\0\x0E\xE8V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90b\0\x02W\x90\x82b\0\x0E\xE8V[P\x90PP`@Q\x80``\x01`@R\x80`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01eGoerli`\xD0\x1B\x81RP\x81R` \x01`\x05\x81R` \x01`@Q\x80``\x01`@R\x80`<\x81R` \x01b\0\x1B;`<\x919\x90R`@Qegoerli`\xD0\x1B\x81R`\t\x90`\x06\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90b\0\x02\xE1\x90\x82b\0\x0E\xE8V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90b\0\x03\x02\x90\x82b\0\x0E\xE8V[PP`@\x80Q`\xA0\x81\x01\x82R`\x07``\x82\x01\x81\x81RfSepolia`\xC8\x1B`\x80\x84\x01R\x82Rb\xAA6\xA7` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x17\x81R\x7Fhttps://rpc.sepolia.dev\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x82\x84\x01R\x91Qfsepolia`\xC8\x1B\x81R\x90\x92P`\t\x91\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90b\0\x03\xA2\x90\x82b\0\x0E\xE8V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90b\0\x03\xC3\x90\x82b\0\x0E\xE8V[PP`@\x80Q`\xA0\x81\x01\x82R`\x08``\x82\x01\x81\x81RgOptimism`\xC0\x1B`\x80\x84\x01R\x82R`\n` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x1B\x81R\x7Fhttps://mainnet.optimism.io\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x82\x84\x01R\x91Qgoptimism`\xC0\x1B\x81R\x90\x92P`\t\x91\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90b\0\x04c\x90\x82b\0\x0E\xE8V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90b\0\x04\x84\x90\x82b\0\x0E\xE8V[PP`@\x80Q`\xA0\x81\x01\x82R`\x0F``\x82\x01\x81\x81RnOptimism Goerli`\x88\x1B`\x80\x84\x01R\x82Ra\x01\xA4` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x1A\x81R\x7Fhttps://goerli.optimism.io\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x82\x84\x01R\x91Qnoptimism_goerli`\x88\x1B\x81R\x90\x92P`\t\x91\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90b\0\x053\x90\x82b\0\x0E\xE8V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90b\0\x05T\x90\x82b\0\x0E\xE8V[PP`@\x80Q`\xA0\x81\x01\x82R`\x0C``\x82\x01\x81\x81RkArbitrum One`\xA0\x1B`\x80\x84\x01R\x82Ra\xA4\xB1` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x1C\x81R\x7Fhttps://arb1.arbitrum.io/rpc\0\0\0\0\x91\x81\x01\x91\x90\x91R\x82\x84\x01R\x91Qkarbitrum_one`\xA0\x1B\x81R\x90\x92P`\t\x91\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90b\0\x05\xFD\x90\x82b\0\x0E\xE8V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90b\0\x06\x1E\x90\x82b\0\x0E\xE8V[P\x90PP`@Q\x80``\x01`@R\x80`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01\x7FArbitrum One Goerli\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81R` \x01b\x06n\xED\x81R` \x01`@Q\x80``\x01`@R\x80`%\x81R` \x01b\0\x1Bw`%\x919\x90R`@Q\x7Farbitrum_one_goerli\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\t\x90`\x13\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90b\0\x06\xD8\x90\x82b\0\x0E\xE8V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90b\0\x06\xF9\x90\x82b\0\x0E\xE8V[PP`@\x80Q`\xA0\x81\x01\x82R`\r``\x82\x01\x81\x81RlArbitrum Nova`\x98\x1B`\x80\x84\x01R\x82Ra\xA4\xBA` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x1C\x81R\x7Fhttps://nova.arbitrum.io/rpc\0\0\0\0\x91\x81\x01\x91\x90\x91R\x82\x84\x01R\x91Qlarbitrum_nova`\x98\x1B\x81R\x90\x92P`\t\x91\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90b\0\x07\xA4\x90\x82b\0\x0E\xE8V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90b\0\x07\xC5\x90\x82b\0\x0E\xE8V[PP`@\x80Q`\xA0\x81\x01\x82R`\x07``\x82\x01\x81\x81Rf(7\xB6<\xB3\xB7\xB7`\xC9\x1B`\x80\x84\x01R\x82R`\x89` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x17\x81R\x7Fhttps://polygon-rpc.com\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x82\x84\x01R\x91Qf87\xB6<\xB3\xB7\xB7`\xC9\x1B\x81R\x90\x92P`\t\x91\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90b\0\x08c\x90\x82b\0\x0E\xE8V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90b\0\x08\x84\x90\x82b\0\x0E\xE8V[PP`@\x80Q`\xA0\x81\x01\x82R`\x0E``\x82\x01\x81\x81RmPolygon Mumbai`\x90\x1B`\x80\x84\x01R\x82Rb\x018\x81` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x1E\x81R\x7Fhttps://rpc-mumbai.matic.today\0\0\x91\x81\x01\x91\x90\x91R\x82\x84\x01R\x91Qmpolygon_mumbai`\x90\x1B\x81R\x90\x92P`\t\x91\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90b\0\t2\x90\x82b\0\x0E\xE8V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90b\0\tS\x90\x82b\0\x0E\xE8V[P\x90PP`@Q\x80``\x01`@R\x80`@Q\x80`@\x01`@R\x80`\t\x81R` \x01hAvalanche`\xB8\x1B\x81RP\x81R` \x01a\xA8j\x81R` \x01`@Q\x80``\x01`@R\x80`%\x81R` \x01b\0\x1A\xD9`%\x919\x90R`@Qhavalanche`\xB8\x1B\x81R`\t\x90\x81\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90b\0\t\xE3\x90\x82b\0\x0E\xE8V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90b\0\n\x04\x90\x82b\0\x0E\xE8V[P\x90PP`@Q\x80``\x01`@R\x80`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mAvalanche Fuji`\x90\x1B\x81RP\x81R` \x01a\xA8i\x81R` \x01`@Q\x80``\x01`@R\x80`*\x81R` \x01b\0\x1B\x9C`*\x919\x90R`@Qmavalanche_fuji`\x90\x1B\x81R`\t\x90`\x0E\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90b\0\n\x9F\x90\x82b\0\x0E\xE8V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90b\0\n\xC0\x90\x82b\0\x0E\xE8V[P\x90PP`@Q\x80``\x01`@R\x80`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n!'!\x10)\xB6\xB0\xB9:\x10!\xB40\xB4\xB7`\x89\x1B\x81RP\x81R` \x01`8\x81R` \x01`@Q\x80``\x01`@R\x80`!\x81R` \x01b\0\x1B\xC6`!\x919\x90R`@Qn171/\xB9\xB6\xB0\xB9:/\xB1\xB40\xB4\xB7`\x89\x1B\x81R`\t\x90`\x0F\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90b\0\x0B\\\x90\x82b\0\x0E\xE8V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90b\0\x0B}\x90\x82b\0\x0E\xE8V[P\x90PP`@Q\x80``\x01`@R\x80`@Q\x80`@\x01`@R\x80`\x17\x81R` \x01\x7FBNB Smart Chain Testnet\0\0\0\0\0\0\0\0\0\x81RP\x81R` \x01`a\x81R` \x01`@Q\x80``\x01`@R\x80`.\x81R` \x01b\0\x1A\xAB`.\x919\x90R`@Q\x7Fbnb_smart_chain_testnet\0\0\0\0\0\0\0\0\0\x81R`\t\x90`\x17\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90b\0\x0C5\x90\x82b\0\x0E\xE8V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90b\0\x0CV\x90\x82b\0\x0E\xE8V[PP`@\x80Q`\xA0\x81\x01\x82R`\x0C``\x82\x01\x81\x81Rk#\xB77\xB9\xB4\xB9\x90!\xB40\xB4\xB7`\xA1\x1B`\x80\x84\x01R\x82R`d` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x1B\x81R\x7Fhttps://rpc.gnosischain.com\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x82\x84\x01R\x91Qk3\xB77\xB9\xB4\xB9\xAF\xB1\xB40\xB4\xB7`\xA1\x1B\x81R\x90\x92P`\t\x91\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90b\0\x0C\xFE\x90\x82b\0\x0E\xE8V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90b\0\r\x1F\x90\x82b\0\x0E\xE8V[P\x90PP_\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1C`\x01`\x01`\xA0\x1B\x03\x16c\x9D*\xD7*`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\r\x82W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Rb\0\r\xAB\x91\x90\x81\x01\x90b\0\x10\xA3V[\x90P_[\x81Q\x81\x10\x15b\0\x0E@W\x81\x81\x81Q\x81\x10b\0\r\xCEWb\0\r\xCEb\0\x11\xCCV[` \x02` \x01\x01Q` \x01Q`\t\x83\x83\x81Q\x81\x10b\0\r\xF1Wb\0\r\xF1b\0\x11\xCCV[` \x02` \x01\x01Q_\x01Q`@Qb\0\x0E\x0B\x91\x90b\0\x11\xE0V[\x90\x81R` \x01`@Q\x80\x91\x03\x90 `\x02\x01\x90\x81b\0\x0E*\x91\x90b\0\x0E\xE8V[P\x80b\0\x0E7\x81b\0\x11\xFDV[\x91PPb\0\r\xAFV[P_\x91PP\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x0EqW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x0E\x90WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x0E\xE3W_\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x0E\xBEWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x0E\xDFW\x82\x81U`\x01\x01b\0\x0E\xCAV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x0F\x04Wb\0\x0F\x04b\0\x0EHV[b\0\x0F\x1C\x81b\0\x0F\x15\x84Tb\0\x0E\\V[\x84b\0\x0E\x96V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x0FRW_\x84\x15b\0\x0F:WP\x85\x83\x01Q[_\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x0E\xDFV[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x0F\x82W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x0FaV[P\x85\x82\x10\x15b\0\x0F\xA0W\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x0F\xD5Wb\0\x0F\xD5b\0\x0EHV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x10\x06Wb\0\x10\x06b\0\x0EHV[`@R\x91\x90PV[_[\x83\x81\x10\x15b\0\x10*W\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x10\x10V[PP_\x91\x01RV[_\x82`\x1F\x83\x01\x12b\0\x10BW_\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x10^Wb\0\x10^b\0\x0EHV[b\0\x10s`\x1F\x82\x01`\x1F\x19\x16` \x01b\0\x0F\xDBV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15b\0\x10\x88W_\x80\xFD[b\0\x10\x9B\x82` \x83\x01` \x87\x01b\0\x10\x0EV[\x94\x93PPPPV[_` \x80\x83\x85\x03\x12\x15b\0\x10\xB5W_\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x10\xCCW_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12b\0\x10\xE0W_\x80\xFD[\x81Q\x81\x81\x11\x15b\0\x10\xF5Wb\0\x10\xF5b\0\x0EHV[\x80`\x05\x1Bb\0\x11\x06\x85\x82\x01b\0\x0F\xDBV[\x91\x82R\x83\x81\x01\x85\x01\x91\x85\x81\x01\x90\x89\x84\x11\x15b\0\x11 W_\x80\xFD[\x86\x86\x01\x92P[\x83\x83\x10\x15b\0\x11\xBFW\x82Q\x85\x81\x11\x15b\0\x11?W_\x80\x81\xFD[\x86\x01`@\x81\x8C\x03`\x1F\x19\x01\x81\x13\x15b\0\x11WW_\x80\x81\xFD[b\0\x11ab\0\x0F\xB0V[\x89\x83\x01Q\x88\x81\x11\x15b\0\x11sW_\x80\x81\xFD[b\0\x11\x83\x8E\x8C\x83\x87\x01\x01b\0\x102V[\x82RP\x90\x82\x01Q\x90\x87\x82\x11\x15b\0\x11\x99W_\x80\x81\xFD[b\0\x11\xA9\x8D\x8B\x84\x86\x01\x01b\0\x102V[\x81\x8B\x01R\x84RPP\x91\x86\x01\x91\x90\x86\x01\x90b\0\x11&V[\x99\x98PPPPPPPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x82Qb\0\x11\xF3\x81\x84` \x87\x01b\0\x10\x0EV[\x91\x90\x91\x01\x92\x91PPV[_`\x01\x82\x01b\0\x12\x1BWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[P`\x01\x01\x90V[a\x08{\x80b\0\x120_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0`W_5`\xE0\x1C\x80c\n\x92T\xE4\x14a\0dW\x80ca\xBC\"\x1A\x14a\0nW\x80cp\xF9\x85\xBE\x14a\0\x9EW\x80c\xB9\x13\xA5\xCA\x14a\0\xB1W\x80c\xBAAO\xA6\x14a\0\xB9W\x80c\xFAv&\xD4\x14a\0\xD1W[_\x80\xFD[a\0la\0\xDEV[\0[`\x11Ta\0\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0la\0\xAC6`\x04a\x06\x8EV[a\x01nV[a\0la\x02GV[a\0\xC1a\x03)V[`@Q\x90\x15\x15\x81R` \x01a\0\x95V[`\x07Ta\0\xC1\x90`\xFF\x16\x81V[`@Qa\0\xEA\x90a\x06\x81V[`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x01\x03W=_\x80>=_\xFD[P`\x11\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U`@Qc?\xB5\xC1\xCB`\xE0\x1B\x81R_`\x04\x82\x01Rc?\xB5\xC1\xCB\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x01VW_\x80\xFD[PZ\xF1\x15\x80\x15a\x01hW=_\x80>=_\xFD[PPPPV[`\x11T`@Qc?\xB5\xC1\xCB`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c?\xB5\xC1\xCB\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x01\xB1W_\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xC3W=_\x80>=_\xFD[PPPPa\x02D`\x11_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x83\x81\xF5\x8A`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x1AW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02>\x91\x90a\x06\xA5V[\x82a\x04OV[PV[`\x11_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xD0\x9D\xE0\x8A`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x02\x93W_\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xA5W=_\x80>=_\xFD[PPPPa\x03'`\x11_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x83\x81\xF5\x8A`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xFCW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03 \x91\x90a\x06\xA5V[`\x01a\x04OV[V[`\x07T_\x90a\x01\0\x90\x04`\xFF\x16\x15a\x03JWP`\x07Ta\x01\0\x90\x04`\xFF\x16\x90V[_sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04JW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R_\x92\x90\x91a\x03\xD6\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a\x06\xE9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x03\xF0\x91a\x07\x0CV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x04)W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x04.V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x04F\x91\x90a\x07\x1EV[\x91PP[\x91\x90PV[\x80\x82\x14a\x05vW\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qa\x04\xC0\x90` \x80\x82R`\"\x90\x82\x01R\x7FError: a == b not satisfied [uin`@\x82\x01Rat]`\xF0\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x11^\x1C\x19X\xDD\x19Y`\xB2\x1B``\x82\x01R` \x81\x01\x83\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x10X\xDD\x1DX[`\xB2\x1B``\x82\x01R` \x81\x01\x84\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1a\x05va\x05zV[PPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06pW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R_\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x06\x13\x92\x91` \x01a\x06\xE9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x06-\x91a\x07\x0CV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x06fW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x06kV[``\x91P[PPPP[`\x07\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UV[a\x01\x08\x80a\x07>\x839\x01\x90V[_` \x82\x84\x03\x12\x15a\x06\x9EW_\x80\xFD[P5\x91\x90PV[_` \x82\x84\x03\x12\x15a\x06\xB5W_\x80\xFD[PQ\x91\x90PV[_\x81Q_[\x81\x81\x10\x15a\x06\xDBW` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a\x06\xC1V[P_\x93\x01\x92\x83RP\x90\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R_a\x07\x04`\x04\x83\x01\x84a\x06\xBCV[\x94\x93PPPPV[_a\x07\x17\x82\x84a\x06\xBCV[\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x07.W_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\x17W_\x80\xFD\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\xEC\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15`\x0EW_\x80\xFD[P`\x046\x10`:W_5`\xE0\x1C\x80c?\xB5\xC1\xCB\x14`>W\x80c\x83\x81\xF5\x8A\x14`OW\x80c\xD0\x9D\xE0\x8A\x14`hW[_\x80\xFD[`M`I6`\x04`}V[_UV[\0[`V_T\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`M_\x80T\x90\x80`v\x83`\x93V[\x91\x90PUPV[_` \x82\x84\x03\x12\x15`\x8CW_\x80\xFD[P5\x91\x90PV[_`\x01\x82\x01`\xAFWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xE2)9(\xA79lU\xA5\xC3\x16#\xD2\xD6\xCD\xB7\xDCjR\x82l\xA7\t\x93\xFC\xA5h7\xA5\xC7\x9A\x8DdsolcC\0\x08\x15\x003\xA2dipfsX\"\x12 8\xED\xEE\x83m\xA0W\x0E\xBDV\xFE!7,q\x04=\xB75\x01\xA0Z\xAEm\xD7\xD3\rGBj9/dsolcC\0\x08\x15\x003https://data-seed-prebsc-1-s1.binance.org:8545https://api.avax.network/ext/bc/C/rpchttps://mainnet.infura.io/v3/6770454bc6ea42c58aac12978531b93fhttps://goerli.infura.io/v3/6770454bc6ea42c58aac12978531b93fhttps://goerli-rollup.arbitrum.io/rpchttps://api.avax-test.network/ext/bc/C/rpchttps://bsc-dataseed1.binance.org";
    /// The bytecode of the contract.
    pub static COUNTERTEST_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0`W_5`\xE0\x1C\x80c\n\x92T\xE4\x14a\0dW\x80ca\xBC\"\x1A\x14a\0nW\x80cp\xF9\x85\xBE\x14a\0\x9EW\x80c\xB9\x13\xA5\xCA\x14a\0\xB1W\x80c\xBAAO\xA6\x14a\0\xB9W\x80c\xFAv&\xD4\x14a\0\xD1W[_\x80\xFD[a\0la\0\xDEV[\0[`\x11Ta\0\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0la\0\xAC6`\x04a\x06\x8EV[a\x01nV[a\0la\x02GV[a\0\xC1a\x03)V[`@Q\x90\x15\x15\x81R` \x01a\0\x95V[`\x07Ta\0\xC1\x90`\xFF\x16\x81V[`@Qa\0\xEA\x90a\x06\x81V[`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x01\x03W=_\x80>=_\xFD[P`\x11\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U`@Qc?\xB5\xC1\xCB`\xE0\x1B\x81R_`\x04\x82\x01Rc?\xB5\xC1\xCB\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x01VW_\x80\xFD[PZ\xF1\x15\x80\x15a\x01hW=_\x80>=_\xFD[PPPPV[`\x11T`@Qc?\xB5\xC1\xCB`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c?\xB5\xC1\xCB\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x01\xB1W_\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xC3W=_\x80>=_\xFD[PPPPa\x02D`\x11_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x83\x81\xF5\x8A`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x1AW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02>\x91\x90a\x06\xA5V[\x82a\x04OV[PV[`\x11_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xD0\x9D\xE0\x8A`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x02\x93W_\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xA5W=_\x80>=_\xFD[PPPPa\x03'`\x11_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x83\x81\xF5\x8A`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xFCW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03 \x91\x90a\x06\xA5V[`\x01a\x04OV[V[`\x07T_\x90a\x01\0\x90\x04`\xFF\x16\x15a\x03JWP`\x07Ta\x01\0\x90\x04`\xFF\x16\x90V[_sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x04JW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R_\x92\x90\x91a\x03\xD6\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a\x06\xE9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x03\xF0\x91a\x07\x0CV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x04)W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x04.V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x04F\x91\x90a\x07\x1EV[\x91PP[\x91\x90PV[\x80\x82\x14a\x05vW\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qa\x04\xC0\x90` \x80\x82R`\"\x90\x82\x01R\x7FError: a == b not satisfied [uin`@\x82\x01Rat]`\xF0\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x11^\x1C\x19X\xDD\x19Y`\xB2\x1B``\x82\x01R` \x81\x01\x83\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x10X\xDD\x1DX[`\xB2\x1B``\x82\x01R` \x81\x01\x84\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1a\x05va\x05zV[PPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x06pW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R_\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x06\x13\x92\x91` \x01a\x06\xE9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x06-\x91a\x07\x0CV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x06fW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x06kV[``\x91P[PPPP[`\x07\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UV[a\x01\x08\x80a\x07>\x839\x01\x90V[_` \x82\x84\x03\x12\x15a\x06\x9EW_\x80\xFD[P5\x91\x90PV[_` \x82\x84\x03\x12\x15a\x06\xB5W_\x80\xFD[PQ\x91\x90PV[_\x81Q_[\x81\x81\x10\x15a\x06\xDBW` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a\x06\xC1V[P_\x93\x01\x92\x83RP\x90\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R_a\x07\x04`\x04\x83\x01\x84a\x06\xBCV[\x94\x93PPPPV[_a\x07\x17\x82\x84a\x06\xBCV[\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x07.W_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\x17W_\x80\xFD\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\xEC\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15`\x0EW_\x80\xFD[P`\x046\x10`:W_5`\xE0\x1C\x80c?\xB5\xC1\xCB\x14`>W\x80c\x83\x81\xF5\x8A\x14`OW\x80c\xD0\x9D\xE0\x8A\x14`hW[_\x80\xFD[`M`I6`\x04`}V[_UV[\0[`V_T\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`M_\x80T\x90\x80`v\x83`\x93V[\x91\x90PUPV[_` \x82\x84\x03\x12\x15`\x8CW_\x80\xFD[P5\x91\x90PV[_`\x01\x82\x01`\xAFWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xE2)9(\xA79lU\xA5\xC3\x16#\xD2\xD6\xCD\xB7\xDCjR\x82l\xA7\t\x93\xFC\xA5h7\xA5\xC7\x9A\x8DdsolcC\0\x08\x15\x003\xA2dipfsX\"\x12 8\xED\xEE\x83m\xA0W\x0E\xBDV\xFE!7,q\x04=\xB75\x01\xA0Z\xAEm\xD7\xD3\rGBj9/dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static COUNTERTEST_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CounterTest<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CounterTest<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CounterTest<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CounterTest<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CounterTest<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CounterTest))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CounterTest<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    COUNTERTEST_ABI.clone(),
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
                COUNTERTEST_ABI.clone(),
                COUNTERTEST_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `IS_TEST` (0xfa7626d4) function
        pub fn is_test(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 118, 38, 212], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `counter` (0x61bc221a) function
        pub fn counter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([97, 188, 34, 26], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failed` (0xba414fa6) function
        pub fn failed(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([186, 65, 79, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUp` (0x0a9254e4) function
        pub fn set_up(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 146, 84, 228], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `testIncrement` (0xb913a5ca) function
        pub fn test_increment(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 19, 165, 202], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `testSetNumber` (0x70f985be) function
        pub fn test_set_number(
            &self,
            x: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 249, 133, 190], x)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `log` event
        pub fn log_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogFilter> {
            self.0.event()
        }
        ///Gets the contract's `log_address` event
        pub fn log_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogAddressFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogArray1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogArray2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_3_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogArray3Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_bytes` event
        pub fn log_bytes_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogBytesFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_bytes32` event
        pub fn log_bytes_32_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogBytes32Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_int` event
        pub fn log_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogIntFilter> {
            self.0.event()
        }
        ///Gets the contract's `log_named_address` event
        pub fn log_named_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedAddressFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedArray1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedArray2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_3_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedArray3Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_bytes` event
        pub fn log_named_bytes_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedBytesFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_bytes32` event
        pub fn log_named_bytes_32_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedBytes32Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_decimal_int` event
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedDecimalIntFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_decimal_uint` event
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedDecimalUintFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_int` event
        pub fn log_named_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedIntFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_string` event
        pub fn log_named_string_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedStringFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_uint` event
        pub fn log_named_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedUintFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_string` event
        pub fn log_string_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogStringFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_uint` event
        pub fn log_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogUintFilter> {
            self.0.event()
        }
        ///Gets the contract's `logs` event
        pub fn logs_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogsFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CounterTestEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CounterTest<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "log", abi = "log(string)")]
    pub struct LogFilter(pub ::std::string::String);
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "log_address", abi = "log_address(address)")]
    pub struct LogAddressFilter(pub ::ethers::core::types::Address);
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "log_array", abi = "log_array(uint256[])")]
    pub struct LogArray1Filter {
        pub val: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "log_array", abi = "log_array(int256[])")]
    pub struct LogArray2Filter {
        pub val: ::std::vec::Vec<::ethers::core::types::I256>,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "log_array", abi = "log_array(address[])")]
    pub struct LogArray3Filter {
        pub val: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "log_bytes", abi = "log_bytes(bytes)")]
    pub struct LogBytesFilter(pub ::ethers::core::types::Bytes);
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "log_bytes32", abi = "log_bytes32(bytes32)")]
    pub struct LogBytes32Filter(pub [u8; 32]);
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "log_int", abi = "log_int(int256)")]
    pub struct LogIntFilter(pub ::ethers::core::types::I256);
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "log_named_address", abi = "log_named_address(string,address)")]
    pub struct LogNamedAddressFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,uint256[])")]
    pub struct LogNamedArray1Filter {
        pub key: ::std::string::String,
        pub val: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,int256[])")]
    pub struct LogNamedArray2Filter {
        pub key: ::std::string::String,
        pub val: ::std::vec::Vec<::ethers::core::types::I256>,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,address[])")]
    pub struct LogNamedArray3Filter {
        pub key: ::std::string::String,
        pub val: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "log_named_bytes", abi = "log_named_bytes(string,bytes)")]
    pub struct LogNamedBytesFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "log_named_bytes32", abi = "log_named_bytes32(string,bytes32)")]
    pub struct LogNamedBytes32Filter {
        pub key: ::std::string::String,
        pub val: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "log_named_decimal_int",
        abi = "log_named_decimal_int(string,int256,uint256)"
    )]
    pub struct LogNamedDecimalIntFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::I256,
        pub decimals: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "log_named_decimal_uint",
        abi = "log_named_decimal_uint(string,uint256,uint256)"
    )]
    pub struct LogNamedDecimalUintFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::U256,
        pub decimals: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "log_named_int", abi = "log_named_int(string,int256)")]
    pub struct LogNamedIntFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::I256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "log_named_string", abi = "log_named_string(string,string)")]
    pub struct LogNamedStringFilter {
        pub key: ::std::string::String,
        pub val: ::std::string::String,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "log_named_uint", abi = "log_named_uint(string,uint256)")]
    pub struct LogNamedUintFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "log_string", abi = "log_string(string)")]
    pub struct LogStringFilter(pub ::std::string::String);
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "log_uint", abi = "log_uint(uint256)")]
    pub struct LogUintFilter(pub ::ethers::core::types::U256);
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "logs", abi = "logs(bytes)")]
    pub struct LogsFilter(pub ::ethers::core::types::Bytes);
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CounterTestEvents {
        LogFilter(LogFilter),
        LogAddressFilter(LogAddressFilter),
        LogArray1Filter(LogArray1Filter),
        LogArray2Filter(LogArray2Filter),
        LogArray3Filter(LogArray3Filter),
        LogBytesFilter(LogBytesFilter),
        LogBytes32Filter(LogBytes32Filter),
        LogIntFilter(LogIntFilter),
        LogNamedAddressFilter(LogNamedAddressFilter),
        LogNamedArray1Filter(LogNamedArray1Filter),
        LogNamedArray2Filter(LogNamedArray2Filter),
        LogNamedArray3Filter(LogNamedArray3Filter),
        LogNamedBytesFilter(LogNamedBytesFilter),
        LogNamedBytes32Filter(LogNamedBytes32Filter),
        LogNamedDecimalIntFilter(LogNamedDecimalIntFilter),
        LogNamedDecimalUintFilter(LogNamedDecimalUintFilter),
        LogNamedIntFilter(LogNamedIntFilter),
        LogNamedStringFilter(LogNamedStringFilter),
        LogNamedUintFilter(LogNamedUintFilter),
        LogStringFilter(LogStringFilter),
        LogUintFilter(LogUintFilter),
        LogsFilter(LogsFilter),
    }
    impl ::ethers::contract::EthLogDecode for CounterTestEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(CounterTestEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(CounterTestEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogArray1Filter::decode_log(log) {
                return Ok(CounterTestEvents::LogArray1Filter(decoded));
            }
            if let Ok(decoded) = LogArray2Filter::decode_log(log) {
                return Ok(CounterTestEvents::LogArray2Filter(decoded));
            }
            if let Ok(decoded) = LogArray3Filter::decode_log(log) {
                return Ok(CounterTestEvents::LogArray3Filter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(CounterTestEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(CounterTestEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(CounterTestEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(CounterTestEvents::LogNamedAddressFilter(decoded));
            }
            if let Ok(decoded) = LogNamedArray1Filter::decode_log(log) {
                return Ok(CounterTestEvents::LogNamedArray1Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray2Filter::decode_log(log) {
                return Ok(CounterTestEvents::LogNamedArray2Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray3Filter::decode_log(log) {
                return Ok(CounterTestEvents::LogNamedArray3Filter(decoded));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(CounterTestEvents::LogNamedBytesFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(CounterTestEvents::LogNamedBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(CounterTestEvents::LogNamedDecimalIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(CounterTestEvents::LogNamedDecimalUintFilter(decoded));
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(CounterTestEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(CounterTestEvents::LogNamedStringFilter(decoded));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(CounterTestEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(CounterTestEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(CounterTestEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(CounterTestEvents::LogsFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CounterTestEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LogFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogAddressFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray1Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray2Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray3Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes32Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogIntFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedAddressFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedArray1Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedArray2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedArray3Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedBytesFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedBytes32Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedDecimalIntFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedDecimalUintFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedIntFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedStringFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedUintFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogStringFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogUintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogsFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LogFilter> for CounterTestEvents {
        fn from(value: LogFilter) -> Self {
            Self::LogFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter> for CounterTestEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogArray1Filter> for CounterTestEvents {
        fn from(value: LogArray1Filter) -> Self {
            Self::LogArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogArray2Filter> for CounterTestEvents {
        fn from(value: LogArray2Filter) -> Self {
            Self::LogArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogArray3Filter> for CounterTestEvents {
        fn from(value: LogArray3Filter) -> Self {
            Self::LogArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogBytesFilter> for CounterTestEvents {
        fn from(value: LogBytesFilter) -> Self {
            Self::LogBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogBytes32Filter> for CounterTestEvents {
        fn from(value: LogBytes32Filter) -> Self {
            Self::LogBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogIntFilter> for CounterTestEvents {
        fn from(value: LogIntFilter) -> Self {
            Self::LogIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedAddressFilter> for CounterTestEvents {
        fn from(value: LogNamedAddressFilter) -> Self {
            Self::LogNamedAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray1Filter> for CounterTestEvents {
        fn from(value: LogNamedArray1Filter) -> Self {
            Self::LogNamedArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray2Filter> for CounterTestEvents {
        fn from(value: LogNamedArray2Filter) -> Self {
            Self::LogNamedArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray3Filter> for CounterTestEvents {
        fn from(value: LogNamedArray3Filter) -> Self {
            Self::LogNamedArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytesFilter> for CounterTestEvents {
        fn from(value: LogNamedBytesFilter) -> Self {
            Self::LogNamedBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytes32Filter> for CounterTestEvents {
        fn from(value: LogNamedBytes32Filter) -> Self {
            Self::LogNamedBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalIntFilter> for CounterTestEvents {
        fn from(value: LogNamedDecimalIntFilter) -> Self {
            Self::LogNamedDecimalIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalUintFilter> for CounterTestEvents {
        fn from(value: LogNamedDecimalUintFilter) -> Self {
            Self::LogNamedDecimalUintFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedIntFilter> for CounterTestEvents {
        fn from(value: LogNamedIntFilter) -> Self {
            Self::LogNamedIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedStringFilter> for CounterTestEvents {
        fn from(value: LogNamedStringFilter) -> Self {
            Self::LogNamedStringFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedUintFilter> for CounterTestEvents {
        fn from(value: LogNamedUintFilter) -> Self {
            Self::LogNamedUintFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter> for CounterTestEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUintFilter> for CounterTestEvents {
        fn from(value: LogUintFilter) -> Self {
            Self::LogUintFilter(value)
        }
    }
    impl ::core::convert::From<LogsFilter> for CounterTestEvents {
        fn from(value: LogsFilter) -> Self {
            Self::LogsFilter(value)
        }
    }
    ///Container type for all input parameters for the `IS_TEST` function with signature `IS_TEST()` and selector `0xfa7626d4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "IS_TEST", abi = "IS_TEST()")]
    pub struct IsTestCall;
    ///Container type for all input parameters for the `counter` function with signature `counter()` and selector `0x61bc221a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "counter", abi = "counter()")]
    pub struct CounterCall;
    ///Container type for all input parameters for the `failed` function with signature `failed()` and selector `0xba414fa6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "failed", abi = "failed()")]
    pub struct FailedCall;
    ///Container type for all input parameters for the `setUp` function with signature `setUp()` and selector `0x0a9254e4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setUp", abi = "setUp()")]
    pub struct SetUpCall;
    ///Container type for all input parameters for the `testIncrement` function with signature `testIncrement()` and selector `0xb913a5ca`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "testIncrement", abi = "testIncrement()")]
    pub struct TestIncrementCall;
    ///Container type for all input parameters for the `testSetNumber` function with signature `testSetNumber(uint256)` and selector `0x70f985be`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "testSetNumber", abi = "testSetNumber(uint256)")]
    pub struct TestSetNumberCall {
        pub x: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CounterTestCalls {
        IsTest(IsTestCall),
        Counter(CounterCall),
        Failed(FailedCall),
        SetUp(SetUpCall),
        TestIncrement(TestIncrementCall),
        TestSetNumber(TestSetNumberCall),
    }
    impl ::ethers::core::abi::AbiDecode for CounterTestCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <IsTestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsTest(decoded));
            }
            if let Ok(decoded) = <CounterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Counter(decoded));
            }
            if let Ok(decoded) = <FailedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Failed(decoded));
            }
            if let Ok(decoded) = <SetUpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetUp(decoded));
            }
            if let Ok(decoded) = <TestIncrementCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TestIncrement(decoded));
            }
            if let Ok(decoded) = <TestSetNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TestSetNumber(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CounterTestCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsTest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Counter(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Failed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetUp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TestIncrement(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TestSetNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CounterTestCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsTest(element) => ::core::fmt::Display::fmt(element, f),
                Self::Counter(element) => ::core::fmt::Display::fmt(element, f),
                Self::Failed(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::TestIncrement(element) => ::core::fmt::Display::fmt(element, f),
                Self::TestSetNumber(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IsTestCall> for CounterTestCalls {
        fn from(value: IsTestCall) -> Self {
            Self::IsTest(value)
        }
    }
    impl ::core::convert::From<CounterCall> for CounterTestCalls {
        fn from(value: CounterCall) -> Self {
            Self::Counter(value)
        }
    }
    impl ::core::convert::From<FailedCall> for CounterTestCalls {
        fn from(value: FailedCall) -> Self {
            Self::Failed(value)
        }
    }
    impl ::core::convert::From<SetUpCall> for CounterTestCalls {
        fn from(value: SetUpCall) -> Self {
            Self::SetUp(value)
        }
    }
    impl ::core::convert::From<TestIncrementCall> for CounterTestCalls {
        fn from(value: TestIncrementCall) -> Self {
            Self::TestIncrement(value)
        }
    }
    impl ::core::convert::From<TestSetNumberCall> for CounterTestCalls {
        fn from(value: TestSetNumberCall) -> Self {
            Self::TestSetNumber(value)
        }
    }
    ///Container type for all return fields from the `IS_TEST` function with signature `IS_TEST()` and selector `0xfa7626d4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsTestReturn(pub bool);
    ///Container type for all return fields from the `counter` function with signature `counter()` and selector `0x61bc221a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CounterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `failed` function with signature `failed()` and selector `0xba414fa6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FailedReturn(pub bool);
}
