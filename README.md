# Google Chat Tauri

Run Google Chat as a "real" application with Tauri

Tech stack:

[![tauri badge](https://img.shields.io/badge/made_with-tauri-FFC131?logo=tauri&style=for-the-badge)](https://tauri.app) [![vite badge](https://img.shields.io/badge/bundled_with-vite-BC33FE?logo=vite&style=for-the-badge)](https://vitejs.dev) [![rust badge](https://img.shields.io/badge/built_with-rust-DEA584?logo=rust&style=for-the-badge)](https://www.typescriptlang.org/)

Pros:

- Not Electron
- Relatively small, both in file size and memory usage
- Notifications
- Cool splash screen

Cons:

- No support for third party auth

Plans:

- [ ] Linux, macOS, and Windows builds
  - [ ] AUR package
  - [ ] Flathub
- [x] System tray
  - [ ] Add options
  - [ ] Reactive for notifications
  - [ ] Fix monochrome on non-macOS
- [ ] Custom CSS & JS

Non-plans:

- Adding features outside of my use case
- Turning this into a "big project"
- Mobile builds

Credits:

- <https://github.com/squalou/google-chat-linux/> - Inspiration and icons
- `create-tauri-app` - Scaffolding template
