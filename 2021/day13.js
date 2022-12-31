var input = document.body.innerText.trim().split('\n\n');
var points = input[0].split('\n').map((s) => s.split(',').map((s) => parseInt(s)));
var folds = input[1].split('\n').map((s) => { var m = s.match(/^fold along (.)=(.+)$/); return {axis: m[1], position: parseInt(m[2])}; });
var fold = (points, fold) => {
    if (fold.axis == 'x') return points.map((p) => [p[0] > fold.position ? 2 * fold.position - p[0] : p[0], p[1]]);
    if (fold.axis == 'y') return points.map((p) => [p[0], p[1] > fold.position ? 2 * fold.position - p[1] : p[1]]);
};
console.log("Part 1:", Object.keys(Object.fromEntries(fold(points, folds[0]).map((p) => [p, true]))).length);
var result = Object.fromEntries(folds.reduce(fold, points).map((p) => [p, true]));
console.log("Part 2:");
console.log(Array(6).fill().map(($, y) => Array(40).fill().map(($, x) => result[[x, y]] ? '█' : ' ').join('')).join('\n'));
