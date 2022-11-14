/*
 * @Author: YYL1999 2291003927@qq.com
 * @Date: 2022-11-14 14:38:59
 * @LastEditors: YYL1999 2291003927@qq.com
 * @LastEditTime: 2022-11-14 14:44:00
 * @FilePath: \rust\string.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
let s = '323';

let s_t = s.to_string();

let mut m = String::from('cdcd');

m.push_str("dc");

let n = &m[0..4];