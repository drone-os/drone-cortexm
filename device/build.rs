#[macro_use]
extern crate drone_stm32_svd;

fn main() {
  drone_stm32_svd::generate_rest(svd_feature!());
}
