var input = document.body.innerText.trim().split('\n').map((s) => {
    var [_, ing, alg] = s.match(/^(.+) \(contains (.+)\)$/);
    return {ingredients: ing.split(' '), allergens: alg.split(', ')};
});
var allergens = {};
input.forEach((label) => label.allergens.forEach((allergen) => {
    allergens[allergen] = allergens[allergen]
        ? allergens[allergen].filter((ingredient) => label.ingredients.indexOf(ingredient) >= 0)
        : label.ingredients;
}));
var possible = Object.fromEntries(Object.values(allergens).flat().map((ingredient) => [ingredient, true]));
var result = input.map((label) => label.ingredients).flat().filter((ingredient) => !possible[ingredient]).length;
console.log("Part 1:", result);
var names = Object.keys(allergens).sort();
var assignment = {};
while (Object.keys(assignment).length < names.length)
    Object.entries(allergens).filter((allergen) => allergen[1].length == 1).forEach((allergen) => {
        var ingredient = allergen[1][0];
        assignment[allergen[0]] = ingredient;
        Object.entries(allergens).forEach((allergen) => {
            var index = allergen[1].indexOf(ingredient);
            if (index >= 0) allergen[1].splice(index, 1);
        });
    });
var result = names.map((name) => assignment[name]).join(',');
console.log("Part 2:", result);
