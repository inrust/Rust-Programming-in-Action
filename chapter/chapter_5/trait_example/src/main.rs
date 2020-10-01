use std::fmt::{Display, Formatter, Result};

trait Geometry {
    fn area(&self) -> f32;

    fn perimeter(&self) -> f32;
}

#[derive(Clone, Copy)]
struct Rectangle {
    width: f32,
    height: f32,
}

impl Geometry for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn perimeter(&self) -> f32 {
        (self.width + self.height) * 2.0
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Rectangle: ({}, {})", self.width, self.height)
    }
}

#[derive(Clone, Copy)]
struct Circle {
    radius: f32,
}

impl Geometry for Circle {
    fn area(&self) -> f32 {
        3.14 * self.radius * self.radius
    }

    fn perimeter(&self) -> f32 {
        3.14 * 2.0 * self.radius
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Circle: ({})", self.radius)
    }
}

fn main() {
    let rect = Rectangle { width: 10.5, height: 5.5 };
    print_by_impl_trait(rect);

    let circle = Circle { radius: 3.0 };
    print_by_trait_bound(circle);

    area_add_by_impl_trait(rect, circle);
    area_add_by_trait_bound(rect, circle);

    let geometry = return_geometry();
    // area_add_by_impl_trait(geometry, circle);
    area_add_by_impl_trait(geometry, rect);
}

fn return_geometry() -> impl Geometry {
    Rectangle {
        width: 12.5,
        height: 5.5,
    }
}

fn print_by_impl_trait(geometry: impl Geometry + Display) {
    println!("{}, area: {}, perimeter: {}",
             geometry, geometry.area(), geometry.perimeter());
}

fn print_by_trait_bound<T: Geometry + Display>(geometry: T) {
    println!("{}, area: {}, perimeter: {}",
             geometry, geometry.area(), geometry.perimeter());
}

fn area_add_by_impl_trait(geo1: impl Geometry, geo2: impl Geometry) {
    println!("geo1.area: {}, geo2.area: {}, total area: {}",
             geo1.area(), geo2.area(), geo1.area() + geo2.area());
}

fn area_add_by_trait_bound<T: Geometry, U: Geometry>(geo1: T, geo2: U) {
    println!("geo1.area: {}, geo2.area: {}, total area: {}",
             geo1.area(), geo2.area(), geo1.area() + geo2.area());
}
