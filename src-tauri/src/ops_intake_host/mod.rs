//! Trusted operator host over accepted intake/approval packs. No generic agent API.
mod process;
mod runtime;
mod store;
pub mod types;
pub use runtime::HostRuntime;

#[cfg(test)]
mod tests;
