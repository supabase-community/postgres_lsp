pub(crate) struct SimpleTokioDebouncer<Args> {
    handle: tokio::task::JoinHandle<()>,
    tx: tokio::sync::mpsc::Sender<Args>,
}

impl<Args> SimpleTokioDebouncer<Args> {
    pub fn new<F>(timeout: std::time::Duration, mut callback: F) -> Self
    where
        F: FnMut(Args) + Send + 'static,
        Args: Send + 'static,
    {
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        let handle = tokio::spawn(async move {
            let mut maybe_args: Option<Args> = None;
            let mut instant = tokio::time::Instant::now() + timeout;

            loop {
                tokio::select! {
                    // If the timeout is reached, execute and reset the last received action
                    _ = tokio::time::sleep_until(instant) => {
                        match maybe_args {
                            Some(args) => {
                                callback(args);
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

    pub async fn debounce(&self, args: Args)
    {
        self.tx.send(args).await.unwrap();
    }

    pub async fn shutdown(self) {
        let _ = self.handle.await.unwrap();
    }
}
