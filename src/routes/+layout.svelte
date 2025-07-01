<script lang="ts">
  import type { Snippet } from "svelte";
  import "../style.css";

  interface Props {
    children: Snippet;
  }

  const { children }: Props = $props();
</script>

{@render children()}
