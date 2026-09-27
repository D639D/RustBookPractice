enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
    Triangle(f64, f64, f64),
}

impl Shape {
    fn square(&self) -> Option<f64> {
        match self {
            Self::Circle(a) => {
                if self.validity()? {
                    Some(3.1415 * (a.powf(2.0)))
                } else {
                    panic!("Радиус должен быть больше 0!")
                }
            }

            Self::Rectangle { width, height } => {
                if self.validity()? {
                    Some(width * height)
                } else {
                    panic!("Сумма двух сторон прямоугольника должна быть больше 0!")
                }
            }

            Self::Triangle(a, b, c) => {
                if self.validity()? {
                    let p = (a + b + c) / 2.0;
                    Some((p * (p - a) * (p - b) * (p - c)).powf(0.5))
                } else {
                    panic!("Сумма любых двух сторон должна быть больше третьей")
                }
            }
        }
    }

    fn perimeter(&self) -> Option<f64> {
        match self {
            Self::Circle(a) => Some(2.0 * 3.1415 * a),

            Self::Rectangle { width, height } => Some(2.0 * (width + height)),

            Self::Triangle(a, b, c) => Some(a + b + c),
        }
    }

    fn validity(&self) -> Option<bool> {
        match self {
            Self::Circle(a) => {
                if *a > 0.0 {
                    Some(true)
                } else {
                    Some(false)
                }
            }

            Self::Rectangle { width, height } => {
                if (width + height) > 0.0 {
                    Some(true)
                } else {
                    Some(false)
                }
            }

            Self::Triangle(a, b, c) => {
                if (a + b > *c)
                    && (a + c > *b)
                    && (b + c > *a)
                    && (*a > 0.0 && *b > 0.0 && *c > 0.0)
                {
                    Some(true)
                } else {
                    Some(false)
                }
            }
        }
    }
}

fn title(figure: &Shape) -> &str {
    match figure {
        Shape::Circle(_) => "Круг",
        Shape::Rectangle { height: _, width: _ } => "Прямоугольник",
        Shape::Triangle(_a, _b, _c) => "Треугольник",
    }
}

// fn largest_area() -> Option<&Shape> {
    
// }

fn main() {
    let circle1 = Shape::Circle(5.0);
    let rectangle1 = Shape::Rectangle {
        width: 10.0,
        height: 15.0,
    };
    let triangle1 = Shape::Triangle(10.0, 15.0, 10.0);
    
    println!("\n1. Square: {:?}", Shape::square(&circle1));
    println!("2. Square: {:?}", Shape::square(&rectangle1));
    println!("3. Square: {:?}\n", Shape::square(&triangle1));
    
    println!("1. Perimeter: {:?}", Shape::perimeter(&triangle1));
    println!("2. Perimeter: {:?}", Shape::perimeter(&triangle1));
    println!("3. Perimeter: {:?}\n", Shape::perimeter(&triangle1));

    println!("1. Titel: {:?}", title(&Shape::Circle(10.0)));
    println!("2. Titel: {:?}", title(&Shape::Rectangle { width: 10.0, height: 10.0 }));
    println!("3. Titel: {:?}", title(&Shape::Triangle(2.0, 4.0, 2.0)));
}
