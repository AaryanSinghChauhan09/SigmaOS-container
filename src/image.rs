//! Container image model.

#[derive(Debug, Clone)]
pub struct ContainerImage {
    pub id: String,
    pub repo: String,
    pub tag: String,
    pub size_mb: u32,
}

#[derive(Debug, Default)]
pub struct ImageManager {
    pub images: Vec<ContainerImage>,
}

impl ImageManager {
    pub fn new() -> Self { Self::default() }
    pub fn add_image(&mut self, img: ContainerImage) { self.images.push(img); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_manager() {
        let mut im = ImageManager::new();
        im.add_image(ContainerImage { id: "sha256:abc".into(), repo: "alpine".into(), tag: "latest".into(), size_mb: 5 });
        assert_eq!(im.images.len(), 1);
    }
}
