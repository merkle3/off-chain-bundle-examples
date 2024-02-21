pub use counter_script::*;
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
pub mod counter_script {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("IS_SCRIPT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("IS_SCRIPT"),
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
                    ::std::borrow::ToOwned::to_owned("run"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("run"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static COUNTERSCRIPT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@Ra\0\x0Ca\0-V[`\x07U`\t\x80T`\xFF\x19\x16`\x01\x17\x90U4\x80\x15a\0'W_\x80\xFD[Pa\x11\x85V[`@\x80Q`\xA0\x81\x01\x82R`\x05``\x82\x01\x81\x81Rd\x10[\x9D\x9A[`\xDA\x1B`\x80\x84\x01R\x82Razi` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x15\x81R\x7Fhttp://127.0.0.1:8545\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x82\x84\x01R\x91Qd\x18[\x9D\x9A[`\xDA\x1B\x81R_\x92`\x08\x91\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90a\0\xC3\x90\x82a\x0E\x81V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90a\0\xE2\x90\x82a\x0E\x81V[PP`@\x80Q`\xA0\x81\x01\x82R`\x07``\x82\x01\x81\x81Rf\x12\x18\\\x99\x1A\x18]`\xCA\x1B`\x80\x84\x01R\x82Razi` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x15\x81R\x7Fhttp://127.0.0.1:8545\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x82\x84\x01R\x91Qf\x1A\x18\\\x99\x1A\x18]`\xCA\x1B\x81R\x90\x92P`\x08\x91\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90a\x01\x7F\x90\x82a\x0E\x81V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90a\x01\x9E\x90\x82a\x0E\x81V[P\x90PP`@Q\x80``\x01`@R\x80`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01f\x13XZ[\x9B\x99]`\xCA\x1B\x81RP\x81R` \x01`\x01\x81R` \x01`@Q\x80``\x01`@R\x80`=\x81R` \x01a\x12\xF1`=\x919\x90R`@Qf\x1BXZ[\x9B\x99]`\xCA\x1B\x81R`\x08\x90`\x07\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90a\x02'\x90\x82a\x0E\x81V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90a\x02F\x90\x82a\x0E\x81V[P\x90PP`@Q\x80``\x01`@R\x80`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01eGoerli`\xD0\x1B\x81RP\x81R` \x01`\x05\x81R` \x01`@Q\x80``\x01`@R\x80`<\x81R` \x01a\x13.`<\x919\x90R`@Qegoerli`\xD0\x1B\x81R`\x08\x90`\x06\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90a\x02\xCD\x90\x82a\x0E\x81V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90a\x02\xEC\x90\x82a\x0E\x81V[PP`@\x80Q`\xA0\x81\x01\x82R`\x07``\x82\x01\x81\x81RfSepolia`\xC8\x1B`\x80\x84\x01R\x82Rb\xAA6\xA7` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x17\x81R\x7Fhttps://rpc.sepolia.dev\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x82\x84\x01R\x91Qfsepolia`\xC8\x1B\x81R\x90\x92P`\x08\x91\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90a\x03\x8A\x90\x82a\x0E\x81V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90a\x03\xA9\x90\x82a\x0E\x81V[PP`@\x80Q`\xA0\x81\x01\x82R`\x08``\x82\x01\x81\x81RgOptimism`\xC0\x1B`\x80\x84\x01R\x82R`\n` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x1B\x81R\x7Fhttps://mainnet.optimism.io\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x82\x84\x01R\x91Qgoptimism`\xC0\x1B\x81R\x90\x92P\x81\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90a\x04E\x90\x82a\x0E\x81V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90a\x04d\x90\x82a\x0E\x81V[PP`@\x80Q`\xA0\x81\x01\x82R`\x0F``\x82\x01\x81\x81RnOptimism Goerli`\x88\x1B`\x80\x84\x01R\x82Ra\x01\xA4` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x1A\x81R\x7Fhttps://goerli.optimism.io\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x82\x84\x01R\x91Qnoptimism_goerli`\x88\x1B\x81R\x90\x92P`\x08\x91\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90a\x05\x11\x90\x82a\x0E\x81V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90a\x050\x90\x82a\x0E\x81V[PP`@\x80Q`\xA0\x81\x01\x82R`\x0C``\x82\x01\x81\x81RkArbitrum One`\xA0\x1B`\x80\x84\x01R\x82Ra\xA4\xB1` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x1C\x81R\x7Fhttps://arb1.arbitrum.io/rpc\0\0\0\0\x91\x81\x01\x91\x90\x91R\x82\x84\x01R\x91Qkarbitrum_one`\xA0\x1B\x81R\x90\x92P`\x08\x91\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90a\x05\xD7\x90\x82a\x0E\x81V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90a\x05\xF6\x90\x82a\x0E\x81V[P\x90PP`@Q\x80``\x01`@R\x80`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01\x7FArbitrum One Goerli\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81R` \x01b\x06n\xED\x81R` \x01`@Q\x80``\x01`@R\x80`%\x81R` \x01a\x13j`%\x919\x90R`@Q\x7Farbitrum_one_goerli\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x08\x90`\x13\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90a\x06\xAD\x90\x82a\x0E\x81V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90a\x06\xCC\x90\x82a\x0E\x81V[PP`@\x80Q`\xA0\x81\x01\x82R`\r``\x82\x01\x81\x81RlArbitrum Nova`\x98\x1B`\x80\x84\x01R\x82Ra\xA4\xBA` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x1C\x81R\x7Fhttps://nova.arbitrum.io/rpc\0\0\0\0\x91\x81\x01\x91\x90\x91R\x82\x84\x01R\x91Qlarbitrum_nova`\x98\x1B\x81R\x90\x92P`\x08\x91\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90a\x07u\x90\x82a\x0E\x81V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90a\x07\x94\x90\x82a\x0E\x81V[PP`@\x80Q`\xA0\x81\x01\x82R`\x07``\x82\x01\x81\x81Rf(7\xB6<\xB3\xB7\xB7`\xC9\x1B`\x80\x84\x01R\x82R`\x89` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x17\x81R\x7Fhttps://polygon-rpc.com\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x82\x84\x01R\x91Qf87\xB6<\xB3\xB7\xB7`\xC9\x1B\x81R\x90\x92P`\x08\x91\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90a\x080\x90\x82a\x0E\x81V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90a\x08O\x90\x82a\x0E\x81V[PP`@\x80Q`\xA0\x81\x01\x82R`\x0E``\x82\x01\x81\x81RmPolygon Mumbai`\x90\x1B`\x80\x84\x01R\x82Rb\x018\x81` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x1E\x81R\x7Fhttps://rpc-mumbai.matic.today\0\0\x91\x81\x01\x91\x90\x91R\x82\x84\x01R\x91Qmpolygon_mumbai`\x90\x1B\x81R\x90\x92P`\x08\x91\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90a\x08\xFB\x90\x82a\x0E\x81V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90a\t\x1A\x90\x82a\x0E\x81V[P\x90PP`@Q\x80``\x01`@R\x80`@Q\x80`@\x01`@R\x80`\t\x81R` \x01hAvalanche`\xB8\x1B\x81RP\x81R` \x01a\xA8j\x81R` \x01`@Q\x80``\x01`@R\x80`%\x81R` \x01a\x12\xCC`%\x919\x90R`@Qhavalanche`\xB8\x1B\x81R`\x08\x90`\t\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90a\t\xA8\x90\x82a\x0E\x81V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90a\t\xC7\x90\x82a\x0E\x81V[P\x90PP`@Q\x80``\x01`@R\x80`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mAvalanche Fuji`\x90\x1B\x81RP\x81R` \x01a\xA8i\x81R` \x01`@Q\x80``\x01`@R\x80`*\x81R` \x01a\x13\x8F`*\x919\x90R`@Qmavalanche_fuji`\x90\x1B\x81R`\x08\x90`\x0E\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90a\n_\x90\x82a\x0E\x81V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90a\n~\x90\x82a\x0E\x81V[P\x90PP`@Q\x80``\x01`@R\x80`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n!'!\x10)\xB6\xB0\xB9:\x10!\xB40\xB4\xB7`\x89\x1B\x81RP\x81R` \x01`8\x81R` \x01`@Q\x80``\x01`@R\x80`!\x81R` \x01a\x13\xB9`!\x919\x90R`@Qn171/\xB9\xB6\xB0\xB9:/\xB1\xB40\xB4\xB7`\x89\x1B\x81R`\x08\x90`\x0F\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90a\x0B\x17\x90\x82a\x0E\x81V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90a\x0B6\x90\x82a\x0E\x81V[P\x90PP`@Q\x80``\x01`@R\x80`@Q\x80`@\x01`@R\x80`\x17\x81R` \x01\x7FBNB Smart Chain Testnet\0\0\0\0\0\0\0\0\0\x81RP\x81R` \x01`a\x81R` \x01`@Q\x80``\x01`@R\x80`.\x81R` \x01a\x12\x9E`.\x919\x90R`@Q\x7Fbnb_smart_chain_testnet\0\0\0\0\0\0\0\0\0\x81R`\x08\x90`\x17\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90a\x0B\xEB\x90\x82a\x0E\x81V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90a\x0C\n\x90\x82a\x0E\x81V[PP`@\x80Q`\xA0\x81\x01\x82R`\x0C``\x82\x01\x81\x81Rk#\xB77\xB9\xB4\xB9\x90!\xB40\xB4\xB7`\xA1\x1B`\x80\x84\x01R\x82R`d` \x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x1B\x81R\x7Fhttps://rpc.gnosischain.com\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x82\x84\x01R\x91Qk3\xB77\xB9\xB4\xB9\xAF\xB1\xB40\xB4\xB7`\xA1\x1B\x81R\x90\x92P`\x08\x91\x01\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x81Q\x81\x90a\x0C\xB0\x90\x82a\x0E\x81V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90a\x0C\xCF\x90\x82a\x0E\x81V[P\x90PP_\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1C`\x01`\x01`\xA0\x1B\x03\x16c\x9D*\xD7*`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r1W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\rX\x91\x90\x81\x01\x90a\x10\x1EV[\x90P_[\x81Q\x81\x10\x15a\r\xDFW\x81\x81\x81Q\x81\x10a\rwWa\rwa\x112V[` \x02` \x01\x01Q` \x01Q`\x08\x83\x83\x81Q\x81\x10a\r\x97Wa\r\x97a\x112V[` \x02` \x01\x01Q_\x01Q`@Qa\r\xAF\x91\x90a\x11FV[\x90\x81R` \x01`@Q\x80\x91\x03\x90 `\x02\x01\x90\x81a\r\xCC\x91\x90a\x0E\x81V[P\x80a\r\xD7\x81a\x11aV[\x91PPa\r\\V[P_\x91PP\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0E\x0FW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0E-WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x0E|W_\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x0EYWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0ExW\x82\x81U`\x01\x01a\x0EeV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\x9AWa\x0E\x9Aa\r\xE7V[a\x0E\xAE\x81a\x0E\xA8\x84Ta\r\xFBV[\x84a\x0E3V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x0E\xE1W_\x84\x15a\x0E\xCAWP\x85\x83\x01Q[_\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0ExV[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x0F\x0FW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x0E\xF0V[P\x85\x82\x10\x15a\x0F,W\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0F^Wa\x0F^a\r\xE7V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0F\x8CWa\x0F\x8Ca\r\xE7V[`@R\x91\x90PV[_[\x83\x81\x10\x15a\x0F\xAEW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0F\x96V[PP_\x91\x01RV[_\x82`\x1F\x83\x01\x12a\x0F\xC5W_\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F\xDEWa\x0F\xDEa\r\xE7V[a\x0F\xF1`\x1F\x82\x01`\x1F\x19\x16` \x01a\x0FdV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x10\x05W_\x80\xFD[a\x10\x16\x82` \x83\x01` \x87\x01a\x0F\x94V[\x94\x93PPPPV[_` \x80\x83\x85\x03\x12\x15a\x10/W_\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x10EW_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x10XW_\x80\xFD[\x81Q\x81\x81\x11\x15a\x10jWa\x10ja\r\xE7V[\x80`\x05\x1Ba\x10y\x85\x82\x01a\x0FdV[\x91\x82R\x83\x81\x01\x85\x01\x91\x85\x81\x01\x90\x89\x84\x11\x15a\x10\x92W_\x80\xFD[\x86\x86\x01\x92P[\x83\x83\x10\x15a\x11%W\x82Q\x85\x81\x11\x15a\x10\xAFW_\x80\x81\xFD[\x86\x01`@\x81\x8C\x03`\x1F\x19\x01\x81\x13\x15a\x10\xC6W_\x80\x81\xFD[a\x10\xCEa\x0F<V[\x89\x83\x01Q\x88\x81\x11\x15a\x10\xDFW_\x80\x81\xFD[a\x10\xED\x8E\x8C\x83\x87\x01\x01a\x0F\xB6V[\x82RP\x90\x82\x01Q\x90\x87\x82\x11\x15a\x11\x02W_\x80\x81\xFD[a\x11\x10\x8D\x8B\x84\x86\x01\x01a\x0F\xB6V[\x81\x8B\x01R\x84RPP\x91\x86\x01\x91\x90\x86\x01\x90a\x10\x98V[\x99\x98PPPPPPPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x82Qa\x11W\x81\x84` \x87\x01a\x0F\x94V[\x91\x90\x91\x01\x92\x91PPV[_`\x01\x82\x01a\x11~WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[P`\x01\x01\x90V[a\x01\x0C\x80a\x11\x92_9_\xF3\xFE`\x80`@R4\x80\x15`\x0EW_\x80\xFD[P`\x046\x10`:W_5`\xE0\x1C\x80c\n\x92T\xE4\x14`>W\x80c\xC0@b&\x14`@W\x80c\xF8\xCC\xBFG\x14`FW[_\x80\xFD[\0[`>`fV[`\tT`R\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1C`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xC9\x80@`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15`\xBFW_\x80\xFD[PZ\xF1\x15\x80\x15`\xD0W=_\x80>=_\xFD[PPPPV\xFE\xA2dipfsX\"\x12 {\xA7\xA1\xE82\xCE8\xBB\x8D\x82#I; V\x94~\xC7*\xF0M\xAE3\x9A\xC0\xE4\xDE]\xC6\x16\x91^dsolcC\0\x08\x15\x003https://data-seed-prebsc-1-s1.binance.org:8545https://api.avax.network/ext/bc/C/rpchttps://mainnet.infura.io/v3/6770454bc6ea42c58aac12978531b93fhttps://goerli.infura.io/v3/6770454bc6ea42c58aac12978531b93fhttps://goerli-rollup.arbitrum.io/rpchttps://api.avax-test.network/ext/bc/C/rpchttps://bsc-dataseed1.binance.org";
    /// The bytecode of the contract.
    pub static COUNTERSCRIPT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0EW_\x80\xFD[P`\x046\x10`:W_5`\xE0\x1C\x80c\n\x92T\xE4\x14`>W\x80c\xC0@b&\x14`@W\x80c\xF8\xCC\xBFG\x14`FW[_\x80\xFD[\0[`>`fV[`\tT`R\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1C`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xC9\x80@`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15`\xBFW_\x80\xFD[PZ\xF1\x15\x80\x15`\xD0W=_\x80>=_\xFD[PPPPV\xFE\xA2dipfsX\"\x12 {\xA7\xA1\xE82\xCE8\xBB\x8D\x82#I; V\x94~\xC7*\xF0M\xAE3\x9A\xC0\xE4\xDE]\xC6\x16\x91^dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static COUNTERSCRIPT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CounterScript<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CounterScript<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CounterScript<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CounterScript<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CounterScript<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CounterScript))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CounterScript<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    COUNTERSCRIPT_ABI.clone(),
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
                COUNTERSCRIPT_ABI.clone(),
                COUNTERSCRIPT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `IS_SCRIPT` (0xf8ccbf47) function
        pub fn is_script(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([248, 204, 191, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `run` (0xc0406226) function
        pub fn run(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 64, 98, 38], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUp` (0x0a9254e4) function
        pub fn set_up(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 146, 84, 228], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CounterScript<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "IS_SCRIPT", abi = "IS_SCRIPT()")]
    pub struct IsScriptCall;
    ///Container type for all input parameters for the `run` function with signature `run()` and selector `0xc0406226`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "run", abi = "run()")]
    pub struct RunCall;
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CounterScriptCalls {
        IsScript(IsScriptCall),
        Run(RunCall),
        SetUp(SetUpCall),
    }
    impl ::ethers::core::abi::AbiDecode for CounterScriptCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <IsScriptCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsScript(decoded));
            }
            if let Ok(decoded) = <RunCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Run(decoded));
            }
            if let Ok(decoded) = <SetUpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetUp(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CounterScriptCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsScript(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Run(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetUp(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for CounterScriptCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsScript(element) => ::core::fmt::Display::fmt(element, f),
                Self::Run(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetUp(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IsScriptCall> for CounterScriptCalls {
        fn from(value: IsScriptCall) -> Self {
            Self::IsScript(value)
        }
    }
    impl ::core::convert::From<RunCall> for CounterScriptCalls {
        fn from(value: RunCall) -> Self {
            Self::Run(value)
        }
    }
    impl ::core::convert::From<SetUpCall> for CounterScriptCalls {
        fn from(value: SetUpCall) -> Self {
            Self::SetUp(value)
        }
    }
    ///Container type for all return fields from the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsScriptReturn(pub bool);
}
