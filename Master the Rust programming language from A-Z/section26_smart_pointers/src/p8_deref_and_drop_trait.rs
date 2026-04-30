use std::ops::{Deref, DerefMut};
struct CustomBox<T, U> {
    data: T,
    more_data: U,

}

impl<T, U> CustomBox<T, U> {
    fn new(data: T, more_data: U) -> Self {
        Self {
            data, more_data
        }
    }
}

impl<T, U> Deref for CustomBox<T, U> {
    type Target = U;

    fn deref(&self) -> &Self::Target {
        &self.more_data
    }

    
}

impl<T, U> DerefMut for CustomBox<T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.more_data
    }
}

impl<T, U> Drop for CustomBox<T, U> {
    fn drop(&mut self) {
        println!("I'm cleaning up related files on the hard drive");
        println!("I'm terminating a network connection");
        println!("I'm removing the CustomBox from memory");
    }
}

fn main() {
    let mut boxy = Box::new(3.14);
    *boxy = 6.42;
    println!("{}", *boxy);

    let mut custom_boxy = CustomBox::new(3.14, "Hello");
    *custom_boxy ="Goodbye";
    println!("{}", *custom_boxy);
}
