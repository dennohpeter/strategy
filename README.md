### Strategy

[![Rust](https://github.com/dennohpeter/strategy/actions/workflows/general.yml/badge.svg)](https://github.com/dennohpeter/strategy/actions/workflows/general.yml)
[![Rust](https://github.com/dennohpeter/strategy/actions/workflows/audit.yml/badge.svg)](https://github.com/dennohpeter/strategy/actions/workflows/audit.yml)
[![](https://img.shields.io/badge/License-MIT-green.svg)](./LICENSE)
[![](https://img.shields.io/crates/v/strategy)](https://crates.io/crates/strategy)

### About

A framework for writing MEV strategies and bots in Rust

### Install

`Cargo.toml`

```toml
[dependencies]
strategy = { version = "0.1.1" }
```

### Usage

`strategy.rs`

```rust

use strategy::types::Strategy, Event, Action};

pub struct Sandwicher<M> {
    provider: Arc<M>
}


impl<M> Sandwicher<M> {
    pub fn new(provider: Arc<M>) -> Self {
        Self {
            provider
        }
    }
}

#[async_trait]
impl<M: Middleware + 'static> Strategy<Event, Action> for Sandwicher<M> {
    async fn process_event(&mut self, event: Event) -> Option<Action> {

        // Process incoming event/tx

        None
    }
}
```
