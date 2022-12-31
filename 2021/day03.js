var input = document.body.innerText.trim().split('\n');
var freq = Array.from(input[0]).map((x, i) => input.reduce((a, v) => { a[v[i]]++; return a }, {'0': 0, '1': 0}));
var gamma = Array.from(input[0]).map((x, i) => freq[i][0] > freq[i][1] ? "0" : "1").join('');
var delta = Array.from(input[0]).map((x, i) => freq[i][0] > freq[i][1] ? "1" : "0").join('');
console.log("Part 1:", parseInt(gamma, 2) * parseInt(delta, 2));
var oxy = Array.from(input[0]).reduce((rem, x, i) => {
    var freq = rem.reduce((a, v) => { a[v[i]]++; return a }, {'0': 0, '1': 0})
    var req = freq[0] > freq[1] ? "0" : "1";
    return rem.length == 1 ? rem : rem.filter((v) => v[i] == req);
}, input)[0];
var co2 = Array.from(input[0]).reduce((rem, x, i) => {
    var freq = rem.reduce((a, v) => { a[v[i]]++; return a }, {'0': 0, '1': 0})
    var req = freq[0] > freq[1] ? "1" : "0";
    return rem.length == 1 ? rem : rem.filter((v) => v[i] == req);
}, input)[0];
console.log("Part 2:", parseInt(oxy, 2) * parseInt(co2, 2));
