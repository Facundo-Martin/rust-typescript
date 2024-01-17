fn iterator() {
    let data = vec![1,2,3];
    let mut foo = data.iter().map(|x| x + 1);
    
    let mut new_vector = vec![];

    while let Some(x) = foo.next() {
        new_vector.push(x)
    }
    println!("{:?}",new_vector); 
}

fn print_list() {
    let file = std::fs::read_to_string("lines").unwrap();
    file.lines().enumerate().filter(|(i, _)| i % 2 == 0).skip(2).take(2).for_each(|(_, line)| println!("{}", line))
 }



fn main() {
    iterator();
    print_list();
}
