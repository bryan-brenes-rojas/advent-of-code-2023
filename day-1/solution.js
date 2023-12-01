const fs = require("node:fs");

function getLines(data) {
  let line = "";
  const lines = [];
  for (let char of data) {
    if (char !== "\n") {
      line = line.concat(char);
    } else {
      lines.push(line);
      line = "";
    }
  }
  return lines;
}

function getMessage(line) {
  let left = 0;
  let right = line.length - 1;
  let hasFoundLeftIndex = null;
  let hasFoundRightIndex = null;
  let num = "";
  while (left < line.length) {
    if (
      hasFoundLeftIndex === null &&
      !isNaN(line[left]) &&
      left !== hasFoundRightIndex
    ) {
      hasFoundLeftIndex = left;
      num = +line[left] + num;
    }
    if (
      hasFoundRightIndex === null &&
      !isNaN(line[right]) &&
      right !== hasFoundLeftIndex
    ) {
      hasFoundRightIndex = right;
      num = num + +line[right];
    }
    left++;
    right--;
  }
  return num.length !== 1 ? +num : +`${num}${num}`;
}

function main() {
  const input = fs.readFileSync("input-file.txt", "utf8");
  const lines = getLines(input);
  const messages = [];
  for (let line of lines) {
    messages.push(getMessage(line));
  }
  console.log(
    "Code: ",
    messages.reduce((curr, acc) => acc + curr, 0),
  );
}

// Solution: 53974
main();
