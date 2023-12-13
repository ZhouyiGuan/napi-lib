const { myMultithreadsFunc,myMultithreadsFuncFatal } = require('../index.js');

const callbackAdd = (err, arg1, arg2) => {
  if (err) {
    console.log(`callbackAdd is called, err:`, err);
  } else {
    console.log(`callbackAdd is called, result:`,arg1+arg2);
  }
}
myMultithreadsFunc(callbackAdd);


const callbackAdd2 = (arg1, arg2) => {
    console.log(`callbackAdd is called, result:`,arg1+arg2);
}
myMultithreadsFuncFatal(callbackAdd2);