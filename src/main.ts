import {
  isPermissionGranted,
  requestPermission,
} from "@tauri-apps/api/notification";

let permissionGranted = await isPermissionGranted();
if (!permissionGranted) {
  const permission = await requestPermission();
  permissionGranted = permission === "granted";
}

window.location.replace("https://mail.google.com/chat/u/0");
