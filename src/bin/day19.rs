use std::{cmp::{max, min}, collections::HashMap, io::{Error, ErrorKind}, ops::{Deref, DerefMut, Range}, str::FromStr};

use enum_map::{Enum, EnumMap};

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

    let mut total: usize = 0;
    let mut stack = vec![("in", RatingsIntervals::all())];
    while let Some((id, mut range)) = stack.pop() {
        let rules = &workflows.get(id).unwrap().rules;
        for rule in rules {
            let matching_range = range.intersect(&rule.requirements());
            if !matching_range.is_empty() {
                let destination = rule.destination();
                if destination == "A" {
                    total += matching_range.size();
                } else if destination != "R" {
                    stack.push((destination, matching_range));
                }
            }
            range = range.intersect(&rule.negated_requirements());
            if range.is_empty() { break }
        }
    }

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

const MIN_INCLUSIVE: i32 = 1;
const MAX_EXCLUSIVE: i32 = 4001;
const FULL_RANGE: Range<i32> = MIN_INCLUSIVE..MAX_EXCLUSIVE;

#[derive(Clone)]
struct RatingsIntervals {
    ratings: EnumMap<Category, Range<i32>>
}

impl RatingsIntervals {
    fn all() -> RatingsIntervals {
        RatingsIntervals::from_fn(|_| FULL_RANGE)
    }

    fn none() -> RatingsIntervals {
        RatingsIntervals::from_fn(|_| 0..0)
    }

    fn intersect(&self, other: &RatingsIntervals) -> RatingsIntervals {
        RatingsIntervals::from_fn(|cat| intersect(&self.ratings[cat], &other.ratings[cat]))
    }

    fn from_fn<F: FnMut(Category) -> Range<i32>>(f: F) -> RatingsIntervals {
        RatingsIntervals { ratings: EnumMap::from_fn(f) }
    }

    fn is_empty(&self) -> bool {
        self.ratings.values().any(Range::is_empty)
    }

    fn size(&self) -> usize {
        self.ratings.values().map(|range| (range.end - range.start) as usize).product()
    }
}

impl std::fmt::Debug for RatingsIntervals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.ratings.fmt(f)
    }
}

impl Deref for RatingsIntervals {
    type Target = EnumMap<Category, Range<i32>>;
    
    fn deref(&self) -> &Self::Target {
        &self.ratings
    }
}

impl DerefMut for RatingsIntervals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ratings
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

    fn requirements(&self) -> RatingsIntervals {
        let mut ratings = RatingsIntervals::all();
        match self {
            Rule::Less(category, threshold, _) => {
                ratings[*category] = MIN_INCLUSIVE..*threshold;
            },
            Rule::Greater(category, threshold, _) => {
                ratings[*category] = (threshold+1)..MAX_EXCLUSIVE;
            }
            Rule::Always(_) => {},
        }
        ratings
    }

    fn negated_requirements(&self) -> RatingsIntervals {
        match self {
            Rule::Less(category, threshold, _) => {
                let mut ratings = RatingsIntervals::all();
                ratings[*category] = *threshold..MAX_EXCLUSIVE;
                ratings
            },
            Rule::Greater(category, threshold, _) => {
                let mut ratings = RatingsIntervals::all();
                ratings[*category] = 0..(threshold+1);
                ratings
            },
            Rule::Always(_) => RatingsIntervals::none(),
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
    start..end
}
