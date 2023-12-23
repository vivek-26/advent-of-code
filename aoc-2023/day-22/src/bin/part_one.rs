use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

#[aoc_runner::main(22)]
fn main(input: &str) -> usize {
    let mut bricks = input
        .split('\n')
        .map(|line| {
            line.split(|c: char| !c.is_ascii_digit())
                .map(|w| w.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<_>>();

    bricks.sort_by_key(|&(_, _, z1, _, _, _)| z1);

    let mut adjacent = vec![(HashSet::new(), HashSet::new()); bricks.len()];
    let mut space = HashMap::new();
    for i in 0..bricks.len() {
        let (x1, y1, mut z1, x2, y2, mut z2) = bricks[i];
        while z1 > 1
            && (x1..=x2)
                .cartesian_product(y1..=y2)
                .all(|(x, y)| !space.contains_key(&(x, y, z1 - 1)))
        {
            z2 -= 1;
            z1 -= 1;
        }

        for (x, y) in (x1..=x2).cartesian_product(y1..=y2) {
            for z in z1..=z2 {
                space.insert((x, y, z), i);
            }
            if let Some(&j) = space.get(&(x, y, z1 - 1)) {
                adjacent[j].0.insert(i);
                adjacent[i].1.insert(j);
            }
        }

        bricks[i] = (x1, y1, z1, x2, y2, z2);
    }

    let mut falling = HashSet::new();
    let mut answer = 0;
    for b in 0..bricks.len() {
        falling.clear();
        disintegrate_all(&adjacent, &mut falling, b);
        answer += (falling.len() == 1) as usize;
    }

    answer
}

fn disintegrate_all(
    adjacent: &[(HashSet<usize>, HashSet<usize>)],
    falling: &mut HashSet<usize>,
    i: usize,
) {
    falling.insert(i);
    for &above in &adjacent[i].0 {
        if adjacent[above].1.iter().all(|x| falling.contains(x)) {
            disintegrate_all(adjacent, falling, above);
        }
    }
}
