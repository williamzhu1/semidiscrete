use std::time::Instant;

use once_cell::sync::Lazy;

pub mod io;
pub mod sdr_config;
pub mod discrete_item;
pub mod edge_extension;

pub static EPOCH: Lazy<Instant> = Lazy::new(Instant::now);
