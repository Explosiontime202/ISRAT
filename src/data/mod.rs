use std::{cmp::Ordering, collections::HashMap};

pub struct CompetitionData<'a> {
    pub name: String,
    pub date_string: String,
    pub place: String,
    pub executor: String,
    pub organizer: String,
    pub count_teams: u32,
    pub team_distribution: [u32; 2], // count_groups x count_teams_per_group
    pub teams: Option<Vec<Vec<Team>>>, // for each group a vector of teams, ordered by ids
    pub group_names: Option<Vec<String>>, // a vector of the group names, ordered by id
    pub current_interim_result: Option<Vec<Vec<InterimResultEntry>>>, // a ResultEntry vector for each group in descending order
    pub match_results: Vec<Vec<GameResult<'a>>>, // a GameResult vector for each group
}

impl CompetitionData<'_> {
    pub fn empty() -> CompetitionData<'static> {
        CompetitionData {
            name: String::from(""),
            date_string: String::from(""),
            place: String::from(""),
            executor: String::from(""),
            organizer: String::from(""),
            count_teams: 0,
            team_distribution: [0, 0],
            teams: None,
            group_names: None,
            current_interim_result: None,
            match_results: vec![],
        }
    }

    pub fn calc_interim_result(&mut self) {
        self.current_interim_result = Some(
            self.teams
                .as_ref()
                .unwrap()
                .iter()
                .enumerate()
                .map(|(group_idx, group)| {
                    // create table with entries for all teams in this group
                    // and the mapping from the team name to the table index
                    let mut mapping = HashMap::new();
                    let mut table: Vec<InterimResultEntry> = group
                        .iter()
                        .enumerate()
                        .map(|(team_idx, team)| {
                            mapping.insert(&team.name, team_idx);
                            InterimResultEntry {
                                team_idx: team_idx,
                                match_points: [0, 0],
                                stock_points: [0, 0],
                                quotient: 0.0,
                            }
                        })
                        .collect();

                    // evaluate the match results for this group
                    self.match_results[group_idx].iter().for_each(|result| {
                        {
                            let idx_a = *mapping.get(&result.team_a.name).unwrap();
                            let entry_a = table.get_mut(idx_a).unwrap();
                            entry_a.match_points[0] += match result.result {
                                GameResultEnum::WinnerA => 2,
                                GameResultEnum::Draw => 1,
                                GameResultEnum::WinnerB => 0,
                            };
                            entry_a.match_points[1] += match result.result {
                                GameResultEnum::WinnerA => 0,
                                GameResultEnum::Draw => 1,
                                GameResultEnum::WinnerB => 2,
                            };
                            entry_a.stock_points[0] += result.points[0];
                            entry_a.stock_points[1] += result.points[1];
                        }
                        {
                            let idx_b = *mapping.get(&result.team_b.name).unwrap();
                            let entry_b = table.get_mut(idx_b).unwrap();
                            entry_b.match_points[0] += match result.result {
                                GameResultEnum::WinnerA => 0,
                                GameResultEnum::Draw => 1,
                                GameResultEnum::WinnerB => 2,
                            };
                            entry_b.match_points[1] += match result.result {
                                GameResultEnum::WinnerA => 2,
                                GameResultEnum::Draw => 1,
                                GameResultEnum::WinnerB => 0,
                            };
                            entry_b.stock_points[0] += result.points[1];
                            entry_b.stock_points[1] += result.points[0];
                        }
                    });

                    // calculate quotient
                    table.iter_mut().for_each(|entry| {
                        entry.quotient = if entry.stock_points[0] == 0 {
                            0.0
                        } else {
                            (entry.stock_points[0] as f32) / (entry.stock_points[1] as f32)
                        };
                    });

                    // sort the table
                    table.sort_by(|a, b| {
                        // TODO: Check if ordering is correctly implemented!
                        if a.match_points[0] > b.match_points[1] {
                            Ordering::Greater
                        } else if a.match_points[0] < b.match_points[1] {
                            Ordering::Less
                        } else if a.quotient < b.quotient {
                            Ordering::Greater
                        } else if a.quotient > b.quotient {
                            Ordering::Less
                        } else if a.stock_points[0] > b.stock_points[0] {
                            Ordering::Greater
                        } else if a.stock_points[0] < b.stock_points[0] {
                            Ordering::Less
                        } else {
                            Ordering::Equal
                        }
                    });
                    table
                })
                .collect(),
        );
    }
}

pub struct Team {
    pub name: String,
    pub player_names: [Option<String>; 6], // maximal 6 possible players per team
}

pub struct InterimResultEntry {
    pub team_idx: usize,
    pub match_points: [i32; 2],
    pub stock_points: [i32; 2],
    pub quotient: f32,
}

pub struct GameResult<'a> {
    pub team_a: &'a Team,
    pub team_b: &'a Team,
    pub points: [i32; 2],
    pub result: GameResultEnum,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameResultEnum {
    WinnerA,
    Draw,
    WinnerB,
}

pub fn calc_group_possibilities(count_teams: u32) -> Vec<[u32; 2]> {
    if count_teams == 0 {
        Vec::new()
    } else {
        let mut possibilities = vec![[1, count_teams]];
        for group_count in 2..count_teams {
            if count_teams % group_count == 0 {
                possibilities.push([group_count, count_teams / group_count]);
            }
        }

        possibilities
    }
}
