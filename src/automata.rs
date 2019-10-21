pub mod state;
pub mod transition;

use state::{Set, State};
use std::collections::HashSet;
use std::fmt;
use std::ops::Index;
use transition::Transition;

#[derive(Debug, Clone, Default)]
pub struct Automata {
    alphabet: Vec<String>,
    states: Vec<State>,
    entry_state: usize,
    accept_states: Vec<usize>,
    transitions: Vec<Vec<Transition>>,
}

pub trait AutomataIndex {
    fn index(&self, automata: &Automata) -> usize;
}

impl AutomataIndex for usize {
    fn index(&self, _automata: &Automata) -> usize {
        *self
    }
}

impl AutomataIndex for State {
    fn index(&self, automata: &Automata) -> usize {
        automata.find(self).unwrap()
    }
}

impl<I: AutomataIndex> Index<I> for Automata {
    type Output = State;

    fn index(&self, index: I) -> &Self::Output {
        &self.states[index.index(self)]
    }
}

impl Automata {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn len(&self) -> usize {
        self.states.len()
    }

    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }

    pub fn find(&self, state: &State) -> Option<usize> {
        for (index, s) in self.states.iter().enumerate() {
            if s == state {
                return Some(index);
            }
        }

        None
    }

    pub fn push_state(&mut self, state: State) {
        self.states.push(state);
        self.transitions.push(vec![]);
    }

    pub fn push_symbol(&mut self, symbol: &str) {
        self.alphabet.push(symbol.to_owned());
    }

    pub fn push_accept_state(&mut self, index: impl AutomataIndex) {
        self.accept_states.push(index.index(self));
    }

    pub fn push_transition_from(&mut self, index: impl AutomataIndex, transition: Transition) {
        let index = index.index(&self);
        if !self.transitions[index].contains(&transition) {
            self.transitions[index].push(transition);
        }
    }

    pub fn set_entry_state(&mut self, index: impl AutomataIndex) {
        self.entry_state = index.index(self);
    }

    pub fn transitions_from(&self, index: impl AutomataIndex) -> &Vec<Transition> {
        &self.transitions[index.index(self)]
    }

    pub fn move_from_with(&self, index: impl AutomataIndex, symbol: &str) -> Vec<usize> {
        let mut transitions = Vec::new();

        for transition in self.transitions_from(index.index(self)) {
            if transition.symbol() == symbol {
                transitions.push(transition.end_state());
            }
        }

        transitions
    }

    pub fn is_deterministic(&self) -> bool {
        for state_transition in &self.transitions {
            if state_transition.len() != self.alphabet.len() {
                return false;
            }

            let mut found_symbols = HashSet::new();
            for transition in state_transition {
                // found a repeated symbol, it's not deterministic
                if found_symbols.contains(transition.symbol()) {
                    return false;
                }

                found_symbols.insert(transition.symbol());
            }
        }

        true
    }

    pub fn e_closure_set(&self, input: &[impl AutomataIndex]) -> Vec<usize> {
        let mut stack: Vec<usize> = input.iter().map(|i| i.index(self)).collect();
        let mut e_closure_set = stack.clone();

        while let Some(state) = stack.pop() {
            let transitions_of_state = self.transitions_from(state);
            for transition in transitions_of_state {
                if transition.symbol() != "" {
                    continue;
                }

                if !e_closure_set.contains(&transition.end_state()) {
                    stack.push(transition.end_state());
                    e_closure_set.push(transition.end_state());
                }
            }
        }

        e_closure_set
    }

    pub fn to_deterministic(&self) -> Self {
        let e_closure_set = self.e_closure_set(&[self.entry_state]);
        let state = {
            let mut set = Set::new();
            for state in e_closure_set {
                set.append(&mut self[state].tags().clone());
            }

            State::from(set)
        };

        let mut afd_automata = Automata::new();
        afd_automata.push_state(state);
        afd_automata.alphabet = self.alphabet.clone();

        let mut non_marked_state = vec![0];

        while let Some(non_marked) = non_marked_state.pop() {
            for symbol in &self.alphabet {
                let state = &afd_automata[non_marked];
                let mut next_states = Vec::new();
                for tag in state.tags() {
                    let mut moved =
                        self.move_from_with(self.find(&tag.clone().into()).unwrap(), symbol);
                    next_states.append(&mut moved);
                }

                if next_states.is_empty() {
                    continue;
                }

                let e_closure_set = self.e_closure_set(&next_states);

                let mut accept_state = false;
                let state = {
                    let mut set = Set::new();
                    for state in e_closure_set {
                        set.append(&mut self[state].tags().clone());
                        accept_state = accept_state || self.accept_states.contains(&state);
                    }

                    State::from(set)
                };

                if !afd_automata.states.contains(&state) {
                    afd_automata.push_state(state.clone());
                    let index = afd_automata.find(&state).unwrap();
                    non_marked_state.push(index);

                    if accept_state {
                        afd_automata.push_accept_state(index);
                    }
                }

                let index = afd_automata.find(&state).unwrap();
                afd_automata.push_transition_from(non_marked, Transition::new(symbol, index));
            }
        }

        if afd_automata.is_deterministic() {
            return afd_automata;
        }

        afd_automata.push_state("!".into());
        let never_state_index = afd_automata.len() - 1;
        for state_transition in &mut afd_automata.transitions {
            if state_transition.len() == afd_automata.alphabet.len() {
                continue;
            }

            let mut existing = HashSet::new();
            for transition in state_transition.iter() {
                existing.insert(transition.symbol().to_owned());
            }

            for symbol in &afd_automata.alphabet {
                if !existing.contains(symbol) {
                    state_transition.push(Transition::new(symbol, never_state_index));
                }
            }
        }

        afd_automata
    }
}

