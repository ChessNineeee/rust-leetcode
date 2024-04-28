pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
    let point0 = points.get(0).unwrap();
    let point1 = points.get(1).unwrap();
    let point2 = points.get(2).unwrap();

    let v1 = (point1[0] - point0[0], point1[1] - point0[1]);
    let v2 = (point2[0] - point0[0], point2[1] - point0[1]);

    v1.0 * v2.1 - v1.1 * v2.0 != 0
}

fn main() {
    println!("{}", is_boomerang(vec![vec![1, 0]]))
}
