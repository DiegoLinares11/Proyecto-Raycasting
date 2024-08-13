mod maze; // Esto importa el módulo 'maze' desde el archivo maze.rs


use std::env;

fn main() {
    // Obtener los argumentos desde la línea de comandos
    let args: Vec<String> = env::args().collect();

    // Verificar que se hayan pasado los argumentos necesarios
    if args.len() != 3 {
        eprintln!("Uso: {} <ancho> <alto>", args[0]);
        std::process::exit(1);
    }

    // Convertir los argumentos a números enteros
    let w: usize = args[1].parse().expect("El ancho debe ser un número entero");
    let h: usize = args[2].parse().expect("El alto debe ser un número entero");

    // Crear una instancia de la estructura Maze
    let mut maze = maze::Maze::new(w, h); 
    
    // Generar el laberinto
    maze.generate(); 
    
    // Mostrar el laberinto
    println!("{}", maze::display_maze(maze.render()));
}