use std::ops::Range;

#[aoc_runner::main(5)]
fn main(input: &str) -> i128 {
    let lines: Vec<_> = input.split("\n\n").collect();

    let seeds_ranges = parse_seeds_ranges(lines.first().unwrap());
    let translations = parse_translations(&lines[1..]);

    let mut ranges = seeds_ranges;
    for stage in translations {
        ranges = process_stage(ranges, &stage);
    }

    ranges.sort_by_key(|range| range.start);

    ranges.first().unwrap().start
}

fn parse_seeds_ranges(input: &str) -> Vec<Range<i128>> {
    input
        .split(": ")
        .last()
        .unwrap()
        .split(' ')
        .map(|f| f.parse::<i128>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|f| Range {
            start: f[0],
            end: f[0] + f[1],
        })
        .collect()
}

fn parse_translations(lines: &[&str]) -> Vec<Vec<(i128, i128, i128)>> {
    lines
        .iter()
        .map(|translation| {
            let mut a = translation
                .split(":\n")
                .skip(1)
                .flat_map(|f| f.split('\n'))
                .map(parse_translation)
                .collect::<Vec<_>>();
            a.sort_by_key(|a| a.1);
            a
        })
        .collect()
}

fn parse_translation(line: &str) -> (i128, i128, i128) {
    let parts: Vec<_> = line
        .split(' ')
        .map(|f| f.parse::<i128>().unwrap())
        .collect();
    (parts[0], parts[1], parts[2])
}

fn process_stage(ranges: Vec<Range<i128>>, stage: &[(i128, i128, i128)]) -> Vec<Range<i128>> {
    let mut new_ranges = vec![];

    for range in ranges {
        let mut curr = range;
        for translation in stage {
            let offset = translation.0 - translation.1;
            if curr.start <= curr.end
                && curr.start < translation.1 + translation.2
                && translation.1 <= curr.end
            {
                curr = process_translation(curr, translation, offset, &mut new_ranges);
            }
        }
        if curr.start <= curr.end {
            new_ranges.push(curr);
        }
    }

    new_ranges
}

fn process_translation(
    mut curr: Range<i128>,
    translation: &(i128, i128, i128),
    offset: i128,
    new_ranges: &mut Vec<Range<i128>>,
) -> Range<i128> {
    if curr.start < translation.1 {
        new_ranges.push(Range {
            start: curr.start,
            end: translation.1 - 1,
        });
        curr.start = translation.1;
        if curr.end < translation.1 + translation.2 {
            new_ranges.push(Range {
                start: curr.start + offset,
                end: curr.end + offset,
            });
            curr.start = curr.end + 1;
        } else {
            new_ranges.push(Range {
                start: curr.start + offset,
                end: translation.1 + translation.2 - 1 + offset,
            });
            curr.start = translation.1 + translation.2;
        }
    } else if curr.end < translation.1 + translation.2 {
        new_ranges.push(Range {
            start: curr.start + offset,
            end: curr.end + offset,
        });
        curr.start = curr.end + 1;
    } else {
        new_ranges.push(Range {
            start: curr.start + offset,
            end: translation.1 + translation.2 - 1 + offset,
        });
        curr.start = translation.1 + translation.2;
    }

    curr
}
