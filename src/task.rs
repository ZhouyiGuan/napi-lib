use napi::{
    Error,
    Env, 
    JsNumber, 
    Result, 
    Task,
    bindgen_prelude::{AsyncTask,AbortSignal,Promise},
};

struct AsyncTaskDivide {
    arg_1: u32,
    arg_2: u32,
}

impl AsyncTaskDivide {
    pub fn new(arg_1: u32, arg_2: u32) -> Self {
        Self {
            arg_1,
            arg_2,
        }
    }
}
 
impl Task for AsyncTaskDivide {
    type Output = u32;
    type JsValue = JsNumber;

    fn compute(&mut self) -> Result<Self::Output> {
        std::thread::sleep(std::time::Duration::from_secs(1));
        if self.arg_2 == 0 {
            return Err(Error::from_reason("arg_2 can not be 0".to_string()));
        }
        Ok(self.arg_1/self.arg_2)
    }

    fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
        env.create_uint32(output)
    }

    fn reject(&mut self, _env: Env, err: Error) -> Result<Self::JsValue> {
        Err(err)
    }

    fn finally(&mut self, _env: Env) -> Result<()> {
        Ok(())
      }
}
 
#[napi]
fn my_divide(input_1: u32, input_2: u32,signal: AbortSignal) -> AsyncTask<AsyncTaskDivide> {
    let task = AsyncTaskDivide::new(input_1, input_2);
    AsyncTask::with_signal(task,signal)
}

#[napi]
fn my_divide_without_signal(input_1: u32, input_2: u32) -> AsyncTask<AsyncTaskDivide> {
    let task = AsyncTaskDivide::new(input_1, input_2);
    AsyncTask::new(task)
}


#[napi] 
pub async fn my_double(p: Promise<u32>) -> Result<u32> {
  let arg = p.await?;
  Ok(arg*2)
}
