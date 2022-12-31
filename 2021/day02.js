var input = document.body.innerText.trim().split('\n').map((s) => {
    var p = s.split(' ');
    return {op: p[0][0], arg: parseInt(p[1], 10)};
});
var e1 = input.reduce((p, a) => {
    if (a.op == 'f') return {x: p.x + a.arg, y: p.y};
    if (a.op == 'd') return {x: p.x, y: p.y + a.arg};
    if (a.op == 'u') return {x: p.x, y: p.y - a.arg};
}, {x: 0, y: 0});
console.log("Part 1:", e1.x * e1.y);
var e2 = input.reduce((p, a) => {
    if (a.op == 'f') return {x: p.x + a.arg, y: p.y + p.aim * a.arg, aim: p.aim};
    if (a.op == 'd') return {x: p.x, y: p.y, aim: p.aim + a.arg};
    if (a.op == 'u') return {x: p.x, y: p.y, aim: p.aim - a.arg};
}, {x: 0, y: 0, aim: 0});
console.log("Part 2:", e2.x * e2.y);
