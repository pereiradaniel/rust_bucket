fn main() {

    // Loops through an array using while:
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // Loops through an array using for (element):
    let b = [15, 25, 35, 45, 55];
    for element in b {
        println!("the value is: {element}");
    }

    // Loops through a Range of numbers in descending order:
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}