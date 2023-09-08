use std::pin::Pin;

use anyhow::Result;
use async_trait::async_trait;
use ethers::types::{transaction::eip2718::TypedTransaction, Transaction};
use tokio_stream::Stream;

/// A stream of events emitted by a [`Collector`](Collector).
pub type CollectorStream<'a, E> = Pin<Box<dyn Stream<Item = E> + Send + 'a>>;

/// Collector trait, which defines a source of events.
#[async_trait]
pub trait Collector<E>: Send + Sync {
    /// Returns the core event stream for the collector
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, E>>;
}

/// Executor trait, responsible for executing actions returning by strategies.
#[async_trait]
pub trait Executor<A>: Send + Sync {
    /// Execute an action.
    async fn execute(&self, action: A) -> anyhow::Result<()>;
}

/// Strategy trait, which defines the core logic for each opportunity.
#[async_trait]
pub trait Strategy<E, A>: Send + Sync {
    /// Process an event and return an action if needed
    async fn process_event(&mut self, event: E) -> Option<A>;
}

/// Convenience enum containing all the events that can be emitted by collectors.
pub type Event = Transaction;

/// Convenience enum containing all the actions that can be executed by executors.
pub type Action = TypedTransaction;
