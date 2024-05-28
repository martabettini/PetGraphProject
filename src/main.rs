use csv::ReaderBuilder;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::dot::{Dot, Config};
use petgraph::Undirected;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct TitleBasicsRecord {
    tconst: String,
    titleType: String,
    primaryTitle: String,
    originalTitle: String,
    isAdult: String,
    startYear: String,
    endYear: String,
    runtimeMinutes: String,
    genres: String,
}

#[derive(Debug, Deserialize)]
struct TitlePrincipalsRecord {
    tconst: String,
    ordering: String,
    nconst: String,
    category: String,
    job: String,
    characters: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut movies: HashMap<String, Vec<String>> = HashMap::new();
    let mut actor_indices: HashMap<String, NodeIndex> = HashMap::new();
    
    let mut graph = Graph::<String, u32, Undirected>::new_undirected();

    let data_dir = "dataset/";
    let title_basics_path = format!("{}{}", data_dir, "title.basics.tsv");
    let title_principals_path = format!("{}{}", data_dir, "title.principals.tsv");

    let _ = File::open(&title_basics_path).map_err(|e| format!("Failed to open {}: {}", title_basics_path, e))?;
    let _ = File::open(&title_principals_path).map_err(|e| format!("Failed to open {}: {}", title_principals_path, e))?;

    // Parsing title.basics.tsv to get movie IDs
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .delimiter(b'\t')
        .from_path(&title_basics_path)?;
    
    let mut count = 0;
    for result in rdr.deserialize() {
        if count >= 1000_000 {
            break;
        }
        let record: TitleBasicsRecord = result?;
        println!("{:?}", record.tconst);
        movies.insert(record.tconst, Vec::new());
        count += 1;
    }

    // Parsing title.principals.tsv to get actors in each movie
    let mut rdr1 = ReaderBuilder::new()
        .flexible(true)
        .delimiter(b'\t')
        .from_path(&title_principals_path)?;

    count = 0;
    for result in rdr1.deserialize() {
        if count >= 1000_000 {
            break;
        }
        let record: TitlePrincipalsRecord = result?;
        if record.category == "actor" || record.category == "actress" {
            if let Some(actors) = movies.get_mut(&record.tconst) {
                actors.push(record.nconst);
            }
        }
        count += 1;
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
    //let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    //std::fs::write("graph.dot", format!("{:?}", dot))?;

    println!("Graph saved to graph.dot");

    Ok(())
}