

// #[derive(Debug)]
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

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

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn height(&self) -> bool {
        self.height > 0
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

pub fn learn_structs() {
    
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    rect1.width = 40;
    rect1.height = 60;

    let rect2 = build_rectangle(55, 66);
    let rect3= Rectangle {
        width: 10, 
        ..rect2
    };
        
    dbg!(&rect3);
    println!("rect3: {:?}", rect3);
    println!("The area of the rectangle is {} square pixels.", rect3.area());
    println!("The rectangle has a nonzero width: {}", rect3.width());
    println!("The rectangle has a nonzero height: {}", rect3.height());
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(32);
    dbg!(&square);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("black: ({}, {}, {})", black.0, black.1, black.2);
    println!("origin: ({}, {}, {})", origin.0, origin.1, origin.2);
}

fn build_rectangle(width: u32, height: u32) -> Rectangle {
    Rectangle { width, height }
}
