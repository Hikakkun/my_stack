use my_stack::stack::Stack;

fn main() {
    let mut stack = Stack::<i32>::new(3);
    stack.push(1).unwrap();
    stack.push(2).unwrap();
    stack.push(3).unwrap();
    println!("{:?}", stack);
    println!("top = {:?}, size = {}", stack.top(), stack.size());

    stack.pop().unwrap();
    stack.pop().unwrap();
    stack.pop().unwrap();
    println!("{:?}", stack);
    println!("top = {:?}, size = {}", stack.top(), stack.size());
}
