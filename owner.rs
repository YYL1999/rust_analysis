/*
 * @Author: 雷鹏飞 pflei@gyasset.com
 * @Date: 2022-11-10 13:42:51
 * @LastEditors: 雷鹏飞 pflei@gyasset.com
 * @LastEditTime: 2022-11-10 13:53:40
 * @FilePath: \rust\owner.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
fn main() {
  let mut s = String::from('abc');
  change(&mut s);

  takes_ownership(s);
  let x = 5;

  makes_copy(x);
}

fn takes_ownership(some_thing: String) {
  println('{}', some_thing);
}

fn makes_copy(an: i32) {
  println('{}', an);
}

fn change(info: &mut String) {
  info.push_str(", ccc")
  println('{}', info);
}

let mut s = String::from("hello");

let r1 = &s; // 没问题
let r2 = &s; // 没问题
let r3 = &mut s; // 大问题

println!("{}, {}, and {}", r1, r2, r3);