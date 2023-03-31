use chrono::offset::Local;
use rand::Rng;
use serde::Deserialize;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::thread::{self, JoinHandle};
use tectonic::config::PersistentConfig;
use tectonic::driver::{OutputFormat, ProcessingSessionBuilder};
use tectonic::status::NoopStatusBackend;

use crate::data::read_write::save_to_file;

use self::read_write::read_from_file;

pub mod read_write;

#[derive(Debug)]
pub struct Competition {
    pub data: Option<CompetitionData>,
    pub spawned_threads: Vec<JoinHandle<()>>, // stores the join handles to the threads used to export pdf and html documents
    pub current_interim_result: Vec<Option<Vec<InterimResultEntry>>>, // a ResultEntry vector for each group in descending order
    pub absolute_dir_path: Option<PathBuf>,   // absolute path to the folder to store the export documents and autosaves
    pub absolute_file_path: Option<PathBuf>,  // absolute path to the data file, must not be in absolute_dir_path
}

impl Default for Competition {
    fn default() -> Self {
        Self::empty()
    }
}

impl Competition {
    pub fn empty() -> Self {
        Competition {
            data: None,
            spawned_threads: vec![],
            current_interim_result: vec![],
            absolute_dir_path: None,
            absolute_file_path: None,
        }
    }

    pub fn get_current_interim_result_for_group(&mut self, group_idx: usize) -> &Option<Vec<InterimResultEntry>> {
        // calculate interim result if not available
        if self.current_interim_result[group_idx].is_none() {
            debug_assert!(self.data.is_some());
            self.current_interim_result[group_idx] = Some(self.data.as_mut().unwrap().calc_interim_result_for_group(group_idx));
        }
        &self.current_interim_result[group_idx]
    }

    pub fn handle_open_file(&mut self, path: PathBuf) -> Result<(), String> {
        if !path.exists() {
            return Err(format!("Path does not exists: {}", path.display()));
        } else if !path.is_absolute() {
            return Err(format!(
                "An absolute file path is required. This path is not absolute: {}",
                path.display()
            ));
        }

        let data_res = read_from_file(path.clone());
        let competition_data = match data_res {
            Ok(data) => data,
            Err(msg) => return Err(msg),
        };

        self.data = Some(competition_data);

        self.absolute_dir_path = match path.parent() {
            Some(parent_path) => Some(parent_path.to_path_buf()),
            None => return Err(String::from("Could not retrieve absolute path to parent folder!")),
        };

        self.absolute_file_path = Some(path);

        Ok(())
    }

    pub fn handle_save_file(&mut self, mut path: PathBuf) -> Result<(), String> {
        if path.exists() && path.is_dir() {
            return Err(String::from("Path references a directory!"));
        }

        // TODO: Replace by logging
        println!("Save data to {}", path.display().to_string());

        if self.data.is_none() {
            return Err(String::from("No competition data available!"));
        }

        // adjust file name to have the right extension
        path.set_extension("json");

        // update path
        self.absolute_file_path = Some(path.clone());
        self.absolute_dir_path = self.absolute_file_path.as_ref().unwrap().parent().map(|path| path.to_path_buf());

        save_to_file(path, self.data.as_ref().unwrap())
    }

    pub fn export_result_list(&mut self) {
        debug_assert!(self.data.is_some());
        self.current_interim_result = self.data.as_mut().unwrap().calc_all_interim_result();
        self.export_pdf(
            format!("result-{}", Local::now().format("%Y%m%d-%H%M")),
            self.data.as_ref().unwrap().get_result_as_latex(&self.current_interim_result),
        );
    }

    pub fn export_start_list(&mut self) {
        debug_assert!(self.data.is_some());
        self.export_pdf(
            format!("startlist-{}", Local::now().format("%Y%m%d-%H%M")),
            self.data.as_ref().unwrap().get_start_list_as_latex(),
        );
    }

    pub fn export_team_match_plans(&mut self) {
        debug_assert!(self.data.is_some());
        self.export_pdf(
            format!("team_matchplans-{}", Local::now().format("%Y%m%d-%H%M")),
            self.data.as_ref().unwrap().get_team_match_plans_as_latex(),
        );
    }

    pub fn export_lane_match_plans(&mut self) {
        debug_assert!(self.data.is_some());
        self.export_pdf(
            format!("lane_matchplans-{}", Local::now().format("%Y%m%d-%H%M")),
            self.data.as_ref().unwrap().get_lane_match_plans_as_latex(),
        );
    }

