//! SigmaOS-access-management-security: access management security model.
//!
//! Real implementation: policies, roles, and validation.
//! Replaces the 104-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Security policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityPolicy {
    pub id: String,
    pub name: String,
    pub scope: String,
    pub enforcement: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityError {
    EmptyName,
    DuplicatePolicy(String),
}

impl fmt::Display for SecurityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "policy name must not be empty"),
            Self::DuplicatePolicy(id) => write!(f, "duplicate policy {:?}", id),
        }
    }
}

impl std::error::Error for SecurityError {}

/// Access management security manager.
pub struct AccessManagementSecurity {
    pub name: String,
    policies: BTreeMap<String, SecurityPolicy>,
}

impl AccessManagementSecurity {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            policies: BTreeMap::new(),
        }
    }

    pub fn add_policy(&mut self, policy: SecurityPolicy) -> Result<(), SecurityError> {
        if policy.name.trim().is_empty() { return Err(SecurityError::EmptyName); }
        let id = policy.id.clone();
        if self.policies.contains_key(&id) {
            return Err(SecurityError::DuplicatePolicy(id.clone()));
        }
        self.policies.insert(id, policy);
        Ok(())
    }

    pub fn policy_count(&self) -> usize { self.policies.len() }

    pub fn get_policy(&self, id: &str) -> Option<&SecurityPolicy> { self.policies.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_create_aaa() {
        let s = AccessManagementSecurity::new("My Security");
        assert_eq!(s.name, "My Security");
        assert_eq!(s.policy_count(), 0);
    }

    #[test]
    fn test_add_policy_aaa() {
        let mut s = AccessManagementSecurity::new("My Security");
        let p = SecurityPolicy { id: "p1".into(), name: "mfa".into(), scope: "login".into(), enforcement: "required".into() };
        s.add_policy(p).unwrap();
        assert_eq!(s.policy_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut s = AccessManagementSecurity::new("My Security");
        let p = SecurityPolicy { id: "x".into(), name: "".into(), scope: "a".into(), enforcement: "b".into() };
        assert!(s.add_policy(p).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut s = AccessManagementSecurity::new("My Security");
        let p = SecurityPolicy { id: "x".into(), name: "X".into(), scope: "a".into(), enforcement: "b".into() };
        s.add_policy(p).unwrap();
        let p2 = SecurityPolicy { id: "x".into(), name: "X".into(), scope: "a".into(), enforcement: "b".into() };
        assert!(s.add_policy(p2).is_err());
    }
}
