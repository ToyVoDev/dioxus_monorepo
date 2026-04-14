use crate::{
    error::AppError,
    models::{NewReactionMessage, NewSelfAssignableRole},
    queries::{create_reaction_message, create_self_assignable_role, delete_self_assignable_role},
};
use rust_i18n::t;
use {
    crate::state::{AppState, MessageType},
    poise::serenity_prelude as serenity,
    std::sync::Arc,
    tokio::sync::Mutex,
};

fn get_pool() -> Result<deadpool_diesel::postgres::Pool, AppError> {
    crate::state::GLOBAL_POOL
        .get()
        .cloned()
        .ok_or_else(|| AppError::Other("Pool not initialized".to_string()))
}

#[poise::command(
    slash_command,
    required_permissions = "MANAGE_ROLES",
    subcommands("minecraft_geyser_restart", "minecraft_geyser_stop")
)]
pub async fn minecraft_geyser(_ctx: crate::state::Context<'_>) -> Result<(), AppError> {
    unreachable!()
}

#[poise::command(slash_command, required_permissions = "MANAGE_ROLES")]
pub async fn minecraft_geyser_restart(ctx: crate::state::Context<'_>) -> Result<(), AppError> {
    ctx.say(String::from("Restarting Java/Bedrock Minecraft Server"))
        .await?;
    Ok(())
}

#[poise::command(slash_command, required_permissions = "MANAGE_ROLES")]
pub async fn minecraft_geyser_stop(ctx: crate::state::Context<'_>) -> Result<(), AppError> {
    ctx.say(String::from("Stopping Java/Bedrock Minecraft Server"))
        .await?;
    Ok(())
}

#[poise::command(
    slash_command,
    required_permissions = "MANAGE_ROLES",
    subcommands("minecraft_modded_restart", "minecraft_modded_stop")
)]
pub async fn minecraft_modded(_ctx: crate::state::Context<'_>) -> Result<(), AppError> {
    unreachable!()
}

#[poise::command(slash_command, required_permissions = "MANAGE_ROLES")]
pub async fn minecraft_modded_restart(ctx: crate::state::Context<'_>) -> Result<(), AppError> {
    ctx.say(String::from("Restarting Modded Minecraft Server"))
        .await?;
    Ok(())
}

#[poise::command(slash_command, required_permissions = "MANAGE_ROLES")]
pub async fn minecraft_modded_stop(ctx: crate::state::Context<'_>) -> Result<(), AppError> {
    ctx.say(String::from("Stopping Modded Minecraft Server"))
        .await?;
    Ok(())
}

#[poise::command(
    slash_command,
    required_permissions = "MANAGE_ROLES",
    subcommands("terraria_broadcast_message", "terraria_restart", "terraria_stop")
)]
pub async fn terraria(_ctx: crate::state::Context<'_>) -> Result<(), AppError> {
    unreachable!()
}

#[poise::command(slash_command, required_permissions = "MANAGE_ROLES")]
pub async fn terraria_broadcast_message(
    ctx: crate::state::Context<'_>,
    #[description = "The message to broadcast"] message: String,
) -> Result<(), AppError> {
    ctx.say(format!(
        "Broadcasting message: `{}` to Terraria Server",
        message
    ))
    .await?;
    Ok(())
}

#[poise::command(slash_command, required_permissions = "MANAGE_ROLES")]
pub async fn terraria_restart(ctx: crate::state::Context<'_>) -> Result<(), AppError> {
    ctx.say(String::from("Restarting Terraria Server")).await?;
    Ok(())
}

#[poise::command(slash_command, required_permissions = "MANAGE_ROLES")]
pub async fn terraria_stop(ctx: crate::state::Context<'_>) -> Result<(), AppError> {
    ctx.say(String::from("Stopping Terraria Server")).await?;
    Ok(())
}

#[poise::command(slash_command, required_permissions = "MANAGE_ROLES")]
pub async fn game_roles(ctx: crate::state::Context<'_>) -> Result<(), AppError> {
    let pool = get_pool()?;
    if let Some(guild) = ctx.partial_guild().await {
        let self_assignable_roles = crate::queries::get_self_assignable_roles(
            pool.clone(),
            u64::from(guild.id).to_string(),
        )
        .await?;
        let message = self_assignable_roles
            .iter()
            .map(|sar| format!("{} = <@&{}>", sar.emoji, sar.role_id))
            .collect::<Vec<_>>()
            .join("\n");
        let sent_message = ctx
            .say(format!("{}\n{}", t!("roles.intro"), message,))
            .await?;
        let sent_message_id = sent_message.message().await?.id;
        let new_message = NewReactionMessage {
            guild_id: u64::from(guild.id).to_string(),
            message_id: u64::from(sent_message_id).to_string(),
            message_type: MessageType::RoleAssigner.to_string(),
        };
        let _new_message = create_reaction_message(pool, new_message).await?;
    }
    Ok(())
}

#[poise::command(slash_command, required_permissions = "MANAGE_ROLES")]
pub async fn register_self_assignable_role(
    ctx: crate::state::Context<'_>,
    #[description = "Pick a role"] role: serenity::RoleId,
    #[description = "Reaction emoji"] emoji: String,
) -> Result<(), AppError> {
    let pool = get_pool()?;
    let new_role = NewSelfAssignableRole {
        emoji: emoji.clone(),
        guild_id: u64::from(ctx.guild_id().unwrap()).to_string(),
        role_id: u64::from(role).to_string(),
    };
    let _new_role = create_self_assignable_role(pool, new_role).await?;
    ctx.say(format!(
        "Registered self-assignable role: <@&{}> with emoji {}",
        role, emoji
    ))
    .await?;
    Ok(())
}

#[poise::command(slash_command, required_permissions = "MANAGE_ROLES")]
pub async fn deregister_self_assignable_role(
    ctx: crate::state::Context<'_>,
    #[description = "Pick a role"] role: serenity::RoleId,
) -> Result<(), AppError> {
    let pool = get_pool()?;
    delete_self_assignable_role(
        pool,
        u64::from(ctx.guild_id().unwrap()).to_string(),
        u64::from(role).to_string(),
    )
    .await?;
    ctx.say(format!("Deregistered self-assignable role: <@&{}>", role))
        .await?;
    Ok(())
}

pub async fn event_handler(
    _ctx: &serenity::Context,
    _event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Arc<Mutex<AppState>>, AppError>,
    _data: &Arc<Mutex<AppState>>,
) -> Result<(), AppError> {
    // TODO: Implement reaction-based role assignment using diesel queries
    Ok(())
}
