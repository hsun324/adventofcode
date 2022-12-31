var inputRaw = document.body.innerText.trim().split('\n').map((s) => s.split('').map((s2) => s2 == '#'))
var input = (x, y) => inputRaw[x][y % inputRaw[0].length];
console.log("Part 1:", inputRaw.reduce((a, row, x) => a + (input(x, x * 3) ? 1 : 0), 0))
var a = inputRaw.reduce((a, row, x) => a + (input(x, x) ? 1 : 0), 0);
var b = inputRaw.reduce((a, row, x) => a + (input(x, x * 3) ? 1 : 0), 0);
var c = inputRaw.reduce((a, row, x) => a + (input(x, x * 5) ? 1 : 0), 0);
var d = inputRaw.reduce((a, row, x) => a + (input(x, x * 7) ? 1 : 0), 0);
var e = inputRaw.reduce((a, row, x) => a + (x % 2 == 0 && input(x, x / 2) ? 1 : 0), 0);
console.log("Part 2:", a * b * c * d * e);
