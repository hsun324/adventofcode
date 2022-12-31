var input = document.body.innerText.trim().split(',').map((s) => parseInt(s, 10));
console.log("Part 1:", Math.min(...Array(1000).fill(0).map((q, v) => input.map((x) => Math.abs(x - v)).reduce((a, b) => a + b))));
console.log("Part 2:", Math.min(...Array(1000).fill(0).map((q, v) => input.map((x) => { var k = Math.abs(x - v); return k * (k + 1) / 2; } ).reduce((a, b) => a + b))));
