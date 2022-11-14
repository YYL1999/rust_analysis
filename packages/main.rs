/*
 * @Author: YYL1999 2291003927@qq.com
 * @Date: 2022-11-14 14:18:55
 * @LastEditors: YYL1999 2291003927@qq.com
 * @LastEditTime: 2022-11-14 14:20:53
 * @FilePath: \rust\packages\main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
mod car;
pub use crate::car::font::cook;
fn main () {
  cook();
}

pub mod company {
  pub mod it {
    fn code() {

    }
  }
}