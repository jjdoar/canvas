pub mod render;

#[derive(Debug, Clone)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0);
        assert!(height > 0);

        Self {
            width,
            height,
            pixels: vec![0; width * height * 4],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    pub fn pixels_mut(&mut self) -> &mut [u8] {
        &mut self.pixels
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> &[u8; 4] {
        assert!(x < self.width);
        assert!(y < self.height);

        let index = (y * self.width + x) * 4;
        self.pixels[index..index + 4].try_into().unwrap()
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: &[u8; 4]) {
        assert!(x < self.width);
        assert!(y < self.height);

        let index = (y * self.width + x) * 4;
        self.pixels[index..index + 4].copy_from_slice(color);
    }

    pub fn clear(&mut self, color: &[u8; 4]) {
        for pixel in self.pixels.chunks_exact_mut(4) {
            pixel.copy_from_slice(color);
        }
    }
}
