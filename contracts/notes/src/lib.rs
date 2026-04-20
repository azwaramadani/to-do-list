#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data Task
#[contracttype]
#[derive(Clone, Debug)]
pub struct Task {
    id: u64,
    title: String,
    completed: bool,
}

// Storage key
const TASK_DATA: Symbol = symbol_short!("TASK_DATA");

#[contract]
pub struct TodoContract;

#[contractimpl]
impl TodoContract {

    // Ambil semua task
    pub fn get_tasks(env: Env) -> Vec<Task> {
        env.storage().instance().get(&TASK_DATA).unwrap_or(Vec::new(&env))
    }

    // Tambah task baru
    pub fn create_task(env: Env, title: String) -> String {
        let mut tasks: Vec<Task> = env.storage().instance().get(&TASK_DATA).unwrap_or(Vec::new(&env));
        
        let task = Task {
            id: env.prng().gen::<u64>(),
            title,
            completed: false,
        };

        tasks.push_back(task);
        env.storage().instance().set(&TASK_DATA, &tasks);

        String::from_str(&env, "Task berhasil ditambahkan")
    }

    // Tandai task selesai
    pub fn complete_task(env: Env, id: u64) -> String {
        let mut tasks: Vec<Task> = env.storage().instance().get(&TASK_DATA).unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {
            let mut task = tasks.get(i).unwrap();

            if task.id == id {
                task.completed = true;
                tasks.set(i, task);

                env.storage().instance().set(&TASK_DATA, &tasks);
                return String::from_str(&env, "Task selesai!");
            }
        }

        String::from_str(&env, "Task tidak ditemukan")
    }

    // Hapus task
    pub fn delete_task(env: Env, id: u64) -> String {
        let mut tasks: Vec<Task> = env.storage().instance().get(&TASK_DATA).unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {
            if tasks.get(i).unwrap().id == id {
                tasks.remove(i);
                env.storage().instance().set(&TASK_DATA, &tasks);
                return String::from_str(&env, "Task berhasil dihapus");
            }
        }

        String::from_str(&env, "Task tidak ditemukan")
    }
}