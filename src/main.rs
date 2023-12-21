use dijkstra::Dijkstra;

fn main() {
    let vertices = vec!["Monaire", "Poirott", "Milis", "Bouche", "Tempest", "Ranoa", "Jura", "Asura"];
    let adjacency_matrix = vec![
        vec![0, 4, 13, 0, 0, 0, 0, 0],
        vec![0, 0, 5, 8, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 5, 10, 0, 0],
        vec![0, 0, 0, 0, 3, 0, 3, 14],
        vec![0, 0, 0, 0, 0, 6, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 5],
        vec![0, 0, 0, 0, 0, 0, 0, 12],
        vec![0, 0, 0, 0, 0, 0, 0, 0]
    ];
    let source = "Monaire";
    let destination = "Asura";

    let mut dijkstra = Dijkstra::new(adjacency_matrix, vertices, source);

    let history = dijkstra.run();

    let path = dijkstra.shortest_path(destination);
    match path {
        Some(value) => println!("{}", value.join(" -> ")),
        None => eprintln!("There exists no path from {source} to {destination}"),
    }
    println!();

    dijkstra.print_table(history);
    dijkstra.print_table_alt();
}
