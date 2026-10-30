use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt;
use std::io::{self, BufRead, Write};
use std::marker::PhantomData;
use std::sync::Arc;
use std::thread;

// --------------------------------------------------
// MACRO: reusable console messages
// --------------------------------------------------

macro_rules! message {
    ($($arg:tt)*) => {
        println!("[Task Manager] {}", format_args!($($arg)*))
    };
}

// --------------------------------------------------
// ERROR HANDLING
// --------------------------------------------------

#[derive(Debug)]
enum AppError {
    Io(io::Error),
    InvalidInput(String),
    ThreadPanic,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(error) => write!(f, "Input/output error: {error}"),
            AppError::InvalidInput(error) => write!(f, "{error}"),
            AppError::ThreadPanic => write!(f, "Report thread failed"),
        }
    }
}

impl Error for AppError {}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::Io(error)
    }
}

// --------------------------------------------------
// TRYFROM / TRYINTO: validated task IDs and priorities
// --------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TaskId(usize);

impl TryFrom<&str> for TaskId {
    type Error = AppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let id = value.trim().parse::<usize>()
            .map_err(|_| AppError::InvalidInput(
                "Task ID must be a positive number.".into()
            ))?;

        if id == 0 {
            return Err(AppError::InvalidInput(
                "Task ID must be greater than zero.".into()
            ));
        }

        Ok(TaskId(id))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Priority {
    Low,
    Medium,
    High,
}

impl TryFrom<&str> for Priority {
    type Error = AppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "1" | "low" => Ok(Priority::Low),
            "2" | "medium" => Ok(Priority::Medium),
            "3" | "high" => Ok(Priority::High),
            _ => Err(AppError::InvalidInput(
                "Priority must be 1 (Low), 2 (Medium), or 3 (High).".into()
            )),
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
        };

        write!(f, "{text}")
    }
}

// --------------------------------------------------
// PHANTOM TYPE PARAMETERS: task states
// --------------------------------------------------

struct Pending;
struct Completed;

struct Task<State> {
    id: TaskId,
    description: String,
    priority: Priority,
    state: PhantomData<State>,
}

impl Task<Pending> {
    fn new(id: TaskId, description: String, priority: Priority) -> Self {
        Self {
            id,
            description,
            priority,
            state: PhantomData,
        }
    }

    fn complete(self) -> Task<Completed> {
        Task {
            id: self.id,
            description: self.description,
            priority: self.priority,
            state: PhantomData,
        }
    }
}

impl<State> Task<State> {
    fn display(&self, status: &str) {
        println!(
            "ID: {} | {} | Priority: {} | {}",
            self.id.0, self.description, self.priority, status
        );
    }
}

// --------------------------------------------------
// INPUT HELPERS
// --------------------------------------------------

