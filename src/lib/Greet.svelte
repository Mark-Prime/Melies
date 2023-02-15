<script>
  import { invoke } from "@tauri-apps/api/tauri"

  let greetMsg = ""

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet")
  }
</script>

<div>
  <div class="row">
    <button on:click={greet}>
      Run
    </button>
  </div>
  <p>{greetMsg}</p>
</div>