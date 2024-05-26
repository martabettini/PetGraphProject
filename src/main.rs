use csv::ReaderBuilder;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let mut movies: HashMap<String, Vec<String>> = HashMap::new();
    let mut actor_indices: HashMap<String, NodeIndex> = HashMap::new();
    let mut graph = Graph::<String, u32, Undirected>::new_undirected();

    let data_dir = "dataset/";
    let title_basics_path = format!("{}{}", data_dir, "basic.finto.tsv");
    let title_principals_path = format!("{}{}", data_dir, "principals.finto.tsv");

    let _ = File::open(&title_basics_path).map_err(|e| format!("Failed to open {}: {}", title_basics_path, e))?;
    let _ = File::open(&title_principals_path).map_err(|e| format!("Failed to open {}: {}", title_principals_path, e))?;

    // Parsing title.basics.tsv to get movie IDs
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .delimiter(b'\t')
        .from_path(&title_basics_path)?;

    for result in rdr.records() {
        match result {
            Ok(record) => {
                if record.len() > 0 {
                    let movie_id = record[0].trim().to_string();
                    movies.insert(movie_id, Vec::new());
                } else {
                    println!("Record incompleto trovato: {:?}", record);
                }
            },
            Err(e) => {
                // Ignora gli errori di lettura e continua con la prossima riga
                println!("Error reading record: {}", e);
                continue;
            }
        }
    }

    // Parsing title.principals.tsv to get actors in each movie
    let mut rdr1 = ReaderBuilder::new()
        .flexible(true)
        .delimiter(b'\t')
        .from_path(&title_principals_path)?;

    for result in rdr1.records() {
        match result {
            Ok(record) => {
                //println!("{}", record.len());
                //println!("{:?}", record.get(0));
                //println!("{:?}", record.get(1));
                //println!("{:?}", record);
                
                // Accedi al campo che contiene la stringa da suddividere
                let field = &record[0]; // Supponiamo che la stringa da suddividere sia nel primo campo
                
                // Dividi la stringa in base agli spazi e colleziona le parole in un vettore
                let words: Vec<&str> = field.split_whitespace().collect();

                    let movie_id = words[0];
                    let person_id = words[2];
                    let category = words[3];
                    println!("{:?}", words[0]);
                    println!("{:?}", words[2]);
                    println!("{:?}", words[3]);

                    if category == "actor" || category == "actress" {
                        if let Some(actors) = movies.get_mut(movie_id) {
                            actors.push(person_id.to_string());
                        }
                    }
                
            },
            Err(e) => {
                // Ignora gli errori di lettura e continua con la prossima riga
                println!("Error reading record: {}", e);
                continue;
            }
        }
    }

    // Create nodes for all actors
    for actors in movies.values() {
        for actor in actors {
            actor_indices.entry(actor.clone()).or_insert_with(|| graph.add_node(actor.clone()));
        }
    }

    // Create the actor-actor graph
    for actors in movies.values() {
        for i in 0..actors.len() {
            for j in (i + 1)..actors.len() {
                let actor1 = &actors[i];
                let actor2 = &actors[j];

                let actor1_index = actor_indices[actor1];
                let actor2_index = actor_indices[actor2];

                if let Some(edge) = graph.find_edge(actor1_index, actor2_index) {
                    let edge_weight = graph.edge_weight_mut(edge).unwrap();
                    *edge_weight += 1;
                } else {
                    graph.add_edge(actor1_index, actor2_index, 1);
                }
            }
        }
    }

    // Example of printing the graph
    for node in graph.node_indices() {
        println!("{:?}: {:?}", node, graph[node]);
    }

    for edge in graph.edge_indices() {
        let (source, target) = graph.edge_endpoints(edge).unwrap();
        println!("{:?} -- {:?} --> {:?} : {:?}", graph[source], graph[edge], graph[target], graph.edge_weight(edge).unwrap());
    }

    Ok(())
}

