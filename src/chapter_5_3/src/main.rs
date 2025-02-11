#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
} 

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //имя метода и поля структуры могут совпадать
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

fn main() {
    let rectangle1 = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "Площадь прямоугольника {} квадратных пикселей",
        rectangle1.area()
    );

    println!("Прямоугольник имеет ширину больше 0: {}", rectangle1.width());

    let rectangle2 = Rectangle {
        width: 10,
        height: 40
    };

    let rectangle3 = Rectangle {
        width: 60,
        height: 45
    };

    println!("Может ли rectangle1 вместить в себя rectangle2: {}", rectangle1.can_hold(&rectangle2));
    println!("Может ли rectangle1 вместить в себя rectangle3: {}", rectangle1.can_hold(&rectangle3));

    let square = Rectangle::square(32);
    println!("{square:#?}");
}
