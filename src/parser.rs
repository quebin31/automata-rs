use crate::automata::transition::Transition;
use crate::automata::Automata;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub trait AutomataParser {
    fn parse(&self) -> Automata;
}

#[derive(Debug, Clone, Default)]
pub struct FileParser {
    filename: String,
}

#[derive(Debug, PartialEq, Eq)]
enum Expecting {
    Nothing,
    NumberOfStates,
    States,
    NumberOfAcceptStates,
    AcceptStates,
    NumberOfSymbols,
    Symbols,
    NumberOfTransitions,
    Transitions,
}

impl FileParser {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_filename(filename: &str) -> Self {
        Self {
            filename: filename.to_owned(),
        }
    }

    pub fn change_filename(&mut self, filename: &str) {
        self.filename = filename.to_owned();
    }
}

impl AutomataParser for FileParser {
    fn parse(&self) -> Automata {
        let path = Path::new(&self.filename)
            .canonicalize()
            .expect("Failed to canonicalize");

        let file =
            File::open(&path).unwrap_or_else(|_| panic!("Failed to open file in path: {:?}", path));
        let reader = BufReader::new(file);
        let mut expecting = Expecting::Nothing;
        let mut total_found = 0;
        let mut total_expected = 0;

        let mut automata = Automata::new();
        for line in reader.lines() {
            let line = line.expect("Failed to get line");
            match line.trim() {
                "Estados" => expecting = Expecting::NumberOfStates,
                "Estados de aceptación" => expecting = Expecting::NumberOfAcceptStates,
                "Alfabeto" => expecting = Expecting::NumberOfSymbols,
                "Transiciones" => expecting = Expecting::NumberOfTransitions,
                "" => continue,
                line => match expecting {
                    Expecting::NumberOfStates => {
                        expecting = Expecting::States;
                        total_expected = line.parse().expect("Failed to convert number")
                    }

                    Expecting::States => {
                        let states: Vec<_> = line.split_ascii_whitespace().collect();
                        if total_expected > states.len() {
                            panic!("Too much/few states!");
                        }

                        for state in states {
                            automata.push_state(state.into());
                        }
                    }

                    Expecting::NumberOfAcceptStates => {
                        expecting = Expecting::AcceptStates;
                        total_expected = line.parse().expect("Failed to convert number")
                    }

                    Expecting::AcceptStates => {
                        let states: Vec<_> = line.split_ascii_whitespace().collect();
                        if total_expected > states.len() {
                            panic!("Too much/few states!");
                        }

                        for state in states {
                            if let Some(state) = automata.find(&state.into()) {
                                automata.push_accept_state(state)
                            } else {
                                panic!("Unknown state given!");
                            }
                        }
                    }

                    Expecting::NumberOfSymbols => {
                        expecting = Expecting::Symbols;
                        total_expected = line.parse().expect("Failed to convert number")
                    }

                    Expecting::Symbols => {
                        let symbols: Vec<_> = line.split_ascii_whitespace().collect();
                        if total_expected > symbols.len() {
                            panic!("Too much/few symbols!");
                        }

                        for symbol in symbols {
                            automata.push_symbol(&symbol);
                        }
                    }

                    Expecting::NumberOfTransitions => {
                        expecting = Expecting::Transitions;
                        total_expected = line.parse().expect("Failed to convert number")
                    }

                    Expecting::Transitions => {
                        if total_found == total_expected {
                            panic!("Too much states!");
                        }

                        let transition_line: Vec<_> = line.split_ascii_whitespace().collect();
                        if transition_line.len() != 3 {
                            panic!("Transition syntax error!");
                        }

                        let symbol = if transition_line[1] == "-1" {
                            ""
                        } else {
                            transition_line[1]
                        };

                        if let Some(beg_state) = automata.find(&transition_line[0].into()) {
                            if let Some(end_state) = automata.find(&transition_line[2].into()) {
                                automata.push_transition_from(
                                    beg_state,
                                    Transition::new(symbol, end_state),
                                );
                            } else {
                                panic!("Unkown end state given!");
                            }
                        } else {
                            panic!("Unknown begin state given!");
                        }

                        total_found += 1;
                    }

                    Expecting::Nothing => panic!("Unexpected line"),
                },
            }
        }

        automata
    }
}
