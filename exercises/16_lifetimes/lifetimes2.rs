// Don't change this function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // TODO: Fix the compiler error by moving one line.

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        // Solution2: You can move the print statement into the inner block so
        // that it is executed before `string2` is dropped.
        println!("The longest string is '{result}'");
        // `string2` dropped here (end of the inner scope).
    }
}
