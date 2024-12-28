// Description: Main file for the Discord bot.

// Importing the necessary libraries.
use gemini_ai::GeminiContentGenBuilder;
use serenity::all::{ChannelId, CreateAttachment, Http, Typing};
use serenity::builder::CreateMessage;
use serenity::async_trait;
use serenity::model::channel::Message;
use std::sync::Arc;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use dotenvy;
use gemini_ai::decode_gemini;

// Creating a struct for the event handler.
struct Handler;

// Implementing the EventHandler trait for the Handler struct.
#[async_trait]
impl EventHandler for Handler {

    // Message Event Handler
    async fn message(&self, ctx: Context, msg: Message) {

        // Type in the Discord channel.
        let typing: Typing = type_dc(ctx.http.clone(), msg.channel_id).await.unwrap();

        // If the message is from a bot, ignore it.
        if msg.author.bot {
            return;
        }

        println!("Request received!");
        println!("Processing request...");
            
        // Get the message content and set it as the query.
        let query: String = msg.content.to_string();
            
        // Generate the response using the Gemini AI API.
        let builder: String = GeminiContentGenBuilder::new()
            .env("GEMINI_API_KEY")
            .model(gemini_ai::Models::GEMINI_1_5_PRO_002)
            .no_memory()
            .kind(gemini_ai::Kind::Text)
            .instruction("")
            .text(&query)
            .max_token(gemini_ai::TokenLen::Default)
            .build()
            .output();

        println!("Response received!");

        // Decode the response
        let text: String = decode_gemini(&builder).unwrap().candidates[0].content.parts[0]
            .text
            .clone();

        // Discord message character limit
        let max_length: usize = 2000;

        // If the response is too long, send it as a file.
        if text.len() > max_length {
            //Convert the response to bytes
            let data: &[u8] = text.as_bytes();

            // Create a file with the response data
            let file: CreateAttachment = CreateAttachment::bytes(data, "response.txt");

            // Create a message to send with the file
            let mut create_message: CreateMessage = CreateMessage::default();
            create_message = create_message.content("My response is too long for Discord, so I'm sending it to you as a file:");
            
            // Try to send the file to the Discord channel.
            if let Err(e) = msg.channel_id.send_files(&ctx.http, vec![file], create_message.clone()).await {
                println!("Error sending file: {e:?}");
            }
        } else {
            // Send the response as a message if it is not too long.
            if let Err(e) = msg.channel_id.say(&ctx.http, text).await {
                println!("Error sending message: {e:?}");
            }
        }

        // Stop typing in the Discord channel.
        typing.stop();
    }

    // Ready Event Handler
    async fn ready(&self, _: Context, ready: Ready) {
        // Print a message to the console when the bot is connected.
        println!("{} is connected!", ready.user.name);
    }
}

// Main function for the Discord bot.
#[tokio::main]
async fn main() {

    // Try to load the .env file and exit if it fails.
    if let Err(err) = dotenvy::dotenv() {
        println!("Failed to load .env file: {}", err);
        std::process::exit(1);
    }

    // Try to get the Discord API key from the environment variables and exit if it fails.
    let bot_token: String = match std::env::var("DISCORD_API_KEY") {
        Ok(token) => token,
        Err(_) => {
            println!("DISCORD_API_KEY not found in environment variables.");
            std::process::exit(1);
        }
    };

    // Set the intents for the bot.
    let intents: GatewayIntents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new Discord client.
    let mut client: Client = Client::builder(bot_token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start the Discord client.
    if let Err(e) = client.start().await {
        println!("Client error: {e:?}");
    }
}

// Function to type in the Discord channel.
async fn type_dc(ctx: Arc<Http>, channel_id: ChannelId) -> Result<Typing, serenity::Error> {
    let typing: Typing = Typing::start(ctx, channel_id);
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    Ok(typing)
}