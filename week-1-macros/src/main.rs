struct Rect {
    width: u32,
    height: u32,
}
struct Sqare {
    side: u32,
}

trait Shape {
    fn area(&self) -> u32;
    fn parameter(&self) -> u32;
}

impl Shape for Rect {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }
    fn parameter(&self) -> u32 {
        return 2 * (self.height + self.width);
    }
}

impl Shape for Sqare {
    fn area(&self) -> u32 {
        return self.side * self.side;
    }
    fn parameter(&self) -> u32 {
        return 4 * self.side;
    }
}

fn main() {
    let r = Rect {
        width: 10,
        height: 20,
    };
    let s = Sqare { side: 10 };
    let (rect_area, rect_parameter) = get_area(r);
    let (sqare_area, sqare_parameter) = get_area(s);

    println!("the area and parameter of rectangle is {} {} respectively ", rect_area, rect_parameter);
    println!("the area and parameter of square is {} {} respectively ", sqare_area, sqare_parameter);
}

fn get_area(k: impl Shape) -> (u32,u32) {
    return (k.area(), k.parameter());
}
