use crate::Result;
use reth_primitives::{rpc::BlockId, Address, H256, U256};

/// Provides access to storage data
pub trait StorageProvider: Send + Sync + 'static {
    /// Returns the value from a storage position at a given address and `BlockId`
    fn storage_at(&self, address: Address, index: U256, at: BlockId) -> Result<Option<H256>>;
}
