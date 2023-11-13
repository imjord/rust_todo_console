
use std::io;

use std::io::Write;
use std::fs;
use dialoguer::Select;
use std::fs::File;
use console::Style;
use std::str;


use std::process;
use std::path::Path;

struct Todo {
    content: String
}



impl Todo {
    fn new(content: &str) -> Self {
        Self {
            content: content.to_string()
        }
    }
}





fn get_todos()  {
    
    let green = Style::new().green();
    let yellow = Style::new().yellow();


    println!("{}", green.apply_to("Getting todos..."));
   
    let list = fs::read_to_string("list.json");

    match list {
        Ok(data) =>{
            if data.is_empty() || data.trim().is_empty() {
                println!("Todo list is empty... please create some todos!")
            } else {
                println!("Current Todos List: {}", yellow.apply_to(data));
            }
           

        }
        Err(_err) => {
            println!("There are no todos in the list. please create one");
        }
    }
}



fn delete_todos() -> Result<(), io::Error> {

    let red = Style::new().red();




    println!("{}", red.apply_to("Deleting todos..."));


    let list_path_exist: Result<bool, io::Error> = Path::new("list.json").try_exists();

    match list_path_exist {

        Ok(true) => {
            


            let list: Result<Vec<u8>, io::Error> = fs::read("list.json");

            match list {

               

                Ok(data) =>{
                    let parserd = String::from_utf8(data);

                    match parserd {
                        Ok(item) => {
                            let items: Vec<&str> = item.split('\n').collect();
                            if items.is_empty() || item.trim().is_empty() {
                                println!("the list is empty please create a todo!")
                            } else {
                                let selection = Select::new()
                                .with_prompt("Please delete a todo")
                                .items(&item.split('\n').collect::<Vec<&str>>())
                                .interact()
                                .unwrap();
    
                                if let Some(selected_item) = items.get(selection) {
                                    println!("You chose to delete: {}", selected_item);
                        
                                    let updated_items: Vec<&str> = items
                                        .iter()
                                        .enumerate()
                                        .filter_map(|(index, &todo)| if index != selection { Some(todo) } else { None })
                                        .collect();
                        
                                    let updated_content = updated_items.join("\n");
                        
                                    fs::write("list.json", &updated_content).unwrap();
                                } else {
                                    println!("Invalid selection");
                                }
                            }
                           
                        }
                        Err(err) => {
                            eprintln!("Error parsing the data {} ", err);
                        }

                    }

                }
                Err(_err) => {
                    println!("Error reading the file");
                }
            }


            Ok(())
        }
        Ok(false) => {
            println!("Cant find the file try creating a todo first!");
            Ok(())
        }
        Err(err) => {
         
            println!("Error checking file existence {:?}", err);
            Err(err)
          
        }
    }

 
}



fn create_todos() -> Result<(), io::Error> {

    let green = Style::new().green();

    print!("{}", green.apply_to("Creating todo : "));
    std::io::stdout().flush().unwrap();

    let list_path_exist: Result<bool, io::Error> = Path::new("list.json").try_exists();

    match list_path_exist {

        Ok(true) => {
            let mut todos: Vec<Todo> = Vec::new(); 

            let list = fs::read_to_string("list.json");

            match list {
                Ok(data) =>{
                    todos.push(Todo { content:data });
                }
                Err(_err) => {
                    println!("There are no todos in the list. please create one");
                }
            }


            let mut input = String::new();  
            
            std::io::stdin().read_line(&mut input)?; 
            
            let new_todo = Todo::new(input.trim());  
            todos.push(new_todo);
            let combined_content: String = todos.iter().map(|todo| &todo.content as &str).collect::<Vec<_>>().join("\n");


            fs::write("list.json", combined_content.trim()).unwrap(); 

            Ok(())
            
        }
        Ok(false) => {
            println!("Creating todos list.json...");
            let _file = File::create("list.json")?;
            create_todos()
        }
        Err(err) => {
         
            println!("Error checking file existence {:?}", err);
            Err(err)
          
        }
    }
    


}


fn app() {

    let actions = vec!["Get Todos", "Create Todos", "Delete Todos", "Quit"];
   
    let selection = Select::new()
    .with_prompt("Please select an action")
    .items(&actions)
    .interact()
    .unwrap();



    if actions[selection] == "Get Todos"{
        get_todos();
    } else if actions[selection] == "Create Todos" {
        create_todos();
    } else if actions[selection] == "Delete Todos" {
        delete_todos();
    } else {
        println!("quitting...");
         process::exit(0)
    }
}


fn main() {
    println!("Welcome to Rust Todo Console...");
   loop {
    app();
   }
}
