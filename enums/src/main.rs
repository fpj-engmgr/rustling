#[derive(Debug)]
enum Shape {
    Circle(f64), // radius
    Rectangle(f64, f64), // width, height
    Triangle(f64, f64, f64) // sides a, b, c
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        match *self {
            Shape::Circle(r)            => 2.0 * 3.14159 * r,
            Shape::Rectangle(w, h)      => 2.0 * (w + h),
            Shape::Triangle(a, b, c)    => (a + b + c)
        }
    }
}
fn main() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_shape is {:?}", my_shape);

    match my_shape {
        Shape::Circle(r) => println!("Circle with radios {}", r),
        Shape::Rectangle(w, h) => println!("{} x {} Rectangle", w, h),
        Shape::Triangle(a, b, c) => println!("Triangle with sides {}, {} and {}", a, b, c)
    }

    let perimeter = my_shape.get_perimeter();
    println!("Perimeter is {}", perimeter);
}