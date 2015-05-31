use std::fmt;
use std::slice::Iter;

#[derive(Debug, Clone)]
pub struct BaseballTeam(pub String);

impl fmt::Display for BaseballTeam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct BaseballDivision {
    teams: Vec<BaseballTeam>,
    wins: Vec<i32>,
    losses: Vec<i32>,
    remaining: Vec<i32>, // "Note that a team's number of remaining games does not necessarily equal the sum of the remaining games against teams in its division because a team may play opponents outside its division."
    remaining_per_team: Vec<Vec<i32>>
}

impl BaseballDivision {
    pub fn new(teams: Vec<BaseballTeam>, wins: Vec<i32>, losses: Vec<i32>, remaining: Vec<i32>,
               remaining_per_team: Vec<Vec<i32>>) -> BaseballDivision {
        let team_count = teams.len();
        assert_eq!(wins.len(), team_count);
        assert_eq!(losses.len(), team_count);
        assert_eq!(remaining.len(), team_count);
        assert_eq!(remaining_per_team.len(), team_count);
        BaseballDivision {
            teams: teams,
            wins: wins,
            losses: losses,
            remaining: remaining,
            remaining_per_team: remaining_per_team,
        }
    }

    pub fn teams(&self) -> Iter<BaseballTeam> {
        self.teams.iter()
    }

    pub fn certificate_of_elimination(&self, team: &BaseballTeam) -> Option<Vec<String>> {
        unimplemented!();
    }
}
