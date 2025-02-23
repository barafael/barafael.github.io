use std::{collections::HashMap, pin::Pin};

use futures::{StreamExt, stream::FuturesUnordered};
use tokio::{
    select,
    sync::{mpsc, oneshot},
};
use tracing::warn;

/// An actor message for the [`Cache`].
#[derive(Debug)]
pub enum Message {
    /// A message to get a value from the cache.
    Get(String, oneshot::Sender<Option<String>>),

    /// A message to set a value in the cache.
    Set(String, String),

    /// A clear signal to remove all values from the cache.
    Clear,

    Process(u32),
}

enum TaskStatus {
    Completed,
    Failed,
}

enum Event {
    Message(Message),
    TaskStatus(u32, TaskStatus),
}

/// A simple cache actor.
#[derive(Debug, Default)]
pub struct Cache {
    cache: HashMap<String, String>,
    tasks: FuturesUnordered<Pin<Box<dyn Future<Output = (u32, TaskStatus)> + Send + 'static>>>,
}

impl Cache {
    pub async fn event_loop(mut self, mut rx: mpsc::Receiver<Message>) -> Self {
        loop {
            // Collect some event.
            let event = select! {
                message = rx.recv() => {
                    let Some(message) = message else {
                        break;
                    };
                    Event::Message(message)
                }
                Some((id, status)) = self.tasks.next() => {
                    Event::TaskStatus(id, status)
                }
            };

            // Process the event.
            if let Err(error) = self.on_event(event) {
                warn!(?error, "Error processing event");
            }
        }

        // Await completion of remaining tasks.
        while let Some((id, status)) = self.tasks.next().await {
            if let Err(error) = self.on_task_status(id, status) {
                warn!(?error, "Error processing task status");
            }
        }

        self
    }

    /// Process an event.
    ///
    /// Let's pretend this can fail.
    fn on_event(&mut self, event: Event) -> Result<(), anyhow::Error> {
        match event {
            Event::Message(message) => self.on_message(message)?,
            Event::TaskStatus(id, task_status) => self.on_task_status(id, task_status)?,
        }
        Ok(())
    }

    /// Let's pretend this can fail.
    fn on_message(&mut self, message: Message) -> Result<(), anyhow::Error> {
        match message {
            Message::Get(key, tx) => {
                let _ = tx.send(self.cache.get(&key).cloned());
            }
            Message::Set(key, value) => {
                self.cache.insert(key, value);
            }
            Message::Clear => {
                self.cache.clear();
            }
            Message::Process(id) => {
                self.start_task(id);
            }
        }
        Ok(())
    }

    /// Report task completion.
    ///
    /// Let's pretend this can fail.
    fn on_task_status(&self, id: u32, task_status: TaskStatus) -> Result<(), anyhow::Error> {
        match task_status {
            TaskStatus::Completed => println!("Task {id} completed"),
            TaskStatus::Failed => println!("Task {id} failed"),
        }
        Ok(())
    }

    fn start_task(&mut self, id: u32) {
        let work = async move {
            let handle = tokio::task::spawn_blocking(move || {
                // Coincidentally, the task at `id` takes `id` seconds to complete.
                std::thread::sleep(std::time::Duration::from_secs(id as u64));
                if id % 2 == 0 {
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("Oddly, task failed"))
                }
            });
            let result = match handle.await {
                Ok(Ok(())) => TaskStatus::Completed,
                _ => TaskStatus::Failed,
            };
            (id, result)
        };
        self.tasks.push(Box::pin(work));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn basic_cache() {
        let (tx, rx) = mpsc::channel(32);
        let cache = Cache::default();
        let cache = tokio::spawn(cache.event_loop(rx));

        let (tx1, rx1) = oneshot::channel();
        tx.send(Message::Set("key".to_string(), "value".to_string()))
            .await
            .unwrap();
        tx.send(Message::Get("key".to_string(), tx1)).await.unwrap();
        assert_eq!(rx1.await.unwrap(), Some("value".to_string()));

        let (tx2, rx2) = oneshot::channel();
        tx.send(Message::Clear).await.unwrap();
        tx.send(Message::Get("key".to_string(), tx2)).await.unwrap();
        assert!(rx2.await.unwrap().is_none());

        drop(tx);

        let cache = cache.await.unwrap();
        assert!(cache.cache.is_empty());
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn process_tasks() {
        let (tx, rx) = mpsc::channel(32);
        let cache = Cache::default();
        let cache = tokio::spawn(cache.event_loop(rx));

        tx.send(Message::Process(1)).await.unwrap();
        tx.send(Message::Process(2)).await.unwrap();
        tx.send(Message::Process(3)).await.unwrap();
        tx.send(Message::Process(4)).await.unwrap();

        drop(tx);
        let cache = cache.await.unwrap();
        assert!(cache.cache.is_empty());
    }
}
