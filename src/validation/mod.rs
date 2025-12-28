//! Validation middleware for oxicode.
//!
//! This module provides validation constraints for deserialization,
//! ensuring data integrity and security during decoding.
//!
//! ## Features
//!
//! - **Size Limits**: Limit string/collection lengths
//! - **Range Constraints**: Validate numeric values
//! - **Custom Validators**: User-defined validation functions
//! - **Checksum Verification**: Optional integrity checking
//!
//! ## Example
//!
//! ```rust,ignore
//! use oxicode::validation::{Validator, Constraints};
//!
//! // Create a validator with constraints
//! let mut validator = Validator::new();
//! validator.add_constraint("name", Constraints::max_len(100));
//! validator.add_constraint("age", Constraints::range(0..=120));
//!
//! // Validate data
//! let result = validator.validate(&data)?;
//! ```

mod constraints;
mod validator;

pub use constraints::{Constraint, Constraints, ValidationResult};
pub use validator::{FieldValidation, ValidationError, Validator};

/// Configuration for validation behavior.
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Whether to fail fast on the first validation error.
    pub fail_fast: bool,

    /// Maximum depth for nested structure validation.
    pub max_depth: usize,

    /// Whether to enable checksum verification.
    pub verify_checksum: bool,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            fail_fast: true,
            max_depth: 64,
            verify_checksum: false,
        }
    }
}

impl ValidationConfig {
    /// Create a new validation configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set fail-fast behavior.
    #[inline]
    pub fn with_fail_fast(mut self, fail_fast: bool) -> Self {
        self.fail_fast = fail_fast;
        self
    }

    /// Set maximum validation depth.
    #[inline]
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    /// Enable or disable checksum verification.
    #[inline]
    pub fn with_checksum(mut self, verify: bool) -> Self {
        self.verify_checksum = verify;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = ValidationConfig::default();
        assert!(config.fail_fast);
        assert_eq!(config.max_depth, 64);
        assert!(!config.verify_checksum);
    }

    #[test]
    fn test_config_builder() {
        let config = ValidationConfig::new()
            .with_fail_fast(false)
            .with_max_depth(128)
            .with_checksum(true);

        assert!(!config.fail_fast);
        assert_eq!(config.max_depth, 128);
        assert!(config.verify_checksum);
    }
}
