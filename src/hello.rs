#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
fn log(n: u32) {
	println!("{}", n);
}