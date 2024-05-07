
fn main() {
    let numar = 10;

    if numar > 5 {
        println!("Numărul este mai mare decât 5.");
    } else {
        println!("Numărul este 5 sau mai mic.");
    }

    // Folosind `for` pentru a itera peste un interval de valori
    for i in 1..=5 {
        println!("{}", i);
    }
}
