use unicode_segmentation::UnicodeSegmentation; // can handle graphemes
use std::collections::HashMap; // std lib hash map

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

    // Update string
    // Appending to a String with push_str and push
    let mut s = String::from("foo");
    s.push_str("bar"); // using push_str
    println!("{s}");   

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}"); // push_str does not take ownership of s2

    // push, method to add a single character
    let mut s = String::from("lo");
    s.push('l'); // push single character
    println!("{s}");
    
    // concatenation with + operator and format! macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    // using format
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); // use reference, does not take ownership
    println!("{s} {s1} {s2} {s3}");

    // Indexing into String
    // internal representation
    // String is a wrapper around Vec<u8> => UTF-8
    let hello = String::from("Hola"); // len 4, mean 4 bytes long
    let hello = String::from("Здравствуйте"); // len 24, mean 24 bytes long
    let hello = String::from("你好"); // len 6, mean 6 bytes long

    let hello = "Здравствуйте";
    // let answer = &hello[0]; // error

    // println!("{answer}");
    
    // slice, tricky to use
    let hello = "Здравствуйте"; // 2 bytes per character

    let s = &hello[0..4]; // Зд

    println!("{s}");

    // let s = &hello[0..1]; // panic

    // method for iterating over String
    for c in "नमस्ते".chars() {
        println!("{c}"); // unicode scalar
    }

    for b in "नमस्ते".bytes() {
        println!("{b}"); // bytes
    }

    // graphemes
    for g in "नमस्ते".graphemes(true) {
        println!("{g}"); // print graphemes न म स् ते
    }

    // HashMap
    // create hash map
    let mut scores = HashMap::new();
    // use insert to populate
    scores.insert(String::from("Blue"), 10); // String "Blue" as key and 10 as value
    scores.insert(String::from("Shapirre"), 40); // its homogenous, so the key always same type and values also same type

    println!("{:?}", scores); // print {Blue: 10, Shapirre: 40}

    // accessing values
    let team_name = String::from("Blue");
    // get value by key
    let score = scores.get(&team_name).copied().unwrap_or(0); // get and unwrap_or 0
    println!("{score}");
    // using for loop
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
