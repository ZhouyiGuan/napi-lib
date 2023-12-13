const { myDivide, myDivideWithoutSignal, myDouble } = require('../index.js');


const controller_1 = new AbortController()
myDivide(20,0,controller_1.signal).then((output) => {
    console.log(`task1 finished`,output);
}).catch((err) => { 
    console.log(`task1 aborted`,err);
});

async function main() {
    const result = await myDivideWithoutSignal(20,10);
    console.log(`task2 finished`, result);
}
main();

(async () => {
    const result = await myDouble (
        new Promise((resolve) => {
        setTimeout(() => resolve(10), 1000)
        })
    );
    console.log(`task3 finished`,result);
})();


