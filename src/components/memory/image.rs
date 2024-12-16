pub trait ImageStorage {
    fn load_image(&mut self, image: Vec<u8>);
    fn clear_image(&mut self);
    fn save_image(&self);
}