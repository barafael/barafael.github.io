use futures::{Sink, SinkExt};
use std::time::Duration;
use tokio::{select, sync::mpsc, time::interval};

const PING: u8 = 4;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Forwarder;

impl Forwarder {
    pub async fn event_loop<S>(self, mut rx: mpsc::Receiver<u8>, mut sink: S) -> Self
    where
        S: Sink<u8> + Unpin,
        S::Error: std::fmt::Debug,
    {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            // First: get the next message.
            let message = select! {
                message = rx.recv() => {
                    let Some(message) = message else {
                        break;
                    };
                    message
                }
                _ = interval.tick() => {
                    PING
                }
            };
            // Second: handle it.
            if let Err(error) = sink.send(message).await {
                tracing::warn!(?error, "Sink closed");
                break;
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;
    use tokio_util::sync::PollSender;

    #[tokio::test(start_paused = true)]
    async fn it_works() {
        // Pre-conditions
        let (tx, rx) = mpsc::channel(10);
        let (sink, mut stream_receiver) = mpsc::channel(10);
        let sink = PollSender::new(sink);

        // Action
        let forwarder = Forwarder::default();
        let forwarder_handle = tokio::spawn(forwarder.event_loop(rx, sink));

        let first_ping = stream_receiver.recv().await.unwrap();

        tx.send(1).await.unwrap();
        tx.send(2).await.unwrap();
        tx.send(3).await.unwrap();

        let one = stream_receiver.recv().await.unwrap();
        let two = stream_receiver.recv().await.unwrap();
        let three = stream_receiver.recv().await.unwrap();
        let four = stream_receiver.recv().await.unwrap();
        drop(tx);
        forwarder_handle.await.unwrap();

        // Post-conditions
        assert_eq!(first_ping, PING);
        assert_eq!((one, two, three, four), (1, 2, 3, PING));
        assert!(stream_receiver.is_closed());
        assert!(stream_receiver.is_empty());
    }
}
