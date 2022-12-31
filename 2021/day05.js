var input =  document.body.innerText.trim().split('\n').map((s1) => s1.split(' -> ').map((s2) => s2.split(',').map((s3) => parseInt(s3, 10))))
var intersect = input.reduce((arr, ln) => {
    var i = ln[0][0], j = ln[0][1];
    var di = Math.sign(ln[1][0] - ln[0][0]), dj = Math.sign(ln[1][1] - ln[0][1]);
    var l = Math.max(Math.abs(ln[0][0] - ln[1][0]), Math.abs(ln[0][1] - ln[1][1]));
    if (di * dj == 0)
        for (var k = 0; k <= l; i += di, j += dj, k++)
            arr[[i, j]] = (arr[[i, j]] || 0) + 1;
    return arr;
}, {})
console.log("Part 1:", Object.entries(intersect).filter((v) => v[1] >= 2).length)
var intersect = input.reduce((arr, ln) => {
    var i = ln[0][0], j = ln[0][1];
    var di = Math.sign(ln[1][0] - ln[0][0]), dj = Math.sign(ln[1][1] - ln[0][1]);
    var l = Math.max(Math.abs(ln[0][0] - ln[1][0]), Math.abs(ln[0][1] - ln[1][1]));
    for (var k = 0; k <= l; i += di, j += dj, k++)
        arr[[i, j]] = (arr[[i, j]] || 0) + 1;
    return arr;
}, {})
console.log("Part 2:", Object.entries(intersect).filter((v) => v[1] >= 2).length)
