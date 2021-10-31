// *******************************************************************************************************************
//
// Civic Senior Blockchain Engiennier Challenge 2
//
// *******************************************************************************************************************

// Usages.
use std::collections::HashMap;
use std::env;
use std::process;

// Node struct.
#[derive(Debug)]
struct Node {
  name: String,
  depth: i32,
  branch: Vec<String>,
}

// Implementation to check the equality among two Nodes, used in tests
impl PartialEq for Node {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name && self.depth == other.depth && self.branch == other.branch
  }
}

// Depth is given by the index of the letter.
fn get_node_depth(node: &str) -> i32 {
  let letters = HashMap::from([
    ("a", 1),
    ("b", 2),
    ("c", 3),
    ("d", 4),
    ("e", 5),
    ("f", 6),
    ("g", 7),
    ("h", 8),
    ("i", 9),
    ("j", 10),
    ("k", 11),
    ("l", 12),
    ("m", 13),
    ("n", 14),
    ("o", 15),
    ("p", 16),
    ("q", 17),
    ("r", 18),
    ("s", 19),
    ("t", 20),
    ("u", 21),
    ("v", 22),
    ("w", 23),
    ("x", 24),
    ("y", 25),
    ("z", 26),
  ]);
  return letters[node];
}

// ***
// Function to make the dag from the given string passed as a vector.
// ***
fn make_dag(mut edges_arr: Vec<&str>) -> Vec<Node> {
  // unitiliazed DAG
  let mut tree_dag: Vec<Node> = Vec::new();
  let mut is_inserted = vec![];

  for i in edges_arr.iter_mut() {
    let branch_arr: Vec<&str> = i.split('-').collect();
    let depth_aux = get_node_depth(branch_arr[0]);

    // check if the node has already been added
    if is_inserted.contains(&branch_arr[0]) {
      // if it already exist, then we add the new branch to the node
      let mut index = 0;
      let mut name_aux: String = "init".to_owned();
      let mut depth_aux: i32 = 0;
      let mut branch_aux: Vec<String> = vec![];
      for (pos, e) in tree_dag.iter().enumerate() {
        if e.name == branch_arr[0] {
          name_aux = e.name.clone();
          depth_aux = e.depth.clone();
          branch_aux = e.branch.clone();
          branch_aux.push(branch_arr[1].to_string());
          index = pos;
        }
      }
      let node = Node {
        name: name_aux,
        depth: depth_aux,
        branch: branch_aux,
      };
      tree_dag[index] = node;
    } else {
      // node doesnt not exits, we initialize it, we send the branch as an vect
      let node = Node {
        name: branch_arr[0].to_string(),
        depth: depth_aux,
        branch: [branch_arr[1].to_string()].to_vec(),
      };
      tree_dag.push(node);
      is_inserted.push(branch_arr[0]);
    }
  }
  return tree_dag;
} // end make_dag

// ***
// Function strip to remove the node and reconnect the edges with the parent node.
// ***
fn strip_dag(_del_ver: &str, mut tree_dag: Vec<Node>) -> Vec<Node> {
  // find index of the node we are deleting
  let mut index_delete = 0;
  for (pos, e) in tree_dag.iter().enumerate() {
    if e.name == _del_ver {
      index_delete = pos;
    }
  }

  // the childs will reconnect to the parent node from the deleated vertex
  let mut parent_node_branch = vec![];

  // find parent key
  // 2) sort by depth
  tree_dag.sort_by_key(|d| d.depth);
  //dagArr = sortDagArr(dagArr)

  // 3) find first higher depth
  let mut parent_index = 0;
  for (pos, i) in tree_dag.iter().enumerate() {
    //if the node is a child we reconnect it
    if get_node_depth(&i.name) < get_node_depth(_del_ver) {
      // it will update until it wont succed, the dagArr has the depth increasing
      parent_node_branch = i.branch.clone();
      parent_index = pos;
    }
  }

  // Reconnect childs with parent node
  for i in tree_dag[index_delete].branch.iter_mut() {
    parent_node_branch.push(i.to_string());
  }
  tree_dag[parent_index].branch = parent_node_branch.clone();

  // Remove all vertex where _delVer appears
  for i in tree_dag.iter_mut() {
    i.branch.retain(|x| x != _del_ver);
  }

  // remove vertex node, the strip character node
  tree_dag.remove(index_delete);

  return tree_dag;
} // end strip_dag

// ***
// Function to print the DAG, sort alphabetically.
// ***
fn print_dag(tree_dag: Vec<Node>) -> String {
  let mut vect_out = vec![];
  let mut s_out = "".to_owned();
  for i in tree_dag.iter() {
    let node = i;
    for a in node.branch.iter() {
      let s = node.name.clone() + "-" + a;
      vect_out.push(s);
    }
  }
  // alphabtecially order the DAG
  vect_out.sort();

  // Pass the vector into a string following the pattern from the requirements
  for i in vect_out.iter() {
    s_out = s_out + i + ",";
  }

  // remove last character, the last ','
  s_out.pop();

  return s_out;
} // end print_dag

