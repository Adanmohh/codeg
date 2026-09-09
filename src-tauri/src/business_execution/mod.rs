//! Protected original-operator AI sessions and managed task deliverables.
//! No caller identity, provider credential, arbitrary path or command DTO.
//! Reuses Codeg's existing identity/task/runner; exact attribution in NOTICE.
pub mod common;
mod files;
mod receipts;
mod records;
mod scope;
mod session_store;
pub mod types;
pub mod validation;

#[cfg(all(test, unix))]
mod file_tests;
#[cfg(test)]
mod pagination_tests;
#[cfg(test)]
mod session_tests;
#[cfg(test)]
mod tests;
