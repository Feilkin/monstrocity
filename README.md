

# Monstrocity, a cyberpunk MUD/RPG on Telegram

I have too much free time, please hire me :(

## Roadmap to launch

 * Figure out what I am doing
     - [General architecture](https://drive.google.com/file/d/1UrUYaW737-YAamB5Ux8NQZhiucGrA0vH/view?usp=sharing) (WIP)
 * Webhook
     - Get updates from Telegram, push to queue (done)
     - Token check (low prio)
     - Spam Filtering (low prio)
 * Workers
     - Make diagrams and charts (WIP)
     - Parallel
     - Dialogs (state sharing)
 * Dialogs
     - Make diagrams and charts (high prio)
     - Design API
     - Design implementation
     - Import? (low prio)
 * Game Design
     - Story (med prio)
     - Characters
     - Combat System
     - NPC (vendors, story)
     - ???


## Log

### Day <= 0

I did some work before I started to stream, and this idea has been in my mind for more than a year.

### Day 1

*I have to start counting somewhere, don't I?*

Got the basic Webhook done, going to install it to the server later. I don't think I'll have to touch it in a while, as it is fairly simple.

Worker threads are spawned, but they don't actually do anything. Need to figure out the API for the bot.

### Day 2

Made Dialogs shareable between threads, and implemented some basic functionality in the Workers. We can now parse commands form the messages, and get Dialog objects from the Bot.

### Day 3

(Off-stream) Made Dialogs work, but the code needs to be cleaned up.