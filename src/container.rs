//! Container model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerState { Created, Running, Paused, Stopped, Deleting }

#[derive(Debug, Clone)]
pub struct ContainerConfig {
    pub image: String,
    pub command: Vec<String>,
    pub env: Vec<String>,
    pub working_dir: String,
}

#[derive(Debug, Clone)]
pub struct Container {
    pub id: String,
    pub name: String,
    pub config: ContainerConfig,
    pub state: ContainerState,
    pub pid: Option<u32>,
}

impl Container {
    pub fn new(id: String, name: String, config: ContainerConfig) -> Self {
        Self { id, name, config, state: ContainerState::Created, pid: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_lifecycle() {
        let mut c = Container::new("123".into(), "test".into(), ContainerConfig {
            image: "alpine".into(),
            command: vec!["sh".into()],
            env: vec![],
            working_dir: "/".into(),
        });
        c.state = ContainerState::Running;
        assert_eq!(c.state, ContainerState::Running);
    }
}
