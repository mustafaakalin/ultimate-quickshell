use async_trait::async_trait;
use std::{future::Future, time::Duration};
use tokio::{sync::watch, task::JoinHandle};
use tracing::{error, warn};

#[derive(Debug, Clone)]
pub struct ActorContext {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct RestartPolicy {
    pub max_restarts: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
}

impl Default for RestartPolicy {
    fn default() -> Self {
        Self {
            max_restarts: 5,
            base_delay: Duration::from_millis(250),
            max_delay: Duration::from_secs(10),
        }
    }
}

#[async_trait]
pub trait Actor: Send + Sync + 'static {
    async fn run(&mut self, context: ActorContext, shutdown: watch::Receiver<bool>) -> anyhow::Result<()>;
}

pub struct Supervisor {
    shutdown: watch::Sender<bool>,
    tasks: Vec<JoinHandle<()>>,
}

impl Supervisor {
    pub fn new() -> Self {
        let (shutdown, _) = watch::channel(false);
        Self { shutdown, tasks: Vec::new() }
    }

    pub fn spawn<A>(&mut self, name: impl Into<String>, mut actor: A, policy: RestartPolicy)
    where
        A: Actor,
    {
        let name = name.into();
        let context = ActorContext { name: name.clone() };
        let shutdown = self.shutdown.subscribe();

        let task = tokio::spawn(async move {
            let mut restart_count = 0u32;

            loop {
                let child_shutdown = shutdown.clone();
                let result = actor.run(context.clone(), child_shutdown).await;

                if *shutdown.borrow() {
                    break;
                }

                match result {
                    Ok(()) => {
                        warn!(actor = %name, "actor stopped unexpectedly");
                    }
                    Err(error) => {
                        error!(actor = %name, %error, "actor failed");
                    }
                }

                if restart_count >= policy.max_restarts {
                    error!(actor = %name, "restart budget exhausted");
                    break;
                }

                let exponent = restart_count.min(6);
                let factor = 1u32 << exponent;
                let delay = policy.base_delay.saturating_mul(factor).min(policy.max_delay);
                restart_count = restart_count.saturating_add(1);

                tokio::select! {
                    _ = tokio::time::sleep(delay) => {},
                    _ = wait_shutdown(shutdown.clone()) => break,
                }
            }
        });

        self.tasks.push(task);
    }

    pub async fn shutdown(self) {
        let _ = self.shutdown.send(true);
        for task in self.tasks {
            let _ = task.await;
        }
    }
}

async fn wait_shutdown(mut shutdown: watch::Receiver<bool>) {
    while !*shutdown.borrow() {
        if shutdown.changed().await.is_err() {
            break;
        }
    }
}
