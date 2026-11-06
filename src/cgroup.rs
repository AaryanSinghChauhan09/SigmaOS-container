//! Cgroup model.

#[derive(Debug, Clone)]
pub struct CgroupLimits {
    pub memory_mb: u32,
    pub cpu_shares: u32,
    pub pids: u32,
}

#[derive(Debug, Clone)]
pub struct Cgroup {
    pub name: String,
    pub limits: CgroupLimits,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cgroup() {
        let c = Cgroup { name: "test".into(), limits: CgroupLimits { memory_mb: 512, cpu_shares: 1024, pids: 100 } };
        assert_eq!(c.limits.memory_mb, 512);
    }
}
