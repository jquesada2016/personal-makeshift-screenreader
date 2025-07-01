<script lang="ts">
  import type { Shortcut } from "$lib/settings";

  interface Props {
    label: string;
    shortcut?: Shortcut;
    onchange: (shortcut: Shortcut) => void;
    onerror: (msg: string) => void;
  }

  let { label, shortcut, onchange, onerror }: Props = $props();

  let isCapturing = $state(false);
</script>

<svelte:document
  onkeydown={(e) => {
    if (!isCapturing) {
      return;
    }

    const { metaKey, ctrlKey, altKey, shiftKey, key, code } = e;

    switch (key) {
      case "Unidentified":
      case "Meta":
      case "Control":
      case "Alt":
      case "Shift":
        return;
      case "Escape":
        isCapturing = false;
        return;
    }

    e.preventDefault();

    if (!metaKey && !ctrlKey && !altKey) {
      onerror(
        "The shortcut must include one or more of thefollowing keys: `Meta`, `Ctrl`, or `Alt`",
      );

      return;
    }

    onchange({
      metaKey,
      ctrlKey,
      altKey,
      shiftKey,
      key: code,
    });
  }}
  onkeyup={(_) => (isCapturing = false)}
/>

<div class="hidden" role="alert">
  {#if isCapturing}
    Please hold down the keys you would like to use to perform this action. To
    cancel press `Escape`.
  {/if}
</div>

<button
  class="input w-full"
  class:input-primary={!isCapturing}
  class:input-warning={isCapturing}
  onclick={() => (isCapturing = true)}
>
  <span class="label">{label}</span>
  <div aria-live="polite">
    {#if !shortcut}
      <kbd class="kbd">&lt;unassigned&gt;</kbd>
    {:else}
      {#if shortcut.metaKey}
        <kbd class="kbd">Meta</kbd> +
      {/if}
      {#if shortcut.ctrlKey}
        <kbd class="kbd">Ctrl</kbd> +
      {/if}
      {#if shortcut.altKey}
        <kbd class="kbd">Alt</kbd> +
      {/if}
      {#if shortcut.shiftKey}
        <kbd class="kbd">Shift</kbd> +
      {/if}

      <kbd class="kbd">{shortcut.key}</kbd>
    {/if}
  </div>
</button>
