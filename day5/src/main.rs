use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let contents: Vec<&str> = contents.split("\n\n").collect();
    let page_rules: Vec<&str> = contents.get(0).unwrap().split("\n").collect();

    let mut page_rules_map: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut rectified_median_sum = 0;
    let mut median_sum = 0;

    for page_rule in page_rules {
        let page_rule: Vec<&str> = page_rule.split("|").collect();
        let page: usize = page_rule.get(0).unwrap().parse::<usize>().unwrap();
        let preceeds: usize = page_rule.get(1).unwrap().parse::<usize>().unwrap();

        page_rules_map.entry(page).or_insert_with(Vec::new).push(preceeds);
    }

    let manual_updates: Vec<&str> = contents.get(1).unwrap().split("\n").collect();

    for manual_update in manual_updates {
        let pages: Result<Vec<usize>, _> = manual_update
            .split(',')
            .map(|c| c.trim().parse::<usize>())
            .collect();

        match pages {
            Ok(pages) => {
                let mut pages_read: Vec<usize> = vec![];
                let mut rectified = false;
                for page in pages {
                    if wrong_order(page, &mut pages_read, &page_rules_map) {
                        rectified = true;
                    } else {
                        pages_read.push(page);
                    }
                }

                let median_index = pages_read.len() / 2;
                let median = pages_read.get(median_index).unwrap();
                if rectified {
                    rectified_median_sum += median;
                } else {
                    median_sum += median;
                }
            }
            Err(e) => {
                eprintln!("failed to parse numbers: {}", e);
            }
        }
    }

    println!("The sum of the middle page numbers is:\n\t- Correct updates: {}\n\t- Rectified updates: {}", median_sum, rectified_median_sum);
}

fn wrong_order(page: usize, pages_read: &mut Vec<usize>, page_rules_map: &HashMap<usize, Vec<usize>>) -> bool {
    match page_rules_map.get(&page) {
        Some(preceeds) => {
            for (index, &page_read) in pages_read.iter().enumerate() {
                if preceeds.contains(&page_read) {
                    pages_read.insert(index, page);

                    return true;
                }
            }
        }
        None => ()
    }

    false
}
