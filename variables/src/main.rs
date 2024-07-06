const STARTING_MISSILES :i32 = 8;
const READY_AMOUNT :i32 = 2;


fn main() {
    let  (mut missile , ready ) :(i32,i32) = (STARTING_MISSILES,READY_AMOUNT);
    println!("Firing {} out of {} missiles...", ready,missile);
    missile = missile - ready;
    println!("{} missiles left...", missile);
}
