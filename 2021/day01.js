var input = document.body.innerText.trim().split('\n').map((s) => parseInt(s, 10))
console.log("Part 1:", input.slice(1).filter((v, i) => v > input[i]).length)
console.log("Part 2:", input.slice(3).filter((v, i) => v > input[i]).length)
