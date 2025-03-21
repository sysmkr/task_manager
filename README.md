## Task manager

<br>

This is a simple command-line to-do list application built using the Rust programming language.
The application allows you to manage a list of tasks with basic CRUD (Create, Read, Update, Delete)
operations. It supports adding, marking tasks as completed, removing tasks, and viewing the current
list of tasks. The to-do list is persisted in a JSON file so the data is retained between sessions.

<br>

---

<br>

### Features

- **Add tasks** : Add a new task to the to-do list.
- **List tasks** : View all tasks, with their completion status.
- **Checking tasks** : Matk tasks as done once they are finished.
- **Remove tasks** : Remove tasks from the list.
- **Persistence** : Tasks are saved to a JSON file and loaded on each run.

<br>

---

<br>

### Requirements

- **Rust** (1.60 or newer) installed on the system :

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  # Check if correctly installed : rustc --version
  ```

<br>

---

<br>

### Installation

1. Clone the repository
  ```bash
  git clone https://github.com/ovrcomr/tasky.git
  ```

2. Go to the directory
  ```bash
  cd task_manager
  ```

3. Launch the app
  ```bash
  cargo run
  ```

<br>

---

<br>

### Usage

<br>

After building and running the project, it is possible to interact with the application using the command line :

#### Commands

```bash
cargo run -- add <task_name>
cargo run -- remove <task_name>
cargo run -- check <task_name>
cargo run -- list
```

<br>

---

<br>

### File Structure

<br>

```txt
task_manager/
├── src/
│   ├── data.json          # Storage for interactions
│   ├── main.rs            # Main application logic
│   ├── tools.rs           # Functions used in 'main.rs'
│   └── task.rs            # Task struct and methods
├── Cargo.toml             # Cargo configuration file
```

<br>

---

<br>

### Dependencies

- **serde** : For serializing and deserializing task data into JSON format.
- **serde_json** : For reading and writing JSON files.
