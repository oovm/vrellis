use vrellis_core::{VrellisPoint, VrellisShape};

#[test]
fn square() {
    let square = VrellisShape::Square;
    let out = vec![
        VrellisPoint { n: 0, x: 0, y: 0 },
        VrellisPoint { n: 1, x: 50, y: 0 },
        VrellisPoint { n: 2, x: 100, y: 0 },
        VrellisPoint { n: 3, x: 100, y: 50 },
        VrellisPoint { n: 4, x: 100, y: 100 },
        VrellisPoint { n: 5, x: 50, y: 100 },
        VrellisPoint { n: 6, x: 0, y: 100 },
        VrellisPoint { n: 7, x: 0, y: 50 },
    ];
    assert_eq!(out, square.sample(8, 100, 100))
}

#[test]
fn circle() {
    let circle = VrellisShape::Circle;
    let out = vec![
        VrellisPoint { n: 0, x: 100, y: 50 },
        VrellisPoint { n: 1, x: 85, y: 15 },
        VrellisPoint { n: 2, x: 50, y: 0 },
        VrellisPoint { n: 3, x: 15, y: 15 },
        VrellisPoint { n: 4, x: 0, y: 50 },
        VrellisPoint { n: 5, x: 15, y: 85 },
        VrellisPoint { n: 6, x: 50, y: 100 },
        VrellisPoint { n: 7, x: 85, y: 85 },
    ];
    assert_eq!(out, circle.sample(8, 100, 100))
}
