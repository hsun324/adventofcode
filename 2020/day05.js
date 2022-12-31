var input = document.body.innerText.trim().split('\n').map((s) =>
    parseInt(s.slice(0, 7).split('').map((c) => c == 'B' ? '1' : '0').join(''), 2) * 8 + parseInt(s.slice(7).split('').map((c) => c == 'R' ? '1' : '0').join(''), 2));
console.log("Part 1:", Math.max(...input));
console.log("Part 2:", input.find((x) => input.indexOf(x) != -1 && input.indexOf(x + 1) == -1 && input.indexOf(x + 2) != -1) + 1);
