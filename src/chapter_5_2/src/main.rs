#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "Площадь прямоугольника {} квадратных пикселей",
        area_with_params(width1, height1)
    );

    let rectangle1 = (40, 50);

    println!(
        "Площадь прямоугольника {} квадратных пикселей",
        area_for_dimencions(rectangle1)
    );

    let rectangle2 = Rectangle {
        width: 50,
        height: 60
    };

    println!(
        "Площадь прямоугольника {} квадратных пикселей",
        area_for_struct(&rectangle2)
    );

    println!("rectangle {:#?}", rectangle2);

    dbg!(&rectangle2);
}

fn area_with_params(width: u32, height: u32) -> u32 {
    width * height
}

fn area_for_dimencions(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_for_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
