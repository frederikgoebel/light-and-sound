use super::xyy;
use super::LinearSRGB;
use super::OkLab;
use super::XyY;
use super::F;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Xyz {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl From<XyY> for Xyz {
    fn from(value: XyY) -> Self {
        let x = (value.x * value.y2) / value.y;
        let y = value.y2;
        let z = ((1.0 - value.x - value.y) * value.y2) / value.y;
        Xyz { x, y, z }
    }
}

impl From<LinearSRGB> for Xyz {
    fn from(value: LinearSRGB) -> Self {
        // TODO why does philips hue apply gamma correction here?
        // let mut srgb = srgb;
        // srgb.r = if (srgb.r > 0.04045) {
        //     ((srgb.r + 0.055) / (1.0 + 0.055)).powf(2.4)
        // } else {
        //     (srgb.r / 12.92)
        // };
        // srgb.g = if (srgb.g > 0.04045) {
        //     ((srgb.g + 0.055) / (1.0 + 0.055)).powf(2.4)
        // } else {
        //     (srgb.g / 12.92)
        // };
        // srgb.b = if (srgb.b > 0.04045) {
        //     ((srgb.b + 0.055) / (1.0 + 0.055)).powf(2.4)
        // } else {
        //     (srgb.b / 12.92)
        // };

        let x = value.r * 0.4124 + value.g * 0.3576 + value.b * 0.1805;
        let y = value.r * 0.2126 + value.g * 0.7152 + value.b * 0.0722;
        let z = value.r * 0.0193 + value.g * 0.1192 + value.b * 0.9505;

        Xyz { x, y, z }
    }
}

impl From<OkLab> for Xyz {
    fn from(value: OkLab) -> Self {
        let mut l = 0.9999999984505198 * value.l
            + 0.396_337_792_173_767_86 * value.a
            + 0.215_803_758_060_758_8 * value.b;

        let mut m = 1.000_000_008_881_760_9 * value.l
            + -0.105_561_342_323_656_35 * value.a
            + -0.063_854_174_771_705_91 * value.b;

        let mut s = 1.000_000_054_672_410_8 * value.l
            + -0.089_484_182_094_965_75 * value.a
            + -1.291_485_537_864_091_7 * value.b;

        l = l * l * l;
        m = m * m * m;
        s = s * s * s;

        let x = 1.2268798733741557 * l + -0.5578149965554813 * m + 0.28139105017721583 * s;
        let y = -0.04057576262431372 * l + 1.1122868293970594 * m + -0.07171106666151701 * s;
        let z = -0.07637294974672142 * l + -0.4214933239627914 * m + 1.5869240244272418 * s;

        Xyz { x, y, z }
    }
}
