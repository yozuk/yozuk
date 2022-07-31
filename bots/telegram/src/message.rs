use image::{ImageOutputFormat, RgbaImage};
use mediatype::{media_type, names::*};
use std::io::Cursor;
use std::str;
use std::str::FromStr;
use teloxide::RequestError;
use teloxide::{
    prelude::*,
    types::{InputFile, ParseMode},
};
use yozuk_sdk::prelude::*;

const MAX_TEXT_LENGTH: usize = 2048;
const IMAGE_PREVIEW_SIZE: u32 = 32;

pub async fn render_output(
    bot: AutoSend<Bot>,
    msg: &Message,
    output: Output,
) -> Result<(), RequestError> {
    for data in output.metadata {
        if let Metadata::Color { color } = data {
            if let Ok(color) = css_color::Rgba::from_str(&color) {
                let color = image::Rgba([
                    (color.red * 255.0) as u8,
                    (color.green * 255.0) as u8,
                    (color.blue * 255.0) as u8,
                    (color.alpha * 255.0) as u8,
                ]);
                let mut img = RgbaImage::new(IMAGE_PREVIEW_SIZE, IMAGE_PREVIEW_SIZE);
                for x in 0..IMAGE_PREVIEW_SIZE {
                    for y in 0..IMAGE_PREVIEW_SIZE {
                        img.put_pixel(x, y, color);
                        img.put_pixel(y, x, color);
                    }
                }
                let mut data = Cursor::new(Vec::<u8>::new());
                if img.write_to(&mut data, ImageOutputFormat::Png).is_ok() {
                    bot.send_photo(msg.chat.id, InputFile::memory(data.into_inner()))
                        .send()
                        .await?;
                }
            }
        }
    }
    for block in output.blocks {
        render_block(bot.clone(), msg, block).await?;
    }
    Ok(())
}

async fn render_block(bot: AutoSend<Bot>, msg: &Message, block: Block) -> Result<(), RequestError> {
    match block {
        Block::Comment(comment) => {
            bot.send_message(msg.chat.id, comment.text).send().await?;
        }
        Block::Data(data) => {
            render_data(bot, msg, data).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "[unimplemented]".to_string())
                .send()
                .await?;
        }
    }
    Ok(())
}

async fn render_data(
    bot: AutoSend<Bot>,
    msg: &Message,
    block: block::Data,
) -> Result<(), RequestError> {
    let essence = block.media_type.essence();
    let data = &block.data;
    let text = str::from_utf8(data).ok();

    match text {
        Some(text) if text.len() <= MAX_TEXT_LENGTH => {
            bot.send_message(msg.chat.id, format!("<pre>{}</pre>", text))
                .parse_mode(ParseMode::Html)
                .send()
                .await?;
        }
        _ if essence.ty == IMAGE => {
            bot.send_photo(msg.chat.id, InputFile::memory(data.to_vec()))
                .send()
                .await?;
        }
        _ if essence == media_type!(AUDIO / MPEG) || essence == media_type!(AUDIO / MP4) => {
            bot.send_audio(msg.chat.id, InputFile::memory(data.to_vec()))
                .send()
                .await?;
        }
        _ if essence == media_type!(VIDEO / MP4) => {
            bot.send_video(msg.chat.id, InputFile::memory(data.to_vec()))
                .send()
                .await?;
        }
        _ => {
            let ext = new_mime_guess::get_extensions(essence.ty.as_str(), essence.subty.as_str())
                .and_then(|list| list.first())
                .unwrap_or(&"bin");
            bot.send_document(
                msg.chat.id,
                InputFile::memory(data.to_vec()).file_name(format!("data.{}", ext)),
            )
            .send()
            .await?;
        }
    }

    Ok(())
}
