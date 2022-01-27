use rusqlite::{params, Connection, Result, Row};

use super::util;

#[derive(Debug)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub done: bool,
}

impl Task {
    pub fn new(name: &str) -> Task {
        Task {
            id: util::random_id(),
            name: name.to_owned(),
            done: false,
        }
    }
}

//

#[allow(dead_code)]
pub fn get_by_id(id: &str, conn: &Connection) -> Result<Option<Task>> {
    let mut stmt = conn.prepare("SELECT id, name, done FROM task WHERE task.id = :id;")?;
    let mut tasks = stmt
        .query_map(&[(":id", id)], row_to_task)?
        .collect::<Result<Vec<Task>>>()?;
    Ok(tasks.pop())
}

pub fn get_all(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare("SELECT id, name, done FROM task;")?;

    let tasks: Result<Vec<Task>> = stmt.query_map([], row_to_task)?.collect();
    return tasks;
}

pub fn complete(id: &str, conn: &Connection) -> Result<()> {
    conn.execute(
        "
        UPDATE task 
        SET done = TRUE
        WHERE id = ?
        ;
        ",
        [id],
    )?;
    Ok(())
}

pub fn undo(id: &str, conn: &Connection) -> Result<()> {
    conn.execute(
        "
        UPDATE task 
        SET done = FALSE
        WHERE id = ?
        ;
        ",
        [id],
    )?;
    Ok(())
}

pub fn save(task: Task, conn: &Connection) -> Result<String> {
    // upsert
    conn.execute(
        "
        INSERT INTO task (id, name, done)
            VALUES (?1, ?2, ?3)
        ON CONFLICT (id) DO 
            UPDATE SET
                name = ?2,
                done = ?3
            WHERE task.id = ?1
        ;
        ",
        params![task.id, task.name, task.done],
    )?;
    Ok(task.id)
}

pub fn delete(id: &str, conn: &Connection) -> Result<()> {
    conn.execute(
        "
        DELETE FROM task
        WHERE task.id = ?
        ;
        ",
        [id],
    )?;
    Ok(())
}

fn row_to_task(row: &Row) -> Result<Task> {
    let id = row.get(0)?;
    let name = row.get(1)?;
    let done = row.get::<usize, bool>(2)?;

    Ok(Task { id, name, done })
}

//

pub fn create_table_if_not_exists(conn: &Connection) -> Result<()> {
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS task (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            done INTEGER DEFAULT FALSE CHECK (done == TRUE or done == FALSE)
        );",
        [],
    )?;
    Ok(())
}

//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_task() {
        let conn = Connection::open_in_memory().expect("Failed to open connection in memory");
        create_table_if_not_exists(&conn).expect("Task table could not be created");
        let task = Task::new("Task #1");
        assert!(save(task, &conn).is_ok());
    }

    #[test]
    fn delete_task() {
        // pre-test
        let conn = Connection::open_in_memory().expect("Failed to open connection in memory");
        create_table_if_not_exists(&conn).expect("Task table could not be created");
        let task = Task::new("Task #1");
        let save_res = save(task, &conn);
        assert!(save_res.is_ok());
        // test
        let id = save_res.unwrap();
        let del_res = delete(&id, &conn);
        assert!(del_res.is_ok());
    }

    #[test]
    fn complete_task() {
        // pre-test
        let conn = Connection::open_in_memory().expect("Failed to open connection in memory");
        create_table_if_not_exists(&conn).expect("Task table could not be created");
        let task = Task::new("Task #1");
        let save_res = save(task, &conn);
        assert!(save_res.is_ok());
        // test
        let id = save_res.unwrap();
        let complete_res = complete(&id, &conn);
        assert!(complete_res.is_ok());
        assert_eq!(get_by_id(&id, &conn).unwrap().unwrap().done, true);
    }

    #[test]
    fn undo_task() {
        // pre-test
        let conn = Connection::open_in_memory().expect("Failed to open connection in memory");
        create_table_if_not_exists(&conn).expect("Task table could not be created");
        let task = Task {
            id: util::random_id(),
            name: "Task #1".to_owned(),
            done: true,
        };
        let save_res = save(task, &conn);
        assert!(save_res.is_ok());
        // test
        let id = save_res.unwrap();
        let undo_res = undo(&id, &conn);
        assert!(undo_res.is_ok());
        assert_eq!(get_by_id(&id, &conn).unwrap().unwrap().done, false);
    }
}
