use std::f64::consts::PI;

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
    Triangle(f64, f64, f64),
}

impl Shape {
    fn square(&self) -> f64 {
        match self {
            Self::Circle(a) => {
                if self.validity() {
                    PI * (a.powi(2))
                } else {
                    panic!("Радиус должен быть больше 0!")
                }
            }

            Self::Rectangle { width, height } => {
                if self.validity() {
                    width * height
                } else {
                    panic!("Сумма каждой из сторон должна быть больше 0!")
                }
            }

            Self::Triangle(a, b, c) => {
                if self.validity() {
                    let p = (a + b + c) / 2.0;
                    (p * (p - a) * (p - b) * (p - c)).powf(0.5)
                } else {
                    panic!("Сумма любых двух сторон должна быть больше третьей")
                }
            }
        }
    }

    fn perimeter(&self) -> f64 {
        match self {
            Self::Circle(a) => 2.0 * PI * a,

            Self::Rectangle { width, height } => 2.0 * (width + height),

            Self::Triangle(a, b, c) => a + b + c,
        }
    }

    fn validity(&self) -> bool {
        match self {
            Self::Circle(a) => {
                if *a > 0.0 {
                    true
                } else {
                    false
                }
            }

            Self::Rectangle { width, height } => {
                if *width > 0.0 && *height > 0.0 {
                    true
                } else {
                    false
                }
            }

            Self::Triangle(a, b, c) => {
                if (a + b > *c)
                    && (a + c > *b)
                    && (b + c > *a)
                    && (*a > 0.0 && *b > 0.0 && *c > 0.0)
                {
                    true
                } else {
                    false
                }
            }
        }
    }
}

fn title(figure: &Shape) -> &'static str {
    match figure {
        Shape::Circle(..) => "Круг",
        Shape::Rectangle { .. } => "Прямоугольник",
        Shape::Triangle(..) => "Треугольник",
    }
}

fn largest_area(figures: &[Shape]) -> Option<&Shape> {
    if figures.is_empty() {
        None
    } else {
        let mut tmp = &figures[0];
        for item in figures {
            if item.square() > tmp.square() {
                tmp = item;
            }
        }
        Some(tmp)
    }
}

fn main() {
    let circle1 = Shape::Circle(5.0);
    let rectangle1 = Shape::Rectangle {
        width: 10.0,
        height: 15.0,
    };
    let triangle1 = Shape::Triangle(10.0, 15.0, 10.0);

    println!("\n1. Square: {:?}", circle1.square());
    println!("2. Square: {:?}", rectangle1.square());
    println!("3. Square: {:?}\n", triangle1.square());

    println!("1. Perimeter: {:?}", circle1.perimeter());
    println!("2. Perimeter: {:?}", rectangle1.perimeter());
    println!("3. Perimeter: {:?}\n", triangle1.perimeter());

    println!("1. Title: {:?}", title(&Shape::Circle(10.0)));
    println!(
        "2. Title: {:?}",
        title(&Shape::Rectangle {
            width: 10.0,
            height: 10.0
        })
    );
    println!("3. Title: {:?}\n", title(&Shape::Triangle(2.0, 4.0, 2.0)));

    println!(
        "1. Winner: {:?}",
        largest_area(&[
            Shape::Circle(7.5),
            Shape::Rectangle {
                width: 12.3,
                height: 4.8
            },
            Shape::Triangle(5.0, 6.0, 7.0),
            Shape::Circle(3.2),
            Shape::Rectangle {
                width: 9.1,
                height: 9.1
            }
        ])
    );

    println!(
        "2. Winner: {:?}",
        largest_area(&[
            Shape::Triangle(3.0, 4.0, 5.0),
            Shape::Circle(8.8),
            Shape::Rectangle {
                width: 2.5,
                height: 14.0
            },
            Shape::Triangle(7.5, 8.2, 10.0),
            Shape::Circle(5.5),
            Shape::Rectangle {
                width: 11.0,
                height: 6.4
            }
        ])
    );

    println!(
        "3. Winner: {:?}\n",
        largest_area(&[
            Shape::Circle(1.1),
            Shape::Rectangle {
                width: 20.0,
                height: 1.5
            },
            Shape::Triangle(6.1, 6.1, 6.1),
            Shape::Circle(10.0),
            Shape::Rectangle {
                width: 4.4,
                height: 4.4
            },
            Shape::Triangle(9.0, 12.0, 15.0),
            Shape::Circle(2.7)
        ])
    );

    let mut data = [
        Shape::Circle(6.3),
        Shape::Rectangle {
            width: 8.0,
            height: 8.0,
        },
        Shape::Triangle(4.0, 5.0, 6.0),
        Shape::Circle(2.5),
        Shape::Rectangle {
            width: 15.2,
            height: 3.7,
        },
        Shape::Triangle(1.0, 3.0, 9.0),
        Shape::Circle(11.1),
    ];

    for item in &data {
        println!("1. Name: {:?}", title(item));
        println!("2. Validity: {:?}\n", item.validity());
        if item.validity() {
            println!("3. Square: {:?}", item.square());
            println!("4. Perimeter: {:?}", item.perimeter());
        }
    }
    data[5] = Shape::Triangle(9.0, 3.0, 9.0);
    match largest_area(&data) {
        Some(a) => {
            println!("Title: {:?}", title(a));
            println!("Square: {:?}", a.square())
        }
        None => {
            println!("Фигуры не найдены!")
        }
    }
    let data2: [Shape; 0] = [];
    let result = largest_area(&data2);
    if let Some(a) = result {
        println!("\nResult: {:?}", a)
    } else {
        println!("\nМассив пустой!")
    }
}
