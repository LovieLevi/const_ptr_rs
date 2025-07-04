use const_ptr::Ptr;

const POINTER: Ptr<i32> = Ptr::new();

fn main() {
    println!("{}", POINTER.get());
    *POINTER.get_ref() = 43;
    println!("{}", POINTER.get());
}
