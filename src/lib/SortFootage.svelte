<script>
  import { createEventDispatcher } from "svelte";
  import { faFileVideo } from "@fortawesome/free-solid-svg-icons";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import Modal from "$lib/components/Modal.svelte";
  import Fa from "svelte-fa";

  const dispatch = createEventDispatcher();

  let enabled = false;

  function toggle() {
    enabled = !enabled;
  }

  let videoElement;
</script>
  
<button class="btn btn--tert" on:click={toggle}>
  <Fa icon={faFileVideo} color={`var(--tert)`} />
  Sort Footage
</button>
  
<Modal color="tert" {toggle} {enabled} large tall>
  <video controls bind:this={videoElement}>
    <track kind="captions" />
    <source src={convertFileSrc("C:\\Video\\video.mp4")} type="video/mp4" />
  </video>

  <div class="buttons" slot="footer">
    <button on:click={toggle} class="cancel-btn">Cancel</button>
    <button on:click={toggle} class="cancel-btn">Delete</button>
    <button on:click={toggle}>Save</button>
  </div>
</Modal>

<style lang="scss">
  .btn {
    display: flex;
    gap: 0.5rem;
  }

  .buttons {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1rem;
    padding-top: 1rem;
  }

  video {
    width: 100%;
  }
</style>
