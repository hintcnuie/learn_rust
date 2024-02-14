fn main() {
    // let x = 1;
    // println!("x的值是{},x+1的值是{}",x,x+1);

    // let x = 1;
    // let y = 2;
    // let z = 3;
    // println!("x的值是{2},y的值是{1},z的值是{0}",z,y,x);


    //使用命名参数
    // println!("{a}{b}{c}",
    //         a="我",b="爱",c="Rust语言");

    //:width指定显示参数
    // println!("打印数字1，注意数字1前面的空格{number:>width$}", number=1, width=6);

    // 在数字左边补 0。下面语句输出 "000001"。
    //println!("打印数字1，注意1前面的补齐{number:>0width$}", number=1, width=10);

    //:b打印二进制
    println!("只有{} 分之 {:b} 的人知道二进制, 其他的人都不知道。", 1, 8);

}
