import { invoke } from "@tauri-apps/api/core";
import { createNativeIconLoader } from "$lib/utils/native-icons.js";
export const getNativeProcessIcon = createNativeIconLoader((requests) =>
  invoke("get_process_icons", { requests }),
);
