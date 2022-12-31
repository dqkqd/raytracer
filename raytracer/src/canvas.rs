use std::{
    fs::File,
    io::{BufWriter, Write},
    ops::{Deref, DerefMut},
};

use crate::color::{self, Color, MAX_COLOR};

const LINE_LENGTH: usize = 70;

pub(crate) struct Canvas {
    width: usize,
    height: usize,
    canvas: Vec<Vec<Color>>,
}

#[allow(dead_code)]
impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let canvas = vec![vec![color::BLACK; width]; height];
        Canvas {
            width,
            height,
            canvas,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn color(&self, x: usize, y: usize) -> Option<&Color> {
        self.canvas.get(y)?.get(x)
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: &Color) {
        if let Some(row) = self.canvas.get_mut(y) {
            if let Some(c) = row.get_mut(x) {
                *c = *color;
            }
        }
    }

    pub fn write_ppm(&self, filename: &str) -> std::io::Result<()> {
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);

        let header = format!("P3\n{} {}\n{}\n", self.width, self.height, MAX_COLOR);
        writer.write_all(header.as_bytes())?;

        for colors in &self.canvas {
            let mut chunk_size = 0;
            let mut chunks = vec![];

            for color in colors {
                let (r, g, b) = color.to_u8();
                for c in [r, g, b] {
                    let s = c.to_string();
                    if chunk_size + s.len() + chunks.len() >= LINE_LENGTH && !chunks.is_empty() {
                        writeln!(writer, "{}", &chunks.join(" "))?;
                        chunks.clear();
                        chunk_size = 0;
                    }
                    chunk_size += s.len();
                    chunks.push(s);
                }
            }
            if !chunks.is_empty() {
                writeln!(writer, "{}", &chunks.join(" "))?;
            }
        }

        Ok(())
    }
}

impl Deref for Canvas {
    type Target = Vec<Vec<Color>>;

    fn deref(&self) -> &Self::Target {
        &self.canvas
    }
}

impl DerefMut for Canvas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.canvas
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn default_canvas_has_color_black() {
        let width = 10;
        let height = 20;
        let canvas = Canvas::new(width, height);
        assert_eq!(canvas.width(), width);
        assert_eq!(canvas.height(), height);
        for x in 0..10 {
            for y in 0..20 {
                assert_eq!(canvas.color(x, y).unwrap(), &color::BLACK);
            }
        }
    }

    #[test]
    fn write_pixel_to_canvas() {
        let width = 10;
        let height = 20;
        let mut canvas = Canvas::new(width, height);
        let red = Color::new(1.0, 0.0, 0.0);
        canvas.write_pixel(2, 3, &red);
        assert_eq!(canvas.color(2, 3).unwrap(), &red);
    }

    struct TestConfig {
        filename: String,
    }
    impl TestConfig {
        pub fn new() -> TestConfig {
            let file = NamedTempFile::new().expect("Could not create temporary file");
            let filename = file
                .path()
                .to_str()
                .expect("Path could not convert to str")
                .to_string();
            file.close().expect("Error closing filename");
            TestConfig { filename }
        }
        pub fn filename(&self) -> &str {
            &self.filename
        }
    }
    impl Drop for TestConfig {
        fn drop(&mut self) {
            fs::remove_file(&self.filename).expect("Error removing file");
        }
    }

    #[test]
    fn write_ppm_content() -> std::io::Result<()> {
        let testconfig = TestConfig::new();
        let filename = testconfig.filename();

        let width = 5;
        let height = 3;
        let mut canvas = Canvas::new(width, height);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        canvas.write_pixel(0, 0, &c1);
        canvas.write_pixel(2, 1, &c2);
        canvas.write_pixel(4, 2, &c3);
        canvas.write_ppm(filename)?;

        let contents = fs::read_to_string(filename)?;
        let expected = "P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
";

        assert_eq!(contents, expected);

        Ok(())
    }

    #[test]
    fn write_ppm_line_less_than_70_chars() -> std::io::Result<()> {
        let testconfig = TestConfig::new();
        let filename = testconfig.filename();

        let width = 10;
        let height = 2;
        let mut canvas = Canvas::new(width, height);
        let color = Color::new(1.0, 0.8, 0.6);
        for x in 0..width {
            for y in 0..height {
                canvas.write_pixel(x, y, &color);
            }
        }
        canvas.write_ppm(filename)?;

        let contents = fs::read_to_string(filename)?;
        let expected = "P3
10 2
255
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
";

        assert_eq!(contents, expected);

        Ok(())
    }

    #[test]
    fn write_ppm_terminated_by_newline() -> std::io::Result<()> {
        let testconfig = TestConfig::new();
        let filename = testconfig.filename();

        let width = 20;
        let height = 50;
        let canvas = Canvas::new(width, height);
        canvas.write_ppm(filename)?;

        let contents = fs::read_to_string(filename)?;
        let terminated_char = contents.chars().last().unwrap();
        assert_eq!(terminated_char, '\n');
        Ok(())
    }
}
