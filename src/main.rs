// fn main() {
//     let x: i8 = -5;
//     let y: u32 = 2000;
//     let z: f32 = 20.001;

//     println!("x: {}, y: {}, z: {}", x, y, z);
// }

// fn main() {
//     let is_male = true;
//     let is_above_18 = true;

//     if is_male {
//         println!("You are a male");

//     } else {
//         println!("You are not a male");
//     }

//     if is_male && is_above_18 {
//         print!("You are a legal male");
//     }
// }

// fn main() {
//     let greet: String = String :: from("Hello World");
//     println!("{}", greet);

//     let char1: Option<char> = greet.chars().nth(7);

//     // println!("Character is {}", char1.unwrap());

//     match char1 {
//         Some(c) => println!("{}", c),
//         None => println!("No character at the mentioned index!")
//     }
// }

// fn main() {
//     let greet: String = String :: from("Hello World");
//     let first_word: String = get_first_word(greet);

//     println!("First word is {}", first_word);

// }

// fn get_first_word(greet: String) -> String {
//     let mut ans = String::from("");

//     for char in greet.chars() {
//         ans.push_str(char.to_string().as_str());
//         if char == ' ' {
//             break;
//         }
//     }
//     return  ans;
// }

// fn main() {
//     update_string();
// }

// fn update_string() {
//     let mut s = String:: from("this is a");
//     println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());

//     s.push_str("beautiful day indeed!");
//     println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());

// }

// fn main() {
//     let s1 = String :: from("Hello Anshika");
//     let s2 = s1;

//     println!("{}", s2)
// }

// fn main() {
//     let mut s1 = String::from("Hello");
//     add_partner(&mut s1);
//     println!("{}", s1);
// }

// fn add_partner(name: &mut String) {
//     name.push_str(" Mercedes");
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("myusername"),
//         email: String::from("myusername@gmail.com"),
//         sign_in_count: 1,
//     };
//     let s1 = String::from("Hello");
//     println!("{}", s1);
//     println!("{s1}");

//     println!("User 1 active: {}", user1.active);
//     println!("User username: {:?}", user1.username);
//     println!("User email: {:?}", user1.email);
//     println!("User Count: {:?}", user1.sign_in_count);
// }

// enum Direction {
//     Up,
//     _Down,
//     _Right,
//     _Left,
// }

// fn which_way(go: Direction) -> &'static str {
//     match go {
//         Direction::Up => "up",
//         Direction::_Down => "down",
//         Direction::_Right => "right",
//         Direction::_Left => "left",
//     }
// }

// fn main() {
//     let direction = Direction::Up;
//     let direction_str = which_way(direction);
//     println!("Direction: {}", direction_str);
// }

// enum Cars {
//     Mercedes,
//     Bentley,
//     Porsche,
// }

// struct Info {
//     brand: Cars,
// }

// fn print_car_info(car: Info) {
//     match car.brand {
//         Cars::Mercedes => println!("Mercedes: 200000"),
//         Cars::Bentley => println!("Bentley: 400000"),
//         Cars::Porsche => println!("Porsche: 250000"),
//     }
// }
// fn main() {
//     let car = Info {
//         brand: Cars::Mercedes,
//     };
//     print_car_info(car);
// }

// fn main() {
//     let my_num = 5;
//     let is_lt_5 = my_num < 5;
//     let mut str1 = String :: new();
//     str1.push_str("Hello");
//     println!("{str1}");
//     println!("{}",is_lt_5);
// }

struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn freezing() -> Self {
        Self { degrees_f: 12.0 }
    }

    fn show_temp(&self) {
        println!("{:?}", self.degrees_f);
    }
}

fn main() {
    let hot = Temperature { degrees_f: 37.2 };
    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp();
}
