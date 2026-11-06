//! Namespace model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceType { Mount, Uts, Ipc, Network, Pid, User, Cgroup }

#[derive(Debug, Clone)]
pub struct Namespace {
    pub nstype: NamespaceType,
    pub id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn namespace() {
        let n = Namespace { nstype: NamespaceType::Network, id: "net-1".into() };
        assert_eq!(n.nstype, NamespaceType::Network);
    }
}
