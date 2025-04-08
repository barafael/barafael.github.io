use tokio::sync::{mpsc, oneshot};

#[derive(Debug, PartialEq, Eq)]
pub struct UniqueIdService {
    next_id: u32,
}

pub enum Message {
    GetUniqueId { callback: oneshot::Sender<u32> },
}

impl UniqueIdService {
    pub fn new() -> Self {
        UniqueIdService { next_id: 0 }
    }

    pub async fn event_loop(mut self, mut rx: mpsc::Receiver<Message>) -> Self {
        while let Some(message) = rx.recv().await {
            self.handle_message(message);
        }
        self
    }

    fn handle_message(&mut self, message: Message) {
        match message {
            Message::GetUniqueId {
                callback: respond_to,
            } => {
                // Ignore failure to send on a `oneshot::Sender`.
                //
                // When the `oneshot::Receiver` on the other side has ceased to be, sending fails.
                // This may happen if the `select!` macro polls that receiver, but another branch succeeds first
                // (canceling the `oneshot::Receiver`).
                //
                // In simple words: "We tried sending the response, but hey, they weren't listening anymore. IDC".
                respond_to.send(self.next_id).ok();

                // Note the original example arguably has a bug here.
                // It never returns 0 as the ID, because it increments first.
                // Of course, that could be a feature.
                self.next_id += 1;
            }
        }
    }

    /// Query for a unique id.
    ///
    /// Returns `None` if the actor is not running.
    pub async fn get_unique_id(sender: &mpsc::Sender<Message>) -> Option<oneshot::Receiver<u32>> {
        let (callback, callback_receiver) = oneshot::channel();
        let message = Message::GetUniqueId { callback };

        sender.send(message).await.ok()?;
        Some(callback_receiver)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // marker-start:unittest_uidservice
    #[tokio::test]
    async fn should_increment_unique_id() {
        let actor = UniqueIdService::new();
        let (tx, rx) = mpsc::channel(3);

        let resp1 = UniqueIdService::get_unique_id(&tx).await.unwrap();
        let resp2 = UniqueIdService::get_unique_id(&tx).await.unwrap();
        let resp3 = UniqueIdService::get_unique_id(&tx).await.unwrap();

        // Important for this test:
        drop(tx);

        let service = actor.event_loop(rx).await;

        let nums = tokio::try_join!(resp1, resp2, resp3).unwrap();
        assert_eq!(nums, (0, 1, 2));
        assert_eq!(service, UniqueIdService { next_id: 3 });
    }
    // marker-end:unittest_uidservice
}
