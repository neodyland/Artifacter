use localization::t;
use poise::CreateReply;
use rand::Rng;

use crate::component::profile_components;
use crate::db::find_genshin;
use crate::state::{Context, Error};

/// fetch data from User Id
#[poise::command(
    slash_command,
    description_localized("ja", "UIDからデータを取得します")
)]
pub async fn build(
    ctx: Context<'_>,
    #[description = "UID"]
    #[description_localized("ja", "ユーザーID")]
    uid: Option<i32>,
) -> Result<(), Error> {
    let display_tips = if uid.is_some() {
        let rng: u32 = rand::thread_rng().gen_range(0..100);
        rng < 25
    } else {
        false
    };
    let locale = ctx.locale().unwrap_or("ja");
    let data = ctx.data();
    let mut uid = uid.map(|u| u.to_string());
    if uid.is_none() {
        if let Ok(db_uid) = find_genshin(&data.db, ctx.author().id.get()).await {
            if let Some(db_uid) = db_uid {
                uid = Some(db_uid);
            }
        }
        if uid.is_none() {
            ctx.send(CreateReply::new().content(t!(locale, "main:general.noUserId")))
                .await?;
            return Ok(());
        }
    }
    let uid = uid.unwrap();
    if uid.to_string().len() != 9 {
        ctx.send(CreateReply::new().content(t!(locale, "main:general.userIdMustBeNineDigits")))
            .await?;
        return Ok(());
    }
    ctx.defer().await?;
    let user = match data
        .api
        .profile(uid.clone(), Some(locale.to_string()))
        .await
    {
        Ok(user) => user,
        Err(_) => {
            ctx.send(CreateReply::new().content(t!(locale, "main:general.failedToFetchData")))
                .await?;
            return Ok(());
        }
    };
    if user.characters.is_empty() {
        ctx.send(CreateReply::new().content(t!(locale, "main:general.noCharacters")))
            .await?;
        return Ok(());
    }
    let (embed, components, attachment) = profile_components(locale.to_string(), uid, user);
    let mut builder = CreateReply::default().components(components).embed(embed);
    if let Some(attachment) = attachment {
        builder = builder.attachment(attachment);
    }
    if display_tips {
        builder = builder.content(t!(locale, "main:tips.genshinLink"));
    }
    ctx.send(builder).await?;
    Ok(())
}
