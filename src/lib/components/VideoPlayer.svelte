<script>
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  /** @type {{video: any}} */
  let { video, skipBuffer, skipTo } = $props();
  let videoElement = $state();

  onMount(() => {
    if (skipBuffer) {
      videoElement.currentTime = skipTo;
    }

    videoElement?.play();
  });
</script>

{#if video}
  <video controls bind:this={videoElement} class="video">
    <track kind="captions" />
    <source src={convertFileSrc(video)} type="video/mp4" />
  </video>
{/if}

<style>
  * {
    width: 100%;
  }
</style>
