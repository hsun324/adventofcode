var input = document.body.innerText.trim().split(',').map((s) => parseInt(s, 10)).reduce((a, s) => { a[s]++; return a }, Array(9).fill(0))
var step = (pre) => [pre[1], pre[2], pre[3], pre[4], pre[5], pre[6], pre[7] + pre[0], pre[8], pre[0]];
console.log("Part 1:", Array(80).fill(0).reduce(step, input).reduce((a, b) => a + b, 0))
console.log("Part 2:", Array(256).fill(0).reduce(step, input).reduce((a, b) => a + b, 0))
