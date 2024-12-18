fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let b = [15, 25, 35, 45, 55];

    for element in b {
        println!("the value is: {element}");
    }
}