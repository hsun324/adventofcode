var input = document.body.innerText.trim().split('\n\n').map((s) => s.split('\n').slice(1).map((s) => s.split(',').map((s) => parseInt(s))));
var rots = [
    (p) => [ p[0],  p[1],  p[2]], //   0
    (p) => [ p[0], -p[2],  p[1]], //  90
    (p) => [ p[0], -p[1], -p[2]], // 180
    (p) => [ p[0],  p[2], -p[1]], // 270
];
var axes = [
    (p) => [ p[0],  p[1],  p[2]], // +x
    (p) => [-p[0],  p[1], -p[2]], // -x
    (p) => [-p[1],  p[0],  p[2]], // +y
    (p) => [ p[1], -p[0],  p[2]], // -y
    (p) => [ p[2],  p[1], -p[0]], // +z
    (p) => [-p[2],  p[1],  p[0]], // -z
];
var perms = axes.flatMap((axis) => rots.flatMap((rot) => (p) => rot(axis(p))));
var join = (x, y) => {
    var xmap = Object.fromEntries(x.map((x) => [x, true]));
    for (var p = 0; p < perms.length; p++) {
        var points = y.map(perms[p]);
        for (var i = 0; i < x.length; i++) {
            for (var j = 0; j < y.length; j++) {
                var r = x[i];
                var c = points[j];
                var k = [r[0] - c[0], r[1] - c[1], r[2] - c[2]];
                var offset = points.map((p) => [p[0] + k[0], p[1] + k[1], p[2] + k[2]]);
                var m = offset.filter((p) => xmap[p]);
                if (m.length >= 12) {
                    return [k, offset];
                }
            }
        }
    }
    return [false, []];
};
var start = performance.now();
var pos = Array(input.length);
var points = Array(input.length);
pos[0] = [0, 0, 0];
points[0] = input[0];
var resolve = (current) => {
    input.forEach((y, i) => {
        if (points[i]) return;
        var [off, result] = join(points[current], y);
        if (off) {
            console.log("Solved " + i + ": [" + off + "] after " + (performance.now() - start).toFixed(2) + "ms");
            pos[i] = off;
            points[i] = result;
            resolve(i);
        }
    });
};
resolve(0);
var beacons = Object.fromEntries(points.flat().map((x) => [x, true]));
var result = Object.keys(beacons).length;
console.log("Part 1:", result, "-", (performance.now() - start).toFixed(2) + "ms");
var start = performance.now();
var max = Math.max(...pos.flatMap((x) => pos.map((y) => Math.abs(x[0] - y[0]) + Math.abs(x[1] - y[1]) + Math.abs(x[2] - y[2]))));
console.log("Part 2:", max, "-", (performance.now() - start).toFixed(2) + "ms");

