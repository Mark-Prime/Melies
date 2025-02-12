<script>
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { onMount } from 'svelte';

  export let video;
  let videoElement;

  onMount(() => {
    videoElement?.play()
  })

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