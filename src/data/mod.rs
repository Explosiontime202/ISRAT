use chrono::offset::Local;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::thread::{self, JoinHandle};
// use tectonic::config::PersistentConfig;
// use tectonic::driver::{OutputFormat, ProcessingSessionBuilder};
// use tectonic::status::NoopStatusBackend;

use self::read_write::read_from_file;
use crate::data::read_write::save_to_file;

pub mod read_write;

pub type GroupID = u32;
pub type MatchID = u32;
pub type TeamID = u32;

#[derive(Debug)]
pub struct Competition {
    pub data: Option<CompetitionData>,
    pub spawned_threads: Vec<JoinHandle<()>>, // stores the join handles to the threads used to export pdf and html documents
    pub current_interim_result: Vec<Vec<InterimResultEntry>>, // a ResultEntry vector for each group in descending order
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

    #[must_use]
    pub fn get_current_interim_result_for_group(&mut self, group_idx: usize) -> &Vec<InterimResultEntry> {
        // calculate interim result if not available
        if self.current_interim_result[group_idx].is_empty() {
            debug_assert!(self.data.is_some());
            self.current_interim_result[group_idx] = self.data.as_mut().unwrap().calc_interim_result_for_group(group_idx);
        }
        &self.current_interim_result[group_idx]
    }

    #[must_use]
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

    #[must_use]
    pub fn save_to_file(&self) -> Result<(), String> {
        if self.absolute_file_path.is_none() {
            return Err(String::from("No file path available"));
        }

        if self.data.is_none() {
            return Err(String::from("No competition data available!"));
        }

        let json = match serde_json::to_string(self.data.as_ref().unwrap()) {
            Ok(json) => json,
            Err(err) => return Err(err.to_string()),
        };

        match fs::write(self.absolute_file_path.as_ref().unwrap(), json) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }

    #[must_use]
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

