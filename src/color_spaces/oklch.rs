use super::OkLab;
use super::F;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct OkLch {
    pub l: F,
    pub c: F,
    pub h: F,
}
impl From<OkLab> for OkLch {
    fn from(value: OkLab) -> Self {
        let c = (value.a * value.a + value.b * value.b).sqrt();
        let h = value.b.atan2(value.a);
        OkLch {
            l: value.l,
            c,
            h: h.to_degrees() + 180.0,
        }
    }
}
