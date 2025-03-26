use super::config_models::{
    AdventurerSecret, Database, DotEnvConfig, GuildCommanderSecret, Server,
};
use super::config_stage::Stage;
use anyhow::Result;

pub fn load() -> Result<DotEnvConfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("SERVER_PORT")
            .expect("SERVER_PORT is not set in .env file")
            .parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")
            .expect("SERVER_BODY_LIMIT is not set in .env file")
            .parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")
            .expect("SERVER_TIMEOUT is not set in .env file")
            .parse()?,
    };

    let database = Database {
        url: std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file"),
    };

    Ok(DotEnvConfig { server, database })
}

pub fn get_state() -> Stage {
    dotenvy::dotenv().ok();
    let state_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_from(state_str.as_str()).unwrap()
}

pub fn get_adventurer_secret() -> Result<AdventurerSecret> {
    dotenvy::dotenv().ok();

    let secret =
        std::env::var("ADVENTURER_SECRET").expect("ADVENTURER_SECRET is not set in .env file");
    let refresh_token = std::env::var("ADVENTURER_REFRESH_TOKEN")
        .expect("ADVENTURER_REFRESH_TOKEN is not set in .env file")
        .parse()?;

    Ok(AdventurerSecret {
        secret,
        refresh_token,
    })
}

pub fn get_guild_commander_secret() -> Result<GuildCommanderSecret> {
    dotenvy::dotenv().ok();
    let secret = std::env::var("GUILD_COMMANDER_SECRET")
        .expect("GUILD_COMMANDER_SECRET is not set in .env file");
    let refresh_token = std::env::var("GUILD_COMMANDER_REFRESH_TOKEN")
        .expect("GUILD_COMMANDER_REFRESH_TOKEN is not set in .env file")
        .parse()?;

    Ok(GuildCommanderSecret {
        secret,
        refresh_token,
    })
}
