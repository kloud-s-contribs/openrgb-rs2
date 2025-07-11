//! Client library for [OpenRGB](https://openrgb.org) SDK server.
//!
//! This client is async and requires a [tokio](https://tokio.rs) runtime to run.
//!
//! # Example
//!
//! ```no_run
//! use openrgb2::OpenRgbClient;
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!
//!     // connect to default server at localhost
//!     let client = OpenRgbClient::connect().await?;
//!     let controllers = client.get_all_controllers().await?;
//!     controllers.init().await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! See [examples](https://github.com/Achtuur/openrgb-rs2/tree/master/examples), and [`OpenRgbClient`] for client API.

#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

#[doc(inline)]
pub use {
    client::*,
    data::*,
    error::{OpenRgbError, OpenRgbResult},
};

pub(crate) use protocol::*;

mod client;
mod error;
pub(crate) mod protocol;
