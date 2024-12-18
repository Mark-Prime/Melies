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
  let isRunning = false;
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

  async function launchOnce() {
    isRunning = true;

    console.log("saving settings", settings);

    await invoke("save_settings", {
      newSettings: JSON.stringify(settings),
    });

    console.log("launching tf2", settings);

    await invoke("launch_tf2", { demoName: startingDemo });

    isRunning = false;
  }

  async function batchRecord() {
    isRunning = true;

    console.log("saving settings", settings);

    await invoke("save_settings", {
      newSettings: JSON.stringify(settings),
    });

    console.log("launching tf2", settings);

    while (true) {
      let resp = await invoke("batch_record", { demoName: startingDemo });

      console.log(resp)

      if (resp === null || resp.complete) {
        break;
      }

      startingDemo = resp.next_demo;
      console.log("next demo", resp.next_demo);
    }

    isRunning = false;
  }

  onMount(() => {
    loadSettings();
  })

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

{#if !["svr", "svr.mov", "svr.mp4"].includes(outputSettings.method)}
  <button class="btn btn--tert btn__launch" on:click={toggle}>
    <Fa icon={faPlay} color={`var(--tert)`} />
    Launch TF2
  </button>

  <Modal color="tert" {toggle} {enabled}>
    {#if isSteamRunning}
      {#if !isRunning}
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
        <h1>Running Team Fortress 2</h1>
        {#if hlaeSettings.playdemo}
          <p>Playing demo: {startingDemo}</p>
        {/if}
      {/if}
    {:else}
      <h1>Steam is not running</h1>
      <p>Open Steam before launching TF2.</p>
    {/if}

    <div class="buttons" slot="footer">
      {#if isSteamRunning}
        <button on:click={toggle} class="cancel-btn">Cancel</button>
        <button on:click={launchOnce} class="btn btn--sec"> Launch Once </button>
        <button on:click={batchRecord} class="btn btn--pri" disabled={outputSettings.method !== "sparklyfx"}> Batch Record </button>
      {/if}
    </div>
  </Modal>
{/if}

<style lang="scss">
  .buttons {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1rem;
    padding-top: 1rem;
  }

  .btn__launch {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
</style>