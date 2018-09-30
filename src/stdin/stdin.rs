use std::io::stdin;
pub fn exec() {
    println!("请输入字符");
    let mut s = String::new();
    stdin().read_line(&mut s).expect("发生异常");
    println!("你输入的是{}", s);
}