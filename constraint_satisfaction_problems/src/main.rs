pub mod algorithms;

use crate::algorithms::{
    constraint,
    ac3
};

fn main() {

}

macro_rules! getcol {
    ( $array:tt, $index:tt ) => {
        let mut v = Vec::new();
        for row in array {
            v.push_back(row[index].unwrap());
        }

        return v;
    }
}

fn try_sudoku() {
    let mut sudoku = constraint::CSP::new();
    let domain = vec![
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
        Some(8),
        Some(9)
    ];
    let variables: [[String; 9]; 9] = [
        std::array::from_fn(|i| format!("A{}", i + 1)),
        std::array::from_fn(|i| format!("B{}", i + 1)),
        std::array::from_fn(|i| format!("C{}", i  +1)),
        std::array::from_fn(|i| format!("D{}", i + 1)),
        std::array::from_fn(|i| format!("E{}", i + 1)),
        std::array::from_fn(|i| format!("F{}", i + 1)),
        std::array::from_fn(|i| format!("G{}", i  +1)),
        std::array::from_fn(|i| format!("H{}", i + 1)),
        std::array::from_fn(|i| format!("I{}", i + 1))
    ];
    let mut i = 0;
    for row in variables.as_ref() {
        for v in row {
            sudoku.add_variable(&v, None, domain.clone(), Some(row.clone().into_iter().filter(|i| i != v).map(|i| i.to_owned()).collect()));
        }
        let mut col = Vec::new();
        let mut iter = variables.as_ref().clone().into_iter().flatten().skip(i);
        for _ in 0..9 {
            col.push(iter.next().unwrap());
            iter.skip(9);
        }
        alldiff2binary!(sudoku, col);
        let tmp = row.clone();
        alldiff2binary!(sudoku, tmp);
        i += 1;
    }
    println!("{:?}", variables);

}

fn try_paint_australia() {
    let mut paint_australia = constraint::CSP::new();
    let domain = vec![
        Some("red"),
        Some("blue"),
        Some("green")
    ];
    paint_australia.add_variable("WA", None, domain.clone(), Some(vec!["NT".to_owned(), "SA".to_owned()]));
    paint_australia.add_variable("NT", None, domain.clone(), Some(vec!["WA".to_owned(), "SA".to_owned(), "Q".to_owned()]));
    paint_australia.add_variable("SA", None, domain.clone(), Some(vec!["WA".to_owned(), "NT".to_owned(), "Q".to_owned(), "NSW".to_owned(), "V".to_owned()]));
    paint_australia.add_variable("Q", None, domain.clone(), Some(vec!["NT".to_owned(), "SA".to_owned(), "NSW".to_owned()]));
    paint_australia.add_variable("NSW", None, domain.clone(), Some(vec!["Q".to_owned(), "SA".to_owned(), "V".to_owned()]));
    paint_australia.add_variable("V", None, domain.clone(), Some(vec!["SA".to_owned(), "NSW".to_owned()]));
    paint_australia.add_variable("T", None, domain.clone(), Some(vec![]));
    paint_australia.add_constraint("WA", "NT", constraint::ConstraintType::Binary, constraint::RuleType::NotEqualTo);
    paint_australia.add_constraint("WA", "SA", constraint::ConstraintType::Binary, constraint::RuleType::NotEqualTo);
    paint_australia.add_constraint("NT", "WA", constraint::ConstraintType::Binary, constraint::RuleType::NotEqualTo);
    paint_australia.add_constraint("NT", "SA", constraint::ConstraintType::Binary, constraint::RuleType::NotEqualTo);
    paint_australia.add_constraint("NT", "Q", constraint::ConstraintType::Binary, constraint::RuleType::NotEqualTo);
    paint_australia.add_constraint("SA", "WA", constraint::ConstraintType::Binary, constraint::RuleType::NotEqualTo);
    paint_australia.add_constraint("SA", "NT", constraint::ConstraintType::Binary, constraint::RuleType::NotEqualTo);
    paint_australia.add_constraint("SA", "Q", constraint::ConstraintType::Binary, constraint::RuleType::NotEqualTo);
    paint_australia.add_constraint("SA", "NSW", constraint::ConstraintType::Binary, constraint::RuleType::NotEqualTo);
    paint_australia.add_constraint("SA", "V", constraint::ConstraintType::Binary, constraint::RuleType::NotEqualTo);
    paint_australia.add_constraint("Q", "NT", constraint::ConstraintType::Binary, constraint::RuleType::NotEqualTo);
    paint_australia.add_constraint("Q", "SA", constraint::ConstraintType::Binary, constraint::RuleType::NotEqualTo);
    paint_australia.add_constraint("Q", "NSW", constraint::ConstraintType::Binary, constraint::RuleType::NotEqualTo);
    paint_australia.add_constraint("NSW", "Q", constraint::ConstraintType::Binary, constraint::RuleType::NotEqualTo);
    paint_australia.add_constraint("NSW", "V", constraint::ConstraintType::Binary, constraint::RuleType::NotEqualTo);
    paint_australia.add_constraint("NSW", "SA", constraint::ConstraintType::Binary, constraint::RuleType::NotEqualTo);
    paint_australia.add_constraint("V", "SA", constraint::ConstraintType::Binary, constraint::RuleType::NotEqualTo);
    paint_australia.add_constraint("V", "NSW", constraint::ConstraintType::Binary, constraint::RuleType::NotEqualTo);
    if let Some(solution) = ac3::ac3(&paint_australia) {
        println!("Solution = ");
        for v in solution.get_variables().values() {
            println!("  {} = {:?}", v.get_name(), v.get_domain());
        }
//        println!("A solution exists for this CSP: {:?}", solution);
    } else {
        println!("No solution exists");
    }
}
