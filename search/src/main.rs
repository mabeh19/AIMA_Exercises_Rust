
mod algorithms;

use algorithms::best_first_search;

fn main() {
    let res = best_first_search::best_first_search().expect("No path found");
    let node = Some(Box::new(res));
    println!("Optimal path from Arad to Bucharest:");
    while node.is_some() {
        let n = node.as_ref().unwrap();
        println!("      {}", n.state);
        
    }
}
