#[aoc_runner::timeit]
fn main() -> usize {
    let input = aoc::read_input(5);
    let mut lines = input.split("\n\n");

    let seeds: Vec<_> = lines
        .next()
        .unwrap_or("")
        .split(": ")
        .last()
        .unwrap_or("")
        .split(' ')
        .map(|f| f.parse::<usize>().unwrap_or(0))
        .collect();

    let mut locations = Vec::new();

    for seed in &seeds {
        let mut location = *seed;
        for translation in lines.clone() {
            let curr_translation: Vec<_> = translation
                .split(":\n")
                .skip(1)
                .flat_map(|f| f.split('\n'))
                .map(|x| {
                    x.split(' ')
                        .map(|f| f.parse::<usize>().unwrap_or(0))
                        .collect::<Vec<_>>()
                })
                .collect();

            location = curr_translation
                .iter()
                .find_map(|translation| {
                    if translation[1] <= location && location <= translation[1] + translation[2] {
                        let offset = location - translation[1];
                        Some(translation[0] + offset)
                    } else {
                        None
                    }
                })
                .unwrap_or(location);
        }

        locations.push(location);
    }

    *locations.iter().min().unwrap_or(&0)
}
