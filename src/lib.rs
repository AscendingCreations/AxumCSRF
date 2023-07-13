#![doc = include_str!("../README.md")]
mod config;
mod error;
mod token;

pub(crate) mod cookies;

#[cfg(feature = "layer")]
mod layer;
#[cfg(feature = "layer")]
mod service;

#[cfg(feature = "layer")]
pub use layer::CsrfLayer;
#[cfg(feature = "layer")]
pub(crate) use service::AxumCsrfService;

pub use config::{CsrfConfig, Key, SameSite};
pub use error::CsrfError;
pub use token::CsrfToken;