impl fmt::Display for Automata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Estados")?;
        for (index, state) in self.states.iter().enumerate() {
            write!(f, "{} = {{", index)?;
            for tag in state.tags() {
                write!(f, " {}", tag)?;
            }
            writeln!(f, " }}")?;
        }

        writeln!(f, "\nEstados de aceptación")?;
        for index in &self.accept_states {
            write!(f, "{} ", index)?;
        }

        writeln!(f, "\n\nAlfabeto")?;
        for symbol in &self.alphabet {
            write!(f, "{} ", symbol)?;
        }

        writeln!(f, "\n\nTransiciones")?;
        for (index, state_transition) in self.transitions.iter().enumerate() {
            for transition in state_transition {
                let symbol = if transition.symbol() == "" {
                    "-1"
                } else {
                    transition.symbol()
                };

                writeln!(f, "{} {} {}", index, symbol, transition.end_state())?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e_closure_set() {
        let mut automata = Automata::new();
        automata.push_state("0".into());
        automata.push_state("1".into());
        automata.push_state("2".into());
        automata.push_state("3".into());
        automata.push_state("4".into());
        automata.push_state("5".into());

        automata.push_accept_state(5);

        automata.push_transition_from(0, Transition::new("a", 1));
        automata.push_transition_from(0, Transition::new("", 2));
        automata.push_transition_from(0, Transition::new("", 3));
        automata.push_transition_from(1, Transition::new("", 3));
        automata.push_transition_from(2, Transition::new("b", 3));
        automata.push_transition_from(3, Transition::new("", 4));
        automata.push_transition_from(4, Transition::new("a", 5));

        let set_01 = automata.e_closure_set(&[0, 1]);
        let set_345 = automata.e_closure_set(&[3, 4, 5]);

        assert_eq!(&set_01, &[0, 1, 3, 4, 2]);
        assert_eq!(&set_345, &[3, 4, 5]);
    }

    #[test]
    fn find_state() {
        let mut automata = Automata::new();
        automata.push_state("p".into());
        automata.push_state("q".into());
        automata.push_state("r".into());

        assert_eq!(Some(0), automata.find(&"p".into()));
        assert_eq!(Some(1), automata.find(&"q".into()));
        assert_eq!(Some(2), automata.find(&"r".into()));
    }
}
