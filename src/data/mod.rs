use chrono::offset::Local;
use std::cmp::Ordering;
use std::fs;
use std::path::Path;
use tectonic::config::PersistentConfig;
use tectonic::driver::{OutputFormat, ProcessingSessionBuilder};
use tectonic::status::NoopStatusBackend;

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

    pub fn get_result_as_html(&self) -> String {
        /*format!(
            r#"<html><head> This is sparta! {}<\head> <\html>"#,
            self.name
        )*/
        todo!();
    }

    pub fn get_result_as_tex(&self) -> String {
        let groups = self.group_names.as_ref().unwrap().iter().enumerate().map(|(i, group_name)| {
            assert!(self.current_interim_result[i].is_some());
            let team_names = &self.teams.as_ref().unwrap()[i];
            let group_result = self.current_interim_result[i].as_ref().unwrap().iter().enumerate().map(|(rank, i_res)| {
                let team = &team_names[i_res.team_idx];
                // TODO: Add "Kreis" specifier from team, currently not implemented
                format!(r"\large {}. & \large {} & \large {} & \large {} : {} & \large {:.3} & \large {} : {} \\",
                rank + 1,
                team.name,
                "202",
                i_res.match_points[0],
                i_res.match_points[1],
                i_res.quotient,
                i_res.stock_points[0],
                i_res.stock_points[1])
            }).collect::<Vec<String>>().join("");

            format!(r"
            \begin{{center}}
                \LARGE \textbf{{{group_name}}}
                \par
                \begin{{tabularx}}{{0.9\textwidth}} {{
                    >{{\hsize=0.5\hsize\linewidth=\hsize \centering\arraybackslash}}X
                    >{{\hsize=3.0\hsize\linewidth=\hsize \raggedright\arraybackslash}}X
                    >{{\hsize=0.5\hsize\linewidth=\hsize \centering\arraybackslash}}X
                    >{{\hsize=0.5\hsize\linewidth=\hsize \centering\arraybackslash}}X
                    >{{\hsize=0.5\hsize\linewidth=\hsize \centering\arraybackslash}}X
                    >{{\hsize=1.0\hsize\linewidth=\hsize \centering\arraybackslash}}X
                }}
                \small Rang & \small Mannschaft & \small Kreis & \small Punkte & \small Quotient & \small Stockpunkte \\
                {}
            \end{{tabularx}}
            \end{{center}}
        ", group_result)})
        .collect::<Vec<String>>().join(r"\vspace{1cm}");

        format!(
            r"\documentclass{{article}}
            \setlength{{\oddsidemargin}}{{-40pt}}
            \setlength{{\textwidth}}{{532pt}}
            \usepackage{{fontspec}}
            \usepackage{{geometry}}
            \usepackage{{hyperref}}
            \usepackage{{tabularx}}
            \geometry{{
             a4paper,
             total={{190mm,257mm}},
             left=10mm,
             top=7.5mm,
             }}
            \setmainfont{{FreeSans}}
            \pagenumbering{{gobble}}
            \begin{{document}}
            \begin{{center}}
                \large \textbf{{
            {}\\ {}\\ am {}\\ {} \\ Durchführer: {}
            }}
            \end{{center}}
            \par\noindent\rule{{\linewidth}}{{0.4pt}}
            \footnotesize ISRAT: \href{{https://github.com/Explosiontime202/ISRAT}}{{https://github.com/Explosiontime202/ISRAT}}
            \hfill
            \footnotesize {}
            {}
            \end{{document}}",
            self.organizer,
            self.name,
            self.date_string,
            self.place,
            self.executor,
            Local::now().date().format("%d.%m.%Y"),
            groups
        )
    }

    pub fn export_pdf(&mut self) {
        // TODO: Spawn thread to process latex document
        self.calc_all_interim_result();
        let latex = self.get_result_as_tex();
        println!("{}", latex);
        let mut status = NoopStatusBackend::default();
        let config =
            PersistentConfig::open(false).expect("failed to open the default configuration file");

        let bundle = config
            .default_bundle(false, &mut status)
            .expect("failed to load the default resource bundle");

        let format_cache_path = config
            .format_cache_path()
            .expect("failed to set up the format cache");
        if !Path::new("./exports").exists() {
            fs::create_dir("./exports").expect("Failed to create directory \"exports!\"");
        } else if !Path::new("./exports").is_dir() {
            // TODO: handle this more beautiful, e.g. popup message
            panic!("\"exports\" exists, but is no directory!");
        }
        let mut sb = ProcessingSessionBuilder::default();
        sb.bundle(bundle)
            .primary_input_buffer(latex.as_bytes())
            .tex_input_name("result_list.tex")
            .format_name("latex")
            .format_cache_path(format_cache_path)
            .keep_logs(false)
            .keep_intermediates(false)
            .print_stdout(false)
            .output_format(OutputFormat::Pdf)
            .output_dir("./exports");

        let mut sess = sb
            .create(&mut status)
            .expect("failed to initialize the LaTeX processing session");
        sess.run(&mut status).expect("the LaTeX engine failed");
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
