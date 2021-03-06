extern crate types;

use types::geo::{Point, Polygon};

pub fn include() -> bool {
    let coords = vec![
        vec![-72.2802114, 42.9275749],
        vec![-72.2809839, 42.9266243],
        vec![-72.2817242, 42.9275749],
        vec![-72.2805333, 42.9249431],
        vec![-72.2792351, 42.9239925],
        vec![-72.276628, 42.9246996],
        vec![-72.2760272, 42.9255323],
        vec![-72.2767353, 42.9260351],
        vec![-72.2772717, 42.9268521],
        vec![-72.2758126, 42.927677],
        vec![-72.2762632, 42.9284311],
        vec![-72.2777116, 42.9287768],
        vec![-72.2785592, 42.928549],
        vec![-72.2793317, 42.9279755],
        vec![-72.2799325, 42.927622],
        vec![-72.2802114, 42.9275984],
    ];

    let mut polygon = Polygon::new();
    polygon.set_coordinates(coords);

    let res = polygon.include(&Point{lat: -72.278, lng: 42.925});

    return res
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_include() {
        assert_eq!(include(), true);
    }
}