use ethers::core::{
    types::{transaction::response::Transaction, Bytes, TxHash, H256, U64, H160},
    utils::keccak256,
};
use serde::{Serialize, Serializer};
use uuid::Uuid;

/// A bundle hash.
pub type BundleHash = H256;

/// An off-chain transaction
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OffChainTransaction {
    pub to: H160,
    pub off_chain_gas_limit: u64,
    pub gas_limit: u64,
    pub data: Bytes,
    pub code: Option<Bytes>,
}

/// A transaction that can be added to a bundle.
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum BundleTransaction {
    /// A pre-signed transaction.
    Signed(Box<Transaction>),
    /// An RLP encoded signed transaction.
    Raw(Bytes),
    /// An off-chain transaction
    OffChain(OffChainTransaction),
}

impl From<Transaction> for BundleTransaction {
    fn from(tx: Transaction) -> Self {
        Self::Signed(Box::new(tx))
    }
}

impl From<Bytes> for BundleTransaction {
    fn from(tx: Bytes) -> Self {
        Self::Raw(tx)
    }
}

impl From<OffChainTransaction> for BundleTransaction {
    fn from(tx: OffChainTransaction) -> Self {
        Self::OffChain(tx)
    }
}

/// A bundle that can be submitted to a Flashbots relay.
///
/// The bundle can include your own transactions and transactions from
/// the mempool.
///
/// Additionally, this bundle can be simulated through a relay if simulation
/// parameters are provided using [`BundleRequest::set_simulation_block`] and
/// [`BundleRequest::set_simulation_timestamp`].
///
/// Please note that some parameters are required, and submitting a bundle
/// without them will get it rejected pre-flight. The required parameters
/// include:
///
/// - At least one transaction ([`BundleRequest::push_transaction`])
/// - A target block ([`BundleRequest::set_block`])
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleRequest {
    #[serde(rename = "txs")]
    #[serde(serialize_with = "serialize_txs")]
    transactions: Vec<BundleTransaction>,
    #[serde(rename = "revertingTxHashes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    revertible_transaction_hashes: Vec<H256>,

    #[serde(rename = "blockNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    target_block: Option<U64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_timestamp: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_timestamp: Option<u64>,

    #[serde(rename = "replacementUuid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_uuid_as_string")]
    uuid: Option<Uuid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    refund_percent: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_recipient: Option<H160>,
}

fn serialize_uuid_as_string<S>(x: &Option<Uuid>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Don't need to handle None option here as handled by
    // #[serde(skip_serializing_if = "Option::is_none")]
    s.serialize_str(&x.unwrap().to_string())
}

pub fn serialize_txs<S>(txs: &[BundleTransaction], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let raw_txs: Vec<BundleTransaction> = txs
        .iter()
        .map(|tx| match tx {
            BundleTransaction::Signed(inner) => BundleTransaction::Raw(inner.rlp()),
            tx => tx.clone(),
        })
        .collect();

    raw_txs.serialize(s)
}

impl BundleRequest {
    /// Creates an empty bundle request.
    pub fn new() -> Self {
        Default::default()
    }

    /// Adds a transaction to the bundle request.
    ///
    /// Transactions added to the bundle can either be novel transactions,
    /// i.e. transactions that you have crafted, or they can be from
    /// one of the mempool APIs.
    pub fn push_transaction<T: Into<BundleTransaction>>(mut self, tx: T) -> Self {
        self.transactions.push(tx.into());
        self
    }

    /// Adds a transaction to the bundle request.
    ///
    /// This function takes a mutable reference to `self` and adds the specified
    /// transaction to the `transactions` vector. The added transaction can either
    /// be a novel transaction that you have crafted, or it can be from one of the
    /// mempool APIs.
    pub fn add_transaction<T: Into<BundleTransaction>>(&mut self, tx: T) {
        self.transactions.push(tx.into());
    }

