const { doubleBuffer, createBuffer } = require('../index.js');


let arr = Buffer.from([1, 2, 3, 4, 5]);
doubleBuffer(arr);
console.log(arr);

let arr2 = createBuffer();
console.log(arr2);