use napi::bindgen_prelude::{Uint8Array,Buffer}; 

#[napi]
fn double_buffer(mut input: Buffer) {
    for item in input.as_mut() {
        *item += 1;
    }
}

 
#[napi]
fn create_buffer() -> Buffer {
  vec![1, 2, 3, 4, 5].into()
}