use mediatype::{media_type, names::*};
use teloxide::RequestError;
use teloxide::{
    prelude2::*,
    types::{InputFile, ParseMode},
};
use yozuk_sdk::prelude::*;

const MAX_TEXT_LENGTH: usize = 2048;

pub async fn render_output(
    bot: AutoSend<Bot>,
    msg: &Message,
    output: Output,
) -> Result<(), RequestError> {
    for section in &output.sections {
        render_section(bot.clone(), msg, &output, section).await?;
    }
    Ok(())
}

async fn render_section(
    bot: AutoSend<Bot>,
    msg: &Message,
    output: &Output,
    section: &Section,
) -> Result<(), RequestError> {
    let essence = section.media_type.essence();

    match () {
        _ if essence.ty == TEXT
            && section.data.len() <= MAX_TEXT_LENGTH
            && section.kind == SectionKind::Value =>
        {
            bot.send_message(msg.chat.id, format!("<pre>{}</pre>", section.as_utf8()))
                .parse_mode(ParseMode::Html)
                .send()
                .await?;
        }
        _ if essence.ty == TEXT
            && (section.data.len() <= MAX_TEXT_LENGTH || section.kind == SectionKind::Comment) =>
        {
            let text = if output.module.is_empty() {
                section.as_utf8().to_string()
            } else {
                format!("<b>{}:</b> {}", output.module, section.as_utf8())
            };
            bot.send_message(msg.chat.id, text)
                .parse_mode(ParseMode::Html)
                .send()
                .await?;
        }
        _ if essence.ty == IMAGE => {
            bot.send_photo(msg.chat.id, InputFile::memory(section.data.to_vec()))
                .send()
                .await?;
        }
        _ if essence == media_type!(AUDIO / MPEG) || essence == media_type!(AUDIO / MP4) => {
            bot.send_audio(msg.chat.id, InputFile::memory(section.data.to_vec()))
                .send()
                .await?;
        }
        _ if essence == media_type!(VIDEO / MP4) => {
            bot.send_video(msg.chat.id, InputFile::memory(section.data.to_vec()))
                .send()
                .await?;
        }
        _ if section.media_type.suffix() == Some(JSON) => {
            if let Ok(value) = serde_json::from_slice::<serde_json::Value>(&section.data) {
                if let Ok(yaml) = serde_yaml::to_string(&value) {
                    bot.send_message(
                        msg.chat.id,
                        format!("<pre>{}</pre>", yaml.trim_start_matches("---\n")),
                    )
                    .parse_mode(ParseMode::Html)
                    .send()
                    .await?;
                    return Ok(());
                }
            }
            bot.send_message(msg.chat.id, format!("<pre>{}</pre>", section.as_utf8()))
                .parse_mode(ParseMode::Html)
                .send()
                .await?;
        }
        _ => {
            let ext = new_mime_guess::get_extensions(essence.ty.as_str(), essence.subty.as_str())
                .and_then(|list| list.first())
                .unwrap_or(&"bin");
            bot.send_document(
                msg.chat.id,
                InputFile::memory(section.data.to_vec()).file_name(format!("data.{}", ext)),
            )
            .send()
            .await?;
        }
    };

    Ok(())
}
