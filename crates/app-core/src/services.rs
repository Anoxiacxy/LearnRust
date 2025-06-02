use async_trait::async_trait;
use common::{CommonError, Result};
use serde::{Deserialize, Serialize};
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_user(&self, id: u64) -> Result<User>;
    async fn create_user(&self, name: String, email: String) -> Result<User>;
}

#[derive(Clone)]
pub struct UserServiceImpl {
    next_id: Arc<AtomicU64>,
}

impl UserServiceImpl {
    pub fn new() -> Self {
        Self {
            next_id: Arc::new(AtomicU64::new(1)),
        }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_user(&self, id: u64) -> Result<User> {
        // 模拟从数据库获取用户
        if id == 0 {
            return Err(CommonError::InvalidInput("Invalid user ID".to_string()));
        }

        Ok(User {
            id,
            name: format!("User {}", id),
            email: format!("user{}@example.com", id),
        })
    }

    async fn create_user(&self, name: String, email: String) -> Result<User> {
        let id = self
            .next_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Ok(User { id, name, email })
    }
}
