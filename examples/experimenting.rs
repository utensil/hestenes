extern crate hestenes;
extern crate typenum;

use hestenes::*;
use typenum::U3;

fn main() {
  let a: ScaledBasisBlade<f32, U3> = (2.0, 0b110.into()).into();
  let b: ScaledBasisBlade<f32, U3> = (3.0, 0b001.into()).into();

  let c = &a^&b;
  let _d : ScaledBasisBlade<f32, U3> = ScaledBasisBlade::new(6.0, 0b111.into());

  println!("{} \\wedge {} = {}", a, b, c);
}