var input = document.body.innerText.trim().split('\n').map((s) => s.split(''));
var en = [-1, 0, 1].flatMap((dx) => [-1, 0, 1].flatMap((dy) => [-1, 0, 1].map((dz) => [dx, dy, dz])));
en.splice(13, 1);
var active = input.flatMap((row, x) => row.flatMap((c, y) => c == '#' ? [[x, y, 0]] : []));
for (var i = 0; i < 6; i++) {
    var weights = {};
    active.forEach((c) => {
        var val = (weights[c] || [0, true, c]);
        weights[c] = [val[0], true, val[2]];
        en.forEach((d) => {
            var key = [c[0] + d[0], c[1] + d[1], c[2] + d[2]];
            var val = (weights[key] || [0, false, key]);
            weights[key] = [val[0] + 1, val[1], val[2]];
        });
    });
    active = Object.values(weights).filter((x) => x[1] ? x[0] == 2 || x[0] == 3 : x[0] == 3).map((x) => x[2]);
}
console.log("Part 1:", active.length);
var en = [-1, 0, 1].flatMap((dx) => [-1, 0, 1].flatMap((dy) => [-1, 0, 1].flatMap((dz) => [-1, 0, 1].map((dw) => [dx, dy, dz, dw]))));
en.splice(40, 1);
var active = input.flatMap((row, x) => row.flatMap((c, y) => c == '#' ? [[x, y, 0, 0]] : []));
for (var i = 0; i < 6; i++) {
    var weights = {};
    active.forEach((c) => {
        var val = (weights[c] || [0, true, c]);
        weights[c] = [val[0], true, val[2]];
        en.forEach((d) => {
            var key = [c[0] + d[0], c[1] + d[1], c[2] + d[2], c[3] + d[3]];
            var val = (weights[key] || [0, false, key]);
            weights[key] = [val[0] + 1, val[1], val[2]];
        });
    });
    active = Object.values(weights).filter((x) => x[1] ? x[0] == 2 || x[0] == 3 : x[0] == 3).map((x) => x[2]);
}
console.log("Part 2:", active.length);
