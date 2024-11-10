use super::linear_srgb::LinearSRGB;
use super::oklab::OkLab;
use super::xyz::Xyz;
use super::F;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XyY {
    pub x: F,
    pub y: F,
    pub y2: F,
}

impl From<Xyz> for XyY {
    fn from(value: Xyz) -> Self {
        let brightness = value.x + value.y + value.z;
        let (mut x, mut y) = (0.313, 0.329); // D65 white point as fallback
        if brightness != 0.0 {
            x = value.x / brightness;
            y = value.y / brightness;
        }

        XyY { x, y, y2: value.y }
    }
}

impl From<OkLab> for XyY {
    fn from(value: OkLab) -> Self {
        let xyz: Xyz = value.into();
        xyz.into()
    }
}

impl From<LinearSRGB> for XyY {
    fn from(value: LinearSRGB) -> Self {
        let xyz: Xyz = value.into();
        xyz.into()
    }
}
