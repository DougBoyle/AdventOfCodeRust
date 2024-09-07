use std::{cmp::{max, min}, collections::HashMap, io::{Error, ErrorKind}, ops::{Deref, DerefMut, Range}, str::FromStr};

use enum_map::{Enum, EnumMap};
use rust_aoc::TopologicalSort;

fn main() {
    let mut lines = rust_aoc::read_input(19);
    let mut workflows = HashMap::new();
    for workflow in lines.by_ref().take_while(|s| !s.is_empty()).map(|s| s.parse::<Workflow>().unwrap()) {
        workflows.insert(workflow.id.clone(), workflow);
    }

    let total: i32 = lines.map(|s| s.parse().unwrap())
        .filter(|part| process(part, &workflows))
        .map(|part| part.sum_value()).sum();
    println!("Part 1: Total {total}"); // 319062

    let mut sort = WorkflowTopologicalSort { workflows: &workflows };
    let sorted = sort.sort();
    let sorted = if let Ok(sorted) = sorted { sorted } else { panic!("Not a DAG") };

    // Calculate in reverse DAG order
    let mut accepted_ranges: HashMap<&str, PartRange> = HashMap::new();
    accepted_ranges.insert("A", PartRange::all());
    

    for id in sorted.into_iter().rev() {
        let rules = &workflows.get(id).unwrap().rules;
        let mut previous_rules = PartRange::all(); // subtractive, constrained as we get further down non-matching rules
        let mut overall_range = PartRange::none(); // additive, paths via each rule
        for rule in rules {
            let destination = rule.destination();
            if let Some(destination_range) = accepted_ranges.get(&destination[..]) {
                let rule_range = rule.requirements();
                let previous_and_this_rule_range = previous_rules.intersect(&rule_range);
                let matching_range = previous_and_this_rule_range.intersect(destination_range);
                if !matching_range.is_empty() {
                    overall_range.include(matching_range);
                }
            }
            previous_rules = previous_rules.intersect(&rule.negated_requirements());
        }
        if !overall_range.is_empty() { accepted_ranges.insert(id, overall_range); }
    }
    
    let total: usize = accepted_ranges.remove("in").unwrap().ranges.iter().map(|range| range.size()).sum();
    println!("Part 2: Total {total}"); // 118638369682135
}

fn process(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
    let mut workflow_id = "in";
    loop {
        match workflow_id {
            "A" => return true,
            "R" => return false,
            _ => {
                let workflow = &workflows[workflow_id];
                workflow_id = workflow.process(part);
            }
        }
    }
}

struct WorkflowTopologicalSort<'a> {
    workflows: &'a HashMap<String, Workflow>,
}

impl TopologicalSort for WorkflowTopologicalSort<'_> {
    type Node = String;

    fn get_all_nodes(&self) -> Vec<&Self::Node> {
        self.workflows.keys().collect()
    }

    fn get_edges(&self, node: &Self::Node) -> Vec<&Self::Node> {
        self.workflows.get(node).unwrap().downstream_nodes().into_iter()
            .filter(|node| *node != "A" && *node != "R")
            .collect()
    }
}

const MIN_INCLUSIVE: i32 = 1;
const MAX_EXCLUSIVE: i32 = 4001;
const FULL_RANGE: Range<i32> = MIN_INCLUSIVE..MAX_EXCLUSIVE;

// Overall type, 1 per node, each RatingsIntervals has different dimensions that prevent merging them
#[derive(Clone)]
struct PartRange {
    ranges: Vec<RatingsIntervals>
}

impl std::fmt::Debug for PartRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.ranges.fmt(f)
    }
}

impl PartRange {
    fn all() -> PartRange {
        PartRange::single(RatingsIntervals::all())
    }

    fn single(ratings: RatingsIntervals) -> PartRange {
        PartRange { ranges: vec![ratings] }
    }

    fn none() -> PartRange {
        PartRange { ranges: vec![] }
    }

    fn intersect(&self, other: &PartRange) -> PartRange {
        // naive pairwise intersections
        self.ranges.iter().flat_map(|range| 
            other.ranges.iter().map(|other_range| range.intersect(other_range))
        ).filter(|range| !range.is_empty()).collect()
    }

