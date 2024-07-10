fn main() {
    println!("Hello, world!");

    // Char len

    println!("Size of char: {}", std::mem::size_of::<char>());
    println!("Size of a: {}", "a".len());
    println!("Size of ß: {}", "ß".len());

    // Strings len
    // “The .len() method returns the number of bytes, not the number of letters or characters.”
    let str1 = "Hello!";
    println!("str1 is {} bytes", str1.len());

    let str2 = "안녕!";
    println!("str2 is {} bytes", str2.len());

    // Strings as bytes
    println!("{:?}", "a".as_bytes());
    println!("{:?}", "ß".as_bytes());

    /*
    “.chars().count() will give you the number of characters or letters, not bytes. Calling .chars()
    first turns a string into a collection of characters, and then .count() counts how many of them there are.”
     */

    println!(
        "str1 is {} bytes and also {} characters.",
        str1.len(),
        str1.chars().count()
    );

    println!(
        "str2 is {} bytes and also {} characters.",
        str2.len(),
        str2.chars().count()
    );
}
