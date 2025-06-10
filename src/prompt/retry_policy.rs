pub struct RetryPolicy {
    max_retries: u32,
}

impl RetryPolicy {
    pub fn new(max_retries: u32) -> Self {
        RetryPolicy { max_retries }
    }

    pub fn retry<F, T, E>(&self, operation: F) -> impl Fn() -> Result<T, E>
    where
        F: Fn() -> Result<T, E> + Clone,
        E: From<String>,
    {
        move || {
            let mut attempts = 0;
            let base_delay_ms = 2000;
            let op = operation.clone();

            loop {
                match op() {
                    Ok(result) => return Ok(result),
                    Err(e) => {
                        attempts += 1;
                        if attempts > self.max_retries {
                            return Err(E::from(format!(
                                "Failed after {} retries: {}",
                                self.max_retries, e
                            )));
                        }

                        let max_delay = base_delay_ms * 2u64.pow(attempts - 1);
                        let jitter = rand::random::<u64>() % (max_delay / 4);
                        let delay = max_delay + jitter;

                        std::thread::sleep(std::time::Duration::from_millis(delay));
                        println!(
                            "Retrying operation (attempt {}/{}) after {} ms: {}",
                            attempts, self.max_retries, delay, e
                        );
                    }
                }
            }
        }
    }
}
