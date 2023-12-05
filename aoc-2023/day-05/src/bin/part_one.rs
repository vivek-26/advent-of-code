fn main() {
    let input = aoc::read_input(5);
    let mut lines = input.split("\n\n");

    let seeds: Vec<_> = lines
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(' ')
        .map(|f| f.parse::<usize>().unwrap())
        .collect();

    let translations: Vec<_> = lines
        .map(|translation| {
            translation
                .split(":\n")
                .skip(1)
                .flat_map(|f| f.split('\n'))
                .map(|x| {
                    x.split(' ')
                        .map(|f| f.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let locations: Vec<_> = seeds
        .iter()
        .map(|seed| {
            translations
                .iter()
                .fold(*seed, |curr_id, curr_translation| {
                    curr_translation
                        .iter()
                        .find_map(|translation| {
                            if translation[1] <= curr_id
                                && curr_id <= translation[1] + translation[2]
                            {
                                let offset = curr_id - translation[1];
                                Some(translation[0] + offset)
                            } else {
                                None
                            }
                        })
                        .unwrap_or(curr_id)
                })
        })
        .collect();

    let answer = locations.iter().min().unwrap();
    println!("answer: {}", answer);
}
