use std::{
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::Serialize;

use super::DHBWCourse;

#[derive(Debug, Serialize)]
pub struct PerformanceOverview {
    inner: RwLock<Inner>,
}

impl PerformanceOverview {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(Inner::default()),
        }
    }

    pub fn read(&self) -> RwLockReadGuard<Inner> {
        self.inner.read().unwrap()
    }

    pub fn write(&self) -> RwLockWriteGuard<Inner> {
        self.inner.write().unwrap()
    }
}

#[derive(Debug, Serialize)]
pub struct Inner {
    last_update: u64,
    courses: Vec<DHBWCourse>,
}

impl Default for Inner {
    fn default() -> Self {
        Self {
            courses: Default::default(),
            last_update: 0,
        }
    }
}

impl Inner {
    pub fn set_courses(&mut self, courses: Vec<DHBWCourse>) {
        self.last_update = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards!")
            .as_secs();
        self.courses = courses;
    }
}
