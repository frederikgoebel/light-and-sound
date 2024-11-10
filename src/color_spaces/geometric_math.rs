use super::F;
#[derive(Copy, Clone)]
pub struct Point {
    pub x: F,
    pub y: F,
}

impl Point {
    pub fn distance(&self, p2: &Point) -> F {
        let dx = self.x - p2.x;
        let dy = self.y - p2.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Returns the squared distance
    pub fn distance2(&self, p2: &Point) -> F {
        let dx = self.x - p2.x;
        let dy = self.y - p2.y;
        dx * dx + dy * dy
    }

    pub fn cross_product(&self, p2: &Point) -> F {
        self.x * p2.y - self.y * p2.x
    }
}

#[derive(Copy, Clone)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

impl Line {
    pub fn closest_point(&self, point: &Point) -> Point {
        let ap = Point {
            x: point.x - self.a.x,
            y: point.y - self.a.y,
        };
        let ab = Point {
            x: point.x - self.b.x,
            y: point.y - self.b.y,
        };

        let ab2 = ab.x * ab.x + ab.y * ab.y;
        let ap_ab = ap.x * ab.x + ap.y * ab.y;

        let mut t = ap_ab / ab2;
        t = t.clamp(0.0, 1.0);

        Point {
            x: self.a.x + ab.x * t,
            y: self.a.y + ab.y * t,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn contains(&self, point: &Point) -> bool {
        let v1 = Point {
            x: self.b.x - self.a.x,
            y: self.b.y - self.a.y,
        };
        let v2 = Point {
            x: self.c.x - self.a.x,
            y: self.c.y - self.a.y,
        };
        let q = Point {
            x: point.x - self.a.x,
            y: point.y - self.a.y,
        };

        let s = q.cross_product(&v2) / v1.cross_product(&v2);
        let t = v1.cross_product(&q) / v1.cross_product(&v2);

        (s >= 0.0) && (t >= 0.0) && (s + t <= 1.0)
    }

    pub fn closest_point(&self, point: &Point) -> Point {
        let a = &Line {
            a: self.a,
            b: self.b,
        }
        .closest_point(point);
        let b = Line {
            a: self.b,
            b: self.c,
        }
        .closest_point(point);
        let c = Line {
            a: self.c,
            b: self.a,
        }
        .closest_point(point);

        let ad = point.distance(a);
        let bd = point.distance(a);
        let cd = point.distance(a);

        let mut lowest = ad;
        let mut closest = a;

        if bd < lowest {
            lowest = bd;
            closest = &b;
        }
        if cd < lowest {
            closest = &c;
        }

        *closest
    }
}
