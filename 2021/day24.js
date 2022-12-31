var input = document.body.innerText.trim().split('\nmul y x\nadd z y\n').map((s) => {
    var ss = s.split('\n').map((s2) => s2.split(' '));
    return [ss[4][2] == 1, ss[5][2] - 0, ss[15][2] - 0];
});
var pairs = [];
var stack = [];
input.forEach((v, i) => {
    if (v[0]) stack.push(i);
    else pairs[stack.pop(i)] = i;
});
var saved = [];
var maximum = input.map((v, i) => {
    var digit = v[0] ? Math.min(9, 9 - input[pairs[i]][1] - v[2]) : saved[i] + v[1];
    if (v[0]) saved[pairs[i]] = digit + v[2];
    return digit;
}).join('');
console.log("Part 1:", maximum);
var minimum = input.map((v, i) => {
    var digit = v[0] ? Math.max(1, 1 - input[pairs[i]][1] - v[2]) : saved[i] + v[1];
    if (v[0]) saved[pairs[i]] = digit + v[2];
    return digit;
}).join('');
console.log("Part 2:", minimum);
