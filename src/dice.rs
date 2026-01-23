use anyhow::{Result, anyhow};
use rand::distr::{Distribution, Uniform};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr, sync::LazyLock};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Dice {
    count: u32,
    sides: u32,
    modifier: i32,
}

impl Dice {
    pub fn new(count: u32, sides: u32, modifier: i32) -> Result<Self> {
        if count > 500 {
            return Err(anyhow!(
                "Too many dice rolls! Please roll 500 dice or fewer."
            ));
        }
        if !(2..=1000).contains(&sides) {
            return Err(anyhow!("Invalid number of sides for the dice."));
        }

        Ok(Dice {
            count,
            sides,
            modifier,
        })
    }

    pub fn roll(&self) -> (u32, Vec<u32>) {
        let uniform = Uniform::new_inclusive(1, self.sides)
            .expect("Unexpected error: roll with a dice with invalid number of sides");
        let mut rng = rand::rng();

        let rolls: Vec<u32> = (0..self.count).map(|_| uniform.sample(&mut rng)).collect();

        (
            rolls
                .iter()
                .sum::<u32>()
                .saturating_add_signed(self.modifier),
            rolls,
        )
    }
}

impl FromStr for Dice {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        static DICE_RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(
                r"(?ix)^
                (?<count>[0-9]*)      # number of rolls (optional - implicit 1)
                d                     # literal 'd'
                (?<sides>[0-9]+)      # number of sides of the dice (required)
                \s*
                (?:                   # modifier (optional)
                    (?<sign>[+-])         # sign for the modifier (required)
                    \s*
                    (?<mod_value>[0-9]+)   # the value of the modifier (required)
                )?
            $",
            )
            .unwrap()
        });
        let captures = DICE_RE
            .captures(s)
            .ok_or_else(|| anyhow!("Invalid dice format"))?;

        let count = captures
            .name("count")
            .map(|m| m.as_str())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse())
            .unwrap_or(Ok(1u32))?;

        let sides = captures["sides"].parse()?;

        let modifier = captures
            .name("mod_value")
            .map(|m| {
                let value = m.as_str().parse::<i32>();
                match &captures["sign"] {
                    "+" => value.map_err(Into::into),
                    "-" => value.map(|n| -n).map_err(Into::into),
                    _ => Err(anyhow!("Invalid sign.")),
                }
            })
            .unwrap_or(Ok(0i32))?;

        Dice::new(count, sides, modifier)
    }
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}d{}", self.count, self.sides)?;

        if self.modifier == 0 {
            return Ok(());
        }

        write!(f, "{:+}", self.modifier)
    }
}
