#[derive(Debug)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point2D {
    fn getX(&self) -> i32 {
        self.x
    }
}

impl Point3D {
    fn getX(&self) -> i32 {
        self.x
    }
}

pub fn run_tst_structs() {
    let p2d = Point2D { x: 3, y: 4 };

    let p3d = Point3D {
        x: 33,
        y: 44,
        z: 55,
    };

    println!("2d point {:?}", p2d);
    println!("3d point {:?}", p3d);
    println!("2d point X: {}", p2d.getX());
    println!("3d point X: {}", p3d.getX());
}
