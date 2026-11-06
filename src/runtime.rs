//! Container runtime manager model.

use crate::container::{Container, ContainerState};

#[derive(Debug, Default)]
pub struct RuntimeManager {
    pub containers: Vec<Container>,
}

impl RuntimeManager {
    pub fn new() -> Self { Self::default() }
    pub fn create_container(&mut self, c: Container) { self.containers.push(c); }
    pub fn start_container(&mut self, id: &str) -> Result<(), String> {
        if let Some(c) = self.containers.iter_mut().find(|x| x.id == id) {
            c.state = ContainerState::Running;
            c.pid = Some(1234);
            Ok(())
        } else {
            Err("Container not found".into())
        }
    }
    pub fn stop_container(&mut self, id: &str) -> Result<(), String> {
        if let Some(c) = self.containers.iter_mut().find(|x| x.id == id) {
            c.state = ContainerState::Stopped;
            c.pid = None;
            Ok(())
        } else {
            Err("Container not found".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::container::ContainerConfig;

    #[test]
    fn runtime_manager() {
        let mut rm = RuntimeManager::new();
        rm.create_container(Container::new("123".into(), "test".into(), ContainerConfig {
            image: "alpine".into(),
            command: vec!["sh".into()],
            env: vec![],
            working_dir: "/".into(),
        }));
        assert!(rm.start_container("123").is_ok());
    }
}
