extern crate rand;

use rng::rand::Rng;
use std::io::stdin;
use std::cmp::Ordering;

pub fn exec() {
    // 随机一个数
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // String 变量
    let mut str = String::new();

    println!("猜数字游戏\n请输入一个数字：");
    // 获得输入
    stdin().read_line(&mut str)
        .expect("无法获得输入");
    // 转换成数字
    let str: i32 = str.trim().parse()
        .expect("请输入一个数字1");
    // 比较
    match str.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}