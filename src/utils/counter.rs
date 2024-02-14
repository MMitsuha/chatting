use std::sync::Arc;

use tokio::{sync::RwLock, time, time::Duration};

pub struct Counter {
    count: Arc<RwLock<u16>>,
    limit: u16,
}

impl Counter {
    pub fn new(limit: u16) -> Self {
        let count = Arc::new(RwLock::new(0));

        {
            let count = count.clone();

            tokio::spawn(async move {
                loop {
                    time::sleep(Duration::from_secs(60)).await;
                    *count.write().await = 0;
                }
            });
        }

        Counter { count, limit }
    }

    pub async fn add(&self) {
        *self.count.write().await += 1;
    }

    pub async fn check(&self) -> bool {
        *self.count.read().await < self.limit
    }
}
