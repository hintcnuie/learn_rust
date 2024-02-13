fn main() {

   //首次定义
   let x = 5;
   println!("变量x的值是:{x}");

   //第一次shadow
   let x = x + 1;
   println!("变量x的值是:{x}");

   {
        //第二次shadow
      let x = x + 2;
      println!("变量x的值是:{x}");
   
      {
         //第三次shadow
         let x = x * 2;
         println!("变量x的值是: {x}");
      
      }
      
      println!("变量x的值是:{x}");
   }
   //第四次shadow，回到原点
   let x = x - 1;
   println!("变量x的值是:{x}");
}
