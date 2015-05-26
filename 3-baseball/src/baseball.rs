use std::fmt;
use std::slice::Iter;

#[derive(Debug, Clone)]
pub struct BaseballTeam(String);

impl fmt::Display for BaseballTeam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct BaseballDivision {
    teams: Vec<BaseballTeam>
}

impl BaseballDivision {
    pub fn new<I: Iterator<Item=String>>(teams: I) -> BaseballDivision {
        BaseballDivision {
            teams: teams.map(|team_name| BaseballTeam(team_name)).collect(),
        }
    }

    pub fn teams(&self) -> Iter<BaseballTeam> {
        self.teams.iter()
    }

    pub fn is_eliminated(&self, team: &BaseballTeam) -> bool {
        unimplemented!();
    }

    pub fn certificate_of_elimination(&self, team: &BaseballTeam) -> Vec<String> {
        unimplemented!();
    }
}