    fn include(&mut self, mut other: PartRange) {
        // TODO: Very naive, would ideally merge compatible ranges
        self.ranges.append(&mut other.ranges);
    }
}

impl Deref for PartRange {
    type Target = Vec<RatingsIntervals>;
    
    fn deref(&self) -> &Self::Target {
        &self.ranges
    }
}

impl DerefMut for PartRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ranges
    }
}

impl FromIterator<RatingsIntervals> for PartRange {
    fn from_iter<T: IntoIterator<Item = RatingsIntervals>>(iter: T) -> Self {
        PartRange { ranges: iter.into_iter().collect() }
    }
}

// 1 combination of ratings, each category muust match the interval for that category
#[derive(Clone)]
struct RatingsIntervals {
    ratings: EnumMap<Category, Intervals>
}

impl RatingsIntervals {
    fn all() -> RatingsIntervals {
        RatingsIntervals { ratings: EnumMap::from_fn(|_| Intervals::all()) }
    }

    fn intersect(&self, other: &RatingsIntervals) -> RatingsIntervals {
        RatingsIntervals { ratings: EnumMap::from_fn(|cat| self.ratings[cat].intersect(&other.ratings[cat])) }
    }

    fn is_empty(&self) -> bool {
        self.ratings.values().any(Intervals::is_empty)
    }

    fn size(&self) -> usize {
        self.ratings.values().map(|intervals| intervals.size()).product()
    }
}

impl std::fmt::Debug for RatingsIntervals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.ratings.fmt(f)
    }
}

impl Deref for RatingsIntervals {
    type Target = EnumMap<Category, Intervals>;
    
    fn deref(&self) -> &Self::Target {
        &self.ratings
    }
}

impl DerefMut for RatingsIntervals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ratings
    }
}

// non-adjacent/overlapping ranges in increasing order, for a single category
#[derive(Clone, Eq, PartialEq)]
struct Intervals {
    ranges: Vec<Range<i32>>
}

impl std::fmt::Debug for Intervals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.ranges.fmt(f)
    }
}

impl Intervals {
    fn all() -> Intervals {
        Intervals::single(FULL_RANGE)
    }

    fn single(range: Range<i32>) -> Intervals {
        Intervals { ranges: vec![range] }
    }

    fn intersect(&self, other: &Intervals) -> Intervals {
        // Could be smarter about this and walk down both intervals in sync,
        // but instead we just take the intersection of each range from other
        // with all of our own interval (in each case producing 1-2 results)
        other.ranges.iter()
            .flat_map(|range| self.intersect_range(range).ranges.into_iter())
            .collect()
    }

    fn intersect_range(&self, range: &Range<i32>) -> Intervals {
        self.ranges.iter().map(|r| intersect(r, range))
            .filter(|range| !Range::is_empty(range))
            .collect()
    }

    fn is_empty(&self) -> bool {
        self.ranges.iter().all(Range::is_empty)
    }

    fn size(&self) -> usize {
        self.ranges.iter().map(|range| range.end - range.start).sum::<i32>() as usize
    }

    // TODO: Currently never unioned, so always increasing!
}

impl FromIterator<Range<i32>> for Intervals {
    fn from_iter<T: IntoIterator<Item = Range<i32>>>(iter: T) -> Self {
        Intervals { ranges: iter.into_iter().collect() }
    }
}

struct Part {
    ratings: EnumMap<Category, i32>
}

impl Part {
    fn sum_value(&self) -> i32 {
        self.ratings.values().sum()
    }
}

impl FromStr for Part {
    type Err = Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &s[1..s.len() - 1]; // drop '{' and '}'
        let ratings = s.split(',').map(|rating| {
            let (category, value) = rust_aoc::split_in_two(rating, '=');
            let category = Category::try_from(category.chars().next().unwrap()).unwrap();
            let value = value.parse().unwrap();
            (category, value)
        }).collect();
        Ok(Part { ratings })
    }
    
}

