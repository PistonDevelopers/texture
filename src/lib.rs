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
    FactoryError(String)
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
pub trait TextureWithFactory: ImageSize + Sized {
    /// Texture factory.
    type Factory;

    /// Create texture from path.
    #[inline(always)]
    fn from_path<P: AsRef<Path>>(
        device: &mut Self::Factory,
        path: P
    ) -> TextureResult<Self> {
        let image = try!(image::open(path));
        let image = match image {
            DynamicImage::ImageRgba8(image) => image,
            image => image.to_rgba()
        };
        Self::from_image(device, &image)
    }

    /// Create texture from RGBA image.
    #[inline(always)]
    fn from_image(
        device: &mut Self::Factory,
        image: &RgbaImage
    ) -> TextureResult<Self> {
        let width = image.width() as usize;
        Self::from_memory(device, image, width, 4)
    }

    /// Create texture from memory buffer. Supported only RGBA and alpha channels images.
    fn from_memory(
        device: &mut Self::Factory,
        memory: &[u8],
        width: usize,
        channels: usize
    ) -> TextureResult<Self>;

    /// Update texture from path.
    #[inline(always)]
    fn update_from_path<P: AsRef<Path>>(
        &mut self,
        device: &mut Self::Factory,
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
        device: &mut Self::Factory,
        image: &RgbaImage
    ) -> TextureResult<()> {
        let width = image.width() as usize;
        self.update_from_memory(device, image, width, 4)
    }

    /// Update texture from memory buffer. Supported only RGBA and alpha channels images.
    fn update_from_memory(
        &mut self,
        device: &mut Self::Factory,
        memory: &[u8],
        width: usize,
        channels: usize
    ) -> TextureResult<()>;
}

/// Interface for device independent backends.
pub trait Texture: TextureWithFactory<Factory = ()> + Sized {
    /// Create texture from path.
    #[inline(always)]
    fn from_path<P: AsRef<Path>>(path: P) -> TextureResult<Self> {
        TextureWithFactory::from_path(&mut (), path)
    }

    /// Create texture from RGBA image.
    #[inline(always)]
    fn from_image(image: &RgbaImage) -> TextureResult<Self> {
        TextureWithFactory::from_image(&mut (), image)
    }

    /// Create texture from memory buffer. Supported only RGBA and alpha channels images.
    #[inline(always)]
    fn from_memory(
        memory: &[u8],
        width: usize,
        channels: usize
    ) -> TextureResult<Self> {
        TextureWithFactory::from_memory(&mut (), memory, width, channels)
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

impl<T: TextureWithFactory<Factory = ()>> Texture for T {}
