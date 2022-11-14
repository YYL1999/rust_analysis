/*
 * @Author: YYL1999 2291003927@qq.com
 * @Date: 2022-11-14 14:25:36
 * @LastEditors: YYL1999 2291003927@qq.com
 * @LastEditTime: 2022-11-14 14:27:27
 * @FilePath: \rust\vector.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
let v: Vec<i32> = Vec::new();

let n = vec![100, 200, 300];

n.push(400);


let str: &i32 = &n[2];

for i in &v {
  println!('{}', i);
}