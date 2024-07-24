# acts
Small terminal menu to run commands.

## Demo

Given a configuration file like this:

```toml
# example_config.toml
[todaysdate]
name = "Today's date"
description = ""
command = "echo $(date)"

[listdir]
name = "List directory"
description = ""
command = "ls -lah"
```

The program can be invoked like this:

```bash
$ eval "$(acts example_config.toml)"
```

And then first you will see this menu displayed in your terminal:

```
  acts                                  
  use wasd, jk, or arrow keys           
  enter to select, q or esc to exit     
  -----------------------               
> listdir                               
  todaysdate       
```

And when you select an option, the command will be executed 
and then executed by your shell.

## Design Considerations

These are the design considerations:

- Commands should be executed by YOUR shell, not inside `acts`. 
This decreases surface area for me to worry about, and ensures the
best cross-platform compatibility.

- Few dependencies, the less the better. I use `std` to parse args
for example, instead of bigger libraries like `clap`. I would have
loved to use a config file format supported by `std` but there are
none, so I use `toml` which is a small dependency.

## Future plans

I would like to add `justfile` as a supported config file format.
In this case, I would just populate the menu with commands from
the `justfile` and execute them with `just`. A problem to be solved
is what to do with just commands that take parameters.

## Dev workflow

While working on the Rust code, it can be invoked like this:

```bash
$ eval "$(cargo run -- example_config.toml)"
```