struct Workflow {
    id: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn process(&self, part: &Part) -> &String {
        self.rules.iter().filter(|rule| rule.matches(part)).next().unwrap().destination()
    }

    fn downstream_nodes(&self) -> Vec<&String> {
        self.rules.iter().map(|rule| rule.destination()).collect()
    }

    // TODO: Move from main loop to here?
    /* 
    fn downstream_requirements(&self) -> Vec<(String, Vec<PartRange>)> {
        let mut result = Vec::new();
        let requirements = PartRange::all();
        for rule in &self.rules {
            let destination = rule.destination();
            let rule_requirement = rule.requirements();
        }
        result
    }
    */
}

impl FromStr for Workflow {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rules) = rust_aoc::split_in_two(s, '{');
        let id = String::from(id);
        let rules = &rules[..rules.len() - 1]; // drop trailing '}' also
        let rules = rules.split(',').map(|s| s.parse()).map(Result::unwrap).collect();
        Ok(Workflow { id, rules })
    }
}

enum Rule {
    Less(Category, i32, String),
    Greater(Category, i32, String),
    Always(String),
}

impl Rule {
    fn matches(&self, part: &Part) -> bool {
        match self {
            Rule::Less(category, threshold, _) => part.ratings[*category] < *threshold,
            Rule::Greater(category, threshold, _) => part.ratings[*category] > *threshold,
            Rule::Always(_) => true,
        }
    }

    fn destination(&self) -> &String {
        match self {
            Rule::Less(_, _, dest) | Rule::Greater(_, _, dest) | Rule::Always(dest) => dest,
        }
    }

    fn requirements(&self) -> PartRange {
        let mut ranges = RatingsIntervals::all();
        match self {
            Rule::Less(category, threshold, _) => {
                ranges[*category] = Intervals::single(MIN_INCLUSIVE..*threshold);
            },
            Rule::Greater(category, threshold, _) => {
                ranges[*category] = Intervals::single((threshold+1)..MAX_EXCLUSIVE);
            }
            Rule::Always(_) => {},
        }
        PartRange::single(ranges)
    }

    fn negated_requirements(&self) -> PartRange {
        match self {
            Rule::Less(category, threshold, _) => {
                let mut ranges = RatingsIntervals::all();
                ranges[*category] = Intervals::single(*threshold..MAX_EXCLUSIVE);
                PartRange::single(ranges)
            },
            Rule::Greater(category, threshold, _) => {
                let mut ranges = RatingsIntervals::all();
                ranges[*category] = Intervals::single(0..(threshold+1));
                PartRange::single(ranges)
            },
            Rule::Always(_) => PartRange::none(),
        }
    }
}

impl FromStr for Rule {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(':') {
            let (condition, destination) = rust_aoc::split_in_two(s, ':');
            let category = (condition.as_bytes()[0] as char).try_into().unwrap();
            let comparison = condition.as_bytes()[1] as char;
            let threshold = condition[2..].parse().unwrap();
            match comparison {
                '<' => Ok(Rule::Less(category, threshold, String::from(destination))),
                '>' => Ok(Rule::Greater(category, threshold, String::from(destination))),
                _ => Err(Error::new(ErrorKind::InvalidInput, format!("Unrecognised rule {s}")))
            }
        } else {
            Ok(Rule::Always(String::from(s)))
        }
    }
}

#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq, Hash)]
enum Category {
    XtremelyCool, Musical, Aerodynamic, Shiny
}

impl TryFrom<char> for Category {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'x' => Ok(Category::XtremelyCool),
            'm' => Ok(Category::Musical),
            'a' => Ok(Category::Aerodynamic),
            's' => Ok(Category::Shiny),
            _ => Err(Error::new(ErrorKind::InvalidInput, format!("Unrecognised category {value}")))
        }
    }
}

fn intersect(a: &Range<i32>, b: &Range<i32>) -> Range<i32> {
    let start = max(a.start, b.start);
    let end = min(a.end, b.end);
  //  println!("{a:?} ^ {b:?} = {:?}", start..end);
    start..end
}
