<script>
  import { invoke } from "@tauri-apps/api/core";
  import { createEventDispatcher } from "svelte";
  import Modal from "$lib/components/Modal.svelte";
  import { faPlay } from "@fortawesome/free-solid-svg-icons";
  import { onMount } from "svelte";
  import Select from "$lib/components/Select.svelte";
  import Input from "$lib/components/Input.svelte";
  import Switch from "$lib/components/Switch.svelte";
  import Fa from "svelte-fa";

  const dispatch = createEventDispatcher();

  let enabled = false;
  let isSteamRunning = false;
  let settings = {};
  let hlaeSettings = {};
  let outputSettings = {};

  let startingDemo = "";

  let demos = [];

  let toggle = () => (enabled = !enabled);

  async function loadSettings() {
    settings = await invoke("load_settings");
    hlaeSettings = settings.hlae;
    outputSettings = settings.output;
  }

  async function loadDemos() {
    try {
      let resp = await invoke("load_demos");

      demos = resp.demos.map((demo) => demo.name);
    } catch (error) {
      alert(error);
    }
  }

  async function checkSteam() {
    isSteamRunning = await invoke("is_steam_running");
  }

  $: {
    if (enabled) {
      loadSettings();
      loadDemos();
      checkSteam();
    }

    if (!enabled) {
      dispatch("close");
    }
  }
</script>

<button class="btn btn--tert" on:click={toggle}>
  <Fa icon={faPlay} color={`var(--tert)`} />
  Launch TF2
</button>

<Modal color="tert" {toggle} {enabled}>
  {#if isSteamRunning}
    <h1>Launch TF2</h1>
    <div class="setting">
      <div class="settings__input-group settings__span">
        <Select
          title="Starting Demo"
          bind:value={startingDemo}
          tooltip={`The first demo to record.`}
          color="tert"
        >
          {#each demos as demo}
            <option value={demo}>{demo}</option>
          {/each}
        </Select>
      </div>
      {#if outputSettings.method === "sparklyfx"}
        <Input
          title="HLAE .exe Path"
          bind:value={hlaeSettings.hlae_path}
          color="tert"
        />
        <Input
          title="SparklyFX .dll Path"
          bind:value={hlaeSettings.sparklyfx_path}
          color="tert"
        />
      {/if}
      <div class="settings__input-group settings__span">
        <Input
          title="TF2 Launch Options"
          bind:value={hlaeSettings.launch_options}
          color="tert"
        />
      </div>
      <Switch
        title="64Bit"
        bind:value={hlaeSettings.use_64bit}
        tooltip="Launches with 64Bit TF2."
        color="tert"
      />
      <Switch
        title="Automatically playdemo"
        bind:value={hlaeSettings.playdemo}
        tooltip="Plays first demo on list as soon as it launches."
        color="tert"
      />
    </div>
  {:else}
    <h1>Steam is not running</h1>
    <p>Open Steam before launching TF2.</p>
  {/if}

  <div class="buttons" slot="footer">
    {#if isSteamRunning}
      <button on:click={toggle} class="cancel-btn">Cancel</button>
      <button on:click={() => console.log('Launch Once')} class="btn btn--sec"> Launch Once </button>
      <button on:click={() => console.log('Batch Record')} class="btn btn--pri"> Batch Record </button>
    {/if}
  </div>
</Modal>

<style lang="scss">
  
  .buttons {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1rem;
    padding-top: 1rem;
  }
</style>