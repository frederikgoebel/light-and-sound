use super::xyy::XyY;
use super::OkLab;
use super::F;
use super::SRGB;

#[derive(Clone)]
pub struct LinearSRGB {
    pub r: F,
    pub g: F,
    pub b: F,
}

impl From<XyY> for LinearSRGB {
    fn from(value: XyY) -> Self {
        // following https://developers.meethue.com/develop/
        // application-design-guidance/color-conversion-formulas-
        // rgb-to-xy-and-back/#xy-to-rgb-color
        let z = 1.0 - value.x - value.y;
        let y = value.y2;
        let x = (y / value.y) * value.x;
        let z = (y / value.y) * z;

        let r = x * 1.656492 - y * 0.354851 - z * 0.255038;
        let g = -x * 0.707196 + y * 1.655397 + z * 0.036152;
        let b = x * 0.051713 - y * 0.121364 + z * 1.01153;

        LinearSRGB { r, g, b }
    }
}

// TODO this is wrong,srgb uses a different gamma correction
impl From<SRGB> for LinearSRGB {
    fn from(srgb: SRGB) -> LinearSRGB {
        LinearSRGB {
            r: srgb.r.powf(2.2),
            g: srgb.g.powf(2.2),
            b: srgb.b.powf(2.2),
        }
    }
}

impl From<OkLab> for LinearSRGB {
    fn from(oklab: OkLab) -> LinearSRGB {
        let l_ = oklab.l + 0.3963377774 * oklab.a + 0.2158037573 * oklab.b;
        let m_ = oklab.l - 0.1055613458 * oklab.a - 0.0638541728 * oklab.b;
        let s_ = oklab.l - 0.0894841775 * oklab.a - 1.2914855480 * oklab.b;
        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;
        LinearSRGB {
            r: 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
            g: -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
            b: -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
        }
    }
}
