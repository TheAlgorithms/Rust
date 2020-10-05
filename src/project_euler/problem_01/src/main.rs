fn main() {
    let mut the_sum = 0;
    for i in 1..1000{
        if i%3==0 || i%5 == 0{
            the_sum+=i;
        }
    }
    println!("{}",the_sum);
}
