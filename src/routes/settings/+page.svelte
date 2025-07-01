<script lang="ts">
  import { message, ask } from "@tauri-apps/plugin-dialog";
  import { KeyboardShortcutInput } from "$lib/component";
  import {
    settings,
    subscribe as subscribeSettings,
    set as setSettings,
    reset as resetSettings,
    type Shortcut,
    reset,
  } from "$lib/settings";
  import { onDestroy } from "svelte";

  let showSettingsShortcut = $state<Shortcut | undefined>();
  let increaseLineThicknessShortcut = $state<Shortcut | undefined>();
  let decreaseLineThicknessShortcut = $state<Shortcut | undefined>();
  let increasePointerGapShortcut = $state<Shortcut | undefined>();
  let decreasePointerGapShortcut = $state<Shortcut | undefined>();

  let cleanup = () => {};

  subscribeSettings((settings) => {
    alert("subscription running");
    showSettingsShortcut = settings.shortcuts.showSettings;
    increaseLineThicknessShortcut = settings.shortcuts.increaseLineThickness;
    decreaseLineThicknessShortcut = settings.shortcuts.decreaseLineThickness;
    increasePointerGapShortcut = settings.shortcuts.increasePointerGap;
    decreasePointerGapShortcut = settings.shortcuts.deacreasePointerGap;
  }).then((unsub) => (cleanup = unsub));

  onDestroy(cleanup);

  $effect(() => {
    showSettingsShortcut;
    increaseLineThicknessShortcut;
    decreaseLineThicknessShortcut;
    increasePointerGapShortcut;
    decreasePointerGapShortcut;

    (async () => {
      const currentSettings = await settings();

      // if (
      //   !(
      //     showSettingsShortcut &&
      //     increaseLineThicknessShortcut &&
      //     decreaseLineThicknessShortcut &&
      //     increasePointerGapShortcut &&
      //     decreasePointerGapShortcut
      //   )
      // ) {
      //   return;
      // }

      // const shortcutsChanged =
      //   currentSettings.shortcuts.showSettings !== showSettingsShortcut ||
      //   currentSettings.shortcuts.increaseLineThickness !==
      //     increaseLineThicknessShortcut ||
      //   currentSettings.shortcuts.decreaseLineThickness !==
      //     decreaseLineThicknessShortcut ||
      //   decreasePointerGapShortcut !== increasePointerGapShortcut ||
      //   decreasePointerGapShortcut !== increasePointerGapShortcut;

      // console.log("shortcutsChanged", shortcutsChanged);

      // if (shortcutsChanged) {
      //   await setSettings({
      //     ...currentSettings,
      //     shortcuts: {
      //       showSettings: showSettingsShortcut,
      //       increaseLineThickness: increaseLineThicknessShortcut,
      //       decreaseLineThickness: decreaseLineThicknessShortcut,
      //       increasePointerGap: increasePointerGapShortcut,
      //       deacreasePointerGap: decreasePointerGapShortcut,
      //     },
      //   });
      // }
    })();
  });
</script>

<main class="p-8">
  <h1 class="text-2xl font-bold mb-8">Settings</h1>

  <fieldset class="fieldset">
    <KeyboardShortcutInput
      label="Show Settings"
      shortcut={showSettingsShortcut}
      onchange={(newShortcut) => (showSettingsShortcut = newShortcut)}
      onerror={(msg) =>
        message(msg, {
          kind: "warning",
        })}
    />
    <KeyboardShortcutInput
      label="Increase Line Thickness"
      shortcut={increaseLineThicknessShortcut}
      onchange={(newShortcut) => (increaseLineThicknessShortcut = newShortcut)}
      onerror={(msg) =>
        message(msg, {
          kind: "warning",
        })}
    />
    <KeyboardShortcutInput
      label="Decrease Line Thickness"
      shortcut={decreaseLineThicknessShortcut}
      onchange={(newShortcut) => (decreaseLineThicknessShortcut = newShortcut)}
      onerror={(msg) =>
        message(msg, {
          kind: "warning",
        })}
    />
    <KeyboardShortcutInput
      label="Increase Pointer Gap"
      shortcut={increasePointerGapShortcut}
      onchange={(newShortcut) => (increasePointerGapShortcut = newShortcut)}
      onerror={(msg) =>
        message(msg, {
          kind: "warning",
        })}
    />
    <KeyboardShortcutInput
      label="Decrease Pointer Gap"
      shortcut={increasePointerGapShortcut}
      onchange={(newShortcut) => (increasePointerGapShortcut = newShortcut)}
      onerror={(msg) =>
        message(msg, {
          kind: "warning",
        })}
    />
    <button
      class="btn btn-warning"
      onclick={async () => {
        const shouldResetSettings = await ask(
          "Are you sure you want to reset all settings to their defaults",
          { kind: "warning" },
        );

        if (!shouldResetSettings) {
          return;
        }

        try {
          await reset();
        } catch (err) {
          console.error("failed resetting shortcuts", err);

          await message("Something went wrong resetting your settings. :(", {
            kind: "error",
          });
        }
      }}
    >
      Reset All Shortcuts
    </button>
  </fieldset>
</main>
