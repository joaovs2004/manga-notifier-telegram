# Manga Notifier Telegram


A simple Telegram bot written in Rust that alert the user when a manga from his choice releases a new chapter. With this bot, users can add mangas to their reading list and receive notifications when a new chapter is released. This project uses the mangadex API.

## Features

- **Add Manga to Your List**: Users can add their favorite manga series to a personal list.
- **Chapter Notifications**: Receive alerts when a new chapter is released for manga in your list.

## How to test

If you have a telegram account, you can use this bot sending a message to https://t.me/MangaChapterAlertBot

## Project demonstration

![Gif demonstration](./demonstration/demo-video.gif)

When a new chapter of a manga on your list is released, you receive a message that looks like this

![Gif demonstration](./demonstration/screenshot_bot.png)

## Prerequisites

Before running the bot, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/)
- [Telegram Bot Token](https://core.telegram.org/bots#botfather): You'll need to create a bot on Telegram and get your unique API token from BotFather.

## Installation

### 1. Clone the repository

```bash
git clone https://github.com/joaovs2004/manga-notifier-telegram
cd manga-notifier-telegram
```

### 2. Initialise the TELOXIDE_TOKEN environmental variable to your token

```bash
export TELOXIDE_TOKEN=<Your token here>
```

### 3. Run the project

```bash
cargo run
```