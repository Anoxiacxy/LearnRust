use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DiError {
    #[error("Service not found: {0}")]
    ServiceNotFound(String),
    #[error("Service already registered: {0}")]
    ServiceAlreadyRegistered(String),
}

pub type Result<T> = std::result::Result<T, DiError>;

pub struct Container {
    services: Arc<RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync>>>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register<T: 'static + Send + Sync>(&self, service: T) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let type_name = std::any::type_name::<T>();

        let mut services = self.services.write().unwrap();
        if services.contains_key(&type_id) {
            return Err(DiError::ServiceAlreadyRegistered(type_name.to_string()));
        }

        services.insert(type_id, Box::new(service));
        Ok(())
    }

    pub fn resolve<T: 'static + Clone>(&self) -> Result<T> {
        let type_id = TypeId::of::<T>();
        let type_name = std::any::type_name::<T>();

        let services = self.services.read().unwrap();
        let service = services
            .get(&type_id)
            .ok_or_else(|| DiError::ServiceNotFound(type_name.to_string()))?;

        service
            .downcast_ref::<T>()
            .map(|s| s.clone())
            .ok_or_else(|| DiError::ServiceNotFound(type_name.to_string()))
    }
}

impl Clone for Container {
    fn clone(&self) -> Self {
        Self {
            services: Arc::clone(&self.services),
        }
    }
}
