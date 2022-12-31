var inputRaw = document.body.innerText.trim().split('\n');
var min = parseInt(inputRaw[0], 10);
var buses = inputRaw[1].split(',').map((s, i) => ({off: i, id: parseInt(s, 10)})).filter((s) => !isNaN(s.id)).sort((a, b) => a.id - b.id);
var opts = buses.map((bus) => ({on: Math.ceil(min / bus.id) * bus.id, id: bus.id})).sort((a, b) => a.on - b.on);
console.log("Part 1:", opts[0].id * (opts[0].on - min));
var busN = buses.reduce((prod, bus) => prod * bus.id, 1);
var inv = (a, n) => Array(n).fill().map(($, i) => i).find((i) => (a * i) % n == 1);
console.log("Part 2:", buses.map((bus) => inv(busN / bus.id, bus.id) * (busN / bus.id) * ((((bus.id - bus.off) % bus.id) + bus.id) % bus.id) % busN).reduce((a, b) => a + b) % busN - 1);
