<script>
// @ts-nocheck

  import { invoke } from "@tauri-apps/api/tauri"
  let settings = {};
  let output_settings = {};
  let recording_settings = {};

  async function loadSettings(){
    settings = JSON.parse(await invoke("load_settings"));
    output_settings = settings.output
    recording_settings = settings.recording
  }

  async function saveSettings(){
    let answer = await invoke("save_settings", {newSettings: JSON.stringify(settings)});
    console.log(answer);
  }
  
  loadSettings();
</script>

<h1>Settings</h1>

<div class="setting">
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">\tf Folder</label>
    <input bind:value={settings.tf_folder} id="tf_folder" class="settings__input"/>
  </div>
</div>

<div class="setting">
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={settings.safe_mode}>
      <span class="slider round"></span>
    </label>
    Prevent overwriting previously made VDMs.
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={settings.clear_events}>
      <span class="slider round"></span>
    </label>
    Clear events after running.
  </div>
</div>

<h2>Output</h2>

<div class="setting">
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Recording Method</label>
    <input bind:value={output_settings.method} id="tf_folder" class="settings__input"/>
  </div>
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Framerate</label>
    <input bind:value={output_settings.framerate} id="tf_folder" class="settings__input" type="number"/>
  </div>
</div>
<div class="setting">
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={output_settings.crosshair}>
      <span class="slider round"></span>
    </label>
    Display Crosshair
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={output_settings.hud}>
      <span class="slider round"></span>
    </label>
    Display HUD
  </div>
</div>
<div class="setting">
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={output_settings.text_chat}>
      <span class="slider round"></span>
    </label>
    Enable Text Chat
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={output_settings.snd_fix}>
      <span class="slider round"></span>
    </label>
    Attempt to fix sound issues.
  </div>
</div>
<div class="setting">
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={output_settings.voice_chat}>
      <span class="slider round"></span>
    </label>
    Enable Voice Chat
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={output_settings.lock}>
      <span class="slider round"></span>
    </label>
    Prevent in-game settings from changing.
  </div>
</div>

<h2>Recording</h2>

<div class="setting">
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Recording Method</label>
    <input bind:value={recording_settings.commands} id="tf_folder" class="settings__input"/>
  </div>
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Framerate</label>
    <input bind:value={recording_settings.end_commands} id="tf_folder" class="settings__input"/>
  </div>
</div>
<div class="setting">
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Delay before initial skip</label>
    <input bind:value={recording_settings.start_delay} id="tf_folder" class="settings__input" type="number"/>
  </div>
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Minimum ticks between clips</label>
    <input bind:value={recording_settings.minimum_ticks_between_clips} id="tf_folder" class="settings__input" type="number"/>
  </div>
</div>
<div class="setting">
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Ticks before bookmarks</label>
    <input bind:value={recording_settings.before_bookmark} id="tf_folder" class="settings__input" type="number"/>
  </div>
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Ticks after bookmarks</label>
    <input bind:value={recording_settings.after_bookmark} id="tf_folder" class="settings__input" type="number"/>
  </div>
</div>
<div class="setting">
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Ticks before killstreak per kill in streak</label>
    <input bind:value={recording_settings.before_killstreak_per_kill} id="tf_folder" class="settings__input" type="number"/>
  </div>
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Ticks after killstreak</label>
    <input bind:value={recording_settings.after_killstreak} id="tf_folder" class="settings__input" type="number"/>
  </div>
</div>
<div class="setting">
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Interval for rewind double taps</label>
    <input bind:value={recording_settings.interval_for_rewind_double_taps} id="tf_folder" class="settings__input" type="number"/>
  </div>
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Rewind amount upon double tap</label>
    <input bind:value={recording_settings.rewind_amount} id="tf_folder" class="settings__input" type="number"/>
  </div>
</div>
<div class="setting">
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Delay before initial skip</label>
    <input bind:value={recording_settings.start_delay} id="tf_folder" class="settings__input" type="number"/>
  </div>
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Minimum ticks between clips</label>
    <input bind:value={recording_settings.minimum_ticks_between_clips} id="tf_folder" class="settings__input" type="number"/>
  </div>
</div>
<div class="setting">
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={recording_settings.record_continuous}>
      <span class="slider round"></span>
    </label>
    Auto-record next demo when complete
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={recording_settings.auto_close}>
      <span class="slider round"></span>
    </label>
    Prevent idle hours
  </div>
</div>

<div class="setting">
  <a href="/">Home</a>
  <button on:click={saveSettings} class="Save">
    Save
  </button>
</div>

<style>
  .setting {
    display: flex;
    justify-content: space-between;
    gap: 1rem;
    margin: auto;
    margin-bottom: 1rem;
    width: 100%;
    max-width: 1000px;
    padding: 0rem 2rem;
  }

  .settings__input-group {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    width: 100%;
  }

  .settings__input-group > input {
    width: 100%;
  }

  .settings__switch {
    display: flex;
    align-items: center;
    gap: 1rem;
    width: 100%;
  }
</style>
  