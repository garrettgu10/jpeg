#[derive(Debug, PartialEq)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn ycbcr(&self) -> YCbCr {
        let r: f64 = self.r as f64;
        let g: f64 = self.g as f64;
        let b: f64 = self.b as f64;
        YCbCr {
            y: (0.299 * r + 0.587 * g + 0.114 * b) as u8,
            cb: (-0.1687 * r - 0.3313 * g + 0.5 * b + 128.0) as u8,
            cr: (0.5 * r - 0.4187 * g - 0.0813 * b + 128.0) as u8,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct YCbCr {
    pub y: u8,
    pub cb: u8,
    pub cr: u8,
}

impl YCbCr {
    pub fn rgb(&self) -> RGB {
        let y = self.y as f64;
        let cb = self.cb as f64;
        let cr = self.cr as f64;
        RGB {
            r: (y + 1.402 * (cr - 128.0)) as u8,
            g: (y - 0.34414 * (cb - 128.0) - 0.71414 * (cr - 128.0)) as u8,
            b: (y + 1.772 * (cb - 128.0)) as u8,
        }
    }
}