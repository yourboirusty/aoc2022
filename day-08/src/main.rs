use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn load_data(path: &str) -> Vec<Vec<u8>> {
    let file = File::open(path).expect("Couldn't open file");
    let buf = BufReader::new(file);

    let mut forest: Vec<Vec<u8>> = Vec::new();

    for line in buf.lines().map(|l| l.expect("Parsing error")) {
        forest.push(
            line.chars()
                .map(|c| c.to_digit(10).expect("NaN") as u8)
                .collect(),
        )
    }

    forest
}

fn visible_sides(tree: (usize, usize), forest: &Vec<Vec<u8>>) -> u8 {
    let tree_value = forest[tree.0][tree.1];
    let mut visible_sides: u8 = 0;

    let edges = [
        (0..tree.0, tree.1..(tree.1 + 1)), //
        (tree.0..forest.len(), tree.1..(tree.1 + 1)),
        (tree.0..(tree.0 + 1), tree.1..forest.len()),
        (tree.0..(tree.0 + 1), 0..tree.1),
    ];

    for edge in edges {
        let mut hidden = false;
        for row_idx in edge.0.clone() {
            for col_idx in edge.1.clone() {
                if (row_idx, col_idx) == tree {
                    continue;
                }
                let other_tree = forest[row_idx][col_idx];
                if tree_value <= other_tree {
                    hidden = true;
                    break;
                }
            }
            if hidden {
                break;
            };
        }
        if !hidden {
            visible_sides += 1;
        }
    }

    visible_sides
}
fn look_at_trees(forest: &Vec<Vec<u8>>) -> u32 {
    let mut trees_visible = (forest.len() * 2 + (forest[0].len() - 2) * 2) as u32;

    for row_idx in 1..forest.len() - 1 {
        for col_idx in 1..forest[row_idx].len() - 1 {
            trees_visible += (visible_sides((row_idx, col_idx), &forest) > 0) as u32;
        }
    }
    trees_visible
}

fn main() {
    let forest = load_data("./data/input");
    println!("{:?}", look_at_trees(&forest));
}