            /*let mut status = NoopStatusBackend::default();
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
            println!("Finished export!");*/
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

#[derive(Debug, Clone, Deserialize, Serialize)]
/// The relevant data about the competition.
pub struct CompetitionData {
    /// The name of the competition.
    pub name: String,
    /// The date when the competition takes place, as String.
    pub date_string: String,
    /// The time when the competition takes place, as String.
    pub time_string: String,
    /// The location where the competition takes place.
    pub location: String,
    /// The executor of the competition, e.g. a sports association.
    pub executor: String,
    /// The organizer of the competition, i.e. a sports club.
    pub organizer: String,
    /// The referee of the competition.
    pub referee: String,
    /// The competition manager of the competition, i.e. the one responsible.
    pub competition_manager: String,
    /// The secretary, tasked to enter the results.
    pub secretary: String,
    /// Additional text at the end of the result list, mostly greetings and sponsor texts.
    pub additional_text: String,
    /// The cumulative amount of teams over all groups.
    pub count_teams: u32,
    /// The groups the competition consists of.
    pub groups: Vec<Group>,
    /// A set of all currently assigned group ids.
    assigned_group_ids: HashSet<GroupID>,
    /// A set of all currently assigned team ids.
    assigned_team_ids: HashSet<TeamID>,
    /// A set of all currently assigned match ids.
    assigned_match_ids: HashSet<TeamID>,
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
            date_string: String::new(),
            time_string: String::new(),
            location: String::new(),
            executor: String::new(),
            organizer: String::new(),
            referee: String::new(),
            competition_manager: String::new(),
            secretary: String::new(),
            additional_text: String::new(),
            count_teams: 0,
            groups: Vec::new(),
            assigned_group_ids: HashSet::default(),
            assigned_team_ids: HashSet::default(),
            assigned_match_ids: HashSet::default(),
        }
    }

    ///
    /// Create a new duplicate-free group id.
    ///
    pub fn new_group_id(&mut self) -> GroupID {
        loop {
            let id = rand::thread_rng().gen();
            if !self.assigned_group_ids.contains(&id) {
                self.assigned_group_ids.insert(id);
                break id;
            }
        }
    }

    ///
    /// Create a new duplicate-free tean id.
    ///
    pub fn new_team_id(&mut self) -> TeamID {
        loop {
            let id = rand::thread_rng().gen();
            if !self.assigned_team_ids.contains(&id) {
                self.assigned_team_ids.insert(id);
                break id;
            }
        }
    }

    ///
    /// Calc the current interim result for all groups.
    ///
    fn calc_all_interim_result(&mut self) -> Vec<Vec<InterimResultEntry>> {
        (0..self.groups.len())
            .map(|group_idx| self.calc_interim_result_for_group(group_idx))
            .collect()
    }

    ///
    /// Calc the current interim result for the group with index `group_idx`.
    ///
    pub fn calc_interim_result_for_group(&self, group_idx: usize) -> Vec<InterimResultEntry> {
        let group = &self.groups[group_idx];

        // create table with entries for all teams in this group
        let mut table: Vec<InterimResultEntry> = (0..group.teams.len())
            .map(|team_idx| InterimResultEntry {
                team_idx,
                match_points: [0, 0],
                stock_points: [0, 0],
                difference: 0,
                quotient: 0.0,
            })
            .collect();

        // evaluate the match results for this group
        group
            .matches
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

    ///
    /// Returns a vector of matches that will be played in the next batch for a given group at index `group_idx`.
    ///
    pub fn next_matches_for_group(&self, group_idx: usize) -> Vec<&Match> {
        let group = &self.groups[group_idx];
        let next_matches = group
            .matches
            .iter()
            .filter(|&match_| match_.result != MatchResult::Break)
            .filter(|&match_| match_.batch == group.current_batch)
            .collect::<Vec<&Match>>();
        next_matches.iter().for_each(|match_| assert_eq!(match_.result, MatchResult::NotPlayed));
        next_matches
    }

    ///
    /// Returns the vector of teams which are on break in the next batch for a given group at index `group_idx`.
    ///
    pub fn teams_on_break_for_group(&self, group_idx: usize) -> Vec<&Team> {
        let group = &self.groups[group_idx];
        group
            .matches
            .iter()
            .filter(|&match_| match_.result == MatchResult::Break)
            .filter(|&match_| match_.batch == group.current_batch)
            .map(|match_| &group.teams[match_.team_a])
            .collect()
    }

    ///
    /// Generate the matches for each group.
    ///
    pub fn generate_matches(&mut self) {
        // inline new_match_id here to satisfy borrow checker
        let mut new_match_id = || loop {
            let id = rand::thread_rng().gen();
            if !self.assigned_match_ids.contains(&id) {
                self.assigned_match_ids.insert(id);
                break id;
            }
        };

        for group in self.groups.iter_mut() {
            assert!(group.matches.is_empty());
            // reserve space for matches: everyone vs everyone (n * (n - 1) / 2) + a break for everyone (n9)
            let team_count = group.teams.len();

            group.matches.reserve(team_count * (team_count - 1) / 2 + team_count);
            group.current_batch = 0;

            if team_count % 2 == 0 {
                // even team count
                if group.with_break {
                    for batch_idx in 0..team_count {
                        // "(batch_idx * 2) / team_count" is a elegant way to subtract 1 if batch_idx >= team_count / 2 else 0
                        for lane_idx in 0..(team_count / 2 - (batch_idx * 2 / team_count)) {
                            // "(batch_idx * 2) / team_count" see above, but add 1 instead of subtract
                            let team_a_idx = (lane_idx as i32 - batch_idx as i32 + (batch_idx as i32 * 2 / team_count as i32))
                                .rem_euclid(team_count as i32) as usize;
                            let team_b_idx = (team_count as i32 - lane_idx as i32 - batch_idx as i32 - 1).rem_euclid(team_count as i32) as usize;
                            group.matches.push(Match {
                                team_a: team_a_idx,
                                team_b: team_b_idx,
                                points: None,
                                result: MatchResult::NotPlayed,
                                batch: batch_idx as u32,
                                lane: lane_idx as u32,
                                id: new_match_id(),
                            })
                        }

                        // add breaks for the teams
                        if batch_idx >= team_count / 2 {
                            let break_idx_a = (team_count as i32 / 2 - batch_idx as i32).rem_euclid(team_count as i32) as usize;
                            group.matches.push(Match {
                                team_a: break_idx_a,
                                team_b: break_idx_a,
                                points: None,
                                result: MatchResult::Break,
                                batch: batch_idx as u32,
                                lane: u32::MAX,
                                id: new_match_id(),
                            });
                            group.matches.push(Match {
                                team_a: (team_count - batch_idx),
                                team_b: (team_count - batch_idx),
                                points: None,
                                result: MatchResult::Break,
                                batch: batch_idx as u32,
                                lane: u32::MAX,
                                id: new_match_id(),
                            });
                        }
                    }
                    group.count_batches = team_count as u32;
                } else {
                    // TODO: Implement match generation for even team count per group without breaks
                    todo!();
                }
            } else {
                // uneven team count
                for batch_idx in 0..team_count {
                    // add matches for this batch
                    for lane_idx in 0..(team_count / 2) {
                        let team_a_idx = (lane_idx as i32 - batch_idx as i32).rem_euclid(team_count as i32) as usize;
                        let team_b_idx = (team_count as i32 - lane_idx as i32 - batch_idx as i32 - 2).rem_euclid(team_count as i32) as usize;
                        group.matches.push(Match {
                            team_a: team_a_idx,
                            team_b: team_b_idx,
                            points: None,
                            result: MatchResult::NotPlayed,
                            batch: batch_idx as u32,
                            lane: lane_idx as u32,
                            id: new_match_id(),
                        });
                    }

                    // add breaks from this batch
                    let break_idx = team_count - batch_idx - 1;
                    group.matches.push(Match {
                        team_a: break_idx,
                        team_b: break_idx,
                        points: None,
                        result: MatchResult::Break,
                        batch: batch_idx as u32,
                        lane: u32::MAX,
                        id: new_match_id(),
                    });
                }
                group.count_batches = team_count as u32;
            }
        }
    }

    ///
    /// Sets the results and points of the matches according to `match_results`.
    /// It is required for those matches, to have as result MatchResult::NotPlayed.
    /// Increases the current batch of the group.
    ///
    pub fn enter_match_results(&mut self, group_idx: usize, match_results: HashMap<MatchID, [u32; 2]>) {
        let mut group = &mut self.groups[group_idx];
        assert!(group.current_batch < group.count_batches);
        for match_ in &mut group.matches {
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
        group.current_batch += 1;
    }

    ///
    /// Returns the current batch for a group.
    ///
    pub fn get_current_batch(&self, group_idx: usize) -> u32 {
        self.groups[group_idx].current_batch
    }

    ///
    /// Returns the matches for the a given group at `index` and a given batch `batch_idx`.
    ///
    pub fn get_batch_for_group(&self, group_idx: usize, batch_idx: u32) -> Vec<&Match> {
        self.groups[group_idx].matches.iter().filter(|_match| _match.batch == batch_idx).collect()
    }

    pub fn get_result_as_html(&self) -> String {
        /*format!(
            r#"<html><head> This is sparta! {}<\head> <\html>"#,
            self.name
        )*/
        todo!();
    }

    ///
    /// Returns the header of a page as latex string.
    ///
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
            self.location,
            self.executor,
            Local::now().format("%d.%m.%Y %H:%M"),
        )
    }

    ///
    /// Returns the result list as latex string.
    ///
    fn get_result_as_latex(&self, current_interim_result: &Vec<Vec<InterimResultEntry>>) -> String {
        // TODO: make this configurable by the user
        let player_names_until = 3; // index of the first team which has NO player names displayed

        let header = self.get_header_as_latex();

        // analyze if all matches were played
        let is_final_result = self
            .groups
            .iter()
            .flat_map(|group| &group.matches)
            .all(|_match| _match.result != MatchResult::NotPlayed);

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
            self.secretary
        );

        let mut count_teams_with_name = 0;
        let mut count_teams_without_name = 0;
        let mut previous_new_page = true; // determines whether the header is printed, for first team true

        let groups = self.groups.iter().enumerate().map(|(group_idx, group)| {
            let group_result = current_interim_result[group_idx].iter().enumerate().map(|(rank, i_res)| {
                let team = &group.teams[i_res.team_idx];
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
                \LARGE \textbf{{{}}}
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
        if is_final_result {format!("Ergebnisliste {}", group.name)} else {format!("Zwischenliste {} nach Spiel {}", group.name, group.current_batch)},
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

    ///
    /// Returns the start list as latex string.
    ///
    fn get_start_list_as_latex(&self) -> String {
        let header = self.get_header_as_latex();

        let mut previous_new_page = true; // determines whether the header is printed, for first team true

        let groups = self
            .groups
            .iter()
            .enumerate()
            .map(|(group_idx, group)| {
                let group_result = group
                    .teams
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
                    group.name,
                    group_result,
                    if self.groups[group_idx].teams.len() > 15 {
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

    ///
    /// Return the team match plan as latex string.
    ///
    fn get_team_match_plans_as_latex(&self) -> String {
        let matchplans = self.groups.iter().map(| group| {
            format!("
                {}
                ",
                group.teams.iter().enumerate().map(|(team_idx, team)| {
                    let mut matches : Vec<&Match> =
                        group.matches
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
                                group.teams[opponent_idx].name
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
            .groups
            .iter()
            .map(|group| {
                group.matches
                .iter()
                .filter(|&_match| _match.result != MatchResult::Break)
                .map(|_match| {
                    let team_a_name = &group.teams[_match.team_a].name;
                    let team_b_name = &group.teams[_match.team_b].name;
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
                    group.name,
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
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Defines a group.
pub struct Group {
    /// The name of the group.
    pub name: String,
    /// The teams in the group.
    pub teams: Vec<Team>,
    /// Whether there is a break for each team, only relevant for even team counts.
    pub with_break: bool,
    /// The amount of batches in this group.
    pub count_batches: u32,
    /// The batch the group is currently in.
    pub current_batch: u32,
    /// The matches for the group.
    pub matches: Vec<Match>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Defines a team.
pub struct Team {
    /// The id of the team.
    pub id: TeamID,
    /// The name of the team.
    pub name: String,
    /// The region of the team.
    pub region: String,
    /// The names of the players, at most 6.
    pub player_names: [Option<String>; 6], // maximal 6 possible players per team
}

impl Team {
    pub fn new(data: &mut CompetitionData, name: String, region: String) -> Self {
        Self::with_player_names(data, name, region, Default::default())
    }

    pub fn with_player_names(data: &mut CompetitionData, name: String, region: String, player_names: [Option<String>; 6]) -> Self {
        Self {
            id: data.new_team_id(),
            name,
            region,
            player_names,
        }
    }
}

#[derive(Debug)]
/// Defines a interim result entry.
pub struct InterimResultEntry {
    /// The idx of the team that is represented by this entry.
    pub team_idx: usize,
    /// The match points of this team.
    pub match_points: [i32; 2],
    /// The stock points of this team.
    pub stock_points: [i32; 2],
    /// The difference in stock points of this team.
    pub difference: i32,
    /// The quotient of the stock points of this team.
    pub quotient: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Defines a match.
pub struct Match {
    /// Opponent A.
    pub team_a: usize,
    /// Opponent B.
    pub team_b: usize,
    /// The points of the teams if the match was already played else None.
    pub points: Option<[i32; 2]>,
    /// The result of the match.
    pub result: MatchResult,
    /// The index of the batch the match is in, e.g. "Spiel 4".
    pub batch: u32,
    /// The number of the lane the match is played on, e.g. "Bahn 2".
    pub lane: u32,
    /// The unique id of the match (unique within a competition).
    pub id: MatchID,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize)]
/// A result of a match.
pub enum MatchResult {
    /// Opponent A won.
    WinnerA,
    /// The match ended with a draw.
    Draw,
    /// Opponent B won.
    WinnerB,
    /// The match was not yet played.
    NotPlayed,
    /// The match represents a break.
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
