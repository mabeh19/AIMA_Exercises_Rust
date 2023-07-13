use itertools::Itertools;

pub mod algorithms;

use crate::algorithms::{
    constraint,
    ac3,
    backtracking_search,
    min_conflicts
};

fn main() {
    println!("Painting australia: ");
    try_paint_australia();
    println!("Solve sudoku: ");
    try_sudoku();
}

fn try_sudoku() {
    
    /* Initialize Sudoku Problem */
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
    let squares: Vec<Vec<String>> = vec![
        vec!["A1".to_string(), "A2".to_string(), "A3".to_string(), "B1".to_string(), "B2".to_string(), "B3".to_string(), "C1".to_string(), "C2".to_string(), "C3".to_string()],
        vec!["A4".to_string(), "A5".to_string(), "A6".to_string(), "B4".to_string(), "B5".to_string(), "B6".to_string(), "C4".to_string(), "C5".to_string(), "C6".to_string()],
        vec!["A7".to_string(), "A8".to_string(), "A9".to_string(), "B7".to_string(), "B8".to_string(), "B9".to_string(), "C7".to_string(), "C8".to_string(), "C9".to_string()],
        vec!["D1".to_string(), "D2".to_string(), "D3".to_string(), "E1".to_string(), "E2".to_string(), "E3".to_string(), "F1".to_string(), "F2".to_string(), "F3".to_string()],
        vec!["D4".to_string(), "D5".to_string(), "D6".to_string(), "E4".to_string(), "E5".to_string(), "E6".to_string(), "F4".to_string(), "F5".to_string(), "F6".to_string()],
        vec!["D7".to_string(), "D8".to_string(), "D9".to_string(), "E7".to_string(), "E8".to_string(), "E9".to_string(), "F7".to_string(), "F8".to_string(), "F9".to_string()],
        vec!["G1".to_string(), "G2".to_string(), "G3".to_string(), "H1".to_string(), "H2".to_string(), "H3".to_string(), "I1".to_string(), "I2".to_string(), "I3".to_string()],
        vec!["G4".to_string(), "G5".to_string(), "G6".to_string(), "H4".to_string(), "H5".to_string(), "H6".to_string(), "I4".to_string(), "I5".to_string(), "I6".to_string()],
        vec!["G7".to_string(), "G8".to_string(), "G9".to_string(), "H7".to_string(), "H8".to_string(), "H9".to_string(), "I7".to_string(), "I8".to_string(), "I9".to_string()],
    ];
    let variables: [[String; 9]; 9] = [
        std::array::from_fn(|i| format!("A{}", i + 1)),
        std::array::from_fn(|i| format!("B{}", i + 1)),
        std::array::from_fn(|i| format!("C{}", i + 1)),
        std::array::from_fn(|i| format!("D{}", i + 1)),
        std::array::from_fn(|i| format!("E{}", i + 1)),
        std::array::from_fn(|i| format!("F{}", i + 1)),
        std::array::from_fn(|i| format!("G{}", i + 1)),
        std::array::from_fn(|i| format!("H{}", i + 1)),
        std::array::from_fn(|i| format!("I{}", i + 1))
    ];
    
    let mut i = 0;
    for row in variables.as_ref() {
        for v in row {
            sudoku.add_variable(&v, None, domain.clone(), Some(row.clone().into_iter().filter(|i| i != v).map(|i| i.to_owned()).collect()));
            for s in squares.clone() {
                if s.contains(&v) {
                    alldiff2binary!(sudoku, s);
                }
            }
        }
        let mut col: Vec<String> = Vec::new();
        let all_variables = variables.as_ref().into_iter().flatten().collect::<Vec<&String>>();
        for c in 0..9 {
            col.push(all_variables[9 * c + i].to_owned());
        }
        alldiff2binary!(sudoku, col);
        let tmp = row.clone();
        alldiff2binary!(sudoku, tmp);
        i += 1;
    }

    /* Set values of known cells */
    sudoku.set_domain("A2", vec![Some(9)]);
    sudoku.set_domain("A4", vec![Some(1)]);
    sudoku.set_domain("B1", vec![Some(1)]);
    sudoku.set_domain("B3", vec![Some(6)]);
    sudoku.set_domain("B6", vec![Some(8)]);
    sudoku.set_domain("B7", vec![Some(7)]);
    sudoku.set_domain("C4", vec![Some(5)]);
    sudoku.set_domain("C8", vec![Some(3)]);
    sudoku.set_domain("D5", vec![Some(6)]);
    sudoku.set_domain("D9", vec![Some(7)]);
    sudoku.set_domain("E3", vec![Some(2)]);
    sudoku.set_domain("F1", vec![Some(4)]);
    sudoku.set_domain("F3", vec![Some(8)]);
    sudoku.set_domain("F6", vec![Some(1)]);
    sudoku.set_domain("F7", vec![Some(6)]);
    sudoku.set_domain("G2", vec![Some(2)]);
    sudoku.set_domain("G7", vec![Some(4)]);
    sudoku.set_domain("H2", vec![Some(7)]);
    sudoku.set_domain("H6", vec![Some(9)]);
    sudoku.set_domain("I1", vec![Some(9)]);
    sudoku.set_domain("I3", vec![Some(4)]);
    sudoku.set_domain("I4", vec![Some(8)]);
    sudoku.set_domain("I9", vec![Some(5)]);

    /* Solve problem */
    if let Some(solution) = min_conflicts::min_conflicts(&sudoku, 1000) {
        println!("solution = ");
        for v in solution.get_variables().values().sorted_by_key(|x| x.get_name()) {
            println!("  {} = {:?}", v.get_name(), v.get_domain());
        }
//        println!("a solution exists for this csp: {:?}", solution);
    } else {
        println!("no solution exists");
    }

    //if let Some(solution) = backtracking_search::backtracking_search(&sudoku) {
    //    println!("solution = ");
    //    for v in solution.get_variables().values().sorted_by_key(|x| x.get_name()) {
    //        println!("  {} = {:?}", v.get_name(), v.get_domain());
    //    }
//  //      println!("a solution exists for this csp: {:?}", solution);
    //} else {
    //    println!("no solution exists");
    //}
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
    if let Some(solution) = backtracking_search::backtracking_search(&paint_australia) {
        println!("Solution = ");
        for v in solution.get_variables().values() {
            println!("  {} = {:?}", v.get_name(), v.get_domain());
        }
//        println!("A solution exists for this CSP: {:?}", solution);
    } else {
        println!("No solution exists");
    }
}
