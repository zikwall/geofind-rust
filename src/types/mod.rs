pub mod geo {
    pub struct Point {
        pub lat: f32,
        pub lng: f32,
    }

    pub struct Polygon {
        coordinates: Vec<Vec<f32>>
    }

    impl Polygon {
        pub fn new() -> Polygon {
            return Polygon {coordinates: vec![]}
        }

        pub fn set_coordinates(&mut self, coordinates: Vec<Vec<f32>>) {
            self.coordinates = coordinates;
        }

        pub fn include(&self, point: &Point) -> bool {
            let mut k_prev = 0;
            let mut result = false;

            for (k, p) in self.coordinates.iter().enumerate() {
                if k <= 0 {
                    k_prev = self.coordinates.len() - 1
                } else {
                    k_prev = k - 1
                }

                let iflng = p[1] < point.lng && self.coordinates[k_prev][1] >= point.lng || self.coordinates[k_prev][1] < point.lng && p[1] >= point.lng;
                let iflat = p[0] <= point.lat || self.coordinates[k_prev][0] <= point.lat;

                if iflng && iflat {
                    if p[0] + (point.lng - p[1]) / (self.coordinates[k_prev][1] - p[1]) * (self.coordinates[k_prev][0] - p[0]) < point.lat {
                        result = !result
                    }
                }
            }

            return result
        }
    }
}