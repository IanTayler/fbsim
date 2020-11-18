use crate::engines::{basic::Basic, Engine};
use std::collections::BTreeMap;

pub struct EngineRegistry {
    pub engines: BTreeMap<String, Box<dyn Engine + Send + Sync>>,
}

impl EngineRegistry {
    pub fn new(engines: BTreeMap<String, Box<dyn Engine + Send + Sync>>) -> Self {
        EngineRegistry { engines: engines }
    }
    pub fn insert(&mut self, name: String, engine: Box<dyn Engine + Send + Sync>) {
        self.engines.insert(name, engine);
    }

    pub fn get(&mut self, name: String) -> Option<&mut Box<dyn Engine + Send + Sync>> {
        if let Some(boxed_engine) = self.engines.get_mut(&name) {
            Some(boxed_engine)
        } else {
            None
        }
    }
}
impl Default for EngineRegistry {
    fn default() -> Self {
        let mut registry =
            EngineRegistry::new(BTreeMap::<String, Box<dyn Engine + Send + Sync>>::new());
        registry.insert("basic".to_string(), Box::new(Basic::new()));
        registry
    }
}
