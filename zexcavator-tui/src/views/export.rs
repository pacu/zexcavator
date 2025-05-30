pub mod send;
pub mod zewif;
pub mod zingolib;

use std::sync::Arc;

use ::zingolib::lightclient::{LightClient, PoolBalances};
use tokio::sync::RwLock;
use tuirealm::ratatui::layout::{Constraint, Direction, Layout};
use tuirealm::ratatui::widgets::{Block, Borders, Paragraph};
use tuirealm::{Component, Frame, MockComponent, NoUserEvent, State};

use crate::Msg;
use crate::app::model::{HasScreenAndQuit, Screen};
use crate::components::HandleMessage;

use crate::components::menu::{Menu, MenuOptions};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportOptions {
    Zingolib,
    ZeWIF,
    Send,
    Cancel,
}

impl MenuOptions for ExportOptions {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![Self::Zingolib, Self::ZeWIF, Self::Send, Self::Cancel]
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Zingolib => "Zingolib",
            Self::ZeWIF => "ZeWIF (WARNING: experimental. Only exports mnemonic phrase)",
            Self::Send => "Send (Not implemented)",
            Self::Cancel => "Cancel",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExportView {
    pub light_client: Arc<RwLock<Option<LightClient>>>,
    pub balance: Arc<RwLock<Option<PoolBalances>>>,
    pub menu: Menu<ExportOptions>,
}

impl ExportView {
    pub fn new(light_client: Arc<RwLock<Option<LightClient>>>) -> Self {
        Self {
            light_client,
            balance: Arc::new(RwLock::new(None)),
            menu: Menu::new("Choose an export option"),
        }
    }

    pub async fn load_balance(&self) -> Option<PoolBalances> {
        let mut client_guard = self.light_client.write().await;
        let client = client_guard.as_mut()?;
        let b = client.do_balance().await;
        self.balance.try_write().unwrap().replace(b.clone());
        drop(client_guard);
        Some(b)
    }
}

impl MockComponent for ExportView {
    fn view(&mut self, frame: &mut Frame, area: tuirealm::ratatui::prelude::Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(area);

        // Show balance summary
        if self.balance.try_read().unwrap().is_some() {
            let balance;
            {
                let balance_guard = self.balance.try_read().unwrap();
                balance = balance_guard.clone().unwrap();
            }

            let total_balance = balance.confirmed_transparent_balance.unwrap_or(0)
                + balance.verified_sapling_balance.unwrap_or(0)
                + balance.verified_orchard_balance.unwrap_or(0);
            let text = format!("Total ZEC found: {:}", total_balance);
            let para =
                Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Balance"));
            frame.render_widget(para, chunks[0]);
        } else {
            let text = "Loading balance...".to_string();
            let para =
                Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Balance"));
            frame.render_widget(para, chunks[0]);
        }

        self.menu.view(frame, chunks[1]);
    }

    fn query(&self, attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> {
        todo!()
    }

    fn attr(&mut self, attr: tuirealm::Attribute, value: tuirealm::AttrValue) {}

    fn state(&self) -> State {
        todo!()
    }

    fn perform(&mut self, cmd: tuirealm::command::Cmd) -> tuirealm::command::CmdResult {
        todo!()
    }
}

impl Component<Msg, NoUserEvent> for ExportView {
    fn on(&mut self, ev: tuirealm::Event<NoUserEvent>) -> Option<Msg> {
        // first let the menu handle arrows/enter/esc:
        if let Some(menu_msg) = self.menu.on(ev.clone()) {
            return Some(menu_msg);
        }
        None
    }
}

impl<T> HandleMessage<T> for ExportView
where
    T: HasScreenAndQuit,
{
    fn handle_message(msg: Msg, model: &mut T) -> Option<Msg> {
        match msg {
            Msg::MenuSelected(option) => {
                if let Some(menu_item) = ExportOptions::from_label(&option) {
                    match menu_item {
                        ExportOptions::Zingolib => {
                            model.navigate_to(Screen::ExportZingolib);
                            todo!()
                        }
                        ExportOptions::ZeWIF => {
                            model.navigate_to(Screen::ExportZewif);
                        }
                        ExportOptions::Send => {
                            model.navigate_to(Screen::ExportSend);
                        }
                        ExportOptions::Cancel => {
                            model.navigate_to(Screen::MainMenu);
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }
}
