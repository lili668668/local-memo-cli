use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "memo")]
#[command(about = "簡單的備忘錄管理工具", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 添加一個新備忘
    Add {
        /// 備忘內容
        content: String,
    },
    /// 列出所有備忘
    List,
}

#[derive(Serialize, Deserialize, Debug)]
struct Memo {
    id: usize,
    content: String,
    created_at: String,
}

#[derive(Serialize, Deserialize, Default)]
struct MemoStore {
    memos: Vec<Memo>,
    next_id: usize,
}

fn get_memo_file_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("無法找到 home 目錄");
    home_dir.join(".memo_data.json")
}

fn load_memos() -> MemoStore {
    let path = get_memo_file_path();
    if path.exists() {
        let content = fs::read_to_string(&path).expect("無法讀取備忘錄檔案");
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        MemoStore::default()
    }
}

fn save_memos(store: &MemoStore) {
    let path = get_memo_file_path();
    let content = serde_json::to_string_pretty(store).expect("無法序列化備忘錄");
    fs::write(&path, content).expect("無法寫入備忘錄檔案");
}

fn add_memo(content: String) {
    let mut store = load_memos();
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let memo = Memo {
        id: store.next_id,
        content: content.clone(),
        created_at: now,
    };

    store.memos.push(memo);
    store.next_id += 1;
    save_memos(&store);

    println!("✓ 已添加備忘: {}", content);
}

fn list_memos() {
    let store = load_memos();

    if store.memos.is_empty() {
        println!("目前沒有任何備忘");
        return;
    }

    println!("備忘列表:");
    println!("{}", "=".repeat(60));
    for memo in &store.memos {
        println!("[{}] {}", memo.id, memo.content);
        println!("    建立時間: {}", memo.created_at);
        println!();
    }
    println!("總共 {} 個備忘", store.memos.len());
}

fn print_help() {
    println!("memo - 簡單的備忘錄管理工具");
    println!();
    println!("使用方式:");
    println!("  memo add <內容>    添加一個新備忘");
    println!("  memo list          列出所有備忘");
    println!("  memo help          顯示此幫助資訊");
    println!();
    println!("範例:");
    println!("  memo add \"明天要買牛奶\"");
    println!("  memo list");
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Add { content }) => add_memo(content),
        Some(Commands::List) => list_memos(),
        None => print_help(),
    }
}
