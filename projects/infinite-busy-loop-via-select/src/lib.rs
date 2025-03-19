use tokio::{select, sync::mpsc};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct BusyCollector(u32);

impl BusyCollector {
    pub async fn event_loop(
        mut self,
        mut rx1: mpsc::Receiver<u8>,
        mut rx2: mpsc::Receiver<u8>,
        mut rx3: mpsc::Receiver<u8>,
    ) -> Self {
        loop {
            tracing::info!("waiting for message");
            // First: get the next message.
            let message = select! {
                message = rx1.recv() => {
                    message
                }
                message = rx2.recv() => {
                    message
                }
                message = rx3.recv() => {
                    message
                }
            };
            // Second: handle it.
            tracing::info!(value = ?message, "got message");
            self.0 += 1;
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Collector;

impl Collector {
    pub async fn event_loop(
        self,
        mut rx1: mpsc::Receiver<u8>,
        mut rx2: mpsc::Receiver<u8>,
        mut rx3: mpsc::Receiver<u8>,
    ) -> Self {
        loop {
            tracing::info!("waiting for message");
            // First: get the next message.
            let message = select! {
                Some(message) = rx1.recv() => {
                    message
                }
                Some(message) = rx2.recv() => {
                    message
                }
                Some(message) = rx3.recv() => {
                    message
                }
                // Difference:
                else => { break }
            };
            // Second: handle it.
            tracing::info!(value = %message, "got message");
        }
        self
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn busy_collector_is_busy() {
        let (tx1, rx1) = mpsc::channel(1);
        let (tx2, rx2) = mpsc::channel(1);
        let (tx3, rx3) = mpsc::channel(1);

        let collector = BusyCollector::default();
        let collector_loop = collector.event_loop(rx1, rx2, rx3);

        tx1.send(1).await.unwrap();
        tx2.send(2).await.unwrap();
        tx3.send(3).await.unwrap();

        drop(tx1);
        drop(tx2);
        drop(tx3);

        let handle = tokio::spawn(collector_loop);
        tokio::time::sleep(Duration::from_secs(1)).await;
        handle.abort();
    }
}
