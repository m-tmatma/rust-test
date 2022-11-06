fn main() {
    let num = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];  
    let mut total = 0;
    for i in num {
        total += i
    }
    println!("{}", total);
}