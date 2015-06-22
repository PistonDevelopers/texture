#![deny(missing_docs)]

//! A library for texture conventions.

use std::fmt::{ Display, Formatter, Error };

pub mod ops;

/// Implemented by all images to be used with generic algorithms.
pub trait ImageSize {
    /// Get the image size.
    fn get_size(&self) -> (u32, u32);

    /// Gets the image width.
    #[inline(always)]
    fn get_width(&self) -> u32 {
        let (w, _) = self.get_size();
        w
    }

    /// Gets the image height.
    #[inline(always)]
    fn get_height(&self) -> u32 {
        let (_, h) = self.get_size();
        h
    }
}

/// Texture creation parameters.
pub struct TextureSettings {
    flip_vertical: bool,
    convert_gamma: bool,
    /// Compress on GPU.
    compress: bool,
    /// Generate mipmap chain.
    generate_mipmap: bool,
}

impl TextureSettings {
    /// Create default settings.
    pub fn new() -> TextureSettings {
        TextureSettings {
            flip_vertical: false,
            convert_gamma: false,
            compress: false,
            generate_mipmap: false,
        }
    }

    /// Gets whether to flip vertical.
    pub fn get_flip_vertical(&self) -> bool { self.flip_vertical }
    /// Sets flip vertical.
    pub fn set_flip_vertical(&mut self, val: bool) { self.flip_vertical = val; }
    /// Sets flip vertical.
    pub fn flip_vertical(mut self, val: bool) -> Self {
        self.set_flip_vertical(val);
        self
    }

    /// Gets wheter to convert gamma, treated as sRGB color space.
    pub fn get_convert_gamma(&self) -> bool { self.convert_gamma }
    /// Sets convert gamma.
    pub fn set_convert_gamma(&mut self, val: bool) { self.convert_gamma = val; }
    /// Sets convert gamma.
    pub fn convert_gamma(mut self, val: bool) -> Self {
        self.set_convert_gamma(val);
        self
    }

    /// Gets wheter compress on the GPU.
    pub fn get_compress(&self) -> bool { self.compress }
    /// Sets compress.
    pub fn set_compress(&mut self, val: bool) { self.compress = val; }
    /// Sets compress.
    pub fn compress(mut self, val: bool) -> Self {
        self.set_compress(val);
        self
    }

    /// Gets generate mipmap.
    pub fn get_generate_mipmap(&self) -> bool { self.generate_mipmap }
    /// Sets generate mipmap.
    pub fn set_generate_mipmap(&mut self, val: bool) {
        self.generate_mipmap = val;
    }
    /// Sets generate mipmap.
    pub fn generate_mipmap(mut self, val: bool) -> Self {
        self.set_generate_mipmap(val);
        self
    }
}

/// Result of an texture creating/updating process.
pub type TextureResult<T> = Result<T, TextureError>;

/// Texture errors.
#[derive(Debug)]
pub enum TextureError {
    /// The error in backend factory.
    FactoryError(String),
}

impl Display for TextureError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            TextureError::FactoryError(ref s) => s.fmt(f)
        }
    }
}

/// Implemented by RGBA8 textures.
pub trait Rgba8Texture<F>: ImageSize {
    /// Create RGBA8 texture from memory.
    fn from_memory<S: Into<[u32; 2]>>(
        factory: &mut F,
        memory: &[u8],
        size: S,
        settings: &TextureSettings
    ) -> TextureResult<Self>;

    /// Update RGBA8 texture.
    fn update<S: Into<[u32; 2]>>(
        &mut self,
        factory: &mut F,
        memory: &[u8],
        size: S,
    ) -> TextureResult<()>;
}
