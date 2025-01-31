use std::sync::Arc;

use serde::{Deserialize, Serialize};

/// Log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LogEntry<C> {
    /// Term
    pub(crate) term: u64,
    /// Index
    pub(crate) index: usize,
    /// The Command
    pub(crate) cmd: Arc<C>,
}

impl<C> LogEntry<C> {
    /// Create a new `LogEntry`
    pub(super) fn new(index: usize, term: u64, cmd: Arc<C>) -> Self {
        Self { term, index, cmd }
    }
}
