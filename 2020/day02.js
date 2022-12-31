var input = document.body.innerText.trim().split('\n').map((s) => s.split(' ')).map((s2) => [s2[0].split('-').map((s3) => parseInt(s3, 10)), s2[1][0], s2[2]]);
console.log("Part 1:", input.filter((r) => {
    var c = Array.from(r[2]).filter((x) => x == r[1]).length;
    return r[0][0] <= c && r[0][1] >= c;
}).length);
console.log("Part 2:", input.filter((r) => (r[2][r[0][0] - 1] == r[1]) != (r[2][r[0][1] - 1] == r[1])).length);
