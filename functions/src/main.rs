fn main() {
    let x = {
        let y = 1;
        y + 1
    };
    println!("x = {x}");

    let a = if_test_1(true);
    println!("if_test_1 1 = {a}");

    let a = if_test_1(false).to_string(); // shadowing (can be even other type)
    println!("if_test_1 2 = {a}");
}

fn if_test_1(p: bool) -> i32 {
    if p {
        return 1;
    }
    2
}

fn if_test_2(p: bool) -> i32 {
    if p {
        1
    } else {
        2
    }
}

// fn if_test_wont_compile(p: bool) -> i32 {
//     if p {
//         1
//     } 
// }