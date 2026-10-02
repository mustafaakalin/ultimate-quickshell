//! Modular plugin runtime.
//!
//! Built-in modules run in-process for low latency/shared state.
//! Third-party or untrusted extensions should use the same manifest and
//! capability model through the stable IPC protocol, keeping the daemon's
//! trust boundary small.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use crate::{CapabilityRegistry, PolicyEngine, Principal};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginKind { BuiltIn, External }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub version: String,
    pub api_version: u16,
    pub kind: PluginKind,
    pub capabilities: Vec<String>,
    pub provides: Vec<String>,
    pub dependencies: Vec<String>,
}

impl PluginManifest {
    pub fn validate(&self) -> Result<(), PluginError> {
        if self.id.is_empty() || self.id.len() > 128 { return Err(PluginError::InvalidId); }
        if self.api_version == 0 { return Err(PluginError::UnsupportedApi); }
        if self.capabilities.iter().any(|c| c.is_empty()) { return Err(PluginError::InvalidCapability); }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("invalid plugin id")]
    InvalidId,
    #[error("unsupported plugin API version")]
    UnsupportedApi,
    #[error("invalid capability declaration")]
    InvalidCapability,
    #[error("plugin already registered: {0}")]
    AlreadyRegistered(String),
    #[error("plugin capability is not registered: {0}")]
    UnknownCapability(String),
    #[error("plugin capability denied: {0}")]
    CapabilityDenied(String),
}

#[async_trait]
pub trait BuiltInPlugin: Send + Sync {
    fn manifest(&self) -> &PluginManifest;
    async fn start(&self, _context: PluginContext) -> Result<(), PluginError> { Ok(()) }
    async fn stop(&self) -> Result<(), PluginError> { Ok(()) }
}

#[derive(Clone)]
pub struct PluginContext {
    pub policy: std::sync::Arc<PolicyEngine>,
    pub capabilities: std::sync::Arc<CapabilityRegistry>,
    pub principal: Principal,
}

impl PluginContext {
    pub fn authorize(&self, capability: &str) -> Result<(), PluginError> {
        self.capabilities.authorize(&self.policy, &self.principal, capability)
            .map_err(|_| PluginError::CapabilityDenied(capability.to_owned()))
    }
}

#[derive(Debug, Clone)]
pub struct PluginRecord {
    pub manifest: PluginManifest,
    pub enabled: bool,
}

#[derive(Default)]
pub struct PluginRegistry {
    records: BTreeMap<String, PluginRecord>,
    capability_index: BTreeMap<String, BTreeSet<String>>,
}

impl PluginRegistry {
    pub fn register(&mut self, manifest: PluginManifest, capabilities: &CapabilityRegistry) -> Result<(), PluginError> {
        manifest.validate()?;
        if self.records.contains_key(&manifest.id) { return Err(PluginError::AlreadyRegistered(manifest.id)); }

        for capability in &manifest.capabilities {
            if capabilities.effect_for(capability).is_none() {
                return Err(PluginError::UnknownCapability(capability.clone()));
            }
        }

        let id = manifest.id.clone();
        for capability in &manifest.capabilities {
            self.capability_index.entry(capability.clone()).or_default().insert(id.clone());
        }
        self.records.insert(id, PluginRecord { manifest, enabled: true });
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&PluginRecord> { self.records.get(id) }
    pub fn iter(&self) -> impl Iterator<Item = &PluginRecord> { self.records.values() }
    pub fn providers(&self, capability: &str) -> impl Iterator<Item = &String> {
        self.capability_index.get(capability).into_iter().flat_map(|ids| ids.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Effect;

    fn registry() -> CapabilityRegistry {
        let mut registry = CapabilityRegistry::default();
        registry.register("environment.snapshot", Effect::Read);
        registry.register("environment.control", Effect::Control);
        registry
    }

    #[test]
    fn rejects_unknown_capability() {
        let manifest = PluginManifest {
            id: "example".into(), version: "0.1.0".into(), api_version: 1,
            kind: PluginKind::BuiltIn, capabilities: vec!["missing".into()],
            provides: vec![], dependencies: vec![],
        };
        let error = PluginRegistry::default().register(manifest, &registry()).unwrap_err();
        assert!(matches!(error, PluginError::UnknownCapability(_)));
    }

    #[test]
    fn indexes_capability_provider() {
        let mut plugins = PluginRegistry::default();
        let manifest = PluginManifest {
            id: "snapshot-ui".into(), version: "0.1.0".into(), api_version: 1,
            kind: PluginKind::BuiltIn, capabilities: vec!["environment.snapshot".into()],
            provides: vec!["ui.surface".into()], dependencies: vec![],
        };
        plugins.register(manifest, &registry()).unwrap();
        assert_eq!(plugins.providers("environment.snapshot").count(), 1);
    }
}
