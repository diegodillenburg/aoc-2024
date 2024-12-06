use std::collections::HashMap;
use std::fs;

fn main() {
    let processor = PageProcessor::new("input.txt");
    processor.process_updates();
}

struct PageRule {
    page: usize,
    preceeds: Vec<usize>,
}

impl PageRule {
    fn new(page: usize, preceeds: Vec<usize>) -> Self {
        Self { page, preceeds }
    }
}

struct ManualUpdate {
    pages: Vec<usize>,
}

impl ManualUpdate {
    fn new(pages: Vec<usize>) -> Self {
        Self { pages }
    }

    fn analyze(&self, page_rules_map: &HashMap<usize, Vec<usize>>) -> (usize, bool) {
        let mut pages_read = Vec::new();
        let mut rectified = false;

        for &page in &self.pages {
            if ManualUpdate::wrong_order(page, &mut pages_read, page_rules_map) {
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

}

struct PageProcessor {
    page_rules_map: HashMap<usize, Vec<usize>>,
    manual_updates: Vec<ManualUpdate>,
}

impl PageProcessor {
    fn new(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let (page_rules, manual_updates) = PageProcessor::parse_contents(&contents);

        let page_rules_map = PageProcessor::build_page_rules_map(&page_rules);
        let manual_updates = manual_updates
            .into_iter()
            .map(ManualUpdate::new)
            .collect();

        PageProcessor { page_rules_map, manual_updates }
    }

    fn parse_contents(contents: &str) -> (Vec<PageRule>, Vec<Vec<usize>>) {
        let parts: Vec<&str> = contents.split("\n\n").collect();
        let page_rules = parts.get(0).unwrap().lines().map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            let page = parts[0].parse().unwrap();
            let preceeds = vec![parts[1].parse().unwrap()];
            PageRule::new(page, preceeds)
        }).collect();

        let manual_updates = parts.get(1).unwrap().lines().map(|line| {
            line.split(',')
                .map(|c| c.trim().parse().unwrap())
                .collect::<Vec<usize>>()
        }).collect();

        (page_rules, manual_updates)
    }

    fn build_page_rules_map(page_rules: &[PageRule]) -> HashMap<usize, Vec<usize>> {
        let mut map = HashMap::new();
        for rule in page_rules {
            map.entry(rule.page).or_insert_with(Vec::new).extend(&rule.preceeds);
        }
        map
    }

    fn process_updates(&self) {
        let mut median_sum = 0;
        let mut rectified_median_sum = 0;

        for update in &self.manual_updates {
            let (median, rectified) = update.analyze(&self.page_rules_map);
            if rectified {
                rectified_median_sum += median;
            } else {
                median_sum += median;
            }
        }

        println!(
            "The sum of the middle page numbers is:\n\t- Correct updates: {}\n\t- Rectified updates: {}",
            median_sum, rectified_median_sum
        );
    }

}
