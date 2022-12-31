var input = Object.fromEntries(document.body.innerText.trim().replaceAll(' bags', '').replaceAll(' bag', '').replaceAll('.', '').split('\n').map((s) => {
    var sp = s.split(' contain ');
    var sp2 = sp[1] == 'no other' ? {} : Object.fromEntries(sp[1].split(', ').map((s2) => {
        var sp3 = s2.split(' ');
        return [sp3.slice(1).join(' '), parseInt(sp3[0], 10)];
    }));
    return [sp[0], sp2];
}));
var has = {};
for (var i = 0; i < 100; i++)
    Object.entries(input).filter((c) => Object.keys(c[1]).find((x) => x == 'shiny gold' || has[x])).forEach((c) => has[c[0]] = true);
console.log("Part 1:", Object.keys(has).length);
var recurse = (cur) => Object.entries(input[cur]).reduce((sum, sub) => sum + sub[1] * (1 + recurse(sub[0])), 0);
console.log("Part 2:", recurse('shiny gold'));
