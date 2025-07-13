//fork isnt part of rust std library. i am stopping with this example showing how to make multiple processes

use std::process::{Command, id};
use std::env;

fn main() {
    let is_child = env::args().any(|arg| arg=="--child");

    println!("Hello world!, process_id(pid) = {}", id());

    if !is_child {
        let _ = Command::new(env::current_exe().unwrap())
    .arg("--child")
    .spawn()
    .and_then(|mut child| child.wait());
    }
}

/*
    Visual representation of process spawning in Rust (fork-like):

         +----------------------+
         |   Initial Process    |
         |  (main binary runs)  |
         +----------+-----------+
                    |
                    | Command::new(current_exe()).spawn()
                    v
         +----------------------+
         |   Child Process      |
         | (same binary runs)   |
         +----------+-----------+
                    |
        prints "Hello world!" with its own PID

    If the child spawns again without a condition, you get:

         Parent
            |
          spawn()
            |
         Child 1
            |
          spawn()
            |
         Child 2
            |
            ...

    Without a stop condition (like "--child" flag or depth), this leads to infinite spawning.

    Fix: Pass a "--child" flag or use an environment variable to stop recursion.

    Example Output:
    --------------------------
    Hello world!, process_id(pid) = 12345 (parent)
    Hello world!, process_id(pid) = 12346 (child)
*/
