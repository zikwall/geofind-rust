struct Point {
    lat: f32,
    lng: f32,
}

impl Point {
    fn origin() -> Point {
        Point { lat: 0.0, lng: 0.0 }
    }

    fn new(x: f32, y: f32) -> Point {
        Point { lat: x, lng: y }
    }
}

struct Polygon {
    coordinates: Vec<Vec<f32>>
}

impl Polygon {
    fn include(&self, point: &Point) -> bool {
        let mut k_prev = 0;
        let mut result = false;

        for (k, p) in self.coordinates.iter().enumerate() {
            println!("{}, {:?})", k, p);
        }

        return true
    }
}

fn main () {
    let polygon = Polygon {
        coordinates: vec![vec![33.4, 55.3], vec![11.45, 56.34], vec![22.45, 33.34]]
    };

    let res = polygon.include(&Point{lat: 34.55, lng: 46.56});
    println!("{}", res);
}