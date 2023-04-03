use crossterm::{cursor, terminal, QueueableCommand};
use rand::Rng;
use std::collections::VecDeque;
use std::io::{stdout, Stdout, Write};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::{spawn_blocking, JoinHandle};

#[tokio::main]
async fn main() {
    let mut stdout_org = stdout();
    stdout_org
        .queue(terminal::Clear(terminal::ClearType::All))
        .unwrap();

    let data = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];
    let set = VecDeque::from_iter(data.iter().cloned());
    let set: Arc<RwLock<VecDeque<i32>>> = Arc::new(RwLock::new(set));
    let stdout_rw_lock: Arc<RwLock<Stdout>> = Arc::new(RwLock::new(stdout_org));

    stdout_rw_lock.write().await.queue(cursor::Hide).unwrap();

    let mut handlers = vec![];
    let cloned_stdout_rw_lock = stdout_rw_lock.clone();
    clear_console(cloned_stdout_rw_lock).await;
    for i in 1..(num_cpus::get() + 1) {
        let cloned_set = set.clone();
        let cloned_stdout_rw_lock = stdout_rw_lock.clone();
        handlers.push(work_task(
            i.try_into().unwrap(),
            cloned_set,
            cloned_stdout_rw_lock,
        ));
    }

    for handler in handlers {
        handler.await.unwrap();
    }
    stdout_rw_lock.write().await.queue(cursor::Show).unwrap();
}

pub fn work_task(
    line: usize,
    set: Arc<RwLock<VecDeque<i32>>>,
    stdout_rw_lock: Arc<RwLock<Stdout>>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let line = line as u16;
        while set.clone().read().await.len() > 0 {
            let item_to_process = read_data(set.clone()).await;
            if item_to_process.is_none() {
                return;
            }
            let item_to_process = item_to_process.unwrap();

            print_message(
                stdout_rw_lock.clone(),
                line,
                format!("Start processing {}....", item_to_process).as_str(),
            )
            .await;

            worker(item_to_process, line, stdout_rw_lock.clone()).await;

            print_message(
                stdout_rw_lock.clone(),
                line,
                format!("Processing complete of item {}", item_to_process).as_str(),
            )
            .await;
        }
    })
}

async fn clear_console(stdout_rw_lock: Arc<RwLock<Stdout>>) -> () {
    let mut stdout = stdout_rw_lock.write().await;
    stdout
        .queue(terminal::Clear(terminal::ClearType::All))
        .unwrap();
}

async fn print_message(stdout_rw_lock: Arc<RwLock<Stdout>>, line: u16, message: &str) -> () {
    let mut stdout = stdout_rw_lock.write().await;
    stdout.queue(cursor::MoveTo(0, line)).unwrap();
    stdout
        .queue(terminal::Clear(terminal::ClearType::CurrentLine))
        .unwrap();
    stdout
        .write_all(format!("task {} - {} ", line, message).as_bytes())
        .unwrap();
}

pub async fn read_data(set: Arc<RwLock<VecDeque<i32>>>) -> Option<i32> {
    let mut resource1 = set.write().await;
    return resource1.pop_front();
}

async fn worker(item_to_process: i32, line: u16, stdout_rw_lock: Arc<RwLock<Stdout>>) {
    let random = spawn_blocking(move || {
        let mut rng = rand::thread_rng();
        let gen: u64 = rng.gen_range(0..1000000);
        return gen;
    })
    .await
    .unwrap();
    let max_val: u64 = 10000 + random;
    let mut _x: u64 = 0;
    loop {
        _x += 1;
        print_message(
            stdout_rw_lock.clone(),
            line,
            format!(
                "item {}: {}%",
                item_to_process,
                calculate_status(_x, max_val)
            )
            .as_str(),
        )
        .await;
        if _x >= max_val {
            break;
        }
    }
}

pub fn calculate_status(i: u64, max_val: u64) -> String {
    let calculate = i / (max_val / 100);
    return calculate.to_string();
}
