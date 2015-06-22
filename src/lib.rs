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

/// An enumeration of Texture and TextureWithDevice errors.
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

/// Unified interface creating/updating texture over backends.
pub trait TextureWithFactory<F>: ImageSize + Sized {
    /// Create texture from memory buffer. Supported only RGBA and alpha channels images.
    fn from_memory(
        device: &mut F,
        memory: &[u8],
        width: u32,
        height: u32,
        channels: u8,
        settings: &TextureSettings
    ) -> TextureResult<Self>;

    /// Update texture from memory buffer. Supported only RGBA and alpha channels images.
    fn update_from_memory(
        &mut self,
        device: &mut F,
        memory: &[u8],
        width: u32,
        height: u32,
        channels: u8
    ) -> TextureResult<()>;
}

/// Interface for device independent backends.
pub trait Texture: TextureWithFactory<()> + Sized {
    /// Create texture from memory buffer. Supported only RGBA and alpha channels images.
    #[inline(always)]
    fn from_memory(
        memory: &[u8],
        width: u32,
        height: u32,
        channels: u8,
        settings: &TextureSettings
    ) -> TextureResult<Self> {
        TextureWithFactory::from_memory(&mut (), memory, width, height, channels, settings)
    }

    /// Update texture from memory buffer. Supports only RGBA and alpha channels images.
    #[inline(always)]
    fn update_from_memory(
        &mut self,
        memory: &[u8],
        width: u32,
        height: u32,
        channels: u8
    ) -> TextureResult<()> {
        TextureWithFactory::update_from_memory(
            self, &mut (), memory, width, height, channels)
    }
}

impl<T: TextureWithFactory<()>> Texture for T {}
