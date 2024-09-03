/// The constants for Multicall (deployed address + supported chains)
mod constants;
pub use constants::{MULTICALL_ADDRESS, MULTICALL_SUPPORTED_CHAINS};

/// The Multicall3 contract bindings. Autogenerated with `sol!`.
mod contract;
pub use contract::IMulticall3;

/// Middleware for creating and handling multicalls
mod middleware;
pub use middleware::{
	Call as MulticallSubCall, Multicall, MulticallVersion, Result as MulticallResult,
};

/// Multicall specific errors
mod error;
pub use error::MulticallError;