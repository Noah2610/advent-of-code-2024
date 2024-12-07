use std::collections::{HashMap, HashSet};
use util::{read_input, AppResult};

type Page = u32;

#[derive(Debug)]
struct PageOrderingRule(Page, Page);

#[derive(Debug)]
struct PageOrderingRules {
    rules: HashMap<RuleKey, PageOrderingRule>,
}

#[derive(Debug)]
struct UpdateManual(Vec<Page>);

#[derive(Debug)]
struct UpdateManuals(Vec<UpdateManual>);

#[derive(PartialEq, Eq, Hash, Debug)]
struct RuleKey(Page, Page);

impl PageOrderingRule {
    pub fn to_key(&self) -> RuleKey {
        RuleKey::from((self.0, self.1))
    }
}

impl UpdateManual {
    pub fn is_correct_order(&self, rules: &PageOrderingRules) -> bool {
        for (i, page_a) in self.0.iter().enumerate() {
            for page_b in self.0.iter().skip(i + 1) {
                if !rules.is_correct_order(*page_a, *page_b) {
                    return false;
                }
            }
        }

        true
    }
}

impl PageOrderingRules {
    pub fn is_correct_order(&self, a: Page, b: Page) -> bool {
        let key = RuleKey::from((a, b));
        let rule = self.rules.get(&key);

        if let Some(rule) = rule {
            rule.0 == a
        } else {
            eprintln!(
                "No rule found for pages {a} and {b}, assuming correct order"
            );
            true
        }
    }
}

impl From<(Page, Page)> for RuleKey {
    fn from((a, b): (Page, Page)) -> Self {
        let sorted = if a < b { (a, b) } else { (b, a) };
        Self(sorted.0, sorted.1)
    }
}

impl<'a> TryFrom<&'a str> for PageOrderingRule {
    type Error = String;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let numbers = input
            .split('|')
            .map(|s| s.parse::<Page>())
            .collect::<Result<Vec<Page>, _>>()
            .map_err(|e| {
                format!("PageOrderingRule failed to parse number, {e}")
            })?;
        if numbers.len() != 2 {
            return Err(format!(
                "Failed to parse PageOrderingRule, expected two numbers separated by '|', received {input}",
            ));
        }

        Ok(Self(
            *numbers.get(0).ok_or("Expected first parsed number")?,
            *numbers.get(1).ok_or("Expected second parsed number")?,
        ))
    }
}

impl<'a> TryFrom<&'a str> for PageOrderingRules {
    type Error = String;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let mut rules = HashMap::new();

        for line in input.lines() {
            let rule = PageOrderingRule::try_from(line)?;
            let key = rule.to_key();
            rules.insert(key, rule);
        }

        Ok(Self { rules })
    }
}

impl<'a> TryFrom<&'a str> for UpdateManual {
    type Error = String;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        input
            .split(',')
            .map(|s| s.parse::<Page>())
            .collect::<Result<Vec<Page>, _>>()
            .map_err(|e| format!("UpdateManual failed to parse number, {e}"))
            .map(Self)
    }
}

impl<'a> TryFrom<&'a str> for UpdateManuals {
    type Error = String;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let mut manuals = Vec::new();

        for line in input.lines() {
            let manual = UpdateManual::try_from(line)?;
            manuals.push(manual);
        }

        Ok(Self(manuals))
    }
}

fn main() -> AppResult {
    let input = read_input()?;
    let input_sections = input.split("\n\n").collect::<Vec<&str>>();
    let ordering_section = *input_sections.get(0).ok_or(
        "Expected input to have ordering rules section, followed by two newlines",
    )?;
    let manuals_section = *input_sections.get(1).ok_or(
        "Expected input to have manuals section, preceded by two newlines",
    )?;

    let ordering_rules = PageOrderingRules::try_from(ordering_section)?;
    let update_manuals = UpdateManuals::try_from(manuals_section)?;

    let sum_correct_manuals_middle = update_manuals
        .0
        .iter()
        .filter(|man| man.is_correct_order(&ordering_rules))
        .map(|man| {
            let len = man.0.len();
            let mid = len / 2;
            man.0
                .get(mid)
                .ok_or(format!("Expected middle page: {:?}", &man.0))
        })
        .sum::<Result<Page, String>>()?;

    println!("Sum of correctly ordered manuals' middle pages: {sum_correct_manuals_middle}");

    Ok(())
}
