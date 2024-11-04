use std::{future::Future, pin::Pin};

type AsyncBlock = Pin<Box<dyn Future<Output = ()> + 'static + Send>>;

pub(crate) struct SimpleTokioDebouncer {
    handle: tokio::task::JoinHandle<()>,
    tx: tokio::sync::mpsc::Sender<AsyncBlock>,
}

impl SimpleTokioDebouncer {
    pub fn new(timeout: std::time::Duration) -> Self {
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        let handle = tokio::spawn(async move {
            let mut maybe_args: Option<AsyncBlock> = None;
            let mut instant = tokio::time::Instant::now() + timeout;

            loop {
                tokio::select! {
                    // If the timeout is reached, execute and reset the last received action
                    _ = tokio::time::sleep_until(instant) => {
                        match maybe_args {
                            Some(block) => {
                                block.await;
                                maybe_args = None;
                            }
                            None => continue,
                        }
                    }

                    // If a new action is received, update the action and reset the timeout
                    cb = rx.recv() => {
                        match cb {
                            Some(cb) => {
                                maybe_args = Some(cb);
                                instant = tokio::time::Instant::now() + timeout;
                            }
                            None => break, // channel closed
                        }
                    }
                }
            }
        });

        Self { handle, tx }
    }

    pub async fn debounce(&self, block: AsyncBlock) {
        self.tx.send(block).await.unwrap();
    }

    pub async fn shutdown(self) {
        let _ = self.handle.await.unwrap();
    }
}
