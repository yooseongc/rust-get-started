fn main() {
    
    // step 1
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // step 2 
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_of_dimensions(rect1)
    );

    // step 3
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_of_rectangle(&rect2)
    );
    println!("rect2 is {rect2:?}");
    println!("rect2 is {rect2:#?}");

    // step 4
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect3);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect3.area()
    );
    if rect3.width() {
        println!("The rectangle has a nonzero width; it is {}", rect3.width);
    }

    // step 5
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // step 6
    println!("size of square: {}", Rectangle::square(30).area());
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_of_dimensions(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn area_of_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
