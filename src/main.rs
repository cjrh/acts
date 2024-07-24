///
/// A simple menu with three options to choose from.
///
use std::collections::{HashMap};

#[derive(Debug, Clone)]
struct Action {
    name: String,
    description: String,
    command: String,
}

fn main() {
    let actions_filename = match std::env::args().nth(1) {
        Some(value) => match value.as_str() {
            "-h" | "--help" => {
                eprintln!(
                    "Usage: {} <actions_filename>",
                    std::env::args().next().unwrap()
                );
                std::process::exit(0);
            }
            _ => value,
        },
        None => {
            eprintln!(
                "Usage: {} <actions_filename>",
                std::env::args().next().unwrap()
            );
            std::process::exit(1);
        }
    };

    // If the filename has the extension ".toml", load it using the toml crate.
    // The toml file will look something like this below. We want to make a vec
    // of `Action` structs from this file.
    /*

       [[todaysdate]]
       name = "Today's date"
       description = ""
       command = "echo ${date}"

       [[listdir]]
       name = "List directory"
       description = ""
       command = "ls -lah"
    */

    let actions: std::collections::HashMap<String, Action> = if actions_filename.ends_with(".toml") {
        let actions = std::fs::read_to_string(actions_filename).unwrap();
        let actions: toml::Value = toml::from_str(&actions).unwrap();
        let actions = actions.as_table().unwrap();
        let mut acts = HashMap::new();
        for (key, value) in actions {
            let action = Action {
                name: value.get("name").unwrap().as_str().unwrap().to_string(),
                description: value
                    .get("description")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
                command: value.get("command").unwrap().as_str().unwrap().to_string(),
            };
            acts.insert(key.to_string(), action);
        }
        acts
    } else {
        eprintln!("The file must be a TOML file.");
        std::process::exit(1);
    };

    use terminal_menu::{button, label, menu, mut_menu, run};
    let mut default_items = vec![
        // label:
        //  not selectable, useful as a title, separator, etc...
        label("acts"),
        label("use wasd, jk, or arrow keys"),
        label("enter to select, q or esc to exit"),
        label("-----------------------"),
    ];
    // Show a button for each action in the actions map
    for (key, _) in actions.iter() {
        default_items.push(
        button(key)
        );
    }

    let menu = menu(default_items);
    run(&menu);
    // you can get the selected buttons name like so:
    // println!("Selected: {}", mut_menu(&menu).selected_item_name());
    let mm = mut_menu(&menu);
    let selected_action_key = mm.selected_item_name();
    let selected_action = actions.get(selected_action_key).unwrap();

    // This will correctly stream the stdout of the command to the stdout of the
    // parent process.
    // use std::io::{self, Write};
    // use std::process::{Command, Stdio};
    // // TODO: Do we really want to do this? I think it's better we return the command
    // //  to run.
    // let mut child = Command::new("sh")
    //     .arg("-c")
    //     .arg(&selected_action.command)
    //     .stdout(Stdio::piped())
    //     .spawn()
    //     .expect("Failed to execute command");

    // if let Some(ref mut stdout) = child.stdout {
    //     io::copy(stdout, &mut io::stdout()).unwrap();
    // }
    // child.wait().unwrap();

    // My preference would be to "get out of the way" and let the user's shell
    // take over. However I can't seem to get that to work. The display of the
    // menu itself seems to get messed up with `sh -c $(acts ...)`, or even
    // with `$ acts ... | bash`. So for now, I will have to stream the output
    // as above.
    println!("{}", String::from_utf8_lossy(selected_action.command.as_bytes()));

}
