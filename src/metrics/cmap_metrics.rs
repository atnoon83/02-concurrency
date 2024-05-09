use anyhow::Result;
use dashmap::DashMap;
use std::fmt::Display;
use std::sync::Arc;

pub struct CMapMetrics {
    pub data: Arc<DashMap<String, i64>>,
}

impl CMapMetrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }
}

impl Display for CMapMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.iter().for_each(|item| {
            writeln!(f, "{}: {}", item.key(), item.value()).unwrap();
        });
        Ok(())
    }
}

impl Clone for CMapMetrics {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
        }
    }
}

impl Default for CMapMetrics {
    fn default() -> Self {
        Self::new()
    }
}
