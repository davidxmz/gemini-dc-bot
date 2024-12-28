# Guide: Discord Gemini AI Bot

A simple Discord bot written in Rust that uses the Gemini AI API to generate responses to user queries.  

## How It Works  
The bot responds with AI-generated answers.  
You can ask it questions in a guild message channel or via direct messages (DMs).  

## Requirements  
To get started, you’ll need two API keys:  
1. **Discord API Key**  
2. **Gemini API Key**  

## Setup  

1. **Prepare the API Keys:**  
   - Create a **Discord API Key** and save it in a `.env` file under the key `DISCORD_API_KEY`.  
   - Create a **Gemini API Key** and save it in the `.env` file under the key `GEMINI_API_KEY`.  

   Your `.env` file should look like this:  
   ```env
   DISCORD_API_KEY=your_discord_api_key
   GEMINI_API_KEY=your_gemini_api_key
   ```

2. **Prepare the Bot**
    - Build the bot yourself or use the binary provided in the releases section.
    - Place your `.env` file in the same directory as the binary file.

3. **Start the bot**
    - Execute your binary