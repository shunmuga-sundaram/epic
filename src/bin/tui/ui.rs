// Copyright 2020 The epic Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Basic TUI to better output the overall system status and status
//! of various subsystems

use chrono::prelude::Utc;
use cursive::direction::Orientation;
use cursive::theme::BaseColor::{Black, Blue, Cyan, White};
use cursive::theme::Color::Dark;
use cursive::theme::PaletteColor::{
	Background, Highlight, HighlightInactive, Primary, Shadow, View,
};
use cursive::theme::{BaseColor, BorderStyle, Color, Theme};
use cursive::traits::Boxable;
use cursive::traits::Identifiable;
use cursive::utils::markup::StyledString;
use cursive::views::{BoxedView, CircularFocus, Dialog, LinearLayout, Panel, StackView, TextView};
use cursive::Cursive;
use cursive::CursiveRunner;
use std::sync::mpsc;

use crate::built_info;
use crate::servers::Server;
use crate::tui::constants::ROOT_STACK;
use crate::tui::types::{TUIStatusListener, UIMessage};
use crate::tui::{logs, menu, mining, peers, status, version};
use epic_util::logger::LogEntry;

pub struct UI {
	cursive: CursiveRunner<Cursive>,
	ui_rx: mpsc::Receiver<UIMessage>,
	ui_tx: mpsc::Sender<UIMessage>,
	controller_tx: mpsc::Sender<ControllerMessage>,
	logs_rx: mpsc::Receiver<LogEntry>,
}

fn modify_theme(theme: &mut Theme) {
	theme.shadow = false;
	theme.borders = BorderStyle::Simple;
	theme.palette[Background] = Dark(Black);
	theme.palette[Shadow] = Dark(Black);
	theme.palette[View] = Dark(Black);
	theme.palette[Primary] = Dark(White);
	theme.palette[Highlight] = Dark(Cyan);
	theme.palette[HighlightInactive] = Dark(Blue);
	// also secondary, tertiary, TitlePrimary, TitleSecondary
}

impl UI {
	/// Create a new UI
	pub fn new(
		controller_tx: mpsc::Sender<ControllerMessage>,
		logs_rx: mpsc::Receiver<LogEntry>,
	) -> UI {
		let (ui_tx, ui_rx) = mpsc::channel::<UIMessage>();

		let backend = cursive::backends::curses::pan::Backend::init().unwrap();

		let mut epic_ui = UI {
			cursive: cursive::CursiveRunner::new(cursive::Cursive::new(), backend),
			ui_tx,
			ui_rx,
			controller_tx,
			logs_rx,
		};

		// Create UI objects, etc
		let status_view = status::TUIStatusView::create();
		let mining_view = mining::TUIMiningView::create();
		let peer_view = peers::TUIPeerView::create();
		let logs_view = logs::TUILogsView::create();
		let version_view = version::TUIVersionView::create();

		let main_menu = menu::create();

		let root_stack = StackView::new()
			.layer(version_view)
			.layer(mining_view)
			.layer(peer_view)
			.layer(logs_view)
			.layer(status_view)
			.with_name(ROOT_STACK)
			.full_height();

		let mut title_string = StyledString::new();
		title_string.append(StyledString::styled(
			format!(
				"epic Version {} (proto: {})",
				built_info::PKG_VERSION,
				Server::protocol_version()
			),
			Color::Dark(BaseColor::Green),
		));

		let main_layer = LinearLayout::new(Orientation::Vertical)
			.child(Panel::new(TextView::new(title_string).full_width()))
			.child(
				LinearLayout::new(Orientation::Horizontal)
					.child(Panel::new(BoxedView::new(main_menu)))
					.child(Panel::new(root_stack)),
			);

		//set theme
		let mut theme = epic_ui.cursive.current_theme().clone();
		modify_theme(&mut theme);
		epic_ui.cursive.set_theme(theme);
		epic_ui.cursive.add_fullscreen_layer(main_layer);

		// Configure a callback (shutdown, for the first test)
		let controller_tx_clone = epic_ui.controller_tx.clone();
		epic_ui.cursive.add_global_callback('q', move |c| {
			let content = StyledString::styled("Shutting down...", Color::Light(BaseColor::Yellow));
			c.add_layer(CircularFocus::wrap_tab(Dialog::around(TextView::new(
				content,
			))));
			controller_tx_clone
				.send(ControllerMessage::Shutdown)
				.unwrap();
		});
		epic_ui.cursive.set_fps(3);
		epic_ui
	}

	/// Step the UI by calling into Cursive's step function, then
	/// processing any UI messages
	pub fn step(&mut self) -> bool {
		if !self.cursive.is_running() {
			return false;
		}

		while let Some(message) = self.logs_rx.try_iter().next() {
			logs::TUILogsView::update(&mut self.cursive, message);
		}

		// Process any pending UI messages
		while let Some(message) = self.ui_rx.try_iter().next() {
			match message {
				UIMessage::UpdateStatus(update) => {
					status::TUIStatusView::update(&mut self.cursive, &update);
					mining::TUIMiningView::update(&mut self.cursive, &update);
					peers::TUIPeerView::update(&mut self.cursive, &update);
					version::TUIVersionView::update(&mut self.cursive, &update);
				}
			}
		}

		// Step the UI
		self.cursive.step();
		true
	}

	/// Stop the UI
	pub fn stop(&mut self) {
		self.cursive.quit();
	}
}

pub struct Controller {
	rx: mpsc::Receiver<ControllerMessage>,
	ui: UI,
}

pub enum ControllerMessage {
	Shutdown,
}

impl Controller {
	/// Create a new controller
	pub fn new(logs_rx: mpsc::Receiver<LogEntry>) -> Result<Controller, String> {
		let (tx, rx) = mpsc::channel::<ControllerMessage>();
		Ok(Controller {
			rx,
			ui: UI::new(tx, logs_rx),
		})
	}

	/// Run the controller
	pub fn run(&mut self, server: Server) {
		let stat_update_interval = 1;
		let mut next_stat_update = Utc::now().timestamp() + stat_update_interval;
		while self.ui.step() {
			while let Some(message) = self.rx.try_iter().next() {
				match message {
					ControllerMessage::Shutdown => {
						warn!("Shutdown in progress, please wait");
						self.ui.stop();
						server.stop();
						return;
					}
				}
			}

			if Utc::now().timestamp() > next_stat_update {
				next_stat_update = Utc::now().timestamp() + stat_update_interval;
				if let Ok(stats) = server.get_server_stats() {
					self.ui.ui_tx.send(UIMessage::UpdateStatus(stats)).unwrap();
				}
			}
		}
		server.stop();
	}
}
