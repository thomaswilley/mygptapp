# mygptapp

A simple desktop app for chatgpt which uses your own api key and has no other dependencies.

![mygptapp screenshot](./src/assets/screenshot.png)

Dependencies:
- nodejs & npm
- tailwindcss
- rust

Installation:
- git clone this repo
- copy and rename mygpt.conf.example to mygpt.conf, and replace username and api_key in mygpt.conf
```bash
$ cargo tauri build --release
# or for local development:
$ cargo tauri dev
```