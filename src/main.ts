import {
  isPermissionGranted,
  requestPermission,
} from "@tauri-apps/api/notification";

let permissionGranted = await isPermissionGranted();
if (!permissionGranted) {
  const permission = await requestPermission();
  permissionGranted = permission === "granted";
}
