use std::env;

use chrono::{DateTime, Timelike, Utc};
use chrono_tz::{Europe::Zurich, Tz};
use env_logger::{self, Env};
use futures::StreamExt;
use log;
use telegram_bot::message::ParseMode;
use telegram_bot::prelude::*;
use telegram_bot::refs::{GroupId, ToChatRef};
use telegram_bot::send_message::SendMessage;
use telegram_bot::{Api, Error, MessageKind, UpdateKind};
use tokio::time::{self, Duration, Instant};

const CUTOFF_HOUR: u32 = 13;
const CUTOFF_MINUTE: u32 = 38;
const MAX_DELAY_S: u32 = 5;

fn get_now() -> DateTime<Tz> {
    // Timezone currently hardcoded to Europe/Zurich
    Utc::now().with_timezone(&Zurich)
}

fn should_trigger(tick: Instant, last_trigger: Instant) -> bool {
    let elapsed_seconds = tick.duration_since(last_trigger).as_secs();
    if elapsed_seconds < (60 + 5) {
        // Don't re-trigger if last trigger was less than ~1 mintes ago
        false
    } else {
        let now = get_now();
        now.hour() == CUTOFF_HOUR && now.minute() == CUTOFF_MINUTE && now.second() < MAX_DELAY_S
    }
}

async fn mark_end_of_the_wonderful_1337_minute<C: ToChatRef>(api: &Api, chat: C) {
    let mut msg = SendMessage::new(
        chat,
        format!("Heareth ye 1337 folks, the time of the wonderful 13:37th minute has passed! It is now {}.", get_now())
    );
    msg.parse_mode(ParseMode::Markdown);
    match api.send(msg).await {
        Ok(_) => log::info!("Sent 13:38 message"),
        Err(e) => log::warn!("Could not send 13:38 message: {}", e),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    log::info!("Starting...");

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let group_id: i64 = env::var("TELEGRAM_GROUP_ID")
        .expect("TELEGRAM_GROUP_ID not set")
        .parse()
        .expect("TELEGRAM_GROUP_ID is not an i64");

    log::info!("API token provided");
    log::info!("Target group id is {}", group_id);

    // Create API
    let api = Api::new(token);

    // Determine chat GroupId
    let chat = GroupId::new(group_id);

    // Fetch new updates via long poll method
    let mut stream = api.stream();

    let mut last_trigger = Instant::now();
    let mut timer = time::interval(Duration::from_millis(500));
    loop {
        // Either accept an incoming message, or a timer tick
        tokio::select! {
            msg_option = stream.next() => {
                log::trace!("Msg");
                if let Some(msg) = msg_option {
                    let update = msg?;
                    if let UpdateKind::Message(message) = update.kind {
                        if let MessageKind::Text { ref data, .. } = message.kind {
                            log::debug!("<{}>: {}", &message.from.first_name, data);
                            if data == "/time" || data.starts_with("/time@") {
                                let now = get_now();
                                api.send(message.text_reply(format!("Current time is {}", now))).await?;
                            } else if data == "/cutoff" || data.starts_with("/cutoff@") {
                                api.send(message.text_reply(format!("The 1337est of all minutes is over at {}:{}", CUTOFF_HOUR, CUTOFF_MINUTE))).await?;
                            } else {
                                api.send(message.text_reply("Hää???")).await?;
                            }
                        }
                    }
                }
            }
            tick = timer.tick() => {
                log::trace!("Tick");
                if should_trigger(tick, last_trigger) {
                    log::info!("Trigger");
                    mark_end_of_the_wonderful_1337_minute(&api, &chat).await;
                    last_trigger = tick;
                }
            }
        }
    }
}
