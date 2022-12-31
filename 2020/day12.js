var input = document.body.innerText.trim().split('\n').map((s) => ({act: s[0], val: parseInt(s.slice(1))}));
var ec = [[0, 1], [1, 0], [0, -1], [-1, 0]];
var pos = {x: 0, y: 0, d: 0};
input.forEach((ins) => {
    if (ins.act == 'N') pos.x += ins.val;
    if (ins.act == 'S') pos.x -= ins.val;
    if (ins.act == 'E') pos.y += ins.val;
    if (ins.act == 'W') pos.y -= ins.val;
    if (ins.act == 'L') pos.d += ins.val / 90;
    if (ins.act == 'R') pos.d -= ins.val / 90;
    if (ins.act == 'F') {
        pos.x += ins.val * ec[(pos.d % 4 + 4) % 4][0];
        pos.y += ins.val * ec[(pos.d % 4 + 4) % 4][1];
    }
});
console.log("Part 1:", Math.abs(pos.x) + Math.abs(pos.y));
var pos = {wx: 1, wy: 10, x: 0, y: 0};
input.forEach((ins) => {
    if (ins.act == 'N') pos.wx += ins.val;
    if (ins.act == 'S') pos.wx -= ins.val;
    if (ins.act == 'E') pos.wy += ins.val;
    if (ins.act == 'W') pos.wy -= ins.val;
    if (ins.act == 'L')
        for (var n = 0; n < ins.val; n += 90)
            [pos.wx, pos.wy] = [pos.wy, -pos.wx];
    if (ins.act == 'R')
        for (var n = 0; n < ins.val; n += 90)
            [pos.wx, pos.wy] = [-pos.wy, pos.wx];
    if (ins.act == 'F') {
        pos.x += ins.val * pos.wx;
        pos.y += ins.val * pos.wy;
    }
});
console.log("Part 2:", Math.abs(pos.x) + Math.abs(pos.y));
