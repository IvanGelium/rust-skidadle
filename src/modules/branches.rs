#![allow(dead_code)]
pub fn run() {
    let number = 3;

    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }

    if number < 5 {
        println!("true")
    } else {
        println!("false")
    }

    /*
    If внезапно можно использовать как выражение,
    если попытаться присовать if в переменную,
    присвоится результат возврата из блока кода {}
     */

    let _wow = if 1 > 2 { "cool" } else { "nice" };
}
