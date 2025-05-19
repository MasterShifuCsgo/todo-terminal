use crate::ask_option;
use crate::hold;
use crate::task;



use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

pub struct Page {
    tasks: Vec<task::Task>,
}

impl Default for Page {
    fn default() -> Self {
        Page { tasks: Vec::new() }
    }
}

impl Page {
    //renders the Menu screen
    pub fn render(&self) {
        let menu: &str = "=== File operations ===
0. Save to file
1. Load from file

=== To-do list commands ===

2. Create new Task
3. Edit existing Task
4. View existing Task\n\n";

        print!("{}", menu);
    }

    pub fn get_immutable_commands(&self) -> Vec<fn(&Page)> {
        vec![Self::save_file, Self::view_tasks]
    }

    pub fn get_mutable_commands(&mut self) -> Vec<fn(&mut Page)> {
        vec![Self::load_file, Self::create_task, Self::edit_task]
    }

    fn save_file(&self) {
        let file_name: &str = "tasks.json";

        let json: String = serde_json::to_string_pretty(&self.tasks).expect("Serade failed");

        let mut file = File::create(file_name).expect("failed to create json file");
        file.write_all(json.as_bytes())
            .expect("Failed writing to json file");

        println!("Created {}", file_name);
    }

    fn load_file(&mut self) {
        println!("What is the filename of the file which to be loaded?");

        let mut file_name = String::new();
        io::stdin()
            .read_line(&mut file_name)
            .expect("File name failed to be parsed.");

        let mut file: File = File::open(file_name).expect("File failed to open.");

        let mut json: String = String::new();
        file.read_to_string(&mut json)
            .expect("File failed to be parsed to a String");

        let json = json.trim();
        //serde_json::from
        let vec = serde_json::to_vec(&json).expect("Failed to convert file data into a vector.");

        print!("{:?}", vec);
    }

    fn view_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            println!(
                "{} === {} ===\n{}\n-------\n{}\n",
                index, task.title, task.desc, task.is_done
            );
        }

        hold();
    }

    fn create_task(&mut self) {
        self.tasks.push(task::Task::default());
    }

    fn edit_task(&mut self) {
        if self.tasks.is_empty() {
            println!("No tasks to display");
        } else {
            println!("\n\n\n===== Tasks =====\n");
            self.view_tasks();
            println!("Select a Task index to edit.");
            let task_index: usize = ask_option() as usize;
            loop {
                //displays selected task
                println!(
                    "\n{} === {} ===\n{}\n-------\n{}",
                    task_index,
                    self.tasks[task_index].title,
                    self.tasks[task_index].desc,
                    self.tasks[task_index].is_done
                );
                println!("Enter 255 to exit the loop");
                println!("0. Change title\n1. Change description\n2. Set 'is done'");
                let choice: usize = ask_option() as usize;
                match choice {
                    255 => {
                        break;
                    }
                    0 => {
                        print!(
                            "current title: {}\nNew title: ",
                            self.tasks[task_index].title
                        );
                        io::stdout().flush().expect("Failed to flush stdout.");

                        let mut new_title: String = String::new();

                        io::stdin()
                            .read_line(&mut new_title)
                            .expect("Reading stdin for title failed.");
                        self.tasks[task_index].title = new_title.trim().to_string();
                    }
                    1 => {
                        print!(
                            "current description: {}\nNew description: ",
                            self.tasks[task_index].desc
                        );
                        let mut new_desc: String = String::new();
                        io::stdin()
                            .read_line(&mut new_desc)
                            .expect("Reading stdin for description failed.");
                        self.tasks[task_index].desc = new_desc.trim().to_string();
                    }
                    2 => {
                        self.tasks[task_index].is_done = !self.tasks[task_index].is_done;
                        print!("Task is now {}", self.tasks[task_index].is_done);
                    }
                    _ => print!("User choise does not match given selection."),
                }
            }
        }
    }
}
