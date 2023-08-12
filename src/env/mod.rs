use std::{collections::HashSet, sync::Arc};

pub mod user;

pub trait Env: Send + Sync {
    fn is_target_domain(&self, v: &str) -> bool;
}

pub fn env(domains: Vec<String>) -> Arc<dyn Env> {
    Arc::new(EnvImpl {
        domains: HashSet::from_iter(domains),
    })
}

struct EnvImpl {
    domains: HashSet<String>,
}

impl Env for EnvImpl {
    fn is_target_domain(&self, v: &str) -> bool {
        self.domains.contains(v)
    }
}
