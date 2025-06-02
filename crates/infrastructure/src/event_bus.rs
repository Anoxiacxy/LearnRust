use async_trait::async_trait;
use domain::events::{DomainEvent, EventPublisher, EventSubscriber, EventType};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};

/// 内存事件总线实现
pub struct InMemoryEventBus {
    subscribers: Arc<Mutex<HashMap<EventType, Vec<mpsc::Sender<DomainEvent>>>>>,
}

impl InMemoryEventBus {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub async fn subscribe_handler<F>(
        &self,
        event_type: EventType,
        handler: F,
    ) -> anyhow::Result<()>
    where
        F: Fn(DomainEvent) + Send + 'static,
    {
        let (tx, mut rx) = mpsc::channel(100);
        
        // 注册订阅者
        let mut subs = self.subscribers.lock().await;
        subs.entry(event_type).or_insert_with(Vec::new).push(tx);
        
        // 启动处理任务
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                handler(event);
            }
        });
        
        Ok(())
    }
}

#[async_trait]
impl EventPublisher for InMemoryEventBus {
    async fn publish(&self, event: DomainEvent) -> anyhow::Result<()> {
        let subs = self.subscribers.lock().await;
        
        if let Some(handlers) = subs.get(&event.event_type) {
            for tx in handlers {
                let _ = tx.send(event.clone()).await;
            }
        }
        
        Ok(())
    }
} 