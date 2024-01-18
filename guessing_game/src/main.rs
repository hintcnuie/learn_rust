use std::{io};
use rand::Rng;


fn main() {
    println!("猜数字游戏!");

    //产生随机数
    let magic_number = rand::thread_rng()
            .gen_range(1..=100);


    loop {
        println!("请输入您的猜测数字（1～100）。");
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("读取数字失败!");
    
        //展现两个数字
        println!("您的猜测数字是：{guess}");
    
        let guess_number : u8 = guess.trim().parse().expect("请输入有效数字！");
    
        //不仅要是否相等，还要比较大还是小，给用户提示信息
        if guess_number == magic_number {
            println!("恭喜您，您猜对了！");
            break;
        } else if guess_number > magic_number{
            println!("您输入的数字大了，请改小。")
        } else {
            println!("您输入的数字小了，请改大。")
        }
    }//end loop
   

}
