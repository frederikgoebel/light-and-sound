use super::LinearSRGB;
use super::OkLab;
use super::F;

#[derive(Default, Debug)]
pub struct SRGB {
    pub r: F,
    pub g: F,
    pub b: F,
}

impl SRGB {
    pub fn new(r: F, g: F, b: F) -> SRGB {
        SRGB { r, g, b }
    }
}

impl From<LinearSRGB> for SRGB {
    fn from(linear_srgb: LinearSRGB) -> SRGB {
        // TODO wrong gamma correction for SRGB
        return SRGB {
            r: linear_srgb.r.powf(1.0 / 2.2),
            g: linear_srgb.g.powf(1.0 / 2.2),
            b: linear_srgb.b.powf(1.0 / 2.2),
        };
    }
}

impl From<OkLab> for SRGB {
    fn from(oklab: OkLab) -> SRGB {
        let linear_srgb: LinearSRGB = oklab.into();
        let srgb: SRGB = linear_srgb.into();
        srgb
    }
}
