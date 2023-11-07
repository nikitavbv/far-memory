use {
    std::sync::Arc,
    crate::manager::ManagerClient,
    super::ReplacementPolicy,
};

pub struct TrackingReplacementPolicy {
    inner: Box<dyn ReplacementPolicy>,
    manager: Arc<ManagerClient>,
}

impl TrackingReplacementPolicy {
    pub fn new(manager: Arc<ManagerClient>, inner: Box<dyn ReplacementPolicy>) -> Self {
        Self {
            manager,
            inner,
        }
    }
}

// TODO: impl ReplacementPolicy
