/*
 * @Author: YYL1999 2291003927@qq.com
 * @Date: 2022-11-10 14:39:39
 * @LastEditors: YYL1999 2291003927@qq.com
 * @LastEditTime: 2022-11-10 14:43:17
 * @FilePath: \rust\struct.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
struct Rectangle {
  width: u32,
  height: u32
}
fn main() {
  let width1 =30;
  let height1 = 50;

  let rect = Rectangle {
    width: 30,
    height: 50
  }

  println!("the area {}", area(&rect));
}

fn area(rect: &Rectangle) -> u32 {
  rect.width * rect.height
}