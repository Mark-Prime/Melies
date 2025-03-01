<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, createEventDispatcher } from "svelte";
  import { faFileVideo } from "@fortawesome/free-solid-svg-icons";
  import Modal from "$lib/components/Modal.svelte";
  import Fa from "svelte-fa";
  import Input from "./components/Input.svelte";
  import VideoPlayer from "./components/VideoPlayer.svelte";
  import { filter } from "mathjs";

  let settings = {};
  let videos = $state([]);
  let deleted = [];
  let saved = [];
  let index = $state(0);
  let folder = $state("");

  const dispatch = createEventDispatcher();

  const forceUpdate = async (_) => {};
	let doRerender = $state(0);

  async function load_files() {
    videos = await invoke("load_files", { folder });
    index = 0;
    doRerender++;
  }

  async function loadSettings() {
    settings = await invoke("load_settings");
    folder = settings.output.folder
  }

  async function open_file(path) {
    await invoke("open_file", { path })
  }

  let enabled = $state(false);

  function toggle() {
    enabled = !enabled;
  }

  function close() {
    enabled = false;
    videos = [];
    index = 0;
    doRerender = 0;
    saved = [];
    deleted = [];
  }

  onMount(() => {
    loadSettings();
  })

  async function save_files(video) {
    saved.push(video.path)
    increment(1);
  }

  async function delete_files(video) {
    deleted.push(video.path)
    await invoke("delete_file", { path: video.path })
    increment(1);
  }

  function increment(num, oldIndex = index) {
    if (saved.length + deleted.length >= videos.length) {
      videos = [];
      deleted = [];
      saved = [];
      index = 0;

      return;
    }

    index += num;

    if (index < 0) {
      index = 0;
      num = 1;
    }

    if (index >= videos.length) {
      index = videos.length - 1;
      num = -1;
    }

    if (saved.includes(videos[index]?.path) || deleted.includes(videos[index]?.path)) {
      increment(num, oldIndex);
    }

    if (index !== oldIndex) {
      doRerender++;
    }
  }

  let videoElement;
</script>
  
<button class="btn btn--tert" onclick={toggle}>
  <Fa icon={faFileVideo} color={`var(--tert)`} />
  Sort Footage
</button>
  
<Modal color="tert" {toggle} {enabled} large={videos.length > 0} max_width={videos.length > 0 ? "1425px" : null}>
  <div class="video-modal">
    <div class="input-group">
      <Input
        title="Recordings Folder"
        color="tert"
        bind:value={folder}
      />
      <button class="btn btn--tert" onclick={load_files}>
        Open
      </button>
    </div>
    {#await forceUpdate(doRerender) then _}
      {#if videos.length > 0}
        <h3>{videos[index]?.name}</h3>
        <div class="video-player">
          <VideoPlayer video={videos[index]?.layers?.video} />
          <div class="side-buttons">
            <div class="layers">
              Open Layer
              <button onclick={() => open_file(videos[index].layers.video)} class="btn">Video</button>
              {#each Object.keys(videos[index]?.layers).filter((layer) => layer !== "video") as layer}
                <button onclick={() => open_file(videos[index].layers[layer])} class="btn btn--sec">{layer}</button>
              {/each} 

              <div class="buttons">
                <button class="btn btn--tert" onclick={() => increment(-1)}>Prev</button>
                <button class="btn btn--tert" onclick={() => increment(1)}>Next</button>
              </div>

              <div>{index + 1}/{videos.length}</div>
            </div>

            <div class="layers">
              <button class="btn" onclick={() => save_files(videos[index])}>Save</button>
              <button class="cancel-btn" onclick={() => delete_files(videos[index])}>Delete</button>
              <button class="cancel-btn" onclick={() => close()}>Cancel</button>
            </div>
          </div>
        </div>
      {/if}
    {/await}
  </div>

  {#snippet footer()}
    <div >
      {#if videos.length == 0}
        <button onclick={toggle} class="cancel-btn">Cancel</button>
      {/if}
    </div>
  {/snippet}
</Modal>

<style lang="scss">
  h3 {
    margin: auto;
  }

  .video-modal {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    margin: auto;
    height: 100%;
  }

  .video-player {
    display: grid;
    grid-template-columns: 1fr min-content;
    gap: 1rem;
  }

  .layers {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .input-group {
    width: 100%;
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 1rem;

    align-items: end;
  }

  .btn {
    display: flex;
    gap: 0.5rem;
  }

  .buttons {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
  }

  div button {
    display: flex;
    justify-content: center;
  }

  .side-buttons {
    display: flex;
    flex-direction: column;
    gap: 1rem;

    justify-content: space-between;
  }
</style>
