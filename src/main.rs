mod calculation;

fn main() {
    for i in 0..20 {
        println!("{}: {}", i, calculation::digit(i));
    }
}