    /// Adds a revertible transaction to the bundle request.
    ///
    /// This differs from [`BundleRequest::push_transaction`] in that the bundle will still be
    /// considered valid if the transaction reverts.
    pub fn push_revertible_transaction<T: Into<BundleTransaction>>(mut self, tx: T) -> Self {
        let tx = tx.into();
        self.transactions.push(tx.clone());

        let tx_hash: H256 = match tx {
            BundleTransaction::Signed(inner) => inner.hash(),
            BundleTransaction::Raw(inner) => keccak256(inner).into(),
            BundleTransaction::OffChain(_) => unreachable!("Off-chain transaction cannot revert"),
        };
        self.revertible_transaction_hashes.push(tx_hash);

        self
    }

    /// Adds a revertible transaction to the bundle request.
    ///
    /// This function takes a mutable reference to `self` and adds the specified
    /// revertible transaction to the `transactions` vector. The added transaction can either
    /// be a novel transaction that you have crafted, or it can be from one of the
    /// mempool APIs. Unlike the `push_transaction` method, the bundle will still be considered
    /// valid even if the added transaction reverts.
    pub fn add_revertible_transaction<T: Into<BundleTransaction>>(&mut self, tx: T) {
        let tx = tx.into();
        self.transactions.push(tx.clone());

        let tx_hash: H256 = match tx {
            BundleTransaction::Signed(inner) => inner.hash(),
            BundleTransaction::Raw(inner) => keccak256(inner).into(),
            BundleTransaction::OffChain(_) => unreachable!("Off-chain transaction cannot revert"),
        };
        self.revertible_transaction_hashes.push(tx_hash);
    }

    /// Get a reference to the transactions currently in the bundle request.
    pub fn transactions(&self) -> &Vec<BundleTransaction> {
        &self.transactions
    }

    /// Get a list of transaction hashes in the bundle request.
    pub fn transaction_hashes(&self) -> Vec<TxHash> {
        self.transactions
            .iter()
            .map(|tx| match tx {
                BundleTransaction::Signed(inner) => keccak256(inner.rlp()).into(),
                BundleTransaction::Raw(inner) => keccak256(inner).into(),
                BundleTransaction::OffChain(_) => TxHash::default(),
            })
            .collect()
    }

    /// Get a reference to the replacement uuid (if any).
    pub fn uuid(&self) -> &Option<Uuid> {
        &self.uuid
    }

    /// Set the replacement uuid of the bundle.
    /// This is used for bundle replacements or cancellations using eth_cancelBundle
    pub fn set_uuid(mut self, uuid: Uuid) -> Self {
        self.uuid = Some(uuid);
        self
    }

    /// Get the target block (if any).
    pub fn block(&self) -> Option<U64> {
        self.target_block
    }

    /// Set the target block of the bundle.
    pub fn set_block(mut self, block: U64) -> Self {
        self.target_block = Some(block);
        self
    }

    /// Get the minimum timestamp for which this bundle is valid (if any),
    /// in seconds since the UNIX epoch.
    pub fn min_timestamp(&self) -> Option<u64> {
        self.min_timestamp
    }

    /// Set the minimum timestamp for which this bundle is valid (if any),
    /// in seconds since the UNIX epoch.
    pub fn set_min_timestamp(mut self, timestamp: u64) -> Self {
        self.min_timestamp = Some(timestamp);
        self
    }

    /// Get the maximum timestamp for which this bundle is valid (if any),
    /// in seconds since the UNIX epoch.
    pub fn max_timestamp(&self) -> Option<u64> {
        self.max_timestamp
    }

    /// Set the maximum timestamp for which this bundle is valid (if any),
    /// in seconds since the UNIX epoch.
    pub fn set_max_timestamp(mut self, timestamp: u64) -> Self {
        self.max_timestamp = Some(timestamp);
        self
    }

    /// Get the refund percent (if any).
    pub fn refund_percent(&self) -> Option<u64> {
        self.refund_percent
    }

    /// Set the refund percent of the bundle.
    pub fn set_refund_percent(mut self, refund: u64) -> Self {
        self.refund_percent = Some(refund);
        self
    }

    /// Get the refund index (if any).
    pub fn refund_index(&self) -> Option<usize> {
        self.refund_index
    }

    /// Set the refund index of the bundle.
    pub fn set_refund_index(mut self, index: usize) -> Self {
        self.refund_index = Some(index);
        self
    }

