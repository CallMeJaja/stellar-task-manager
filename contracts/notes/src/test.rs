#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Ledger, Env, String};

#[test]
fn test_create_task() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TaskManagerContract);
    let client = TaskManagerContractClient::new(&env, &contract_id);

    let title = String::from_str(&env, "Complete Stellar Workshop");
    let description = String::from_str(&env, "Finish all tasks and submit project");
    let priority = Priority::High;
    let deadline = 1234567890u64;

    let result = client.create_task(&title, &description, &priority, &deadline);
    assert_eq!(result, String::from_str(&env, "Task created successfully"));

    let tasks = client.get_all_tasks();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks.get(0).unwrap().title, title);
    assert_eq!(tasks.get(0).unwrap().priority, Priority::High);
    assert_eq!(tasks.get(0).unwrap().status, Status::Pending);
}

#[test]
fn test_get_tasks_by_status() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TaskManagerContract);
    let client = TaskManagerContractClient::new(&env, &contract_id);

    // Create multiple tasks
    client.create_task(
        &String::from_str(&env, "Task 1"),
        &String::from_str(&env, "Description 1"),
        &Priority::High,
        &1234567890u64,
    );

    env.ledger().set_sequence_number(100);
    
    client.create_task(
        &String::from_str(&env, "Task 2"),
        &String::from_str(&env, "Description 2"),
        &Priority::Medium,
        &1234567891u64,
    );

    // Get first task and update its status
    let tasks = client.get_all_tasks();
    let first_task_id = tasks.get(0).unwrap().id;
    client.update_task_status(&first_task_id, &Status::Completed);

    // Filter by status
    let pending_tasks = client.get_tasks_by_status(&Status::Pending);
    let completed_tasks = client.get_tasks_by_status(&Status::Completed);

    assert_eq!(pending_tasks.len(), 1);
    assert_eq!(completed_tasks.len(), 1);
}

#[test]
fn test_update_task_status() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TaskManagerContract);
    let client = TaskManagerContractClient::new(&env, &contract_id);

    client.create_task(
        &String::from_str(&env, "Test Task"),
        &String::from_str(&env, "Test Description"),
        &Priority::Medium,
        &1234567890u64,
    );

    let tasks = client.get_all_tasks();
    let task_id = tasks.get(0).unwrap().id;

    // Update to InProgress
    let result = client.update_task_status(&task_id, &Status::InProgress);
    assert_eq!(result, String::from_str(&env, "Task status updated successfully"));

    let updated_tasks = client.get_all_tasks();
    assert_eq!(updated_tasks.get(0).unwrap().status, Status::InProgress);
}

#[test]
fn test_delete_task() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TaskManagerContract);
    let client = TaskManagerContractClient::new(&env, &contract_id);

    client.create_task(
        &String::from_str(&env, "Task to Delete"),
        &String::from_str(&env, "This will be deleted"),
        &Priority::Low,
        &1234567890u64,
    );

    let tasks = client.get_all_tasks();
    assert_eq!(tasks.len(), 1);

    let task_id = tasks.get(0).unwrap().id;
    let result = client.delete_task(&task_id);
    assert_eq!(result, String::from_str(&env, "Task deleted successfully"));

    let remaining_tasks = client.get_all_tasks();
    assert_eq!(remaining_tasks.len(), 0);
}

#[test]
fn test_delete_nonexistent_task() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TaskManagerContract);
    let client = TaskManagerContractClient::new(&env, &contract_id);

    let result = client.delete_task(&999999u64);
    assert_eq!(result, String::from_str(&env, "Task not found"));
}
