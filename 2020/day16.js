var inputRaw = document.body.innerText.trim().split('\n\n');
var validityRaw = inputRaw[0].split('\n');
var validity = validityRaw.map((s) => {
    var p = s.split(': ');
    var m = p[1].match(/^(.+)-(.+) or (.+)-(.+)$/);
    return {
        name: p[0],
        ranges: [[parseInt(m[1]), parseInt(m[2])], [parseInt(m[3]), parseInt(m[4])]],
        pos: Array(validityRaw.length).fill().map(($, i) => i)
    };
});
var others = inputRaw[2].split('\n').slice(1).map((s) => s.split(',').map((s) => parseInt(s, 10)));
var valid = (field, value) => value >= field.ranges[0][0] && value <= field.ranges[0][1] || value >= field.ranges[1][0] && value <= field.ranges[1][1];
console.log("Part 1:", others.flat().filter((value) => !validity.find((field) => valid(field, value))).reduce((a, b) => a + b));
others.filter((values) => !values.find((value) => !validity.find((field) => valid(field, value)))).forEach((values) => values.forEach((value, i) =>
    validity.filter((field) => !valid(field, value)).forEach((field) => field.pos = field.pos.filter((x) => x != i))));
validity.sort((a, b) => a.pos.length - b.pos.length).reduce((found, field) => {
    field.pos = field.pos.filter((p) => found.indexOf(p) == -1)[0];
    found.push(field.pos);
    return found;
}, []);
var self = inputRaw[1].split('\n')[1].split(',').map((s) => parseInt(s, 10));
console.log("Part 2:", validity.filter((field) => field.name.startsWith('departure')).map((field) => field.pos).map((x) => self[x]).reduce((a, b) => a * b));
