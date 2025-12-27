//! Kimai ML - Rust библиотека

pub mod models;
pub mod preprocessing;
pub mod types;
pub mod grpc_server;

pub use models::*;
pub use preprocessing::*;
pub use types::*;

// Re-export для удобства
pub use models::learning::{LearningModule, PredictionError};
