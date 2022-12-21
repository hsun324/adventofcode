fn main() {
    let heights: Vec<Vec<_>> = std::io::stdin()
        .lines()
        .flatten()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|b| match b {
                    b'S' => 0,
                    b'E' => 27,
                    _ => b - b'a' + 1,
                })
                .collect()
        })
        .collect();

    let start = heights
        .iter()
        .enumerate()
        .flat_map(|(x, r)| std::iter::repeat(x).zip(r.into_iter()).enumerate())
        .find(|(_, (_, &r))| r == 27)
        .map(|(y, (x, _))| (x, y))
        .unwrap();

    let mut part1 = 0;
    let mut part2 = 100000;
    let mut distance: Vec<_> = heights.iter().map(|r| vec![0; r.len()]).collect();
    let mut queue = std::collections::VecDeque::from_iter([(1, start)]);
    while let Some((dist, (x, y))) = queue.pop_front() {
        if distance[x][y] > 0 {
            continue;
        }
        if heights[x][y] == 0 {
            part1 = dist - 1;
        }
        if heights[x][y] <= 1 && dist - 1 < part2 {
            part2 = dist - 1;
        }
        distance[x][y] = dist;
        let allowed = std::cmp::max(1, heights[x][y]) - 1;
        if x > 0 && heights[x - 1][y] >= allowed {
            queue.push_back((dist + 1, (x - 1, y)))
        }
        if x + 1 < heights.len() && heights[x + 1][y] >= allowed {
            queue.push_back((dist + 1, (x + 1, y)))
        }
        if y > 0 && heights[x][y - 1] >= allowed {
            queue.push_back((dist + 1, (x, y - 1)))
        }
        if y + 1 < heights[x].len() && heights[x][y + 1] >= allowed {
            queue.push_back((dist + 1, (x, y + 1)))
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
