# Google Chat Tauri

Run Google Chat as a "real" application with Tauri

## Download (Linux, Windows, macOS):

<https://github.com/ThatOneCalculator/google-chat-tauri/releases/latest>

### AUR package (Arch Linux)

```sh
yay -S google-chat-tauri-bin
```

---

Tech stack:

[![tauri badge](https://img.shields.io/badge/made_with-tauri-FFC131?logo=tauri&style=for-the-badge)](https://tauri.app) [![vite badge](https://img.shields.io/badge/bundled_with-vite-BC33FE?logo=vite&style=for-the-badge)](https://vitejs.dev) [![rust badge](https://img.shields.io/badge/powered_by-rust-DEA584?logo=rust&style=for-the-badge)](https://www.typescriptlang.org/)

Obligatory screenshot:

![image](https://github.com/user-attachments/assets/64483e35-d5ed-46c6-99ba-90aa25779848)

Pros:

- Not Electron
- Relatively small, both in file size (17MB binary and 488KB saved cache on Linux) and memory usage (~800-1000MB average with a space open)
- Notifications

Cons:

- No support for third party auth

Plans:

- [x] Linux, macOS, and Windows builds
  - [x] [AUR package](https://aur.archlinux.org/packages/google-chat-tauri-bin)
  - [ ] Flathub
- [x] System tray
  - [ ] Add options
  - [ ] Reactive for notifications
  - [ ] Fix monochrome on non-macOS
- [ ] Custom CSS & JS
- [ ] Custom settings
  - [ ] Config file
  - [ ] Possibly injecting custom settings into web UI (low priority)

Non-plans:

- Adding features far outside of my use case or a "reasonable" use case
- Turning this into a "big project" (it doesn't need to be)
- Mobile builds

Credits:

- <https://github.com/squalou/google-chat-linux/> - Inspiration
- `create-tauri-app` - Scaffolding template
