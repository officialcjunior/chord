use std::collections::HashMap;
extern crate rand;
use rand::seq::SliceRandom; // 0.7.2

use std::env;
use std::io;


/*

28
1

*/

/* experimental */
struct Ring {
    ring: HashMap <u32, Node>,
}

#[derive(Debug)]
struct Node {
    finger_table : HashMap<u32, u32>,
}

pub fn lookup(node_id: u32, key: u32, ring: HashMap<u32, HashMap<u32, u32>>) {
    let mut values = vec![0];            

    let finger_table = ring.get(&node_id).expect("Node doesn't exist");

    for val in finger_table.values() {
        values.push(*val);
    }

    values.sort();
    values.reverse();

    print!("{:?}", values);

    for i in &values {
       // println!("\nLooking for {:?} at {:?} at node {:?} \n", key, i, node_id);
        if i < &key {
            println!("\n \n Lookup passed to node {:?}", *i);
            lookup(*i, key, ring.clone());
        }
        if i == &key {
            println!(" \n{:?} found at {:?}", key, node_id);
            return;
        }
    }
}

fn main() {
    let active_nodes: Vec<usize> = vec![1, 4, 9, 11, 14, 18, 20, 21, 28];
    let args: Vec<String> = env::args().collect();

    let number_of_rings :u32= args[1].parse().unwrap();
    let number_of_entries :u32= args[2].parse().unwrap();

    let values: Vec<usize> = vec![4,4,9,9,18];

    let mut ring: HashMap<u32, HashMap<u32, u32>> = HashMap::new();

    for node in &active_nodes {
        let mut finger_table: HashMap <u32, u32> = HashMap::new();

        for i in 1 .. number_of_entries {
            let b : u32 = rand::random::<u32>() % 28;
            finger_table.insert(i, b);
        };

        if *node == 1 {
        finger_table = HashMap::from([
            (1,4),
            (2,4),
            (3,9),
            (4,9),
            (5,18),
            ]);
        }

        if *node == 4 {
        finger_table = HashMap::from([
            (1,9),
            (2,9),
            (3,9),
            (4,14),
            (5,20),
            ]);
        }

        if *node == 9 {
        finger_table = HashMap::from([
            (1,11),
            (2,11),
            (3,14),
            (4,18),
            (5,28),
            ]);
        }

        if *node == 18 {
        finger_table = HashMap::from([
            (1,20),
            (2,20),
            (3,28),
            (4,28),
            (5,4),
            ]);
        } 

        if *node == 11 {
        finger_table = HashMap::from([
            (1,14),
            (2,14),
            (3,18),
            (4,20),
            (5,28),
            ]);
        } 

        if *node == 14 {
        finger_table = HashMap::from([
            (1,18),
            (2,18),
            (3,18),
            (4,28),
            (5,1),
            ]);
        } 

        if *node == 18 {
        finger_table = HashMap::from([
            (1,20),
            (2,20),
            (3,28),
            (4,28),
            (5,4),
            ]);
        } 

        if *node == 20 {
        finger_table = HashMap::from([
            (1,21),
            (2,28),
            (3,28),
            (4,29),
            (5,4),
            ]);
        }                                                       

        if *node == 21 {
        finger_table = HashMap::from([
            (1,28),
            (2,28),
            (3,28),
            (4,1),
            (5,9),
            ]);
        }   

        println!("node ={:?} {:?}", node, finger_table);
        ring.insert((*node).try_into().unwrap(), finger_table);
    }

    println!("1- Perform a lookup ");
    println!("2- Add a node to the ring");
    println!("3- Display the finger table for a given node");
    println!("4- Remove the given node");
    println!("5- Quit");

    loop {
        println!("");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice: u32 = input.trim().parse().unwrap();
        println!("");

        if choice == 1 {
            let mut input = String::new();
            println!("Enter the key and initial node");
            io::stdin().read_line(&mut input).unwrap();
            let key: u32 = input.trim().parse().unwrap();            

            input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let node_id: u32 = input.trim().parse().unwrap();            

            lookup(node_id, key, ring.clone());


        } else if choice == 2 {
            let mut input = String::new();
            println!("Enter the key of the new node");
            io::stdin().read_line(&mut input).unwrap();
            let node_id: u32 = input.trim().parse().unwrap();

            let mut node: HashMap<u32, u32> = HashMap::new();
            for j in 1 .. number_of_entries {
                let b : u32 = rand::random::<u32>();
                node.insert(j, b);
            };
            ring.insert(node_id, node);
            
        } else if choice == 3 {
            println!("Enter the node_id you want to display");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let n: u32 = input.trim().parse().unwrap();
           // println!("{:?}", ring.get(&n));

            for entries in &ring.get(&n) {
                println!("{:?}", entries);
            }
        } else if choice == 4 {
            println!("Enter the node_id you want to remove");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let n: u32 = input.trim().parse().unwrap();
            ring.remove(&n);
            println!("Removed Node {}", n);
        } else if choice == 5 {
            break;
        }
    }
}