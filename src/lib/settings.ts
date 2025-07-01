import { Store } from "@tauri-apps/plugin-store";
import { register, unregisterAll } from "@tauri-apps/plugin-global-shortcut";
import { browser } from "$app/environment";

export type Settings = {
  lineThickness: number;
  pointerGap: number;
  shortcuts: Shortcuts;
};

export type Shortcuts = {
  showSettings: Shortcut;
  increaseLineThickness: Shortcut;
  decreaseLineThickness: Shortcut;
  increasePointerGap: Shortcut;
  deacreasePointerGap: Shortcut;
};

export type Shortcut = {
  metaKey: boolean;
  ctrlKey: boolean;
  altKey: boolean;
  shiftKey: boolean;
  key: string;
};

async function getStore() {
  const store = await Store.get("settings.json");

  if (!store) {
    throw new Error("failed to get settings store instance");
  }

  return store;
}

export async function settings(): Promise<Settings> {
  const store = await getStore();

  const settings = await store.get<Settings>("settings");

  if (!settings) {
    throw new Error("settings was falsy");
  }

  return settings;
}

export async function subscribe(f: (settings: Settings) => void) {
  const store = await getStore();

  const settings = await store.get<Settings>("settings");

  if (!settings) {
    throw new Error("settings are missing");
  }

  f(settings);

  return await store.onKeyChange<Settings>("settings", (settings) =>
    f(settings!),
  );
}

export async function set(settings: Settings) {
  const store = await getStore();

  await store.set("settings", settings);
}

export async function reset() {
  const store = await getStore();

  await store.reset();
}
