fn main() {
    println!("=== Rust uses an ownership model ===");

    let x = 74;
    let y = x;

    println!("\tInt value x (stack) is {}", x);
    println!("\tInt value y (stack) is {}", y);
    println!("");


    let s1 = String::from("Hello!");
    let s2 = s1;

    // Uncommenting the following statement results in compiler error
    // println!("\tString value (heap) s1 = {}", s1);
    println!("\tString value (heap) s2 = {}", s2);

    // The compiler error is
    //  error[E0382]: borrow of moved value: `s1`
    // which is caused by s2 taking ownership of the string in the line
    //  let s2 = s1;
}