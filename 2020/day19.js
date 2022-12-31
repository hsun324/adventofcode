var input = document.body.innerText.trim().split('\n\n');
var rules = Object.fromEntries(input[0].split('\n').map((s) => {
    var match = s.match(/^(.+): (?:"(.)"|(\d.*))$/);
    if (match[2]) return [match[1], match[2]];
    if (match[3]) return [match[1], match[3].split(' | ').map((s) => s.split(' '))];
}));
var messages = input[1].split('\n');
var expand = (key) => {
    if (typeof rules[key] === 'string') return rules[key];
    if (typeof rules[key] === 'object') return ["(?:", rules[key].flatMap((opt) => [opt.map(expand), "|"]).slice(0, -1), ")"];
};
var regex = new RegExp(["^", expand(0), "$"].flat(1e9).join(''));
console.log("Part 1:", messages.filter((s) => s.match(regex)).length);
// Cheat around having to match balanced pairs using regex by just adding pairs up to depth 10.
var expand = (key, depth) => {
    if (depth > 10) return [];
    if (key == '8') return [expand(42), "+"];
    if (key == '11') return [expand(42), "(", expand(11, (depth || 0) + 1), ")", "?", expand(31)];
    if (typeof rules[key] === 'string') return rules[key];
    if (typeof rules[key] === 'object') return ["(?:", rules[key].flatMap((opt) => [opt.map(expand), "|"]).slice(0, -1), ")"];
};
var regex = new RegExp(["^", expand(0), "$"].flat(1e9).join(''));
console.log("Part 2:", messages.filter((s) => s.match(regex)).length);
