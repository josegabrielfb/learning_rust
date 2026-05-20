fn count(num: i32) {
    for i in 1..=num {
        println!("{}", i);
    }
}

fn count_down(num: i32) {
    let mut i = num;
    while i >= 1 {
        println!("{}", i);
        i -= 1;
    }
}

fn main() {
    let num = 10;
    count(num);
    count_down(num);
}
