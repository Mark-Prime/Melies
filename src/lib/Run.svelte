<script>
  import { invoke } from "@tauri-apps/api/core";

  let resp = "";

  async function greet() {
    resp = await invoke("ryukbot");
  }
</script>

<button on:click={greet}> Run </button>

<style>
  button {
    width: 100%;
  }
</style>