fn read_input() -> Result<String, AppError> {
    let mut input = String::new();
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

// --------------------------------------------------
// MAIN APPLICATION
// --------------------------------------------------

fn main() -> Result<(), AppError> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut pending_tasks: Vec<Task<Pending>> = Vec::new();
    let mut completed_tasks: Vec<Task<Completed>> = Vec::new();
    let mut next_id: usize = 1;

    loop {
        println!("\n========== TASK MANAGER ==========");
        println!("1. Add task");
        println!("2. List all tasks");
        println!("3. Complete a task");
        println!("4. Delete a task");
        println!("5. Show pending tasks");
        println!("6. Generate report using Arc");
        println!("7. Exit");
        println!("==================================");
        print!("Enter choice: ");
        io::stdout().flush()?;

        // while let: read menu input until EOF
        let input = match lines.next() {
            Some(line) => line?,
            None => {
                message!("End of input. Goodbye!");
                break;
            }
        };

        let choice = match input.trim().parse::<u32>() {
            Ok(number) => number,
            Err(_) => {
                message!("Please enter a valid menu number.");
                continue;
            }
        };

        match choice {
            // ------------------------------------------
            // ADD TASK
            // ------------------------------------------
            1 => {
                print!("Enter task description: ");
                io::stdout().flush()?;

                let description = match lines.next() {
                    Some(line) => line?,
                    None => {
                        message!("No more input available.");
                        break;
                    }
                };

                let description = description.trim().to_string();

                if description.is_empty() {
                    message!("Description cannot be empty.");
                    continue;
                }

                print!("Priority (1=Low, 2=Medium, 3=High): ");
                io::stdout().flush()?;

                let priority_input = match lines.next() {
                    Some(line) => line?,
                    None => {
                        message!("No more input available.");
                        break;
                    }
                };

                // TryInto is available because TryFrom is implemented.
                let priority: Result<Priority, AppError> =
                    priority_input.as_str().try_into();

                let priority = match priority {
                    Ok(value) => value,
                    Err(error) => {
                        message!("{error}");
                        continue;
                    }
                };

                let task = Task::<Pending>::new(
                    TaskId(next_id),
                    description,
                    priority,
                );

                pending_tasks.push(task);
                message!("Task added with ID {next_id}.");
                next_id += 1;
            }

            // ------------------------------------------
            // LIST ALL TASKS
            // ------------------------------------------
            2 => {
                println!("\n--- Pending tasks ---");

                if pending_tasks.is_empty() {
                    println!("No pending tasks.");
                }

                for task in &pending_tasks {
                    task.display("Pending");
                }

                println!("\n--- Completed tasks ---");

                if completed_tasks.is_empty() {
                    println!("No completed tasks.");
                }

                for task in &completed_tasks {
                    task.display("Completed");
                }
            }

            // ------------------------------------------
            // COMPLETE TASK
            // ------------------------------------------
            3 => {
                print!("Enter task ID to complete: ");
                io::stdout().flush()?;

                let input = match lines.next() {
                    Some(line) => line?,
                    None => break,
                };

                // Demonstrate TryFrom
                let id = match TaskId::try_from(input.as_str()) {
                    Ok(id) => id,
                    Err(error) => {
                        message!("{error}");
                        continue;
                    }
                };

                if let Some(index) =
                    pending_tasks.iter().position(|task| task.id == id)
                {
                    let task = pending_tasks.remove(index);
                    completed_tasks.push(task.complete());
                    message!("Task {} completed.", id.0);
                } else if completed_tasks.iter().any(|task| task.id == id) {
                    message!("That task is already completed.");
                } else {
                    message!("Task {} was not found.", id.0);
                }
            }

            // ------------------------------------------
            // DELETE TASK
            // ------------------------------------------
            4 => {
                print!("Enter task ID to delete: ");
                io::stdout().flush()?;

                let input = match lines.next() {
                    Some(line) => line?,
                    None => break,
                };

                let id = match TaskId::try_from(input.as_str()) {
                    Ok(id) => id,
                    Err(error) => {
                        message!("{error}");
                        continue;
                    }
                };

                let old_count =
                    pending_tasks.len() + completed_tasks.len();

                // Closure + higher-order function
                pending_tasks.retain(|task| task.id != id);
                completed_tasks.retain(|task| task.id != id);

                let new_count =
                    pending_tasks.len() + completed_tasks.len();

                if new_count < old_count {
                    message!("Task {} deleted.", id.0);
                } else {
                    message!("Task {} was not found.", id.0);
                }
            }

            // ------------------------------------------
            // SHOW PENDING TASKS
            // ------------------------------------------
            5 => {
                println!("\n--- Pending tasks ---");

                // Closure + filter + higher-order functions
                let filtered = pending_tasks
                    .iter()
                    .filter(|task| task.priority != Priority::Low)
                    .map(|task| {
                        format!(
                            "ID: {} | {} | {}",
                            task.id.0, task.description, task.priority
                        )
                    });

                let results: Vec<String> = filtered.collect();

                if results.is_empty() {
                    println!("No pending medium/high priority tasks.");
                } else {
                    for task in results {
                        println!("{task}");
                    }
                }
            }

            // ------------------------------------------
            // ARC + THREAD REPORT
            // ------------------------------------------
            6 => {
                let mut report: Vec<String> = pending_tasks
                    .iter()
                    .map(|task| {
                        format!(
                            "PENDING: {} - {}",
                            task.id.0, task.description
                        )
                    })
                    .collect();

                report.extend(
                    completed_tasks.iter().map(|task| {
                        format!(
                            "COMPLETED: {} - {}",
                            task.id.0, task.description
                        )
                    })
                );

                // Arc allows multiple owners of shared data.
                let shared_report = Arc::new(report);
                let worker_report = Arc::clone(&shared_report);

                let handle = thread::spawn(move || {
                    println!("\n--- Background task report ---");

                    if worker_report.is_empty() {
                        println!("There are no tasks to report.");
                    }

                    for entry in worker_report.iter() {
                        println!("{entry}");
                    }

                    println!("Total tasks: {}", worker_report.len());
                });

                handle.join().map_err(|_| AppError::ThreadPanic)?;

                message!("Report generated.");
            }

            // ------------------------------------------
            // EXIT
            // ------------------------------------------
            7 => {
                message!("Goodbye!");
                break;
            }

            _ => message!("Choose a number from 1 to 7."),
        }
    }

    Ok(())
}