// ***
// Function to call all the secondary functions to perform the script objective.
// The objective is to create a DAG, Strip it, and Print it.
// ***
pub fn wrapper(dag: &str, del_ver: &str) -> String {
  println!("{:}", "********************".to_owned());
  println!("{:}", " ".to_owned());
  println!("{:}", "Input: ".to_owned() + &dag.to_owned());
  println!("{:}", "Strip: ".to_owned() + &del_ver.to_owned());
  let mut edges: Vec<&str> = dag.split(',').collect();
  edges.sort();
  let _tree_dag = make_dag(edges);
  let strip_tree_dag = strip_dag(del_ver, _tree_dag);
  let print_tree_dag = print_dag(strip_tree_dag);
  println!("{:}", "Output: ".to_owned() + &print_tree_dag);
  println!("{:}", " ".to_owned());
  println!("{:}", "********************".to_owned());
  return print_tree_dag;
} // end wrapper

// ***********************************************************
//               *** Main function ***
// ***********************************************************
fn main() {
  // Obtain input parameters from the CLI
  let args: Vec<String> = env::args().collect();

  // check if user wants to see the example
  if args.len() == 2 && &args[1] == "example" {
    wrapper("a-b,b-c,c-d", "c");
    process::exit(1);
  }
  // check if user has entered 3 parameters
  if args.len() == 3 {
    // I do not check the validity of the parameter sintax
    let dag = &args[1];
    let del_ver = &args[2];
    wrapper(dag, del_ver);
  } else {
    // For all the other cases, we show the help menu
    println!("{:}", "** ** ** ** ** ** **".to_owned());
    println!("{:}", "  *  *  *  *  *  *  ".to_owned());
    println!("{:}", "   *   *   *   *     ".to_owned());
    println!("{:}", " ".to_owned());
    println!("{:}", "Welcome to the DAG world! :)".to_owned());
    println!("{:}", "We love DAGs, this script allows you to create a DAG and strip one edge by reconnecting it to the parent node. The parent node is the parent from the node that you removed.".to_owned());
    println!("{:}", " ".to_owned());
    println!("{:}", "Menu bar: ".to_owned());
    println!("{:}", "  1) Write: cargo run dag removeNode".to_owned());
    println!("{:}", "     a) Where, dag= a-b,b-c,c-d,c-e,e-f".to_owned());
    println!(
      "{:}",
      "     b) Where, removeNode is one character, for example c".to_owned()
    );
    println!(
      "{:}",
      "  2) If you want to see an example: cargo run example".to_owned()
    );
    println!(
      "{:}",
      "     Here, the hardcoded input is: cargo run a-b,b-c,c-d c".to_owned()
    );
    println!("{:}", "  3) To come back here: cargo run".to_owned());
    println!("{:}", " ".to_owned());
    println!("{:}", "IMPORTANT:".to_owned());
    println!(
      "{:}",
      "  1) The program does not check for the syntax of the input parameters".to_owned()
    );
    println!(
      "{:}",
      "     a) In the DAG: Do not enter whitespaces: between characters, beginning or end"
        .to_owned()
    );
    println!(
      "{:}",
      "     b) In the DAG: Input all characters in lowercase".to_owned()
    );
    println!(
      "{:}",
      "     c) Make sure the DAG follows the pattern: a-b,b-c".to_owned()
    );
    println!(
      "{:}",
      "     d) The depth of the node is given by the alphabetical order".to_owned()
    );
    println!("{:}", " ".to_owned());
    println!("{:}", "Powered by git: miguelCyclone".to_owned());
    println!("{:}", " ".to_owned());
    println!("{:}", "   *   *   *   *     ".to_owned());
    println!("{:}", "  *  *  *  *  *  *  ".to_owned());
    println!("{:}", "** ** ** ** ** ** **".to_owned());
  }
} // end main

//
// **************
//  Test modules
// **************
//

//
// Unit tests
//
#[cfg(test)]
mod unit {
  use super::*;
  #[test]
  fn node_depth_ok() {
    assert_eq!(get_node_depth("a"), 1);
    assert_eq!(get_node_depth("k"), 11);
    assert_eq!(get_node_depth("z"), 26);
  }

  #[test]
  fn make_dag_one() {
    let dag = "a-b,c-d,b-c";
    let mut edges: Vec<&str> = dag.split(',').collect();
    edges.sort();
    let vect = make_dag(edges);
    let vect_hard_coded = [
      Node {
        name: "a".to_string(),
        depth: 1,
        branch: ["b".to_string()].to_vec(),
      },
      Node {
        name: "b".to_string(),
        depth: 2,
        branch: ["c".to_string()].to_vec(),
      },
      Node {
        name: "c".to_string(),
        depth: 3,
        branch: ["d".to_string()].to_vec(),
      },
    ];

    assert_eq!(vect.len(), vect_hard_coded.len());
    for (pos, e) in vect.iter().enumerate() {
      assert_eq!(*e, vect_hard_coded[pos]);
    }
  }

