use crate::othello::{Othello, State};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq)]
struct Node {
    parent: Option<Rc<RefCell<Node>>>,
    position: (usize, usize),
    turn: State,
    unexplored: Vec<(usize, usize)>,
    children: Vec<Rc<RefCell<Node>>>,
    wins: i32,
    visits: u32,
}

impl Node {
    fn new(parent: Option<Rc<RefCell<Node>>>, position: (usize, usize), turn: State, unexplored: Vec<(usize, usize)>) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            parent,
            position,
            turn,
            unexplored,
            children: Vec::new(),
            wins: 0,
            visits: 0,
        }))
    }

    fn select_child(&self) -> Rc<RefCell<Node>> {
        let mut best_child_idx = 0;
        let mut best_value = std::f32::MIN;
        let log_total = (2.0 * self.visits as f32).log2();
        for (i, child) in self.children.iter().enumerate() {
            let child_ = child.borrow();
            let uct_value = child_.wins as f32 / child_.visits as f32 + (log_total / child_.visits as f32).sqrt();
            if uct_value > best_value {
                best_child_idx = i;
                best_value = uct_value;
            }
        }
        self.children[best_child_idx].clone()
    }

    fn get_most_visited_child_position(&self) -> (usize, usize) {
        let mut most_visited_child = None;
        let mut most_visits = 0;
        for child in &self.children {
            let child_ = child.borrow();
            if child_.visits > most_visits {
                most_visited_child = Some(child);
                most_visits = child_.visits;
            }
        }
        most_visited_child.unwrap().borrow().position
    }

    fn update(&mut self, result: State) {
        if result == State::Draw {
            // no change to node's wins on draw
        } else if (result == State::BlackWon) == (self.turn == State::BlackTurn) {
            self.wins -= 1; // opponent won
        } else {
            self.wins += 1;
        }
        self.visits += 1;
    }
}

pub fn mcts_move(othello: &Othello, iterations: i16) -> (usize, usize) {
    let root = Node::new(None, (0, 0), othello.state, othello.get_valid_moves());
    for _ in 0..iterations {
        let mut node = root.clone();
        let mut simulation = othello.clone();
        // SELECT
        while node.borrow().unexplored.is_empty() && !node.borrow().children.is_empty() {
            let child = node.borrow().select_child();
            simulation.make_move(child.borrow().position);
            node = child;
        }
        // EXPAND
        if !node.borrow().unexplored.is_empty() {
            let rand_idx = rand::random::<usize>() % node.borrow().unexplored.len();
            let explored_move = node.borrow().unexplored[rand_idx];
            node.borrow_mut().unexplored.remove(rand_idx);
            simulation.make_move(explored_move);
            let child = Node::new(Some(node.clone()), explored_move, simulation.state, simulation.get_valid_moves());
            node.borrow_mut().children.push(child.clone());
            node = child;
        }
        // SIMULATE
        while [State::BlackTurn, State::WhiteTurn].contains(&simulation.state) {
            let valid_moves = simulation.get_valid_moves();
            let random_move = valid_moves[rand::random::<usize>() % valid_moves.len()];
            simulation.make_move(random_move);
        }
        // BACKPROPAGATE
        let result = simulation.state;
        while node.borrow().parent.is_some() {
            node.borrow_mut().update(result);
            let parent = node.borrow().parent.as_ref().unwrap().clone();
            node = parent;
        }
        node.borrow_mut().update(result);
    }
    let best_move = root.borrow().get_most_visited_child_position();
    best_move
}
