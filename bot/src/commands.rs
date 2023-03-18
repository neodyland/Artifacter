use gen::{Lang,locale::Locale, ImageFormat};
use poise::{CreateReply,serenity_prelude::{CreateEmbed,CreateEmbedFooter, CreateAttachment}};
use serde_json::json;

use crate::{Context,Error, util::{convert_rgb, create_components}};



/// fetch data from User Id
#[poise::command(
    slash_command,
    description_localized("ja", "UIDからデータを取得します")
)]
pub async fn build(
    ctx: Context<'_>,
    #[description = "UID"]
    #[description_localized("ja", "ユーザーID")]
    uid: i32,
) -> Result<(), Error> {
    let locale = ctx.locale().unwrap_or("ja");
    let lang = Lang::from(locale);
    if uid.to_string().len() != 9 {
        let msg = CreateReply::new().content(
            Locale::from(json!({
                "ja": "ユーザーIDは9桁の数字でなければなりません。",
                "en": "User ID must be a 9-digit number.",
            }))
            .get(&lang),
        );
        ctx.send(msg).await?;
        return Ok(());
    }
    ctx.defer().await?;
    let data = ctx.data();
    let api = &data.lock().await.api;
    let user = api.simple(uid).await?;
    let ids = user.profile().show_character_list();
    let characters = ids.iter().map(|id| user.character(*id)).collect::<Vec<_>>();
    let characters = characters
        .iter()
        .filter_map(|c| c.as_ref())
        .collect::<Vec<_>>();
    let characters = characters
        .iter()
        .map(|c| c.to_owned().to_owned())
        .collect::<Vec<_>>();
    if characters.is_empty() {
        let msg = CreateReply::new().content(Locale::from(
            json!({"ja":"キャラクターが登録されていません。(もしくは非公開になっています)","en": "No character is set(Or it may be private)"})).get(&lang));
        ctx.send(msg).await?;
        return Ok(());
    }
    let footer = CreateEmbedFooter::new(format!("{}", uid));
    let embed = CreateEmbed::default()
        .title(format!(
            "{}({},{})",
            user.profile().nickname(),
            user.profile().level(),
            user.profile().world_level()
        ))
        .footer(footer)
        .color(convert_rgb([0x00, 0xff, 0x00]))
        .description(user.profile().signature())
        .image("attachment://name_card.png")
        .fields(vec![(
            Locale::from(json!(
                {"ja": "アチーブメント", "en": "Achievements"}
            ))
            .get(&lang),
            user.profile().achievement().to_string(),
            true,
        ),(
            Locale::from(json!(
                {"ja": "螺旋", "en": "Spiral Abyss"}
            ))
            .get(&lang),
            format!(
                "{}{}{}{}",
                user.profile().tower_floor_index(),
                Locale::from(json!(
                    {"ja": "階", "en": "F"}
                ))
                .get(&lang),
                user.profile().tower_level_index(),
                Locale::from(json!(
                    {"ja": "層", "en": "L"}
                )).get(&lang)
            ),
            true,
        )]);
    let card = gen::convert(
        user.profile().name_card().image(&api).await?,
        ImageFormat::Png,
    );
    let attachment = if card.is_some() {
        Some(CreateAttachment::bytes(card.unwrap(), "name_card.png"))
    } else {
        None
    };
    let mut builder = CreateReply::default()
        .components(create_components(characters, api, &lang, &uid))
        .embed(embed);
    if attachment.is_some() {
        builder = builder.attachment(attachment.unwrap());
    }
    ctx.send(builder).await?;
    Ok(())
}

/// get artifact score from image
#[poise::command(
    context_menu_command = "artifact",
    description_localized("ja", "画像から神器スコアを取得します")
)]
pub async fn artifact(
    ctx: Context<'_>,
    #[description = "image"]
    #[description_localized("ja", "画像")]
    msg: serenity::all::Message,
) -> Result<(), Error> {
    let locale = ctx.locale().unwrap_or("ja");
    let lang = Lang::from(locale);
    ctx.defer().await?;
    let data = ctx.data();
    let api = &data.lock().await.api;
    let image = msg.attachments.first();
    if image.is_none() {
        let msg = CreateReply::new().content(
            Locale::from(json!({
                "ja": "画像が添付されていません。",
                "en": "No image attached.",
            }))
            .get(&lang),
        );
        ctx.send(msg).await?;
        return Ok(());
    }
    let image = image.unwrap();
    let image = image.download().await;
    if image.is_err() {
        let msg = CreateReply::new().content(
            Locale::from(json!({
                "ja": "画像のダウンロードに失敗しました。",
                "en": "Failed to download image.",
            }))
            .get(&lang),
        );
        ctx.send(msg).await?;
        return Ok(());
    }
    let image = image.unwrap();
    let msg = CreateReply::new();
    ctx.send(msg).await?;
    Ok(())
}