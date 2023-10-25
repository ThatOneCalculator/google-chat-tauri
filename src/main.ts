import {
  isPermissionGranted,
  requestPermission,
} from "@tauri-apps/api/notification";
import '@fontsource/open-sauce-sans/400.css';
import '@fontsource/open-sauce-sans/700.css';

let permissionGranted = await isPermissionGranted();
if (!permissionGranted) {
  const permission = await requestPermission();
  permissionGranted = permission === "granted";
}
// if (permissionGranted) {
//   sendNotification({ title: "TAURI", body: "Tauri is awesome!" });
// }

window.location.replace("https://mail.google.com/chat/u/0");
