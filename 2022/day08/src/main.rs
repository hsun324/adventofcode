fn fill<T, Rx, Ry>(grid: &Vec<Vec<T>>, visible: &mut Vec<Vec<bool>>, rx: Rx, ry: Ry, iz: T)
where
    T: PartialOrd,
    Rx: IntoIterator<Item = usize>,
    Ry: IntoIterator<Item = usize> + Clone,
{
    let mut sz = &iz;
    for x in rx {
        for y in ry.clone() {
            let gz = &grid[x][y];
            if gz > sz {
                visible[x][y] = true;
                sz = gz;
            }
        }
    }
}

fn find<T, Rx, Ry>(grid: &Vec<Vec<T>>, rx: Rx, ry: Ry, z: T) -> usize
where
    T: PartialOrd,
    Rx: IntoIterator<Item = usize>,
    Ry: IntoIterator<Item = usize> + Clone,
{
    let mut visited = 0;
    for x in rx {
        for y in ry.clone() {
            visited += 1;
            if grid[x][y] >= z {
                return visited;
            }
        }
    }
    visited
}

fn main() {
    let grid: Vec<_> = std::io::stdin()
        .lines()
        .flatten()
        .map(|x| {
            x.as_bytes()
                .iter()
                .map(|b| (b - b'0') as i8)
                .collect::<Vec<i8>>()
        })
        .collect();

    let mut visible: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    for i in 0..grid.len() {
        fill(&grid, &mut visible, i..=i, 0..grid[i].len(), -1);
        fill(&grid, &mut visible, i..=i, (0..grid[i].len()).rev(), -1);
        fill(&grid, &mut visible, 0..grid[i].len(), i..=i, -1);
        fill(&grid, &mut visible, (0..grid[i].len()).rev(), i..=i, -1);
    }
    let part1: usize = visible.iter().flatten().map(|b| *b as usize).sum();
    println!("Part 1: {}", part1);

    let mut part2 = 0;
    for x in 1..grid.len() - 1 {
        for y in 1..grid[x].len() - 1 {
            part2 = std::cmp::max(
                part2,
                find(&grid, x..=x, (0..y).rev(), grid[x][y])
                    * find(&grid, x..=x, y + 1..grid[x].len(), grid[x][y])
                    * find(&grid, (0..x).rev(), y..=y, grid[x][y])
                    * find(&grid, x + 1..grid.len(), y..=y, grid[x][y]),
            );
        }
    }
    println!("Part 2: {}", part2);
}