  #[test]
  fn make_dag_two() {
    let dag = "a-b,b-c,c-d,b-d,c-e";
    let mut edges: Vec<&str> = dag.split(',').collect();
    edges.sort();
    let vect = make_dag(edges);
    let vect_hard_coded = [
      Node {
        name: "a".to_string(),
        depth: 1,
        branch: ["b".to_string()].to_vec(),
      },
      Node {
        name: "b".to_string(),
        depth: 2,
        branch: ["c".to_string(), "d".to_string()].to_vec(),
      },
      Node {
        name: "c".to_string(),
        depth: 3,
        branch: ["d".to_string(), "e".to_string()].to_vec(),
      },
    ];

    assert_eq!(vect.len(), vect_hard_coded.len());
    for (pos, e) in vect.iter().enumerate() {
      assert_eq!(*e, vect_hard_coded[pos]);
    }
  }

  #[test]
  fn strip_dag_one() {
    let del_ver = "c";
    let dag = "a-b,c-d,b-c";
    let mut edges: Vec<&str> = dag.split(',').collect();
    edges.sort();
    let vect = make_dag(edges);
    let strip_tree_dag = strip_dag(del_ver, vect);

    let vect_strip_hard_coded = [
      Node {
        name: "a".to_string(),
        depth: 1,
        branch: ["b".to_string()].to_vec(),
      },
      Node {
        name: "b".to_string(),
        depth: 2,
        branch: ["d".to_string()].to_vec(),
      },
    ];
    assert_eq!(strip_tree_dag.len(), vect_strip_hard_coded.len());
    for (pos, e) in strip_tree_dag.iter().enumerate() {
      assert_eq!(*e, vect_strip_hard_coded[pos]);
    }
  }

  #[test]
  fn strip_dag_two() {
    let del_ver = "c";
    let dag = "a-b,b-c,c-d,b-d,c-e";
    let mut edges: Vec<&str> = dag.split(',').collect();
    edges.sort();
    let vect = make_dag(edges);
    let strip_tree_dag = strip_dag(del_ver, vect);

    let vect_strip_hard_coded = [
      Node {
        name: "a".to_string(),
        depth: 1,
        branch: ["b".to_string()].to_vec(),
      },
      Node {
        name: "b".to_string(),
        depth: 2,
        branch: ["d".to_string(), "d".to_string(), "e".to_string()].to_vec(),
      }
    ];

    assert_eq!(strip_tree_dag.len(), vect_strip_hard_coded.len());
    for (pos, e) in strip_tree_dag.iter().enumerate() {
      assert_eq!(*e, vect_strip_hard_coded[pos]);
    }
  }

  #[test]
  fn print_dag_one() {
    let del_ver = "c";
    let dag= "a-b,c-d,b-c";
    let mut edges: Vec<&str> = dag.split(',').collect();
    edges.sort();
    let vect = make_dag(edges);
    let strip_tree_dag = strip_dag(del_ver, vect);
    let print_tree_dag = print_dag(strip_tree_dag);
    let print_tree_dag_hard_coded = "a-b,b-d";
    assert_eq!(print_tree_dag, print_tree_dag_hard_coded);
  }

  #[test]
  fn print_dag_two() {
    let del_ver = "c";
    let dag = "a-b,b-c,c-d,b-d,c-e";
    let mut edges: Vec<&str> = dag.split(',').collect();
    edges.sort();
    let vect = make_dag(edges);
    let strip_tree_dag = strip_dag(del_ver, vect);
    let print_tree_dag = print_dag(strip_tree_dag);
    let print_tree_dag_hard_coded = "a-b,b-d,b-d,b-e";
    assert_eq!(print_tree_dag, print_tree_dag_hard_coded);
  }

}// end unit test

//
// System tests
//
#[cfg(test)]
mod system {
  use super::*;

  #[test]
  fn default_sample_dag_one() {
    assert_eq!(wrapper("a-b,b-c,c-d", "c"), "a-b,b-d");
  }

  #[test]
  fn dag_two() {
    assert_eq!(wrapper("a-b,b-c,c-d,b-d,c-e", "c"), "a-b,b-d,b-d,b-e");
  }

  #[test]
  fn dag_three() {
    assert_eq!(wrapper("a-b,e-g,g-h,b-c,c-d,b-d,c-e,e-f", "c"), "a-b,b-d,b-d,b-e,e-f,e-g,g-h");
  }
} // end system test

//
// End test modules
//