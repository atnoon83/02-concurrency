use anyhow::Result;
use std::{
    collections::HashMap,
    fmt::Display,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};

use anyhow::anyhow;

#[derive(Debug)]
pub struct AMapMetrics {
    pub data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AMapMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        let map = metric_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        AMapMetrics {
            data: Arc::new(map),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        let counter = self
            .data
            .get(key)
            .ok_or_else(|| anyhow!("value not found for key {}", key))?;
        counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

impl Display for AMapMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.iter().for_each(|(key, value)| {
            writeln!(f, "{}: {}", key, value.load(Ordering::Relaxed)).unwrap();
        });
        Ok(())
    }
}

impl Clone for AMapMetrics {
    fn clone(&self) -> Self {
        AMapMetrics {
            data: Arc::clone(&self.data),
        }
    }
}
