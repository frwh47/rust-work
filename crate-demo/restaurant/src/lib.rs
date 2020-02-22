// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add to waitlist");
        }
    }
}

pub fn eat_at_restaurant() {
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();
}

fn server_order() {
    println!("server_order")
}

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        println!("fix_incorrect_order");
        cook_order();
        super::server_order();
    }

    fn cook_order() {
        println!("cook_order")
    }
}
