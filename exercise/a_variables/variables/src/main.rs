const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;
fn main() {
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    let _x = 1; // Adding "_" before variable tells Rust compiler I am aware that I am not using this variable
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready)
}
