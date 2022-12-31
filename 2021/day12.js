var input = document.body.innerText.trim().split('\n').map((s) => s.split('-'));
var caves = [...new Set(input.flat())];
var links = Object.fromEntries(caves.map((cave) => [cave, []]));
input.forEach((x) => { links[x[0]].push(x[1]); links[x[1]].push(x[0]); });
var compute = (cave, path) => {
    if (cave == 'end') return 1;
    if (cave.toLowerCase() == cave && path.indexOf(cave) != -1) return 0;
    var newPath = [...path, cave];
    return links[cave].map((linked) => compute(linked, newPath)).reduce((a, b) => a + b);
};
console.log("Part 1:", compute('start', []));
var compute = (cave, path, twice) => {
    if (cave == 'end') return 1;
    if (cave.toLowerCase() == cave && path.indexOf(cave) != -1) {
        if (cave == 'start' || cave == 'end' || twice) return 0;
        twice = true;
    }
    var newPath = [...path, cave];
    return links[cave].map((linked) => compute(linked, newPath, twice)).reduce((a, b) => a + b);
};
console.log("Part 2:", compute('start', [], false));
