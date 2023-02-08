// pub struct AppleEater {
//     granny_smiths: u32,
//     golden_delicious: u32,
// }

// impl AppleEater {
//     pub fn report(&self) {
//         println!("I ate {} granny smiths!", self.granny_smiths);
//         println!("I ate {} golden delicious!", self.golden_delicious);
//     }
    
//     pub fn eat_granny_smith(&mut self) {
//         self.granny_smiths += 1;
//     }
// }

// pub trait AsAppleEater {
//     fn as_apple_eater(&self) -> &AppleEater;
//     fn report(&self)
// }



// struct People {
//   eaten_apples: i32,
// }

// impl People {
//   fn eaten_apples(&mut self) {
//     self.eaten_apples += 1;
//     println!("{} apples eaten", self.eaten_apples);
//   }
// }

// struct Student {
//     people: People,
// }

// impl Student {
//     fn eat_apple(&mut self) {
//         self.people.eat_apple();
//     }
// }