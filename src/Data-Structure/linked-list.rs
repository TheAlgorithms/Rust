use List::*;
enum List {
    Cons(u32, Box<List>),
    Nil,
}
impl List {
    // Create an empty list
    fn new()->List {
        Nil
    }
    fn prepend(self, elem:u32)->List {
        Cons(elem, Box::new(self))
    }
    fn len(&self)->u32{
        match *self {
            Cons(_, ref tail)=>1 + tail.len(),
            Nil=>0
        }
    }
    fn stringify(&self)->String {
        match *self {
            Cons(head,ref tail)=>{
                format!("{}, {}", head, tail.stringify())
            },
            Nil=>{
                format!("Nil")
            },
        }
    }
}

fn main() {
    //append linked list through prepend function that append n numbers in linked list.
    // Create an empty linked list
    let mut list = List::new();
    for n in 1..=10
        {list = list.prepend(n);}
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
