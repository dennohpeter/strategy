use std::sync::Arc;

use anyhow::Ok;
use async_trait::async_trait;
use ethers::providers::Middleware;

use crate::types::{Action, Executor};

/// An executor that sends transactions to the mempool.
pub struct MempoolExecutor<M> {
    provider: Arc<M>,
}

impl<M: Middleware> MempoolExecutor<M> {
    pub fn new(provider: Arc<M>) -> Self {
        Self { provider }
    }
}

#[async_trait]
impl<M> Executor<Action> for MempoolExecutor<M>
where
    M: Middleware + 'static,
    M::Error: 'static,
{
    /// Send a transaction to the mempool.
    async fn execute(&self, action: Action) -> anyhow::Result<()> {
        let tx = self.provider.send_transaction(action, None).await?;

        println!("Sent transaction: {:?}", tx);
        Ok(())
    }
}
