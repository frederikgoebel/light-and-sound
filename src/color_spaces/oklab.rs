use super::xyz::Xyz;
use super::LinearSRGB;
use super::OkLch;
use super::XyY;
use super::F;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OkLab {
    pub l: F,
    pub a: F,
    pub b: F,
}

impl From<Xyz> for OkLab {
    fn from(value: Xyz) -> Self {
        let mut l = value.x * 0.8189330101 + value.y * 0.3618667424 + value.z * -0.1288597137;
        let mut m = value.x * 0.0329845436 + value.y * 0.9293118715 + value.z * 0.0361456387;
        let mut s = value.x * 0.0482003018 + value.y * 0.2643662691 + value.z * 0.6338517070;

        l = l.cbrt();
        m = m.cbrt();
        s = s.cbrt();

        let ll = l * 0.2104542553 + m * 0.7936177850 + s * -0.0040720468;
        let a = l * 1.9779984951 + m * -2.4285922050 + s * 0.4505937099;
        let b = l * 0.0259040371 + m * 0.7827717662 + s * -0.8086757660;

        OkLab { l: ll, a, b }
    }
}

impl From<XyY> for OkLab {
    fn from(value: XyY) -> Self {
        let xyy: Xyz = value.into();
        xyy.into()
    }
}

impl From<LinearSRGB> for OkLab {
    fn from(linear_srgb: LinearSRGB) -> OkLab {
        let l = 0.4122214708 * linear_srgb.r
            + 0.5363325363 * linear_srgb.g
            + 0.0514459929 * linear_srgb.b;
        let m = 0.2119034982 * linear_srgb.r
            + 0.6806995451 * linear_srgb.g
            + 0.1073969566 * linear_srgb.b;
        let s = 0.0883024619 * linear_srgb.r
            + 0.2817188376 * linear_srgb.g
            + 0.6299787005 * linear_srgb.b;
        let l_ = l.cbrt();
        let m_ = m.cbrt();
        let s_ = s.cbrt();
        OkLab {
            l: 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
            a: 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
            b: 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_,
        }
    }
}

impl From<OkLch> for OkLab {
    fn from(value: OkLch) -> Self {
        let h = (value.h - 180.0).to_radians();
        let a = value.c * h.cos();
        let b = value.c * h.sin();
        OkLab { l: value.l, a, b }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn oklab_to_xyz() {
        let test_values = [
            (
                OkLab {
                    l: 0.450,
                    a: 1.236,
                    b: -0.019,
                },
                Xyz {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
            ),
            (
                OkLab {
                    l: 1.0,
                    a: 0.0,
                    b: 0.0,
                },
                Xyz {
                    x: 0.950,
                    y: 1.000,
                    z: 1.089,
                },
            ),
            (
                OkLab {
                    l: 0.922,
                    a: -0.671,
                    b: 0.263,
                },
                Xyz {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            ),
            (
                OkLab {
                    l: 0.153,
                    a: -1.415,
                    b: -0.449,
                },
                Xyz {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
            ),
        ];

        for (oklab, xyz) in test_values {
            let mut xyz_calculated: Xyz = oklab.into();
            xyz_calculated.x = (xyz_calculated.x * 1000.0).round() / 1000.0;
            xyz_calculated.y = (xyz_calculated.y * 1000.0).round() / 1000.0;
            xyz_calculated.z = (xyz_calculated.z * 1000.0).round() / 1000.0;
            assert_eq!(xyz_calculated, xyz);
        }
    }

    #[test]
    fn xyz_to_oklab() {
        let test_values = [
            (
                OkLab {
                    l: 1.0,
                    a: 0.0,
                    b: 0.0,
                },
                Xyz {
                    x: 0.950,
                    y: 1.000,
                    z: 1.089,
                },
            ),
            (
                OkLab {
                    l: 0.450,
                    a: 1.236,
                    b: -0.019,
                },
                Xyz {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
            ),
            (
                OkLab {
                    l: 0.922,
                    a: -0.671,
                    b: 0.263,
                },
                Xyz {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            ),
            (
                OkLab {
                    l: 0.153,
                    a: -1.415,
                    b: -0.449,
                },
                Xyz {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
            ),
        ];

        for (oklab, xyz) in test_values {
            let mut oklab_calculated: OkLab = xyz.into();
            oklab_calculated.l = (oklab_calculated.l * 1000.0).round() / 1000.0;
            oklab_calculated.a = (oklab_calculated.a * 1000.0).round() / 1000.0;
            oklab_calculated.b = (oklab_calculated.b * 1000.0).round() / 1000.0;
            assert_eq!(oklab_calculated, oklab);
        }
    }
}
