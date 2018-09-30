use std::io;

fn main() {
    println!("请输入字符");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("err");
    println!("你输入的是{}", s);
}