    /// Get the refund recipient (if any).
    pub fn refund_recipient(&self) -> Option<H160> {
        self.refund_recipient
    }

    /// Set the refund recipient of the bundle.
    pub fn set_refund_recipient(mut self, recipient: H160) -> Self {
        self.refund_recipient = Some(recipient);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::utils::hex::FromHex;
    use uuid::uuid;
    use std::str::FromStr;

    #[test]
    fn bundle_serialize() {
        let bundle = BundleRequest::new()
            .push_transaction(Bytes::from(vec![0x1]))
            .push_revertible_transaction(Bytes::from(vec![0x2]))
            .set_block(2.into())
            .set_min_timestamp(1000)
            .set_max_timestamp(2000);

        assert_eq!(
            &serde_json::to_string(&bundle).unwrap(),
            r#"{"txs":["0x01","0x02"],"revertingTxHashes":["0xf2ee15ea639b73fa3db9b34a245bdfa015c260c598b211bf05a1ecc4b3e3b4f2"],"blockNumber":"0x2","minTimestamp":1000,"maxTimestamp":2000}"#
        );
    }

    #[test]
    fn bundle_serialize_add_transactions() {
        let mut bundle = BundleRequest::new()
            .push_transaction(Bytes::from(vec![0x1]))
            .push_revertible_transaction(Bytes::from(vec![0x2]))
            .set_block(2.into())
            .set_min_timestamp(1000)
            .set_max_timestamp(2000)
            .set_uuid(uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"));

        bundle.add_transaction(Bytes::from(vec![0x3]));
        bundle.add_revertible_transaction(Bytes::from(vec![0x4]));

        assert_eq!(
            &serde_json::to_string(&bundle).unwrap(),
            r#"{"txs":["0x01","0x02","0x03","0x04"],"revertingTxHashes":["0xf2ee15ea639b73fa3db9b34a245bdfa015c260c598b211bf05a1ecc4b3e3b4f2","0xf343681465b9efe82c933c3e8748c70cb8aa06539c361de20f72eac04e766393"],"blockNumber":"0x2","minTimestamp":1000,"maxTimestamp":2000,"replacementUuid":"67e55044-10b1-426f-9247-bb680e5fe0c8"}"#
        );
    }

    #[test]
    fn off_chain_bundle_serialize() {
        let bundle = BundleRequest::new()
            .push_transaction(OffChainTransaction::default())
            .set_block(1.into())
            .set_min_timestamp(1000)
            .set_max_timestamp(2000);

        assert_eq!(
            &serde_json::to_string(&bundle).unwrap(),
            r#"{"txs":[{"to":"0x0000000000000000000000000000000000000000","offChainGasLimit":0,"gasLimit":0,"data":"0x","code":null}],"blockNumber":"0x1","minTimestamp":1000,"maxTimestamp":2000}"#,
        );
    }

    #[test]
    fn off_chain_bundle_serialize_add_transactions() {
        let mut bundle = BundleRequest::new()
            .push_transaction(Bytes::from(vec![0x1]))
            .set_block(1.into())
            .set_min_timestamp(1000)
            .set_max_timestamp(2000)
            .set_refund_percent(90)
            .set_refund_index(1)
            .set_refund_recipient(H160::default());

        bundle.add_transaction(OffChainTransaction {
            to: H160::from_str("0x0000000000000000000000000000000000000001").expect("valid"),
            off_chain_gas_limit: 1000000,
            gas_limit: 500000,
            data: Bytes::from_hex("0xFFFF").expect("valid"),
            code: Some(Bytes::from_hex("0x0000FFFF").expect("valid")),
        });

        assert_eq!(
            &serde_json::to_string(&bundle).unwrap(),
            r#"{"txs":["0x01",{"to":"0x0000000000000000000000000000000000000001","offChainGasLimit":1000000,"gasLimit":500000,"data":"0xffff","code":"0x0000ffff"}],"blockNumber":"0x1","minTimestamp":1000,"maxTimestamp":2000,"refundPercent":90,"refundIndex":1,"refundRecipient":"0x0000000000000000000000000000000000000000"}"#,
        );
    }
}
