//! Image operations for textures.

/// Flips the image vertically.
pub fn flip_vertical(memory: &[u8], size: [u32; 2], channels: u8) -> Vec<u8> {
    let (width, height, channels) = (size[0] as usize, size[1] as usize,
        channels as usize);
    let mut res = vec![0; width * height];
    let stride = width * channels;
    for y in (0..height) {
        for x in (0..width) {
            for c in (0..channels) {
                res[(c + x * channels + (height - 1 - y) * stride) as usize] =
                    memory[(c + x * channels + y * stride) as usize];
            }
        }
    }
    res
}
