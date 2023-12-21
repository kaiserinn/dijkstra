use prettytable::{Table, Row, cell, format, row};

#[derive(Debug)]
pub struct Dijkstra<'a> {
    adjacency_matrix: Vec<Vec<i32>>,
    vertices: Vec<&'a str>,
    source: &'a str,
    queue: Vec<Vertex>,
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    shortest_distance: f64,
    prev_vertex: Option<usize>,
}

impl<'a> Dijkstra<'a> {
    pub fn new(adjacency_matrix: Vec<Vec<i32>>, vertices: Vec<&'a str>, source: &'a str) -> Dijkstra<'a> {
        let mut queue = vec![Vertex { shortest_distance: std::f64::INFINITY, prev_vertex: None }; vertices.len()];
        let index = vertices.iter().position(|&i| i == source).unwrap();
        queue[index].shortest_distance = 0.0;

        Dijkstra {
            adjacency_matrix,
            vertices,
            source,
            queue,
        }
    }

    pub fn run(&mut self) -> Vec<(usize, Vec<Vertex>)> {
        let mut unvisited = self.vertices.clone();
        let mut history: Vec<(usize, Vec<Vertex>)> = Vec::new();

        while !unvisited.is_empty() {
            let mut shortest_index = self.vertices.iter().position(|&i| i == unvisited[0]).unwrap();
            for vertex in &unvisited {
                let curr_index = self.vertices.iter().position(|&i| &i == vertex).unwrap();
                if self.queue[curr_index].shortest_distance < self.queue[shortest_index].shortest_distance {
                    shortest_index = curr_index;
                }
            }

            for (i, distance) in self.adjacency_matrix[shortest_index].iter().enumerate() {
                if distance > &0 {
                    let new_distance = self.queue[shortest_index].shortest_distance + *distance as f64;
                    if  new_distance < self.queue[i].shortest_distance {
                        self.queue[i].shortest_distance = new_distance;
                        self.queue[i].prev_vertex = Some(shortest_index);
                    }
                }
            }

            unvisited.retain(|&i| i != self.vertices[shortest_index]);
            history.push((shortest_index, self.queue.clone()));
        }

        history
    }

    pub fn shortest_path(&self, destination: &str) -> Option<Vec<String>> {
        let dest_index = self.vertices.iter().position(|&i| i == destination)?;
        let source_index = self.vertices.iter().position(|&i| i == self.source)?;
        let mut path: Vec<usize> = Vec::new();

        let mut vertex = dest_index;
        while vertex != source_index {
            path.push(vertex);
            vertex = self.queue[vertex].prev_vertex?;
        }
        path.push(source_index);

        let mut path_string: Vec<String> = Vec::new();
        for i in path {
            path_string.push(self.vertices[i].to_string());
        }

        path_string.reverse();
        Some(path_string)
    }

    pub fn print_table(&self, history: Vec<(usize, Vec<Vertex>)>) {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);

        let mut title_row = Row::new(vec![cell!("v")]);
        for title in &self.vertices {
            title_row.add_cell(cell!(title));
        }
        table.set_titles(title_row);

        for history_item in history {
            let mut row = Row::new(vec![cell!(self.vertices[history_item.0])]);
            for vertex in history_item.1 {
                match vertex.prev_vertex {
                    Some(value) => {
                        row.add_cell(cell!(format!("{}_{}", vertex.shortest_distance, self.vertices[value])));
                    },
                    None => {
                        row.add_cell(cell!(vertex.shortest_distance));
                    }
                }
            }
            table.add_row(row);
        }

        println!("{table}");
    }

    pub fn print_table_alt(&self) {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);
        table.set_titles(row!["Vertex", "Distance", "Previous"]);

        for (i, vertex) in self.queue.iter().enumerate() {
            match vertex.prev_vertex {
                Some(value) => {
                    table.add_row(row![self.vertices[i], vertex.shortest_distance, self.vertices[value]]);
                },
                None => {
                    table.add_row(row![self.vertices[i], vertex.shortest_distance, "-"]);
                }
            }
        }

        println!("{table}");
    }
}
