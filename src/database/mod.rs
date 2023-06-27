use crate::models::Class;
use std::collections::HashMap;

pub struct AppState {
    pub database: Database,
}

pub enum Database {
    Mock(Vec<Class>),
    HashMap(HashMap<u32, Class>),
}

impl Database {
    pub fn get_classes(&self) -> Vec<Class> {
        match self {
            Self::Mock(classes) => classes.clone(),
            Self::HashMap(classes) => classes.values().cloned().collect(),
        }
    }

    pub fn add_class(&mut self, class: Class) {
        match self {
            Self::Mock(classes) => classes.push(class),
            Self::HashMap(classes) => {
                classes.insert(class.id(), class);
            }
        }
    }
}
