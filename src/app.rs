use anyhow::Result;
use hlavi_core::storage::file_storage::FileStorage;
use hlavi_core::{Board, Storage, Task, TaskStatus};
use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusedColumn {
    Open,
    InProgress,
    Review,
    Done,
}

impl FocusedColumn {
    pub fn next(&self) -> Self {
        match self {
            Self::Open => Self::InProgress,
            Self::InProgress => Self::Review,
            Self::Review => Self::Done,
            Self::Done => Self::Done,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Self::Open => Self::Open,
            Self::InProgress => Self::Open,
            Self::Review => Self::InProgress,
            Self::Done => Self::Review,
        }
    }

    pub fn status(&self) -> TaskStatus {
        match self {
            Self::Open => TaskStatus::Open,
            Self::InProgress => TaskStatus::InProgress,
            Self::Review => TaskStatus::Review,
            Self::Done => TaskStatus::Done,
        }
    }
}

pub struct App {
    storage: FileStorage,
    board: Board,
    tasks: Vec<Task>,
    focused_column: FocusedColumn,
    selected_index: usize,
}

impl App {
    pub async fn new() -> Result<Self> {
        let current_dir = env::current_dir()?;
        let storage = FileStorage::new(&current_dir);
        let board = storage.load_board().await?;
        let mut tasks = Vec::new();

        for task_id in board.tasks.values() {
            if let Ok(task) = storage.load_task(task_id).await {
                tasks.push(task);
            }
        }

        Ok(Self {
            storage,
            board,
            tasks,
            focused_column: FocusedColumn::Open,
            selected_index: 0,
        })
    }

    pub async fn reload(&mut self) -> Result<()> {
        self.board = self.storage.load_board().await?;
        self.tasks.clear();

        for task_id in self.board.tasks.values() {
            if let Ok(task) = self.storage.load_task(task_id).await {
                self.tasks.push(task);
            }
        }

        Ok(())
    }

    pub fn move_left(&mut self) {
        self.focused_column = self.focused_column.prev();
        self.selected_index = 0;
    }

    pub fn move_right(&mut self) {
        self.focused_column = self.focused_column.next();
        self.selected_index = 0;
    }

    pub fn move_down(&mut self) {
        let tasks_in_column = self.tasks_in_focused_column().len();
        if tasks_in_column > 0 {
            self.selected_index = (self.selected_index + 1).min(tasks_in_column - 1);
        }
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn focused_column(&self) -> FocusedColumn {
        self.focused_column
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn tasks_in_column(&self, status: TaskStatus) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.status == status).collect()
    }

    fn tasks_in_focused_column(&self) -> Vec<&Task> {
        self.tasks_in_column(self.focused_column.status())
    }
}
