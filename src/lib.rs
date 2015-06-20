#![deny(missing_docs)]

//! A library for texture conventions.

extern crate image;

use std::fmt::{ Display, Formatter, Error };
use std::path::Path;
use image::{ RgbaImage, ImageError, DynamicImage };

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
    /// The image openinng error.
    ImageError(ImageError),

    /// The channels number error.
    InvalidNumberOfChannels(usize),

    /// The error in backend factory.
    FactoryError(String),
}

impl From<ImageError> for TextureError {
    fn from(e: ImageError) -> Self {
        TextureError::ImageError(e)
    }
}

impl Display for TextureError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            TextureError::ImageError(ref e) => e.fmt(f),
            TextureError::InvalidNumberOfChannels(n) =>
                format!("Invalid number of channels: {}, expected 1 or 4", n).fmt(f),
            TextureError::FactoryError(ref s) => s.fmt(f)
        }
    }
}

/// Unified interface creating/updating texture over backends.
pub trait TextureWithFactory<F>: ImageSize + Sized {

    /// Create texture from path.
    #[inline(always)]
    fn from_path<P: AsRef<Path>>(
        device: &mut F,
        path: P,
        settings: &TextureSettings
    ) -> TextureResult<Self> {
        let image = try!(image::open(path));
        let image = match image {
            DynamicImage::ImageRgba8(image) => image,
            image => image.to_rgba()
        };
        Self::from_image(device, &image, settings)
    }

    /// Create texture from RGBA image.
    #[inline(always)]
    fn from_image(
        device: &mut F,
        image: &RgbaImage,
        settings: &TextureSettings
    ) -> TextureResult<Self> {
        let width = image.width() as usize;
        Self::from_memory(device, image, width, 4, settings)
    }

    /// Create texture from memory buffer. Supported only RGBA and alpha channels images.
    fn from_memory(
        device: &mut F,
        memory: &[u8],
        width: usize,
        channels: usize,
        settings: &TextureSettings
    ) -> TextureResult<Self>;

    /// Update texture from path.
    #[inline(always)]
    fn update_from_path<P: AsRef<Path>>(
        &mut self,
        device: &mut F,
        path: P
    ) -> TextureResult<()> {
        let image = try!(image::open(path));
        let image = match image {
            DynamicImage::ImageRgba8(image) => image,
            image => image.to_rgba()
        };
        self.update_from_image(device, &image)
    }

    /// Update texture from RGBA image.
    #[inline(always)]
    fn update_from_image(
        &mut self,
        device: &mut F,
        image: &RgbaImage
    ) -> TextureResult<()> {
        let width = image.width() as usize;
        self.update_from_memory(device, image, width, 4)
    }

    /// Update texture from memory buffer. Supported only RGBA and alpha channels images.
    fn update_from_memory(
        &mut self,
        device: &mut F,
        memory: &[u8],
        width: usize,
        channels: usize
    ) -> TextureResult<()>;
}

/// Interface for device independent backends.
pub trait Texture: TextureWithFactory<()> + Sized {
    /// Create texture from path.
    #[inline(always)]
    fn from_path<P: AsRef<Path>>(
        path: P,
        settings: &TextureSettings
    ) -> TextureResult<Self> {
        TextureWithFactory::from_path(&mut (), path, settings)
    }

    /// Create texture from RGBA image.
    #[inline(always)]
    fn from_image(
        image: &RgbaImage,
        settings: &TextureSettings
    ) -> TextureResult<Self> {
        TextureWithFactory::from_image(&mut (), image, settings)
    }

    /// Create texture from memory buffer. Supported only RGBA and alpha channels images.
    #[inline(always)]
    fn from_memory(
        memory: &[u8],
        width: usize,
        channels: usize,
        settings: &TextureSettings
    ) -> TextureResult<Self> {
        TextureWithFactory::from_memory(&mut (), memory, width, channels, settings)
    }

    /// Update texture from path.
    #[inline(always)]
    fn update_from_path<P: AsRef<Path>>(&mut self, path: P) -> TextureResult<()> {
        TextureWithFactory::update_from_path(self, &mut (), path)
    }

    /// Update texture from RGBA image.
    #[inline(always)]
    fn update_from_image(&mut self, image: &RgbaImage) -> TextureResult<()> {
        TextureWithFactory::update_from_image(self, &mut (), image)
    }

    /// Update texture from memory buffer. Supports only RGBA and alpha channels images.
    #[inline(always)]
    fn update_from_memory(
        &mut self,
        memory: &[u8],
        width: usize,
        channels: usize
    ) -> TextureResult<()> {
        TextureWithFactory::update_from_memory(self, &mut (), memory, width, channels)
    }
}

impl<T: TextureWithFactory<()>> Texture for T {}
