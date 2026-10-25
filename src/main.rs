unsafe extern "C" {
    safe fn check_ventura() -> bool;
}

fn main() {
    if check_ventura() {
        println!("Running at least on Ventura");
    } else {
        println!("Running below Ventura");
    }
}
