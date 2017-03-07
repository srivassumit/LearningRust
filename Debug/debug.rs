#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
    println!(" My name is {:?}", "Sumit");
    println!("{actor:?} is {0} {1}", "James", "Bond", actor="Johnny Depp");

    println!("Printing Structures: {:?}", Structure(3));
    println!("Print Deep: {:?}", Deep(Structure(7)));
}
