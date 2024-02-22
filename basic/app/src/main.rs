use bindings::basic::Basic;
use bindings::basic_only_search::{BasicOnlySearch, BASICONLYSEARCH_BYTECODE};
use ethers::prelude::*;
use ethers::core::rand::thread_rng;
use ethers::types::U256;
use std::convert::TryFrom;
use std::str::FromStr;
use std::sync::Arc;
use ethers_flashbots::Relay;
use url::Url;
use serde::Deserialize;

pub mod bundle;
use bundle::{BundleHash, BundleRequest, OffChainTransaction};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SendBundleResponse {
    pub bundle_hash: Option<BundleHash>,
}

fn build_off_chain_tx(basic: H160, provider: Arc<Provider<Http>>) -> eyre::Result<OffChainTransaction> {
    let basic_contract = Basic::new(basic, provider);
    // Calldata for Basic contract search() method with fees=100 and data=0xFFF
    let calldata = basic_contract
        .search(U256::from(100), Bytes::from_str("0xFFFF")?)
        .calldata()
        .expect("failed to parse search() calldata");
    // Call method on Basic contract deployed on Ethereum with calldata and no bytecode
    // Return calldata from search() method will be called on the `to` Basic contract
    Ok(OffChainTransaction {
        to: basic,
        off_chain_gas_limit: 25000,
        gas_limit: 100000,
        data: calldata,
        code: None,
    })
}

fn build_off_chain_tx_with_bytecode(
    basic_only_search: H160,
    basic_only_settle: H160,
    provider: Arc<Provider<Http>>,
) -> eyre::Result<OffChainTransaction> {
    let basic_only_search_contract = BasicOnlySearch::new(basic_only_search, provider);
    // Calldata for BasicOnlySearch search() method with fees=100 and data=0xFFF
    let calldata = basic_only_search_contract
        .search(U256::from(100), Bytes::from_str("0xFFFF")?)
        .calldata()
        .expect("failed to parse search() calldata");
    // Call method on BasicOnlySearch contract provided as bytecode with calldata
    // Return calldata from search() method will be called on the `to` BasicOnlySettle contract
    let search_bytecode: Bytes = BASICONLYSEARCH_BYTECODE.clone();
    Ok(OffChainTransaction {
        to: basic_only_settle,
        off_chain_gas_limit: 25000,
        gas_limit: 100000,
        data: calldata,
        code: Some(search_bytecode),
    })
}

#[tokio::main]
async fn main() -> eyre::Result<()> {

    let merkle_rpc_url: &str = "https://eth.merkle.io";
    let merkle_bundles_url: &str = "https://bundles.merkle.io";
    let basic_contract_address = H160::from_str("0x0000000000000000000000000000000000000000")?;
    let basic_only_search_contract_address = H160::from_str("0x0000000000000000000000000000000000000000")?;
    let basic_only_settle_contract_address = H160::from_str("0x0000000000000000000000000000000000000000")?;

    // Connect to the network
    let provider = Arc::new(Provider::<Http>::try_from(merkle_rpc_url)?);
    let block_number = provider.get_block_number().await?;

    // This is your searcher identity
    let bundle_signer = LocalWallet::new(&mut thread_rng());

    // Lightweight relay client to send bundles to merkle Builder
    let mev_relay = Relay::new(
        Url::parse(merkle_bundles_url)?,
        Some(bundle_signer),
    );

    // Use Case 1: Basic.sol

    // Construct an off-chain transaction calling search() on Basic contract
    let off_chain_tx = build_off_chain_tx(basic_contract_address, Arc::clone(&provider))?;

    // Build bundle with off-chain transaction
    let bundle = BundleRequest::new()
        .push_transaction(off_chain_tx)
        .set_block(block_number + 1);
    println!("Bundle: {:?}", serde_json::to_string(&bundle).unwrap());

    // Builder response
    let response: Option<SendBundleResponse> = mev_relay
        .request("eth_sendBundle", [bundle])
        .await?;
    let bundle_hash = response
        .map(|resp| resp.bundle_hash)
        .flatten();
    println!("Bundle hash: {bundle_hash:?}");


    // Use Case 2: BasicOnlySearch.sol + BasicOnlySettle.sol

    // Construct an off-chain transaction calling search() on BasicOnlySearch contract that is
    // passed in as bytecode and not deployed on Ethereum. The return calldata will be called on
    // the deployed BasicOnlySettle contract.
    let off_chain_tx = build_off_chain_tx_with_bytecode(
        basic_only_search_contract_address,
        basic_only_settle_contract_address,
        Arc::clone(&provider),
    )?;

    // Build bundle with off-chain transaction
    let bundle = BundleRequest::new()
        .push_transaction(off_chain_tx)
        .set_block(block_number + 1);
    println!("Bundle: {:?}", serde_json::to_string(&bundle).unwrap());

    // Builder response
    let response: Option<SendBundleResponse> = mev_relay
        .request("eth_sendBundle", [bundle])
        .await?;
    let bundle_hash = response
        .map(|resp| resp.bundle_hash)
        .flatten();
    println!("Bundle hash: {bundle_hash:?}");

    Ok(())
}
