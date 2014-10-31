//! Create texture from an alpha channel image in memory.

/// Implemented by textures that can be constructed
/// from an alpha channel image in memory.
pub trait FromMemoryAlpha<D> {
    /// Creates texture.
    /// Closure `f` should be called after texture is created.
    fn from_memory_alpha(
        device: &mut D,
        buffer: &[u8], width: u32, height: u32,
        f: |&mut D, Self| -> Self
    ) -> Option<Self>;
}
