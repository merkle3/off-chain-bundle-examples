///`Log(bytes32[],bytes,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Log {
    pub topics: ::std::vec::Vec<[u8; 32]>,
    pub data: ::ethers::core::types::Bytes,
    pub emitter: ::ethers::core::types::Address,
}
///`Rpc(string,string)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Rpc {
    pub name: ::std::string::String,
    pub url: ::std::string::String,
}
