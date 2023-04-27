<script>
  import { invoke } from "@tauri-apps/api/tauri"

  let resp = ""

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    resp = await invoke("ryukbot")
  }
</script>

<button on:click={greet}>
  Run
</button>

<style>
  button {
    width: 100%;
  }
</style>