    fn export_pdf(&mut self, filename: String, latex: String) {
        self.verify_paths();
        let dir_path = self.absolute_dir_path.as_ref().unwrap().clone();
        self.spawned_threads.push(thread::spawn(move || {
            // TODO: Remove for productive builds
            #[cfg(debug_assertions)]
            {
                println!("{}", latex);
            }

            let mut status = NoopStatusBackend::default();
            let config = PersistentConfig::open(false).expect("failed to open the default configuration file");

            let bundle = config
                .default_bundle(false, &mut status)
                .expect("failed to load the default resource bundle");

            let format_cache_path = config.format_cache_path().expect("failed to set up the format cache");

            let mut sb = ProcessingSessionBuilder::default();
            let export_dir_path = dir_path.join("exports");

            if !export_dir_path.exists() {
                fs::create_dir(&export_dir_path).expect("Export directory creation failed.");
            } else if !export_dir_path.is_dir() {
                // TODO: handle this more beautiful, e.g. popup message
                panic!(r#""exports" exists, but is no directory!"#);
            }

            sb.bundle(bundle)
                .primary_input_buffer(latex.as_bytes())
                .tex_input_name(format!("{}.tex", filename).as_str())
                .format_name("latex")
                .format_cache_path(format_cache_path)
                .keep_logs(false)
                .keep_intermediates(false)
                .print_stdout(false)
                .output_format(OutputFormat::Pdf)
                .output_dir(export_dir_path);

            let mut sess = sb.create(&mut status).expect("failed to initialize the LaTeX processing session");
            sess.run(&mut status).expect("the LaTeX engine failed");
            println!("Finished export!");
        }));
    }

    fn verify_paths(&self) {
        debug_assert!(self.absolute_file_path.is_some());
        if let Some(absolute_file_path) = self.absolute_file_path.as_ref() {
            if absolute_file_path.exists() {
                debug_assert!(absolute_file_path.is_file());
                debug_assert!(absolute_file_path.is_absolute());
            }
        }

        debug_assert!(self.absolute_dir_path.is_some());
        if let Some(absolute_dir_path) = self.absolute_dir_path.as_ref() {
            debug_assert!(absolute_dir_path.exists());
            debug_assert!(absolute_dir_path.is_dir());
            debug_assert!(absolute_dir_path.is_absolute());
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompetitionData {
    pub name: String,
    pub date_string: String,
    pub place: String,
    pub executor: String,
    pub organizer: String,
    pub referee: String,
    pub competition_manager: String,
    pub clerk: String,
    pub additional_text: String,
    pub count_teams: u32,
    pub count_groups: u32,
    pub team_distribution: [u32; 2],      // count_groups x count_teams_per_group
    pub teams: Option<Vec<Vec<Team>>>,    // for each group a vector of teams, ordered by ids
    pub group_names: Option<Vec<String>>, // a vector of the group names, ordered by id
    pub matches: Vec<Vec<Match>>,         // a Match vector for each group
    pub current_batch: Vec<u32>,          // the current batch of matches played for each group
    pub with_break: bool,                 // defines whether theres a break for the teams, only important for a even team count
}

impl Default for CompetitionData {
    fn default() -> Self {
        Self::empty()
    }
}

impl CompetitionData {
    pub fn empty() -> CompetitionData {
        CompetitionData {
            name: String::from(""),
            date_string: String::from(""),
            place: String::from(""),
            executor: String::from(""),
            organizer: String::from(""),
            referee: String::from(""),
            competition_manager: String::from(""),
            clerk: String::from(""),
            additional_text: String::from(""),
            count_teams: 0,
            count_groups: 0,
            team_distribution: [0, 0],
            teams: None,
            group_names: None,
            matches: vec![],
            current_batch: vec![],
            with_break: true,
        }
    }

    fn calc_all_interim_result(&mut self) -> Vec<Option<Vec<InterimResultEntry>>> {
        (0..self.teams.as_ref().unwrap().len())
            .map(|group_idx| Some(self.calc_interim_result_for_group(group_idx)))
            .collect()
    }

    pub fn calc_interim_result_for_group(&self, group_idx: usize) -> Vec<InterimResultEntry> {
        // create table with entries for all teams in this group
        let group_size = self.teams.as_ref().unwrap()[group_idx].len();
        let mut table: Vec<InterimResultEntry> = (0..group_size)
            .map(|team_idx| InterimResultEntry {
                team_idx,
                match_points: [0, 0],
                stock_points: [0, 0],
                difference: 0,
                quotient: 0.0,
            })
            .collect();

        // evaluate the match results for this group
        self.matches[group_idx]
            .iter()
            .filter(|_match| _match.result != MatchResult::NotPlayed && _match.result != MatchResult::Break)
            .for_each(|_match| {
                assert!(_match.points.is_some());
                let points = _match.points.unwrap();
                {
                    let entry_a = table.get_mut(_match.team_a).unwrap();

                    entry_a.match_points[0] += match _match.result {
                        MatchResult::WinnerA => 2,
                        MatchResult::Draw => 1,
                        MatchResult::WinnerB => 0,
                        MatchResult::NotPlayed | MatchResult::Break => panic!(),
                    };
                    entry_a.match_points[1] += match _match.result {
                        MatchResult::WinnerA => 0,
                        MatchResult::Draw => 1,
                        MatchResult::WinnerB => 2,
                        MatchResult::NotPlayed | MatchResult::Break => panic!(),
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
                        MatchResult::NotPlayed | MatchResult::Break => panic!(),
                    };
                    entry_b.match_points[1] += match _match.result {
                        MatchResult::WinnerA => 2,
                        MatchResult::Draw => 1,
                        MatchResult::WinnerB => 0,
                        MatchResult::NotPlayed | MatchResult::Break => panic!(),
                    };
                    entry_b.stock_points[0] += points[1];
                    entry_b.stock_points[1] += points[0];
                }
            });

        // calculate quotient
        table.iter_mut().for_each(|entry| {
            entry.difference = entry.stock_points[0] - entry.stock_points[1];
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
            // TODO: Use new rule, i.e. difference!!
            if a.match_points[0] > b.match_points[0] {
                Ordering::Less
            } else if a.match_points[0] < b.match_points[0] {
                Ordering::Greater
            } else if a.quotient > b.quotient {
                Ordering::Less
            } else if a.quotient < b.quotient {
                Ordering::Greater
            } else if (a.stock_points[0] - a.stock_points[1]) > (b.stock_points[0] - b.stock_points[1]) {
                Ordering::Less
            } else if (a.stock_points[0] - a.stock_points[1]) < (b.stock_points[0] - b.stock_points[1]) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        table
    }

    pub fn next_matches_for_group(&self, group_idx: usize) -> Vec<&Match> {
        let current_batch = self.current_batch[group_idx];
        let next_matches = self.matches[group_idx]
            .iter()
            .filter(|&match_| match_.result != MatchResult::Break)
            .filter(|&match_| match_.batch == current_batch)
            .collect::<Vec<&Match>>();
        next_matches.iter().for_each(|match_| assert_eq!(match_.result, MatchResult::NotPlayed));
        next_matches
    }

    pub fn teams_on_break_for_group(&self, group_idx: usize) -> Vec<&Team> {
        let current_batch = self.current_batch[group_idx];
        let teams = &self.teams.as_ref().unwrap()[group_idx];
        self.matches[group_idx]
            .iter()
            .filter(|&match_| match_.result == MatchResult::Break)
            .filter(|&match_| match_.batch == current_batch)
            .flat_map(|match_| [match_.team_a, match_.team_b])
            .map(|team_id| &teams[team_id])
            .collect()
    }

    pub fn generate_matches(&mut self) {
        assert!(self.matches.is_empty());

        let mut assigned_ids = HashSet::<MatchID>::new();

        let mut new_match_id = || loop {
            let id = Self::generate_new_match_id();
            if !assigned_ids.contains(&id) {
                assigned_ids.insert(id);
                break id;
            }
        };

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
                            let team_a_idx = (lane_idx - batch_idx + (batch_idx * 2 / team_count)).rem_euclid(team_count);
                            let team_b_idx = (team_count - lane_idx - batch_idx - 1).rem_euclid(team_count);
                            batch.push(Match {
                                team_a: team_a_idx as usize,
                                team_b: team_b_idx as usize,
                                points: None,
                                result: MatchResult::NotPlayed,
                                batch: batch_idx as u32,
                                lane: lane_idx as u32,
                                id: new_match_id(),
                            })
                        }

                        if batch_idx >= team_count / 2 {
                            batch.push(Match {
                                team_a: (team_count / 2 - batch_idx).rem_euclid(team_count) as usize,
                                team_b: (team_count / 2 - batch_idx).rem_euclid(team_count) as usize,
                                points: None,
                                result: MatchResult::Break,
                                batch: batch_idx as u32,
                                lane: u32::MAX,
                                id: new_match_id(),
                            });
                            batch.push(Match {
                                team_a: (team_count - batch_idx) as usize,
                                team_b: (team_count - batch_idx) as usize,
                                points: None,
                                result: MatchResult::Break,
                                batch: batch_idx as u32,
                                lane: u32::MAX,
                                id: new_match_id(),
                            });
                        }
                        group.append(&mut batch);
                    }
                    self.matches.push(group);
                }
            } else {
                // TODO: Implement match generation for even team count per group without breaks
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
                        let team_b_idx = (team_count - lane_idx - batch_idx - 2).rem_euclid(team_count);
                        batch.push(Match {
                            team_a: team_a_idx as usize,
                            team_b: team_b_idx as usize,
                            points: None,
                            result: MatchResult::NotPlayed,
                            batch: batch_idx as u32,
                            lane: lane_idx as u32,
                            id: new_match_id(),
                        });
                    }

                    // add break from this batch
                    let break_idx = self.team_distribution[1] - (batch_idx as u32) - 1;
                    batch.push(Match {
                        team_a: break_idx as usize,
                        team_b: break_idx as usize,
                        points: None,
                        result: MatchResult::Break,
                        batch: batch_idx as u32,
                        lane: u32::MAX,
                        id: new_match_id(),
                    });

                    group.append(&mut batch);
                }
                self.matches.push(group);
            }
        }
    }

    fn generate_new_match_id() -> MatchID {
        rand::thread_rng().gen()
    }

    ///
    /// Sets the results and points of the matches according to `match_results`.
    /// It is required for those matches, to have as result MatchResult::NotPlayed.
    /// Increases the current batch of the group.
    ///
    pub fn enter_match_results(&mut self, group_idx: usize, match_results: HashMap<MatchID, [u32; 2]>) {
        for match_ in &mut self.matches[group_idx] {
            if let Some([points_a, points_b]) = match_results.get(&match_.id) {
                assert!(match_.result == MatchResult::NotPlayed);
                match_.points = Some([*points_a as i32, *points_b as i32]);
                match_.result = match points_a.cmp(points_b) {
                    Ordering::Less => MatchResult::WinnerB,
                    Ordering::Equal => MatchResult::Draw,
                    Ordering::Greater => MatchResult::WinnerA,
                };
            }
        }
        self.current_batch[group_idx] += 1;
    }

    pub fn get_result_as_html(&self) -> String {
        /*format!(
            r#"<html><head> This is sparta! {}<\head> <\html>"#,
            self.name
        )*/
        todo!();
    }

    fn get_header_as_latex(&self) -> String {
        format!(
            r"\begin{{center}}
            \large \textbf{{
            {}\\ {}\\ am {}\\ {} \\ Durchführer: {}
            }}
            \end{{center}}
            \par\noindent\rule{{\linewidth}}{{0.4pt}}
            \footnotesize ISRAT: \href{{https://www.github.com/Explosiontime202/ISRAT}}{{https://www.github.com/Explosiontime202/ISRAT}}
            \hfill
            \footnotesize {}
            ",
            self.organizer,
            self.name,
            self.date_string,
            self.place,
            self.executor,
            Local::now().format("%d.%m.%Y %H:%M"),
        )
    }

    fn get_result_as_latex(&self, current_interim_result: &Vec<Option<Vec<InterimResultEntry>>>) -> String {
        // TODO: make this configurable by the user
        let player_names_until = 3; // index of the first team which has NO player names displayed

        let header = self.get_header_as_latex();

        // analyse if all matches were played
        let is_final_result = self
            .matches
            .iter()
            .map(|group_matches| group_matches.iter().all(|_match| _match.result != MatchResult::NotPlayed))
            .all(|b| b);

        let footnote = format!(
            r"
        \vspace{{1cm}}
        \begin{{center}}
        \normalsize
            {}\\
        \end{{center}}
        \vfill
        \par\noindent\rule{{\linewidth}}{{0.4pt}}
        \large
        \begin{{center}}
            \begin{{tabular}}{{
                >{{\centering\arraybackslash}} p{{0.333\tablewidth}}
                >{{\centering\arraybackslash}} p{{0.333\tablewidth}}
                >{{\centering\arraybackslash}} p{{0.333\tablewidth}}}}
                {} & {} & {} \\
                {{[Schiedsrichter]}} & [Wettbewerbsleiter] & [Schriftführer]
            \end{{tabular}}
        \end{{center}}
        \clearpage
        ",
            self.additional_text.replace("\n", r"\\"),
            self.referee,
            self.competition_manager,
            self.clerk
        );

        let mut count_teams_with_name = 0;
        let mut count_teams_without_name = 0;
        let mut previous_new_page = true; // determines whether the header is printed, for first team true

        let groups = self.group_names.as_ref().unwrap().iter().enumerate().map(|(group_idx, group_name)| {
            assert!(current_interim_result[group_idx].is_some());
            let team_names = &self.teams.as_ref().unwrap()[group_idx];
            let group_result = current_interim_result[group_idx].as_ref().unwrap().iter().enumerate().map(|(rank, i_res)| {
                let team = &team_names[i_res.team_idx];
                let display_player_names = rank < player_names_until;

                // join player names, separate by ", ", but if no player name is given, enter "~" to force latex to actually draw the newline
                let player_names = if !display_player_names {
                    count_teams_without_name += 1;
                    String::from("")
                } else {
                    count_teams_with_name += 1;
                    if team.player_names.iter().all(|x| x.is_none()) {
                        String::from("~")
                    } else {team.player_names.iter()
                        .filter_map(|name_opt| name_opt.as_ref().map(|name| {name.as_str()}))
                        .collect::<Vec<&str>>()
                        .join(", ")
                    }
                };

                format!(r"\large {}. & \large \makecell[l]{{{}{}{}}} & \large {} & \large {} & \large {} & \large {:.3} & \large {} & \large {} \\
                ",
                rank + 1,
                if display_player_names {r"\\"} else {""},
                team.name,
                if display_player_names {format!(r"\\ \footnotesize {}", player_names)} else {String::from("")},
                team.region,
                i_res.match_points[0],
                i_res.match_points[1],
                i_res.quotient,
                i_res.stock_points[0],
                i_res.stock_points[1])
            }).collect::<Vec<String>>().join("");

            format!(r"
            {}
            \begin{{center}}
                \LARGE \textbf{{{} {group_name}}}
                \par
                \small
                \begin{{tabular}}{{
                    >{{\centering\arraybackslash}}p{{0.0833\tablewidth}}
		            >{{\raggedright\arraybackslash}}p{{0.5\tablewidth}}
		            >{{\centering\arraybackslash}} p{{0.0833\tablewidth}}
		            >{{\raggedleft\arraybackslash}}p{{\columnspielpunkte-2\tabcolsep}}@{{\large ~:~}}
		            >{{\raggedright\arraybackslash}}p{{\columnspielpunkte-2\tabcolsep}}
		            >{{\centering\arraybackslash}}p{{0.0833\tablewidth}}
		            >{{\raggedleft\arraybackslash}}p{{\columnstockpunkte-2\tabcolsep}}@{{\large ~:~}}
		            >{{\raggedright\arraybackslash}}p{{\columnstockpunkte-2\tabcolsep}}
                }}
                \small Rang & \small Mannschaft & \small Kreis & \multicolumn{{2}}{{c}}{{\small Punkte}} & \small Quotient & \multicolumn{{2}}{{c}}{{\small Stockpunkte}} \\
                {}
            \end{{tabular}}
            \end{{center}}
            {}
        ",
        if previous_new_page {header.as_str()} else {""},
        if is_final_result {format!("Ergebnisliste {group_name}")} else {format!("Zwischenliste {group_name} nach Spiel {}", self.current_batch[group_idx])},
        group_result,
        if 3 * count_teams_with_name + count_teams_without_name > 16 {
            previous_new_page = true;
            footnote.as_str()
        } else {
            previous_new_page = false;
            r"\vspace{0.5cm}"
        }
        )})
        .collect::<Vec<String>>()
        .join("");

        format!(
            r"\documentclass{{article}}

    \usepackage{{array}}
    \usepackage{{calc}}
    \usepackage{{fontspec}}
    \usepackage{{geometry}}
    \usepackage{{hyperref}}
    \usepackage{{makecell}}
    \usepackage{{multirow}}
    \usepackage{{tabularx}}
    
    \setlength{{\oddsidemargin}}{{-40pt}}
    \setlength{{\textwidth}}{{532pt}}
    \newlength{{\tablewidth}}
    \setlength{{\tablewidth}}{{0.8\textwidth}}

    \newlength{{\columnstockpunkte}}
    \setlength{{\columnstockpunkte}}{{\widthof{{Stockpunkte}}}}

    \newlength{{\columnspielpunkte}}
    \setlength{{\columnspielpunkte}}{{\widthof{{Punkte}}}}

    \geometry{{
        a4paper,
        total={{190mm,257mm}},
        left=10mm,
        top=7.5mm,
        bottom=10mm
        }}
    \setmainfont{{FreeSans}}
    \pagenumbering{{gobble}}
    \begin{{document}}
        {}
    \end{{document}}",
            groups
        )
    }

    fn get_start_list_as_latex(&self) -> String {
        let header = self.get_header_as_latex();

        let mut previous_new_page = true; // determines whether the header is printed, for first team true

        assert!(self.group_names.is_some());
        assert!(self.teams.is_some());

        let groups = self
            .teams
            .as_ref()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(group_idx, teams_in_group)| {
                let group_result = teams_in_group
                    .iter()
                    .enumerate()
                    .map(|(rank, team)| {
                        format!(
                            r"\large {}. & \large \makecell[l]{{\\{}\\}} & \large {} \\
                ",
                            rank + 1,
                            team.name,
                            team.region,
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("");

                format!(
                    r"
            {}
            \begin{{center}}
                \LARGE \textbf{{Startliste {}}}
                \par
                \small
                \begin{{tabular}}{{
                    >{{\centering\arraybackslash}}p{{0.125\tablewidth}}
		            >{{\raggedright\arraybackslash}}p{{0.75\tablewidth}}
		            >{{\centering\arraybackslash}} p{{0.125\tablewidth}}
                }}
                \small Startnummer & \small Mannschaft & \small Kreis \\
                {}
            \end{{tabular}}
            \end{{center}}
            {}
        ",
                    if previous_new_page { header.as_str() } else { "" },
                    self.group_names.as_ref().unwrap()[group_idx],
                    group_result,
                    if self.team_distribution[1] > 15 {
                        previous_new_page = true;
                        ""
                    } else {
                        previous_new_page = false;
                        r"\vspace{0.5cm}"
                    }
                )
            })
            .collect::<Vec<String>>()
            .join("");

        format!(
            r"\documentclass{{article}}

            \usepackage{{array}}
            \usepackage{{calc}}
            \usepackage{{fontspec}}
            \usepackage{{geometry}}
            \usepackage{{hyperref}}
            \usepackage{{makecell}}
            \usepackage{{multirow}}
            \usepackage{{tabularx}}
            
            \setlength{{\oddsidemargin}}{{-40pt}}
            \setlength{{\textwidth}}{{532pt}}
            \newlength{{\tablewidth}}
            \setlength{{\tablewidth}}{{0.8\textwidth}}

            \geometry{{
                a4paper,
                total={{190mm,257mm}},
                left=10mm,
                top=7.5mm,
                bottom=10mm
            }}
            \setmainfont{{FreeSans}}
            \pagenumbering{{gobble}}
            \begin{{document}}
            {}
            \end{{document}}",
            groups
        )
    }

    fn get_team_match_plans_as_latex(&self) -> String {
        let matchplans = self.teams.as_ref().unwrap().iter().enumerate().map(|(group_idx, group)| {
            format!("
                {}
                ",
                group.iter().enumerate().map(|(team_idx, team)| {
                    let mut matches : Vec<&Match> =
                        self.matches[group_idx]
                        .iter()
                        .filter(|&_match| {
                            _match.team_a == team_idx || _match.team_b == team_idx
                        })
                        .collect();

                    // TODO: Is matcehes vector already sorted, e.g. is sorting necessary?
                    matches.sort_by(|a, b| { a.batch.cmp(&b.batch) });

                    let matches_string =
                        matches
                        .iter()
                        .map(|_match| {
                            if _match.result == MatchResult::Break {
                                String::from(r"\rule[3pt]{\dimexpr0.8\textwidth - \tabcolsep}{0.4pt} & & & & & & & & & & & & & & & & & & & \small Pause \\
        ")
                            } else {
                            let (opponent_idx, start_of_match) = if _match.team_a == team_idx {(_match.team_b, true)} else {(_match.team_a, false)};
                                format!(
        r"
        \small {} & \small {} & \small {} & & & & & & & & & & & & & & & & & \small {} \\
        ",
                                format!("{}{}", if start_of_match {"@"} else {""}, opponent_idx + 1),
                                _match.lane + 1,
                                _match.team_a + 1,
                                self.teams.as_ref().unwrap()[group_idx][opponent_idx].name
                                )
                            }
                        }).collect::<Vec<String>>().join(r"\hline");

                    format!(
    r#"
    \begin{{tabularx}}{{\textwidth}}{{
        *{{3}}{{|>{{\centering\arraybackslash\hsize=0.06\hsize}}X}}
        *{{8}}{{|>{{\centering\arraybackslash\hsize=0.0325\hsize}}X}}
        "
        *{{8}}{{>{{\centering\arraybackslash\hsize=0.0325\hsize}}X|}}
        >{{\centering\arraybackslash\hsize=0.3\hsize}}X|
    }}
        \multicolumn{{20}}{{>{{\hsize=\dimexpr\textwidth-2\tabcolsep-2\arrayrulewidth\relax}}X}}{{\small {}. {}}} \\
        \hline
        \small Geg. & \small Bahn & \small Ans & \multicolumn{{16}}{{>{{\hsize=\dimexpr0.4875\hsize\relax}}X|}}{{~}} & \small Verein  \\
        \hline
        {matches_string}
        \hline
    \end{{tabularx}}"#,
                        team_idx + 1,
                        team.name
                    )
                }).collect::<Vec<String>>().join(r"\\[2cm]
            "))
        }).collect::<Vec<String>>().join(r"\\[2cm]
        ");
        format!(
            r#"
    \documentclass{{article}}

    \usepackage[a4paper]{{geometry}}
    \usepackage{{fontspec}}
    \usepackage{{tabularx}}

    \geometry{{
        a4paper,
        total={{200mm,290mm}},
        left=0mm,
        top=2mm,
    }}

    \newcommand{{\thickhline}}{{
        \noalign {{\ifnum 0=`}}\fi \hrule height 1.5pt
        \futurelet \reserved@a \@xhline
    }}
    \newcolumntype{{"}}{{@{{\hskip\tabcolsep\vrule width 1.5pt\hskip\tabcolsep}}}}

    \setmainfont{{FreeSans}}
    \pagenumbering{{gobble}}

    \begin{{document}}
    \Large
    
    {matchplans}

    \end{{document}}
    "#
        )
    }

    fn get_lane_match_plans_as_latex(&self) -> String {
        let matchplans = self
            .matches
            .iter()
            .enumerate()
            .map(|(group_idx, group_matches)| {
                let group_name = &self.group_names.as_ref().unwrap()[group_idx];
                group_matches
                .iter()
                .filter(|&_match| _match.result != MatchResult::Break)
                .map(|_match| {
                    let team_a_name = &self.teams.as_ref().unwrap()[group_idx][_match.team_a].name;
                    let team_b_name = &self.teams.as_ref().unwrap()[group_idx][_match.team_b].name;
                    format!(
r"
    \LARGE
    \begin{{tabularx}}{{\textwidth}}{{
        |>{{\centering\arraybackslash\hsize=0.1\hsize}}X
        *{{6}}{{|>{{\centering\arraybackslash\hsize=0.0458\hsize}}X}}
        |>{{\centering\arraybackslash\hsize=0.1\hsize}}X
        |
        *{{6}}{{>{{\centering\arraybackslash\hsize=0.0458\hsize}}X|}}
        >{{\centering\arraybackslash\hsize=0.1\hsize}}X|
        >{{\centering\arraybackslash\hsize=0.15\hsize}}X|
        }}
        \hline
        \multicolumn{{8}}{{|l|}}{{\large \textbf{{{}. {}}}}} & \multicolumn{{8}}{{r|}}{{\large \textbf{{{}. {}}}}} \\
        \hline
        & \small 1 & \small 2 & \small 3 & \small 4 & \small 5 & \small 6 & \small Summe & \small 1 & \small 2 & \small 3 & \small 4 & \small 5 & \small 6 & \small Summe & \small Anspiel {} \\
        \hline
        + & & & & & & & & & & & & & & & \small  Bahn {} \\
        \hline
        -- &&&&&&&&&&&&&&& \small Spiel {} \\
        \hline
        &&&&&&&&&&&&&&& \small {} \\
        \hline
        \multicolumn{{8}}{{|c|}}{{\multirow{{2}}{{*}}{{\shortstack[c]{{\small \\[0.75cm]\rule{{0.8\dimexpr0.475\textwidth}}{{0.4pt}}\\\footnotesize Unterschrift {}}}}}}} & \multicolumn{{8}}{{c|}}{{\multirow{{2}}{{*}}{{\shortstack[c]{{\small \\[0.75cm]\rule{{0.8\dimexpr0.525\textwidth}}{{0.4pt}}\\\footnotesize Unterschrift {}}}}}}} \\
        \multicolumn{{8}}{{|c|}}{{}} & \multicolumn{{8}}{{c|}}{{}} \\
        \hline
    \end{{tabularx}}
",
                    _match.team_a + 1,
                    team_a_name,
                    _match.team_b + 1,
                    team_b_name,
                    _match.team_a + 1,
                    _match.lane + 1,
                    _match.batch + 1,
                    group_name,
                    team_b_name,
                    team_a_name,
                    )
                }).collect::<Vec<String>>().join(r"    \\[2cm]
    ")
            })
            .collect::<Vec<String>>()
            .join(r"    \\[2cm]
    ");

        format!(
            r"
\documentclass{{article}}
\usepackage[a4paper]{{geometry}}
\usepackage{{fontspec}}
\usepackage{{tabularx}}
\usepackage{{diagbox}}
\usepackage{{multirow}}

\geometry{{
    a4paper,
    total={{200mm,290mm}},
    left=0mm,
    top=2mm,
}}
\setmainfont{{FreeSans}}
\pagenumbering{{gobble}}

\begin{{document}}
    {}
\end{{document}}
",
            matchplans
        )
    }

    fn get_as_json_string(&self) -> String {
        let teams = if let Some(teams_vec) = self.teams.as_ref() {
            teams_vec
                .iter()
                .map(|group| {
                    format!(
                        r"[
            {}
        ]",
                        group
                            .iter()
                            .map(|team| {
                                format!(
                                    r#"{{
                "name": "{}",
                "region": "{}",
                "player_names": [
                    {}
                ]
            }}"#,
                                    &team.name,
                                    &team.region,
                                    team.player_names
                                        .iter()
                                        .map(|player_name_opt| {
                                            if let Some(player_name) = player_name_opt {
                                                format!("\"{player_name}\"")
                                            } else {
                                                String::from("null")
                                            }
                                        })
                                        .collect::<Vec<String>>()
                                        .join(",\n                    ")
                                )
                            })
                            .collect::<Vec<String>>()
                            .join(",\n            ")
                    )
                })
                .collect::<Vec<String>>()
                .join(",\n        ")
        } else {
            String::from("")
        };

        let group_names = if let Some(group_names) = self.group_names.as_ref() {
            group_names
                .iter()
                .map(|group_name| format!("\"{group_name}\""))
                .collect::<Vec<String>>()
                .join(",\n        ")
        } else {
            String::from("")
        };

        let matches = self
            .matches
            .iter()
            .map(|group| {
                format!(
                    "[
            {}
        ]",
                    group
                        .iter()
                        .map(|_match| {
                            let points = if let Some(points) = _match.points {
                                format!(
                                    r#"[
                    {},
                    {}
                ]"#,
                                    points[0], points[1]
                                )
                            } else {
                                String::from("null")
                            };
                            format!(
                                r#"{{
                "team_a": {},
                "team_b": {},
                "points": {points},
                "result": "{}",
                "batch": {},
                "lane": {}
            }}"#,
                                _match.team_a, _match.team_b, _match.result, _match.batch, _match.lane,
                            )
                        })
                        .collect::<Vec<String>>()
                        .join(",\n            ")
                )
            })
            .collect::<Vec<String>>()
            .join(",\n        ");

        let current_batch = self
            .current_batch
            .iter()
            .map(|batch| batch.to_string())
            .collect::<Vec<String>>()
            .join(",\n        ");

        format!(
            r#"{{
    "name": "{}",
    "date_string": "{}",
    "place": "{}",
    "executor": "{}",
    "organizer": "{}",
    "referee": "{}",
    "competition_manager": "{}",
    "clerk": "{}",
    "additional_text": "{}",
    "count_teams": {},
    "count_groups": {},
    "team_distribution": [
        {},
        {}
    ],
    "teams": [
        {teams}
    ],
    "group_names": [
        {group_names}
    ],
    "matches": [
        {matches}
    ],
    "current_batch": [
        {current_batch}
    ],
    "with_break": {}
}}
"#,
            self.name,
            self.date_string,
            self.place,
            self.executor,
            self.organizer,
            self.referee,
            self.competition_manager,
            self.clerk,
            self.additional_text.replace("\n", r"\n"),
            self.count_teams,
            self.count_groups,
            self.team_distribution[0],
            self.team_distribution[1],
            self.with_break,
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Team {
    pub name: String,
    pub region: String,
    pub player_names: [Option<String>; 6], // maximal 6 possible players per team
}

#[derive(Debug)]
pub struct InterimResultEntry {
    pub team_idx: usize,
    pub match_points: [i32; 2],
    pub stock_points: [i32; 2],
    pub difference: i32,
    pub quotient: f32,
}

pub type MatchID = u32;

#[derive(Debug, Clone, Deserialize)]
pub struct Match {
    // the both opponents
    pub team_a: usize,
    pub team_b: usize,
    pub points: Option<[i32; 2]>, // the points of the teams if the match was already played
    pub result: MatchResult,      // the result of the match
    pub batch: u32,               // the index of the batch the match is in, e.g. "Spiel 4"
    pub lane: u32,                // the number of the lane the match is played on, e.g. "Bahn 2"
    pub id: MatchID,              // the unique id of the match (unique within a competition)
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum MatchResult {
    WinnerA,
    Draw,
    WinnerB,
    NotPlayed,
    Break,
}

impl Display for MatchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MatchResult::WinnerA => "WinnerA",
                MatchResult::Draw => "Draw",
                MatchResult::WinnerB => "WinnerB",
                MatchResult::NotPlayed => "NotPlayed",
                MatchResult::Break => "Break",
            }
        )
    }
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
