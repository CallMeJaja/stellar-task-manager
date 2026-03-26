#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Priority level for tasks
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum Priority {
    High,
    Medium,
    Low,
}

// Task status
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Pending,
    InProgress,
    Completed,
}

// Task structure with enhanced fields
#[contracttype]
#[derive(Clone, Debug)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub status: Status,
    pub deadline: u64,
}

// Storage key for tasks data
const TASK_DATA: Symbol = symbol_short!("TASKS");

#[contract]
pub struct TaskManagerContract;

#[contractimpl]
impl TaskManagerContract {
    // Get all tasks
    pub fn get_all_tasks(env: Env) -> Vec<Task> {
        env.storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Get tasks by status (filter)
    pub fn get_tasks_by_status(env: Env, status: Status) -> Vec<Task> {
        let all_tasks: Vec<Task> = env.storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env));
        
        let mut filtered_tasks = Vec::new(&env);
        
        for i in 0..all_tasks.len() {
            let task = all_tasks.get(i).unwrap();
            if task.status == status {
                filtered_tasks.push_back(task);
            }
        }
        
        filtered_tasks
    }

    // Create a new task
    pub fn create_task(
        env: Env,
        title: String,
        description: String,
        priority: Priority,
        deadline: u64,
    ) -> String {
        let mut tasks: Vec<Task> = env.storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env));

        let new_task = Task {
            id: env.ledger().sequence() as u64,
            title,
            description,
            priority,
            status: Status::Pending,
            deadline,
        };

        tasks.push_back(new_task);
        env.storage().instance().set(&TASK_DATA, &tasks);

        String::from_str(&env, "Task created successfully")
    }

    // Update task status
    pub fn update_task_status(env: Env, task_id: u64, new_status: Status) -> String {
        let mut tasks: Vec<Task> = env.storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {
            let mut task = tasks.get(i).unwrap();
            if task.id == task_id {
                task.status = new_status;
                tasks.set(i, task);
                env.storage().instance().set(&TASK_DATA, &tasks);
                return String::from_str(&env, "Task status updated successfully");
            }
        }

        String::from_str(&env, "Task not found")
    }

    // Delete a task by ID
    pub fn delete_task(env: Env, task_id: u64) -> String {
        let mut tasks: Vec<Task> = env.storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {
            if tasks.get(i).unwrap().id == task_id {
                tasks.remove(i);
                env.storage().instance().set(&TASK_DATA, &tasks);
                return String::from_str(&env, "Task deleted successfully");
            }
        }

        String::from_str(&env, "Task not found")
    }
}

mod test;








/* --- CONTOH SCRIPT ---

pub fn get_notes(env: Env) -> Vec<Note> {
    // 1. ambil data notes dari storage
    return env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
}

// Fungsi untuk membuat note baru
pub fn create_note(env: Env, title: String, content: String) -> String {
    // 1. ambil data notes dari storage
    let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
    
    // 2. Buat object note baru
    let note = Note {
        id: env.prng().gen::<u64>(),
        title: title,
        content: content,
    };
    
    // 3. tambahkan note baru ke notes lama
    notes.push_back(note);
    
    // 4. simpan notes ke storage
    env.storage().instance().set(&NOTE_DATA, &notes);
    
    return String::from_str(&env, "Notes berhasil ditambahkan");
}

// Fungsi untuk menghapus notes berdasarkan id
pub fn delete_note(env: Env, id: u64) -> String {
    // 1. ambil data notes dari storage 
    let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));

    // 2. cari index note yang akan dihapus menggunakan perulangan
    for i in 0..notes.len() {
        if notes.get(i).unwrap().id == id {
            notes.remove(i);

            env.storage().instance().set(&NOTE_DATA, &notes);
            return String::from_str(&env, "Berhasil hapus notes");
        }
    }

    return String::from_str(&env, "Notes tidak ditemukan")
}


*/