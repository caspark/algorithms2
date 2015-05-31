use flow_network::{FlowEdge, FlowNetwork};
use ford_fulkerson;
use std::f64;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
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

fn factorial(mut n: usize) -> usize {
    if n == 0 {
        1
    } else {
        let mut result = 1usize;
        while n > 0 {
            result *= n;
            n -= 1;
        }
        result
    }
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

    pub fn certificate_of_elimination(&self, team_x: &BaseballTeam) -> Option<Vec<&BaseballTeam>> {
        let team_count = self.teams.len();
        if team_count == 1 {
            return None;
        }

        let team_x_num = self.teams.iter().enumerate()
            .filter_map(|(i, t)| if t == team_x { Some(i) } else { None })
            .nth(0)
            .expect("team must exist");

        // trivial case
        for other_team_num in (0..team_count).filter(|i| i != &team_x_num) {
            if self.wins[team_x_num] + self.remaining[team_x_num] < self.wins[other_team_num] {
                return Some(vec![&self.teams[other_team_num]]);
            }
        }

        // complex case
        let num_game_vertices = factorial(team_count - 1 - 1); // exclude team_x and games between own teams
        let vertex_for_team = |i| num_game_vertices + i + 1;
        let source_vertex = vertex_for_team(team_count);
        let sink_vertex = source_vertex + 1;
        let mut graph = FlowNetwork::new(sink_vertex + 1);

        let mut curr_game_vertex = 0;
        for i in 0..team_count {
            if i == team_x_num {
                continue
            }
            for j in i..team_count {
                if i != j && j != team_x_num {
                    /*println!("curr game vertex is {} / {}, team 0 is {}, team {} is {}, src is {}, sink is {}",
                             curr_game_vertex, num_game_vertices,
                             vertex_for_team(0),
                             team_count - 1, vertex_for_team(team_count - 1),
                             source_vertex, sink_vertex);
                    */
                    assert!(curr_game_vertex <= num_game_vertices, "will use more vertices for games than expected!");

                    // edge from source to game vertex
                    let g_ij = self.remaining_per_team[i][j];
                    graph.add_edge(FlowEdge::new(source_vertex, curr_game_vertex, g_ij as f64, 0.0));

                    // edges from game vertex to each team
                    graph.add_edge(FlowEdge::new(curr_game_vertex, vertex_for_team(i), f64::INFINITY, 0.0));
                    graph.add_edge(FlowEdge::new(curr_game_vertex, vertex_for_team(j), f64::INFINITY, 0.0));

                    curr_game_vertex += 1;
                }
            }
            // edge from team to sink
            let edge_to_sink_capacity = self.wins[team_x_num] + self.remaining[team_x_num] - self.wins[i];
            graph.add_edge(FlowEdge::new(vertex_for_team(i), sink_vertex, edge_to_sink_capacity as f64, 0.0))
        }

        let ff_result = {
            ford_fulkerson::calculate(&graph, source_vertex, sink_vertex)
        };
        for e in graph.adj(source_vertex) {
            if e.residual_capacity_to(e.other(source_vertex)) != 0.0 {
                let mut certificate = Vec::with_capacity(team_count);
                for (i, team) in self.teams.iter().enumerate() {
                    if ff_result.in_cut(vertex_for_team(i)) {
                        certificate.push(team);
                    }
                }
                return Some(certificate);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{N_A, BaseballDivision, BaseballTeam, factorial};

    const N_A: i32 = 0; // used when the number of games remaining for team X is being recorded for team X

    #[test]
    fn factorial_works() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
    }

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
