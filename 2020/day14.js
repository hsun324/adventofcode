var input = document.body.innerText.trim().split('\n').map((s) => {
    var match;
    if (match = s.match(/^mask = (.+)$/)) return {op: 'mask', mask: match[1]};
    if (match = s.match(/^mem\[([0-9]+)\] = ([0-9]+)$/)) return {op: 'write', addr: parseInt(match[1]), value: parseInt(match[2])};
});
var app = (mask, val) => parseInt(val.toString(2).padStart(36, '0').split('').map((c, i) => mask[i] == 'X' ? c : mask[i]).join(''), 2)
var mem = {};
input.reduce((mask, ins) => {
    if (ins.op == 'mask') return ins.mask;
    if (ins.op == 'write') mem[ins.addr] = app(mask, ins.value);
    return mask;
}, '');
console.log("Part 1:", Object.values(mem).reduce((a, b) => a + b));
var appR = (maskR, valR) => {
    if (valR.length == 0) return [[]];
    var sub = appR(maskR.slice(1), valR.slice(1));
    if (maskR[0] == 'X') return sub.flatMap((s) => [['0', ...s], ['1', ...s]]);
    if (maskR[0] == '0') return sub.map((s) => [valR[0], ...s]);
    if (maskR[0] == '1') return sub.map((s) => ['1', ...s]);
};
var app = (mask, val) => appR(mask, val.toString(2).padStart(36, '0').split('')).map((s) => parseInt(s.join(''), 2));
var mem = {};
input.reduce((mask, ins) => {
    if (ins.op == 'mask') return ins.mask;
    if (ins.op == 'write') app(mask, ins.addr).forEach((addr) => mem[addr] = ins.value);
    return mask;
}, '');
console.log("Part 2:", Object.values(mem).reduce((a, b) => a + b));
