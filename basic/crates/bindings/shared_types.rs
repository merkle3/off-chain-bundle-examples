///`FuzzInterface(address,string[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct FuzzInterface {
    pub addr: ::ethers::core::types::Address,
    pub artifacts: ::std::vec::Vec<::std::string::String>,
}
///`FuzzSelector(address,bytes4[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct FuzzSelector {
    pub addr: ::ethers::core::types::Address,
    pub selectors: ::std::vec::Vec<[u8; 4]>,
}
///`AccountAccess((uint256,uint256),uint8,address,address,bool,uint256,uint256,bytes,uint256,bytes,bool,(address,bytes32,bool,bytes32,bytes32,bool)[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct AccountAccess {
    pub chain_info: ChainInfo,
    pub kind: u8,
    pub account: ::ethers::core::types::Address,
    pub accessor: ::ethers::core::types::Address,
    pub initialized: bool,
    pub old_balance: ::ethers::core::types::U256,
    pub new_balance: ::ethers::core::types::U256,
    pub deployed_code: ::ethers::core::types::Bytes,
    pub value: ::ethers::core::types::U256,
    pub data: ::ethers::core::types::Bytes,
    pub reverted: bool,
    pub storage_accesses: ::std::vec::Vec<StorageAccess>,
}
///`ChainInfo(uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct ChainInfo {
    pub fork_id: ::ethers::core::types::U256,
    pub chain_id: ::ethers::core::types::U256,
}
///`DirEntry(string,string,uint64,bool,bool)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct DirEntry {
    pub error_message: ::std::string::String,
    pub path: ::std::string::String,
    pub depth: u64,
    pub is_dir: bool,
    pub is_symlink: bool,
}
///`EthGetLogs(address,bytes32[],bytes,bytes32,uint64,bytes32,uint64,uint256,bool)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct EthGetLogs {
    pub emitter: ::ethers::core::types::Address,
    pub topics: ::std::vec::Vec<[u8; 32]>,
    pub data: ::ethers::core::types::Bytes,
    pub block_hash: [u8; 32],
    pub block_number: u64,
    pub transaction_hash: [u8; 32],
    pub transaction_index: u64,
    pub log_index: ::ethers::core::types::U256,
    pub removed: bool,
}
///`FfiResult(int32,bytes,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct FfiResult {
    pub exit_code: i32,
    pub stdout: ::ethers::core::types::Bytes,
    pub stderr: ::ethers::core::types::Bytes,
}
///`FsMetadata(bool,bool,uint256,bool,uint256,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct FsMetadata {
    pub is_dir: bool,
    pub is_symlink: bool,
    pub length: ::ethers::core::types::U256,
    pub read_only: bool,
    pub modified: ::ethers::core::types::U256,
    pub accessed: ::ethers::core::types::U256,
    pub created: ::ethers::core::types::U256,
}
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
    pub key: ::std::string::String,
    pub url: ::std::string::String,
}
///`StorageAccess(address,bytes32,bool,bytes32,bytes32,bool)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct StorageAccess {
    pub account: ::ethers::core::types::Address,
    pub slot: [u8; 32],
    pub is_write: bool,
    pub previous_value: [u8; 32],
    pub new_value: [u8; 32],
    pub reverted: bool,
}
///`Wallet(address,uint256,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Wallet {
    pub addr: ::ethers::core::types::Address,
    pub public_key_x: ::ethers::core::types::U256,
    pub public_key_y: ::ethers::core::types::U256,
    pub private_key: ::ethers::core::types::U256,
}
