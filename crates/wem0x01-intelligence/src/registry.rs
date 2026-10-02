use crate::IntelligenceError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AssetId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRevision {
    pub revision: u64,
    pub digest: Option<String>,
}

#[derive(Debug, Default)]
pub struct AssetRegistry<T> {
    assets: BTreeMap<AssetId, T>,
    revisions: BTreeMap<AssetId, AssetRevision>,
}

impl<T> AssetRegistry<T> {
    pub fn insert(&mut self, id: AssetId, value: T) -> Result<(), IntelligenceError> {
        if self.assets.contains_key(&id) {
            return Err(IntelligenceError::AssetExists(id.0));
        }
        self.revisions.insert(
            id.clone(),
            AssetRevision {
                revision: 1,
                digest: None,
            },
        );
        self.assets.insert(id, value);
        Ok(())
    }

    pub fn get(&self, id: &AssetId) -> Option<&T> {
        self.assets.get(id)
    }
    pub fn revision(&self, id: &AssetId) -> Option<&AssetRevision> {
        self.revisions.get(id)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&AssetId, &T)> {
        self.assets.iter()
    }
}
