// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-graffi: graffi component model.
//!
//! Real implementation: graffi_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraffiItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraffiError {
    EmptyGraffiItemName,
    DuplicateGraffiItem(String),
    EmptyEventSubject,
}

impl fmt::Display for GraffiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyGraffiItemName => write!(f, "graffi_item name must not be empty"),
            Self::DuplicateGraffiItem(id) => write!(f, "duplicate graffi_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for GraffiError {}

pub struct Graffi {
    pub name: String,
    graffi_items: BTreeMap<String, GraffiItem>,
    events: BTreeMap<String, Event>,
}

impl Graffi {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            graffi_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_graffi_item(&mut self, item: GraffiItem) -> Result<(), GraffiError> {
        if item.name.trim().is_empty() {
            return Err(GraffiError::EmptyGraffiItemName);
        }
        let id = item.id.clone();
        if self.graffi_items.contains_key(&id) {
            return Err(GraffiError::DuplicateGraffiItem(id.clone()));
        }
        self.graffi_items.insert(id, item);
        Ok(())
    }

    pub fn graffi_item_count(&self) -> usize {
        self.graffi_items.len()
    }

    pub fn get_graffi_item(&self, id: &str) -> Option<&GraffiItem> {
        self.graffi_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), GraffiError> {
        if event.subject.trim().is_empty() {
            return Err(GraffiError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_graffi_item(id: &str, name: &str) -> GraffiItem {
        GraffiItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Graffi::new("Graffi");
        assert_eq!(m.name, "Graffi");
        assert_eq!(m.graffi_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_graffi_item() {
        let mut m = Graffi::new("Graffi");
        m.add_graffi_item(sample_graffi_item("a1", "Alpha")).unwrap();
        assert_eq!(m.graffi_item_count(), 1);
        assert!(m.get_graffi_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Graffi::new("Graffi");
        assert!(m.add_graffi_item(sample_graffi_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Graffi::new("Graffi");
        m.add_graffi_item(sample_graffi_item("x", "One")).unwrap();
        assert!(m.add_graffi_item(sample_graffi_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Graffi::new("Graffi");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
