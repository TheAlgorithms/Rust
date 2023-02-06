fn add(a: i32, b: i32) -> i32 {
    //a=6.5
    //
    //>>> add(2, 2)
    //4
    //>>> add(2, -2)
    //0

    return a + b;
}


fn main() {
    let a = 4;
    let b = -2;
    let c = add(a, b);
    println!("Result: {c}");
}
