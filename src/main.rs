fn main() {
    println!("Hello, world!");
    let s = b"8,5,9,4,5,4,5,8,25,2,8,6,7,11,10,2,6,15,6,9,8,4,7,9,7,4,2,4,11,4,7,2,7,5,4,10,2,9,10";
    let num : Vec<char> = s
    .split(|x| *x == b',')
    .map(|x| x[0] as char)
    .collect();
    println!("{:?}", num);
}
