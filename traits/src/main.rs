// struct Rect{
//     height: u32, 
//     width: u32,
// }

// struct Square{
//     width: u32,
// }

// impl Rect {
//     fn area(&self) -> u32{
//         return self.height * self.width;
//     }

//     fn perimeter(&self) -> u32{
//         return (self.height + self.width) * 2;
//     }
// }

// impl Square{
//     fn area(&self) -> u32{
//         return self.width * self.width;
//     }

//     fn perimeter(&self) -> u32{
//         return self.width * 4;
//     }
// }

// fn main(){
//     let r1 = Rect{
//         height: 10, 
//         width: 10,
//     };

//     let s1 = Square{
//         width: 10,
//     };

//     let area1 = r1.area();
//     let area2 = s1.area();

//     let perimeter1 = r1.perimeter();
//     let perimeter2 = s1.perimeter();

//     println!("This is the area of rectangle -> {}", area1);
//     println!("This is the area of square -> {}", area2);

//     println!("This is the perimeter of rectangle -> {}", perimeter1);
//     println!("This is the perimeter of square -> {}", perimeter2);
// }




// This peice of code is so verbose and it's violating the DRY principle. 
// Let's try a different approach to the same code. 


struct Rect{
    height: u32, 
    widht: u32, 
}

struct Square{
    height: u32,
}

impl Shape for Rect{
    fn area(&self) -> u32{
        return self.height * self.widht;
    }
    fn perimeter(&self) -> u32{
        return (self.height + self.widht) * 2;
    }
}

impl Shape for Square{
    fn area(&self) -> u32{
        return (self.height) * (self.height);
    }

    fn perimeter(&self) -> u32{
        return (self.height) * 4;
    }
}

trait Shape{
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
}

fn a_and_p(s: impl Shape) -> (u32, u32){
    return (s.area(), s.perimeter());
}


fn main(){
    let r1 = Rect{
        height: 10, 
        widht: 10, 
    };

    let s1 = Square{
        height: 10, 
    };

    let (a1, p1) = a_and_p(r1);
    let (a2, p2) = a_and_p(s1);

    println!("This is the area of r1 -> {} and this is the perimeter of r1 -> {} ", a1, p1);
    println!("This is the area of s1 -> {} and this is the perimeter of s1 -> {} ", a2, p2);
}


// The above code uses trait and it is a better way to code as comapred to our first approach
// trait is a common behaviour of two type(structs)
