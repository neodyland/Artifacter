use crate::entity::linker;
use gen::{locale::Locale, ImageFormat, Lang};
use poise::{
    serenity_prelude::{
        ButtonStyle, CreateActionRow, CreateAttachment, CreateButton, CreateEmbed,
        CreateEmbedFooter,
    },
    CreateReply,
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde_json::json;
use serenity::model::Timestamp;
use std::time::UNIX_EPOCH;

use crate::{
    util::{convert_rgb, create_components},
    Context, Error,
};

/// get user's profile
#[poise::command(
    context_menu_command = "Get Info",
    description_localized("ja", "プロフィールを表示します"),
    slash_command
)]
pub async fn profile(ctx: Context<'_>, user: serenity::all::User) -> Result<(), Error> {
    let locale = ctx.locale().unwrap_or("ja");
    let lang = Lang::from(locale);
    let data = ctx.data().lock().await;
    let db = &data.db;
    let current = linker::Entity::find_by_id(linker::cast_u64_to_f64(user.id.0.into()))
        .one(db)
        .await?;
    if current.is_none() {
        let msg = CreateReply::new().content(
            Locale::from(json!({
                "ja": "リンクされていません。",
                "en": "Not linked.",
            }))
            .get(&lang),
        );
        ctx.send(msg).await?;
        return Ok(());
    }
    let current = current.unwrap();
    let uid = current.genshin_id;
    ctx.defer().await?;
    let api = &data.api;
    let (user, down) = api.simple(uid).await?;
    let characters = user.characters_vec();
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
        .description(format!(
            "{}{}",
            user.profile().signature(),
            if down {
                let cached =
                    Locale::from(json!({"ja":"\nキャッシュから取得","en": "\nCached data"}));
                cached.get(&lang).to_string()
            } else {
                "".to_string()
            }
        ))
        .image("attachment://name_card.png")
        .timestamp(
            Timestamp::from_unix_timestamp(
                user.lastupdate()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64,
            )
            .unwrap(),
        )
        .fields(vec![
            (
                Locale::from(json!(
                    {"ja": "アチーブメント", "en": "Achievements"}
                ))
                .get(&lang),
                user.profile().achievement().to_string(),
                true,
            ),
            (
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
                    ))
                    .get(&lang)
                ),
                true,
            ),
        ]);
    let namecard = user.profile().name_card_image(&api).await;
    let card = match namecard {
        Some(card) => gen::convert(card, ImageFormat::Png),
        None => None,
    };
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

/// link discord account to genshin account
#[poise::command(slash_command, description_localized("ja", "データをリンクします"))]
pub async fn link(
    ctx: Context<'_>,
    #[description = "UID"]
    #[description_localized("ja", "ユーザーID")]
    uid: i32,
) -> Result<(), Error> {
    let locale = ctx.locale().unwrap_or("ja");
    let lang = Lang::from(locale);
    let data = ctx.data().lock().await;
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
    let db = &data.db;
    let current =
        linker::Entity::find_by_id(linker::cast_u64_to_f64(ctx.author().to_owned().id.0.into()))
            .one(db)
            .await?;
    if current.is_some() {
        let msg = CreateReply::new().content(
            Locale::from(json!({
                "ja": "すでにリンクされています。/unlinkでリンクを解除してください。",
                "en": "Already linked. Please unlink with /unlink.",
            }))
            .get(&lang),
        );
        ctx.send(msg).await?;
        return Ok(());
    }
    let new = linker::ActiveModel {
        discord_id: Set(linker::cast_u64_to_f64(ctx.author().to_owned().id.0.into())),
        genshin_id: Set(uid),
        allow_quote: Set(false),
    };
    let r = new.insert(db).await;
    if r.is_err() {
        let err = r.err().unwrap();
        let msg = CreateReply::new().content(
            Locale::from(json!({
                "ja": format!("リンクに失敗しました。{}",err),
                "en": format!("Failed to link.{}",err),
            }))
            .get(&lang),
        );
        ctx.send(msg).await?;
        return Ok(());
    }
    let msg = CreateReply::new().content(
        Locale::from(json!({
            "ja": "リンクに成功しました。",
            "en": "Successfully linked.",
        }))
        .get(&lang),
    );
    ctx.send(msg).await?;
    Ok(())
}

