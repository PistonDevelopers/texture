#![deny(missing_docs)]

//! A library for texture conventions.
//!
//! Texture trait constructors should pass in a closure like this:
//!
//! ```ignore
//! f: |&mut D, Self| -> Self
//! ```
//!
//! This is a convention such that libraries that construct textures
//! and store them separately from the texture device,
//! lets the caller perform additional operations such as mipmap generation.

pub use image_size::ImageSize;
pub use from_memory_alpha::FromMemoryAlpha;

mod image_size;
mod from_memory_alpha;
