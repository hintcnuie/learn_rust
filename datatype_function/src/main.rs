

fn main() {
    //比一比，哪个数字大？
    let x1 = 2_54;
    let x2 = 0xfe;
    let x3 = 0o377;
    let x4 = 0b1111_1110;

    println!("Hello, x1={x1},x2={x2},x3={x3},x4={x4}");

    //浮点数
    let f1 = 2.0;//f64
    let f2 :f32 = 3.1;//f32
    println!("f1={},f2={}",f1,f2);

    // 加法
    let sum = 0.1 + 0.2;
    // 减法
    let sub = 95.5 - 4.3;
    // 乘法
    let mul = 4 * 30;
    // 除法
    let div = 129.60 / 32.4;
    // 求余
    let rem = 43 % 5;

    println!("sum ={sum}, sub={sub},mul={mul},div={div},rem={rem}");

    let f = false;
    println!("f={f}");
    if f {
        println!("f is true");
    }else{
        println!("f is false");
    }

    //char 类型
    let c = 'c';
    let z: char = '汉'; // with explicit type annotation
    let m = '🤣';
    println!("c={c},z={z},m={m}");

    //tuple 例子
    let tupl:(i32, f64,u32) = (500,3.14, 128);
    println!("元组tupl包含的值是{:?}",tupl);

    //获取tuple的元素
    let (t1,t2,t3) = tupl;
    println!("t1值是{t1},t2是{t2},t3是{t3}");
    //逐个获取元组的元素
    println!("元组tupl中第一个值是{},第二个值是{},第三个值是{}",tupl.0, tupl.1, tupl.2);


    // 创建一个 mutable tuple
    let mut top_three_mountains = ("珠穆朗玛峰", 8848, "乔峰", 184,"干城章嘉峰",8586);
    println!("世界前三高的山峰：{:?}",top_three_mountains);

    //下面修改第二高的山峰名称和高度
    top_three_mountains.2="乔戈里峰";
    top_three_mountains.3=8611;

    println!("确认无误的世界前三高的山峰：{:?}",top_three_mountains);

    //数组

    let natrual_number_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("数组内容：{:?}",natrual_number_array);
    println!("数组的第一个元素：{}",natrual_number_array[0]);

    //数组迭代
    let prime_number = [2,3,5,7,11,13,17,19];
    for prime in prime_number {
        println!("当前素数是：{prime}");
    }
}
