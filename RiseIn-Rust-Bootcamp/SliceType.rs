/*
Slice: Slice are a reference to a contiguous sequence of elements in the collection.
       They are a view into the original collection and do not store any data themselves.
       Slices are used to give a part of a collection to a function or to iterate over a part of the collection. 
*/

/*
Syntax : &[T]
T --> Type of the elements in the collection
& --> Reference to the collection
*/

//Example:1
fn main() {
    //slice of an array of characters
    let arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    let slice: &[char] = &arr[1..3];
    println!("{:?}", slice);

    //slice of a vector of integers
    let vec: Vec<i32> = vec![10, 20, 30, 40, 50];
    let slice = &vec[3..4];
    println!("{:?}", slice);

    //slice for strings
    let s:String=String::from("hello world");
    let hello = &s[1..2];
    let world= &s[8..11];
    println!("{:?} {:?}",hello,world);
}

//Example:2
fn main() {
    let s=String::from("Hello Jay");
    //shortcut for initial index
    let slice=&s[0..3];
    println!("{}",slice);
    let slice=&s[..3];
    println!("{}",slice);

    //shortcut for final index
    let len=s.len();
    let slice=&s[5..len];
    println!("{}",slice);
    let slice=&s[5..];
    println!("{}",slice);

    //shortcut for initial index and final index
    let slice=&s[0..len];
    println!("{}",slice);
    let slice=&s[..]; 
    println!("{}",slice); //Hello Jay
}
