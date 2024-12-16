
/// Models an object which can have an image created of it in the form of an array of bytes.
pub trait Imageable {
    /// Loads the given image.
    fn load_image(&mut self, image: Vec<u8>);

    /// Clears the currently loaded image.
    fn clear_image(&mut self);

    /// Saves the state of the object to a new image.
    fn save_image(&self) -> Vec<u8>;
}