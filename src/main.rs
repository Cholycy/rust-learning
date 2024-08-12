use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use std::io::{self, Write};
use std::collections::BTreeMap;

#[derive(Clone, serde::Deserialize, Serialize)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

impl Todo {
    fn new(id: u32, title: &str) -> Self {
        Todo {
            id,
            title: title.to_string(),
            completed: false,
        }
    }
}

struct TodoList {
    todos: BTreeMap<u32, Todo>,
    next_id: u32,
}

impl TodoList {
    fn new() -> Self{
        TodoList { todos:BTreeMap::new(), next_id: 1, }
    }

    fn save(&self){
        let file = File::create("todos.json").expect("Unable to create file");
        serde_json::to_writer(file, &self.todos).expect("Unable to write data");
    }

    fn load(&mut self){
        let path = Path::new("todos.json");
        if path.exists(){
            let file = File::open(path).expect("Unable to open file");
            self.todos = serde_json::from_reader(file).expect("Unable to read data");
            // self.next_id = self.todos.keys().copied().max().unwrap_or(0) + 1;
        }

    }

    fn renumber_ids(&mut self){
        let mut new_todos = BTreeMap::new();
        let mut new_id = 1;

        for todo in self.todos.values(){
            let mut new_todo = todo.clone();
            new_todo.id = new_id;
            new_todos.insert(new_id, new_todo);
            new_id += 1;
        }

        self.todos = new_todos;
        self.next_id = new_id;
    }

    fn add(&mut self, title: &str){
        let todo = Todo::new(self.next_id, title);
        self.todos.insert(self.next_id, todo);
        self.next_id += 1;
    }

    fn list(&mut self){
        self.renumber_ids();
        for todo in self.todos.values(){
            println!(
                "{}: [{}] {}",
                todo.id,
                if todo.completed {"X"} else {" "},
                todo.title
            )
        };
    }

    fn complete(&mut self, id: u32){
        if let Some(todo) = self.todos.get_mut(&id){
            todo.completed = true;
        }else{
            print!("Todo with id {} not found.", id);
        }
    }

    fn remove(&mut self, id: u32){
        self.todos.remove(&id);
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    todo_list.load();
    
    loop{
        println!("Little todo has started!");
        println!("1. Add Todo");
        println!("2. List Todos");
        println!("3. Complete Todo");
        println!("4. Remove Todo");
        println!("5. Exit");
        println!("Please input one of the above options:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim();

        match choice{
            "1" => {
                let mut title = String::new();
                print!("Enter todo title: ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin().read_line(&mut title).expect("Failed to read line");
                todo_list.add(title.trim());
            }

            "2" => {
                println!("List Todos: ");
                io::stdout().flush().expect("Failed to flush stdout");
                todo_list.list();
            }

            "3" => {
                print!("Enter Complete Todo id: ");
                io::stdout().flush().expect("Failed to flush stdout");
                let mut id=String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id:u32 = id.trim().parse().expect("Invalid id");
                todo_list.complete(id);
            }

            "4" => {
                print!("Enter remove Todo id: ");
                io::stdout().flush().expect("Failed to flush stdout");
                let mut id=String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id:u32 = id.trim().parse().expect("Invalid id");
                todo_list.remove(id);
            }

            "5" => {
                todo_list.save();
                break;
            }
            _ => println!("Invalid choice, please retry."),
        }
    }
}
