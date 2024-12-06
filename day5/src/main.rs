use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = read_file("input.txt");

    let (page_rules, manual_updates) = parse_contents(&contents);

    let page_rules_map = build_page_rules_map(&page_rules);
    let (median_sum, rectified_median_sum) = process_manual_updats(&manual_updates, &page_rules_map);

    println!(
        "The sum of the middle page numbers is:\n\t- Correct updates: {}\n\t- Rectified updates: {}",
        median_sum, rectified_median_sum
    );
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

fn parse_contents(contents: &str) -> (Vec<&str>, Vec<&str>) {
    let parts: Vec<&str> = contents.split("\n\n").collect();
    let page_rules = parts.get(0).unwrap().lines().collect();
    let manual_updates = parts.get(1).unwrap().lines().collect();
    (page_rules, manual_updates)
}

fn build_page_rules_map(page_rules: &[&str]) -> HashMap<usize, Vec<usize>> {
    let mut page_rules_map = HashMap::new();
    for rule in page_rules {
        let parts: Vec<&str> = rule.split('|').collect();
        if let (Ok(page), Ok(preceeds)) = (parts[0].parse(), parts[1].parse()) {
            page_rules_map.entry(page).or_insert_with(Vec::new).push(preceeds);
        }
    }
    page_rules_map
}

fn process_manual_updats(manual_updates: &[&str], page_rules_map: &HashMap<usize, Vec<usize>>) -> (usize, usize) {
    let mut median_sum = 0;
    let mut rectified_median_sum = 0;

    for update in manual_updates {
        match parse_manual_update(update) {
            Ok(pages) => {
                let(median, rectified) = analyze_pages(&pages, page_rules_map);
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

    (median_sum, rectified_median_sum)
}

fn parse_manual_update(update: &str) -> Result<Vec<usize>, std::num::ParseIntError> {
    update.split(',').map(|c| c.trim().parse::<usize>()).collect()
}

fn analyze_pages(pages: &[usize], page_rules_map: &HashMap<usize, Vec<usize>>) -> (usize, bool) {
    let mut pages_read = Vec::new();
    let mut rectified = false;

    for &page in pages {
        if wrong_order(page, &mut pages_read, page_rules_map) {
            rectified = true;
        } else {
            pages_read.push(page);
        }
    }

    let median_index = pages_read.len() / 2;
    let median = *pages_read.get(median_index).unwrap();
    (median, rectified)
}

fn wrong_order(page: usize, pages_read: &mut Vec<usize>, page_rules_map: &HashMap<usize, Vec<usize>>) -> bool {
    if let Some(preceeds) = page_rules_map.get(&page) {
        for (index, &page_read) in pages_read.iter().enumerate() {
            if preceeds.contains(&page_read) {
                pages_read.insert(index, page);
                return true;
            }
        }
    }
    false
}
