#![deny(missing_docs)]
#![feature(std_misc, old_path)]

//! A library for texture conventions.

extern crate image;

use std::error::FromError;
use std::fmt::{ Display, Formatter, Error };
use std::path::AsPath;
use image::{ RgbaImage, ImageError, DynamicImage };

/// Result of an texture creating/updating process.
pub type TexResult<T> = Result<T, TexError>;

/// An enumeration of Texture and TextureWithDevice errors.
#[derive(Debug, PartialEq, Clone)]
pub enum TexError {
    /// The image openinng error.
    Img(ImageError),

    /// The channels number error.
    Channels(usize),

    /// The error in backend.
    BackEnd(String)
}

impl FromError<ImageError> for TexError {
    fn from_error(e: ImageError) -> Self {
        TexError::Img(e)
    }
}

impl Display for TexError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            TexError::Img(ref e) => e.fmt(f),
            TexError::Channels(n) =>
                format!("Wrong number of channels: {}, support only 1 and 4.", n).fmt(f),
            TexError::BackEnd(ref s) => s.fmt(f)
        }
    }
}

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

/// Unified interface creating/updating texture over backends.
pub trait TextureWithDevice: ImageSize + Sized {
    /// Backend specific type.
    type Device;

    /// Create texture from path.
    #[inline(always)]
    fn from_path<P: AsPath>(device: &mut Self::Device, path: P) -> TexResult<Self> {
        let image = try!(image::open(&std::old_path::Path::new(path.as_path().to_str().unwrap())));
        let image = match image {
            DynamicImage::ImageRgba8(image) => image,
            image => image.to_rgba()
        };
        Self::from_image(device, &image)
    }

    /// Create texture from RGBA image.
    #[inline(always)]
    fn from_image(device: &mut Self::Device, image: &RgbaImage) -> TexResult<Self> {
        let width = image.width() as usize;
        Self::from_memory(device, image, width, 4)
    }

    /// Create texture from memory buffer. Supported only RGBA and alpha channels images.
    fn from_memory(device: &mut Self::Device,
                   memory: &[u8], width: usize, channels: usize) -> TexResult<Self>;

    /// Update texture from path.
    #[inline(always)]
    fn update_from_path<P: AsPath>(&mut self, device: &mut Self::Device, path: P) -> TexResult<()> {
        let image = try!(image::open(&std::old_path::Path::new(path.as_path().to_str().unwrap())));
        let image = match image {
            DynamicImage::ImageRgba8(image) => image,
            image => image.to_rgba()
        };
        self.update_from_image(device, &image)
    }

    /// Update texture from RGBA image.
    #[inline(always)]
    fn update_from_image(&mut self, device: &mut Self::Device, image: &RgbaImage) -> TexResult<()> {
        let width = image.width() as usize;
        self.update_from_memory(device, image, width, 4)
    }

    /// Update texture from memory buffer. Supported only RGBA and alpha channels images.
    fn update_from_memory(&mut self, device: &mut Self::Device,
                          memory: &[u8], width: usize, channels: usize) -> TexResult<()>;
}

/// Interface for device independent backends.
pub trait Texture: TextureWithDevice<Device = ()> + Sized {
    /// Create texture from path.
    #[inline(always)]
    fn from_path<P: AsPath>(path: P) -> TexResult<Self> {
        TextureWithDevice::from_path(&mut (), path)
    }

    /// Create texture from RGBA image.
    #[inline(always)]
    fn from_image(image: &RgbaImage) -> TexResult<Self> {
        TextureWithDevice::from_image(&mut (), image)
    }

    /// Create texture from memory buffer. Supported only RGBA and alpha channels images.
    #[inline(always)]
    fn from_memory(memory: &[u8], width: usize, channels: usize) -> TexResult<Self> {
        TextureWithDevice::from_memory(&mut (), memory, width, channels)
    }

    /// Update texture from path.
    #[inline(always)]
    fn update_from_path<P: AsPath>(&mut self, path: P) -> TexResult<()> {
        TextureWithDevice::update_from_path(self, &mut (), path)
    }

    /// Update texture from RGBA image.
    #[inline(always)]
    fn update_from_image(&mut self, image: &RgbaImage) -> TexResult<()> {
        TextureWithDevice::update_from_image(self, &mut (), image)
    }

    /// Update texture from memory buffer. Supported only RGBA and alpha channels images.
    #[inline(always)]
    fn update_from_memory(&mut self, memory: &[u8], width: usize, channels: usize) -> TexResult<()> {
        TextureWithDevice::update_from_memory(self, &mut (), memory, width, channels)
    }
}

impl<T: TextureWithDevice<Device = ()>> Texture for T {}
