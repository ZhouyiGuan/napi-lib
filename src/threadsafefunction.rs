use std::{thread, vec, time};
 
use napi::{
    bindgen_prelude::*,
    threadsafe_function::{
        ErrorStrategy, 
        ThreadsafeFunction, 
        ThreadsafeFunctionCallMode,
        ThreadSafeCallContext
    },
};
 
#[napi(ts_args_type = "callback: (err: null | Error, arg1: number, arg2: number) => void")]
pub fn my_multithreads_func(callback: JsFunction) -> Result<()> {
    let tsfn: ThreadsafeFunction<(u32, u32), ErrorStrategy::CalleeHandled> = callback
        .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(u32, u32)>| {   
            if ctx.value.0 % 2 == 0 {
                Err(Error::new(Status::GenericFailure, "Even number error".to_string()))
            }else {
                let arg1 = ctx.env.create_uint32(ctx.value.0)?;
                let arg2 = ctx.env.create_uint32(ctx.value.1)?;
                Ok(vec![arg1, arg2])
            }
        }
    )?;

    for n in 0..20 {
        let tsfn = tsfn.clone();
        thread::spawn(move || {
            thread::sleep(time::Duration::from_millis(1000));
            tsfn.call(Ok((n,n)), ThreadsafeFunctionCallMode::Blocking);
        });
    }
    Ok(())
}


#[napi(ts_args_type = "callback: (arg1: number, arg2: number) => void")]
pub fn my_multithreads_func_fatal(callback: JsFunction) -> Result<()> {
    let tsfn: ThreadsafeFunction<(u32, u32), ErrorStrategy::Fatal> = callback
        .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(u32, u32)>| {   
            if ctx.value.0 % 2 == 0 {
                println!("Even number error");
                Ok(vec![])
            }else {
                let arg1 = ctx.env.create_uint32(ctx.value.0)?;
                let arg2 = ctx.env.create_uint32(ctx.value.1)?;
                Ok(vec![arg1, arg2])
            }
        }
    )?;

    for n in 0..20 {
        let tsfn = tsfn.clone();
        thread::spawn(move || {
            thread::sleep(time::Duration::from_millis(1000));
            tsfn.call((n,n), ThreadsafeFunctionCallMode::Blocking);
        });
    }
    Ok(())
}