const fs = require("node:fs");

function main() {
  const input = fs.readFileSync("../input-file.txt", "utf8");
  const initTime = new Date().getTime();
  let sum = 0;
  let leftNumber = null;
  let rightNumber = null;

  for (let char of input) {
    if (char !== "\n") {
      if (leftNumber !== null && !isNaN(char)) {
        rightNumber = +char;
      }

      if (leftNumber === null && !isNaN(char)) {
        leftNumber = +char;
      }
    } else {
      sum += leftNumber * 10;
      sum += rightNumber !== null ? rightNumber : leftNumber;
      rightNumber = null;
      leftNumber = null;
    }
  }

  console.log("Code: ", sum);
  console.log("Time: ", (new Date().getTime() - initTime) / 1000);
}

// Solution: 53974 - 0.009 s
main();
