use std::{
    future::Future,
    pin::Pin,
    sync::{atomic::AtomicBool, Arc},
};

type AsyncBlock = Pin<Box<dyn Future<Output = ()> + 'static + Send>>;

pub(crate) struct SimpleTokioDebouncer {
    handle: tokio::task::JoinHandle<()>,
    tx: tokio::sync::mpsc::Sender<AsyncBlock>,
    shutdown_flag: Arc<AtomicBool>,
}

impl SimpleTokioDebouncer {
    pub fn new(timeout: std::time::Duration) -> Self {
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        let shutdown_flag = Arc::new(AtomicBool::new(false));
        let shutdown_flag_clone = shutdown_flag.clone();

        let handle = tokio::spawn(async move {
            let mut maybe_args: Option<AsyncBlock> = None;
            let mut instant = tokio::time::Instant::now() + timeout;

            loop {
                if shutdown_flag_clone.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }

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

        Self {
            handle,
            tx,
            shutdown_flag,
        }
    }

    #[tracing::instrument(name = "Adding task to debouncer", skip(self, block))]
    pub async fn debounce(&self, block: AsyncBlock) {
        if self
            .shutdown_flag
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            tracing::error!(
                "Trying to debounce tasks, but the Debouncer is in the process of shutting down."
            );
            return;
        }

        self.tx.send(block).await.unwrap();
    }

    #[tracing::instrument(name = "Shutting down debouncer", skip(self))]
    pub async fn shutdown(&self) {
        self.shutdown_flag
            .store(true, std::sync::atomic::Ordering::Relaxed);

        let _ = self.handle.abort(); //  we don't care about any errors during shutdown
    }
}
