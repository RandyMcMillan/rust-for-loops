fn for_loop() {
    let mut sum10 = 0;
    for i in 1..11 {
        sum10 += i;
    }
    //sum10 is 55
    println!("sum10 is {sum10}");
}
fn main() {
    for_loop();
}
