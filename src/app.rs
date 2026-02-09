use anyhow::Result;
use hlavi_core::storage::file_storage::FileStorage;
use hlavi_core::{Board, Storage, Ticket, TicketStatus};
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

    pub fn status(&self) -> TicketStatus {
        match self {
            Self::Open => TicketStatus::Open,
            Self::InProgress => TicketStatus::InProgress,
            Self::Review => TicketStatus::Review,
            Self::Done => TicketStatus::Done,
        }
    }
}

pub struct App {
    storage: FileStorage,
    board: Board,
    tickets: Vec<Ticket>,
    focused_column: FocusedColumn,
    selected_index: usize,
}

impl App {
    pub async fn new() -> Result<Self> {
        let current_dir = env::current_dir()?;
        let storage = FileStorage::new(&current_dir);
        let board = storage.load_board().await?;
        let mut tickets = Vec::new();

        for ticket_id in board.tickets.values() {
            if let Ok(ticket) = storage.load_ticket(ticket_id).await {
                tickets.push(ticket);
            }
        }

        Ok(Self {
            storage,
            board,
            tickets,
            focused_column: FocusedColumn::Open,
            selected_index: 0,
        })
    }

    pub async fn reload(&mut self) -> Result<()> {
        self.board = self.storage.load_board().await?;
        self.tickets.clear();

        for ticket_id in self.board.tickets.values() {
            if let Ok(ticket) = self.storage.load_ticket(ticket_id).await {
                self.tickets.push(ticket);
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
        let tickets_in_column = self.tickets_in_focused_column().len();
        if tickets_in_column > 0 {
            self.selected_index = (self.selected_index + 1).min(tickets_in_column - 1);
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

    pub fn tickets_in_column(&self, status: TicketStatus) -> Vec<&Ticket> {
        self.tickets.iter().filter(|t| t.status == status).collect()
    }

    fn tickets_in_focused_column(&self) -> Vec<&Ticket> {
        self.tickets_in_column(self.focused_column.status())
    }
}
