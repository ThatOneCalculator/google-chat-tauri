# Google Chat Tauri

Run Google Chat as a "real" application with Tauri

## Download (Linux, Windows, macOS):

<https://github.com/ThatOneCalculator/google-chat-tauri/releases/latest>

---

Tech stack:

[![tauri badge](https://img.shields.io/badge/made_with-tauri-FFC131?logo=tauri&style=for-the-badge)](https://tauri.app) [![vite badge](https://img.shields.io/badge/bundled_with-vite-BC33FE?logo=vite&style=for-the-badge)](https://vitejs.dev) [![rust badge](https://img.shields.io/badge/built_with-rust-DEA584?logo=rust&style=for-the-badge)](https://www.typescriptlang.org/)

Obligatory screenshot:

![screenshot of google chat settings with a test notification](https://github.com/ThatOneCalculator/google-chat-tauri/assets/44733677/229e3955-94f8-4eaf-81ce-a7d376993406)

Pros:

- Not Electron
- Relatively small, both in file size and memory usage
- Notifications
- Cool splash screen

Cons:

- No support for third party auth

Plans:

- [x] Linux, macOS, and Windows builds
  - [ ] AUR package
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

- <https://github.com/squalou/google-chat-linux/> - Inspiration and icons
- `create-tauri-app` - Scaffolding template
