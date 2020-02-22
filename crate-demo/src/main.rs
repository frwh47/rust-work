use restaurant::back_of_house;
use restaurant::back_of_house::fix_incorrect_order;

fn main() {
    println!("Hello, world!");

    public_mod::public_fn();
    //public_mod::private_fn();

    private_mod::public_fn();
    //private_mod::private_fn();

    restaurant::eat_at_restaurant();

    restaurant::back_of_house::fix_incorrect_order();
    back_of_house::fix_incorrect_order();
    fix_incorrect_order();
}

pub mod public_mod {
    pub fn public_fn() {
        println!("public_mod::public_fn");
        private_fn();
    }
    fn private_fn() {
        println!("public_mod::private_fn");
    }
}

mod private_mod {
    pub fn public_fn() {
        println!("private_mod::public_fn");
        private_fn();
    }
    fn private_fn() {
        println!("private_mod::private_fn");
    }
}
