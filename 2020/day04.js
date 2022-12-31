var input = document.body.innerText.trim().split('\n\n').map((s1) => Object.fromEntries(s1.replaceAll('\n', ' ').split(' ').map((s2) => s2.split(':'))))
console.log("Part 1:", input.filter((o) => o.byr && o.iyr && o.eyr && o.hgt && o.hcl && o.ecl && o.pid).length);
console.log("Part 2:", input.filter((o) =>
    o.byr && o.byr.match(/^19[2-9].|200[0-2]$/) &&
    o.iyr && o.iyr.match(/^201.|2020$/) &&
    o.eyr && o.eyr.match(/^202.|2030$/) &&
    o.hgt && o.hgt.match(/^(1[5-8].|19[0-3])cm|(59|6.|7[0-6])in$/) &&
    o.hcl && o.hcl.match(/^#[a-z0-9]{6}$/) &&
    o.ecl && o.ecl.match(/^amb|blu|brn|gry|grn|hzl|oth$/) &&
    o.pid && o.pid.match(/^[0-9]{9}$/)).length)
