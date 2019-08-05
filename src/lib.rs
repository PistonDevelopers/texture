#![deny(missing_docs)]

//! A generic library for textures.
//!
//! This library is used in Piston for generic code when working with textures.
//!
//! The `ImageSize` trait is used for passing textures around for rendering.
//! For more information, see
//! [Piston-Graphics](https://github.com/pistondevelopers/graphics).

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
#[derive(Clone, Copy)]
pub struct TextureSettings {
    // Whether to convert gamma, treated as sRGB color space
    convert_gamma: bool,
    // Compress on GPU.
    compress: bool,
    // Generate mipmap chain.
    generate_mipmap: bool,
    // Filtering Mode for Minifying
    min: Filter,
    // Filtering Mode for Magnifying
    mag: Filter,
    // Filtering Mode for Minify Mipmapping
    mipmap: Filter
}

impl TextureSettings {
    /// Create default settings.
    pub fn new() -> TextureSettings {
        TextureSettings {
            convert_gamma: false,
            compress: false,
            generate_mipmap: false,
            min: Filter::Linear,
            mag: Filter::Linear,
            mipmap: Filter::Linear,
        }
    }

    /// Gets whether to convert gamma, treated as sRGB color space.
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

    /// Gets minify filter.
    pub fn get_min(&self) -> Filter { self.min }
    /// Sets minify filter.
    pub fn set_min(&mut self, val: Filter) {
        self.min = val
    }
    /// Sets minify filter.
    pub fn min(mut self, val: Filter) -> Self {
        self.set_min(val);
        self
    }

    /// Gets magnify filter
    pub fn get_mag(&self) -> Filter { self.mag }
    /// Sets magnify filter
    pub fn set_mag(&mut self, val: Filter) {
        self.mag = val;
    }
    /// Sets magnify filter
    pub fn mag(mut self, val: Filter) -> Self {
        self.set_mag(val);
        self
    }

    /// Gets minify mipmap filter
    pub fn get_mipmap(&self) -> Filter { self.mipmap }
    /// Sets magnify mipmap filter, and sets generate_mipmap to true.
    pub fn set_mipmap(&mut self, val: Filter) {
        self.set_generate_mipmap(true);
        self.mag = val;
    }
    /// Sets magnify mipmap filter, and sets generate_mipmap to true
    pub fn mipmap(mut self, val: Filter) -> Self {
        self.set_mag(val);
        self
    }

    /// Returns the min and mag filter
    pub fn get_filter(&self) -> (Filter, Filter) { (self.min, self.mag) }
    /// Sets the min and mag filter
    pub fn set_filter(&mut self, val: Filter) {
        self.set_min(val);
        self.set_mag(val);
    }

    /// Sets the min and mag filter
    pub fn filter(mut self, val: Filter) -> Self {
        self.set_filter(val);
        self
    }

}

/// Texture format.
#[derive(Copy, Clone, Debug)]
pub enum Format {
    /// `(red, green, blue, alpha)` with values 0-255.
    Rgba8,
}

/// Implemented by texture operations.
pub trait TextureOp<F> {
    /// The error when performing an operation.
    type Error;
}

/// Implemented by textures for creation.
pub trait CreateTexture<F>: TextureOp<F> + ImageSize + Sized {
    /// Create texture from memory.
    fn create<S: Into<[u32; 2]>>(
        factory: &mut F,
        format: Format,
        memory: &[u8],
        size: S,
        settings: &TextureSettings
    ) -> Result<Self, Self::Error>;
}

/// Implemented by textures for updating.
pub trait UpdateTexture<F>: TextureOp<F> + ImageSize + Sized {
    /// Update the texture.
    ///
    /// The `offset` and `size` arguments represent the position and dimensions of the sub-section
    /// of the texture that is to be updated with the given `memory`.
    fn update<O, S>(
        &mut self,
        factory: &mut F,
        format: Format,
        memory: &[u8],
        offset: O,
        size: S,
    ) -> Result<(), Self::Error>
        where O: Into<[u32; 2]>,
              S: Into<[u32; 2]>;
}

/// Sampling filter
#[derive(Copy, Clone, Debug)]
pub enum Filter {
    /// A Weighted Linear Blend
    Linear,
    /// Nearest Texel
    Nearest
}
