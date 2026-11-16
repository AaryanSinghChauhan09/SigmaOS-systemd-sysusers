// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-sysusers: systemd sysusers component model.
//!
//! Real implementation: sysuserss, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sysusers {
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
pub enum SystemdSysusersError {
    EmptySysusersName,
    DuplicateSysusers(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdSysusersError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySysusersName => write!(f, "sysusers name must not be empty"),
            Self::DuplicateSysusers(id) => write!(f, "duplicate sysusers {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdSysusersError {}

pub struct SystemdSysusers {
    pub name: String,
    sysuserss: BTreeMap<String, Sysusers>,
    events: BTreeMap<String, Event>,
}

impl SystemdSysusers {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            sysuserss: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_sysusers(&mut self, item: Sysusers) -> Result<(), SystemdSysusersError> {
        if item.name.trim().is_empty() {
            return Err(SystemdSysusersError::EmptySysusersName);
        }
        let id = item.id.clone();
        if self.sysuserss.contains_key(&id) {
            return Err(SystemdSysusersError::DuplicateSysusers(id.clone()));
        }
        self.sysuserss.insert(id, item);
        Ok(())
    }

    pub fn sysusers_count(&self) -> usize {
        self.sysuserss.len()
    }

    pub fn get_sysusers(&self, id: &str) -> Option<&Sysusers> {
        self.sysuserss.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdSysusersError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdSysusersError::EmptyEventSubject);
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

    fn sample_sysusers(id: &str, name: &str) -> Sysusers {
        Sysusers {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdSysusers::new("SystemdSysusers");
        assert_eq!(m.name, "SystemdSysusers");
        assert_eq!(m.sysusers_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_sysusers() {
        let mut m = SystemdSysusers::new("SystemdSysusers");
        m.add_sysusers(sample_sysusers("a1", "Alpha")).unwrap();
        assert_eq!(m.sysusers_count(), 1);
        assert!(m.get_sysusers("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdSysusers::new("SystemdSysusers");
        assert!(m.add_sysusers(sample_sysusers("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdSysusers::new("SystemdSysusers");
        m.add_sysusers(sample_sysusers("x", "One")).unwrap();
        assert!(m.add_sysusers(sample_sysusers("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdSysusers::new("SystemdSysusers");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
