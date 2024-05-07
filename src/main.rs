fn main() {
    // vector, only store value with same type Vec<T> generic
    let v: Vec<i32> = Vec::new();

    //v.push(5); // need v to be mutable
    //v.push(6);
    //v.push(7);
    //v.push(8);

    println!("{:?}", v);

    // init vector with value
    let v = vec![1, 2, 3];

    println!("{:?}", v);
}
