//! Protected original-operator AI sessions and managed task deliverables.
//! No caller identity, provider credential, arbitrary path or command DTO.
//! Reuses Codeg's existing identity/task/runner; exact attribution in NOTICE.
pub mod types;
pub mod validation;

#[cfg(test)]
mod tests;
