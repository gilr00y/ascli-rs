use clap::{App, Arg};
use std::cmp::min;

fn main() {
  let matches = App::new("ASCII Graphs")
    .version("1.0")
    .author("Your Name <your.email@example.com>")
    .about("Displays ASCII graphs (bar charts or line graphs) in the terminal")
    .arg(
      Arg::new("graph_type")
        .short('t')
        .long("type")
        .value_name("GRAPH_TYPE")
        // .about("Sets the graph type: 'bar' or 'line'")
        .required(true)
        .takes_value(true),
    )
    .arg(
      Arg::new("values")
        .short('v')
        .long("values")
        .value_name("VALUES")
        // .about("Sets the values for the graph")
        .required(true)
        .multiple_values(true)
        .takes_value(true),
    )
    .get_matches();

  let graph_type = matches.value_of("graph_type").unwrap();
  let values: Vec<usize> = matches
    .values_of("values")
    .unwrap()
    .map(|v| v.parse::<usize>().unwrap())
    .collect();

  match graph_type {
    "bar" => draw_bar_chart(&values),
    "line" => draw_line_graph(&values),
    _ => eprintln!("Invalid graph type. Use 'bar' or 'line'."),
  }
}

fn draw_bar_chart(values: &[usize]) {
  let max_value = match values.iter().max() {
    Some(max) => *max,
    None => return,
  };

  for value in values {
    let bar_width = (value * 60) / max_value;
    let bar = "=".repeat(min(bar_width, 60));
    println!("{:<4} | {:<60}", value, bar);
  }

  // Print x-axis
  println!("    +{:-<60}", "");
  println!("    0{:<59}", values.len() - 1);
}

fn draw_line_graph(values: &[usize]) {
  let max_value = match values.iter().max() {
    Some(max) => *max,
    None => return,
  };

  let width = 60;
  let height = 12;
  let mut grid = vec![vec![' '; width]; height];

  for (index, &value) in values.iter().enumerate() {
    if index < values.len() - 1 {
      let x1 = (index * (width - 1)) / (values.len() - 1);
      let y1 = height - 1 - (value * (height - 1)) / max_value;
      let x2 = ((index + 1) * (width - 1)) / (values.len() - 1);
      let y2 = height - 1 - (values[index + 1] * (height - 1)) / max_value;

      let dx = x2 as isize - x1 as isize;
      let dy = y2 as isize - y1 as isize;
      let steps = usize::max(dx.abs() as usize, dy.abs() as usize);

      for i in 0..=steps {
        let x = x1 as isize + (i as isize * dx) / steps as isize;
        let y = y1 as isize + (i as isize * dy) / steps as isize;
        grid[y as usize][x as usize] = '*';
      }
    }
  }

  // Print y-axis labels and the grid
  for (i, row) in grid.iter().enumerate() {
    let y_label = (max_value * (height-i)) / (height - 1);
    print!("{:<4}| ", y_label);
    let line: String = row.iter().collect();
    println!("{}", line);
  }

  // Print x-axis
  println!("    +{:-<60}", "");
  println!("    0{:<59}", values.len() - 1);
}
