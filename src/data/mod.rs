use std::cmp::Ordering;

pub struct CompetitionData {
    pub name: String,
    pub date_string: String,
    pub place: String,
    pub executor: String,
    pub organizer: String,
    pub count_teams: u32,
    pub team_distribution: [u32; 2], // count_groups x count_teams_per_group
    pub teams: Option<Vec<Vec<Team>>>, // for each group a vector of teams, ordered by ids
    pub group_names: Option<Vec<String>>, // a vector of the group names, ordered by id
    pub current_interim_result: Vec<Option<Vec<InterimResultEntry>>>, // a ResultEntry vector for each group in descending order
    pub matches: Vec<Vec<Match>>, // a Match vector for each group
    pub current_batch: Vec<u32>,  // the current batch of matches played for each group
    pub with_break: bool, // defines whether theres a break for the teams, only important for a even team count
}

impl CompetitionData {
    pub fn empty() -> CompetitionData {
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
            current_interim_result: vec![],
            matches: vec![],
            current_batch: vec![],
            with_break: true,
        }
    }

    pub fn calc_all_interim_result(&mut self) {
        self.current_interim_result = (0..self.teams.as_ref().unwrap().len())
            .map(|group_idx| Some(self.calc_interim_result_for_group(group_idx)))
            .collect();
    }

    pub fn calc_interim_result_for_group(&self, group_idx: usize) -> Vec<InterimResultEntry> {
        // create table with entries for all teams in this group
        let group_size = self.teams.as_ref().unwrap()[group_idx].len();
        let mut table: Vec<InterimResultEntry> = (0..group_size)
            .map(|team_idx| InterimResultEntry {
                team_idx: team_idx,
                match_points: [0, 0],
                stock_points: [0, 0],
                quotient: 0.0,
            })
            .collect();

        // evaluate the match results for this group
        self.matches[group_idx]
            .iter()
            .filter(|_match| _match.result != MatchResult::NotPlayed)
            .for_each(|_match| {
                assert!(_match.points.is_some());
                let points = _match.points.unwrap();
                {
                    let entry_a = table.get_mut(_match.team_a).unwrap();

                    entry_a.match_points[0] += match _match.result {
                        MatchResult::WinnerA => 2,
                        MatchResult::Draw => 1,
                        MatchResult::WinnerB => 0,
                        MatchResult::NotPlayed => panic!(),
                    };
                    entry_a.match_points[1] += match _match.result {
                        MatchResult::WinnerA => 0,
                        MatchResult::Draw => 1,
                        MatchResult::WinnerB => 2,
                        MatchResult::NotPlayed => panic!(),
                    };
                    entry_a.stock_points[0] += points[0];
                    entry_a.stock_points[1] += points[1];
                }
                {
                    let entry_b = table.get_mut(_match.team_b).unwrap();
                    entry_b.match_points[0] += match _match.result {
                        MatchResult::WinnerA => 0,
                        MatchResult::Draw => 1,
                        MatchResult::WinnerB => 2,
                        MatchResult::NotPlayed => panic!(),
                    };
                    entry_b.match_points[1] += match _match.result {
                        MatchResult::WinnerA => 2,
                        MatchResult::Draw => 1,
                        MatchResult::WinnerB => 0,
                        MatchResult::NotPlayed => panic!(),
                    };
                    entry_b.stock_points[0] += points[1];
                    entry_b.stock_points[1] += points[0];
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
            // TODO: Verify that the ordering is correctly implemented!
            // TODO: Implement possibility that two teams on the same place
            if a.match_points[0] > b.match_points[0] {
                Ordering::Less
            } else if a.match_points[0] < b.match_points[0] {
                Ordering::Greater
            } else if a.quotient > b.quotient {
                Ordering::Less
            } else if a.quotient < b.quotient {
                Ordering::Greater
            } else if (a.stock_points[0] - a.stock_points[1])
                > (b.stock_points[0] - b.stock_points[1])
            {
                Ordering::Less
            } else if (a.stock_points[0] - a.stock_points[1])
                < (b.stock_points[0] - b.stock_points[1])
            {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        table
    }

    pub fn generate_matches(&mut self) {
        assert!(self.matches.is_empty());

        let team_count = self.team_distribution[1] as i32;
        if team_count % 2 == 0 {
            // even team count per group
            if self.with_break {
                for _ in 0..self.team_distribution[0] {
                    let mut group = vec![];
                    for batch_idx in 0..team_count {
                        let mut batch = vec![];
                        // "(batch_idx * 2) / team_count" is a elegant way to subtract 1 if batch_idx >= team_count / 2 else 0
                        for lane_idx in 0..(team_count / 2 - (batch_idx * 2 / team_count)) {
                            // "(batch_idx * 2) / team_count" see above, but add 1 instead of subtract
                            let team_a_idx = (lane_idx - batch_idx + (batch_idx * 2 / team_count))
                                .rem_euclid(team_count);
                            let team_b_idx =
                                (team_count - lane_idx - batch_idx - 1).rem_euclid(team_count);
                            batch.push(Match {
                                team_a: team_a_idx as usize,
                                team_b: team_b_idx as usize,
                                points: None,
                                result: MatchResult::NotPlayed,
                                batch: batch_idx as u32,
                                lane: lane_idx as u32,
                            })
                        }
                        group.append(&mut batch);
                    }
                    self.matches.push(group);
                }
            } else {
                todo!();
            }
        } else {
            // uneven team count per group
            for _ in 0..self.team_distribution[0] {
                let mut group = vec![];
                for batch_idx in 0..team_count {
                    let mut batch = vec![];
                    for lane_idx in 0..(team_count / 2) {
                        let team_a_idx = (lane_idx - batch_idx).rem_euclid(team_count);
                        let team_b_idx =
                            (team_count - lane_idx - batch_idx - 2).rem_euclid(team_count);
                        batch.push(Match {
                            team_a: team_a_idx as usize,
                            team_b: team_b_idx as usize,
                            points: None,
                            result: MatchResult::NotPlayed,
                            batch: batch_idx as u32,
                            lane: lane_idx as u32,
                        })
                    }
                    group.append(&mut batch);
                }
                self.matches.push(group);
            }
        }
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

pub struct Match {
    // the both opponents
    //pub team_a: &'a Team,
    //pub team_b: &'a Team,
    pub team_a: usize,
    pub team_b: usize,
    pub points: Option<[i32; 2]>, // the points of the teams if the match was already played
    pub result: MatchResult,      // the result of the match
    pub batch: u32,               // the index of the batch the match is in, e.g. "Spiel 4"
    pub lane: u32,                // the number of the lane the match is played on, e.g. "Bahn 2"
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MatchResult {
    WinnerA,
    Draw,
    WinnerB,
    NotPlayed,
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
