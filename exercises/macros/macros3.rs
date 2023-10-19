// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.


//使用传统的方法[macro_use]可以导出这个宏来使用，但是如果交换顺序的话就不行了
//也可以使用#[macro_export]来表明这个模块是要导出的，然后在宏的末尾加上一个pub(crate) use my_macro也可

#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
