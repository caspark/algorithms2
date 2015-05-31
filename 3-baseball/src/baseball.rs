use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BaseballTeam(pub String);

impl fmt::Display for BaseballTeam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

const N_A: i32 = -1; // used when the number of games remaining for team X is being recorded for team X

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
        for rpt in remaining_per_team.iter() {
            assert_eq!(rpt.len(), team_count);
        }
        BaseballDivision {
            teams: teams,
            wins: wins,
            losses: losses,
            remaining: remaining,
            remaining_per_team: remaining_per_team,
        }
    }

    pub fn teams(&self) -> &[BaseballTeam] {
        self.teams.as_ref()
    }

    pub fn certificate_of_elimination(&self, team: &BaseballTeam) -> Option<Vec<&BaseballTeam>> {
        let team_num = self.teams.iter().enumerate()
            .filter_map(|(i, t)| if t == team { Some(i) } else { None })
            .nth(0)
            .expect("team must exist");

        // trivial case
        for other_team_num in (0..self.teams.len()).filter(|i| i != &team_num) {
            if self.wins[team_num] + self.remaining[team_num] < self.wins[other_team_num] {
                return Some(vec![&self.teams[other_team_num]]);
            }
        }

        //TODO complex case
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::{N_A, BaseballDivision, BaseballTeam};

    fn to_teams<S: Clone + Into<String>>(team_names: &[S]) -> Vec<BaseballTeam> {
        team_names.iter().cloned().map(|s| BaseballTeam(s.into())).collect()
    }

    fn teams_4() -> BaseballDivision { // provided sample input file (teams_4.txt)
        BaseballDivision {
            teams: to_teams(&["Atlanta", "Philadelphia", "New York", "Montreal"]),
            wins: vec![83, 80, 78, 77],
            losses: vec![71, 79, 78, 82],
            remaining: vec![8, 3, 6, 3],
            remaining_per_team: vec![
                vec![N_A, 1, 6, 1],
                vec![1, N_A, 0, 2],
                vec![6, 0, N_A, 0],
                vec![1, 2, 0, N_A],
            ]
        }
    }

    #[test]
    fn detects_trivial_elimination() {
        let div = teams_4();
        let atlanta = &div.teams[0];
        let montreal = &div.teams[3];
        assert_eq!(div.certificate_of_elimination(atlanta), None);
        assert_eq!(div.certificate_of_elimination(montreal), Some(vec![atlanta]));
    }
}
