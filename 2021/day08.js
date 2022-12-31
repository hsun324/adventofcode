var input = document.body.innerText.trim().split('\n').map((s) => s.split(' | ').map(((s2) => s2.split(' '))))
console.log("Part 1:", input.flatMap((v) => v[1].map((x) => x.length).filter((x) => x == 2 || x == 4 || x == 3 || x == 7)).length);
var shapes = ["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"];
var freq = shapes.join('').split('').reduce((a, v) => { a[v] = (a[v] || 0) + 1; return a }, {});
var table = Object.fromEntries(shapes.map((x, i) => [Array.from(x).map((c) => freq[c]).sort(), i]));
console.log("Part 2:", input.map((puzzle) => {
    var freq = puzzle[0].join('').split('').reduce((a, v) => { a[v] = (a[v] || 0) + 1; return a }, {});
    return parseInt(puzzle[1].map((s) => table[Array.from(s).map((c) => freq[c]).sort()]).join(''));
}).reduce((a, b) => a + b));