/// unlink discord account to genshin account
#[poise::command(
    slash_command,
    description_localized("ja", "データのリンクを解除します")
)]
pub async fn unlink(ctx: Context<'_>) -> Result<(), Error> {
    let locale = ctx.locale().unwrap_or("ja");
    let lang = Lang::from(locale);
    let data = ctx.data().lock().await;
    ctx.defer().await?;
    let db = &data.db;
    let current =
        linker::Entity::find_by_id(linker::cast_u64_to_f64(ctx.author().to_owned().id.0.into()))
            .one(db)
            .await?;
    if current.is_none() {
        let msg = CreateReply::new().content(
            Locale::from(json!({
                "ja": "リンクされていません。",
                "en": "Not linked.",
            }))
            .get(&lang),
        );
        ctx.send(msg).await?;
        return Ok(());
    }
    let r =
        linker::Entity::delete_by_id(linker::cast_u64_to_f64(ctx.author().to_owned().id.0.into()))
            .exec(db)
            .await;
    if r.is_err() {
        let err = r.err().unwrap();
        let msg = CreateReply::new().content(
            Locale::from(json!({
                "ja": format!("リンクの解除に失敗しました。{}",err),
                "en": format!("Failed to unlink.{}",err),
            }))
            .get(&lang),
        );
        ctx.send(msg).await?;
        return Ok(());
    }
    let msg = CreateReply::new().content(
        Locale::from(json!({
            "ja": "リンクの解除に成功しました。",
            "en": "Successfully unlinked.",
        }))
        .get(&lang),
    );
    ctx.send(msg).await?;
    Ok(())
}

/// fetch data from User Id
#[poise::command(
    slash_command,
    description_localized("ja", "UIDからデータを取得します")
)]
pub async fn build(
    ctx: Context<'_>,
    #[description = "UID"]
    #[description_localized("ja", "ユーザーID")]
    mut uid: Option<i32>,
) -> Result<(), Error> {
    let locale = ctx.locale().unwrap_or("ja");
    let lang = Lang::from(locale);
    let data = ctx.data().lock().await;
    if uid.is_none() {
        let r = linker::Entity::find_by_id(linker::cast_u64_to_f64(
            ctx.author().to_owned().id.0.into(),
        ))
        .one(&data.db)
        .await;
        if r.is_ok() {
            let r = r.unwrap();
            if r.is_some() {
                uid = Some(r.unwrap().genshin_id);
            }
        }
        if uid.is_none() {
            let msg = CreateReply::new().content(
                Locale::from(json!({
                    "ja": "ユーザーIDが指定されていません。",
                    "en": "User ID is not specified.",
                }))
                .get(&lang),
            );
            ctx.send(msg).await?;
            return Ok(());
        }
    }
    let uid = uid.unwrap();
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
    let api = &data.api;
    let (user, down) = api.simple(uid).await?;
    let characters = user.characters_vec();
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
        .description(format!(
            "{}{}",
            user.profile().signature(),
            if down {
                let cached =
                    Locale::from(json!({"ja":"\nキャッシュから取得","en": "\nCached data"}));
                cached.get(&lang).to_string()
            } else {
                "".to_string()
            }
        ))
        .image("attachment://name_card.png")
        .timestamp(
            Timestamp::from_unix_timestamp(
                user.lastupdate()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64,
            )
            .unwrap(),
        )
        .fields(vec![
            (
                Locale::from(json!(
                    {"ja": "アチーブメント", "en": "Achievements"}
                ))
                .get(&lang),
                user.profile().achievement().to_string(),
                true,
            ),
            (
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
                    ))
                    .get(&lang)
                ),
                true,
            ),
        ]);
    let card = match user.profile().name_card_image(&api).await {
        Some(card) => gen::convert(card, ImageFormat::Png),
        None => None,
    };
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
