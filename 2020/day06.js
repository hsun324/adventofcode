var input = document.body.innerText.trim().split('\n\n').map((s) => s.split('\n'));
console.log("Part 1:", input.map((g) => Object.entries(g.join('').split('').reduce((a, b) => { a[b] = true; return a; }, {})).length).reduce((a, b) => a + b));
console.log("Part 2:", input.map((g) => Object.entries(g.join('').split('').reduce((a, b) => { a[b] = (a[b] || 0) + 1; return a; }, {}))
                                                        .filter((x) => x[1] == g.length).length).reduce((a, b) => a + b));

