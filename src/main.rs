fn iterator() {
    let foo: Vec<_> = vec![1,2,3].iter().map(|x| x + 1).collect();
    println!("{:?}",foo); 
}

fn main() {
    iterator()
}
