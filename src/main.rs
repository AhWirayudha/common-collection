// enum different type can be use as vector, since vector only store same type which is enum type
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

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

    // update vector, need mutable
    let mut v = Vec::new();

    v.push(2);
    v.push(6);
    v.push(7);
    v.push(8);
    // v.push("Test"); // cannot push different type

    println!("{:?}", v); // print [2, 6, 7, 8]
    println!("{:?}", v[0]); // print 2

    // reading element of vector
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let test_v = v[2];
    println!("The third element is {test_v}");

    println!("All element is {:?}", v);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // try access index out of bound
    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100]; // len only 5, panic error
    let does_not_exist = v.get(100); // not error
    println!("{:?}", does_not_exist); // None, can checked with match

    // valid reference
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // borrow before change

    v.push(6); // change with push

    println!("{:?}", v); // [1, 2, 3, 4, 5, 6]

    // println!("The first element is: {first}"); // error because you call it here after mutable change

    // iterating over value in vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    // mutable vector change all element
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // add 50 to each element, use * to dereference the value
    }
    println!("{:?}", v);

    // vector with enum type
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);

    // String, from standart library (wrapper arround vector of byte)
    // create string
    let mut s = String::new(); // create empty string
    // load data into string
    let data = "initial contents";
    let s = data.to_string();
    println!("{s}");

    // option, the method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("{s}");

    // String::from
    let s = String::from("initial contents");
    println!("{s}");

    // String are UTF-8 encoded, all valid sample
    let hello = String::from("السلام عليكم");
    println!("{hello}");
    let hello = String::from("Dobrý den");
    println!("{hello}");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    println!("{hello}");
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");



}
