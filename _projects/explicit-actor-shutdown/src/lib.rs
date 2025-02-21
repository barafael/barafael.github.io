use futures::{Sink, SinkExt};
use std::time::Duration;
use tokio::{select, time::interval};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Beacon(u8);

impl Beacon {
    pub async fn event_loop<S>(mut self, token: CancellationToken, mut sink: S) -> Self
    where
        S: Sink<u8> + Unpin,
        S::Error: std::fmt::Debug,
    {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            // First: get the next message.
            let message = select! {
                _cancellation = token.cancelled() => {
                    break;
                }
                _ = interval.tick() => {
                    self.0
                }
            };
            // Second: handle it.
            if let Err(error) = sink.send(message).await {
                tracing::warn!(?error, "Sink closed");
                break;
            }
            self.0 += 1;
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
        let (sink, mut stream_receiver) = mpsc::channel(10);
        let sink = PollSender::new(sink);

        // Action
        let token = CancellationToken::new();
        let beacon = Beacon::default();
        let beacon_handle = tokio::spawn(beacon.event_loop(token.clone(), sink));

        for index in 0..100 {
            let tick = stream_receiver.recv().await.unwrap();
            assert_eq!(index, tick);
        }
        token.cancel();
        let beacon = beacon_handle.await.unwrap();

        // Post-conditions
        assert!(stream_receiver.is_closed());
        assert!(stream_receiver.is_empty());
        assert_eq!(beacon.0, 100);
    }
}
