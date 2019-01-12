use super::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use super::Vec3;

    #[test]
    fn test_at() {
        let ray = Ray {
            origin: Vec3::new(1.0, 1.0, 1.0),
            direction: Vec3::new(1.0, 2.0, 3.0),
        };
        assert_eq!(ray.at(3.0), Vec3::new(4.0, 7.0, 10.0));
    }
}