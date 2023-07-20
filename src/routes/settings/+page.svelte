<script>
  // @ts-nocheck

  import { invoke } from "@tauri-apps/api/tauri"
  let settings = {};
  let output_settings = {};
  let recording_settings = {};

  async function loadSettings(){
    settings = await invoke("load_settings");
    output_settings = settings.output
    recording_settings = settings.recording
  }

  async function saveSettings(){
    let answer = await invoke("save_settings", {newSettings: JSON.stringify(settings)});
    console.log(answer);
    window.location = "/"
  }
  
  loadSettings();
</script>

<h1>Settings</h1>

<div class="setting">
  <div class="settings__input-group settings__span">
    <label for="tf_folder" class="settings__label">\tf Folder</label>
    <input bind:value={settings.tf_folder} id="tf_folder" class="settings__input input"/>
  </div>
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
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={settings.save_backups}>
      <span class="slider round"></span>
    </label>
    Save backups of events
  </div>
</div>

<h2>Output</h2>

<div class="setting">
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Recording Method</label>
    <select bind:value={output_settings.method} id="tf_folder" class="settings__input input--sec">
      <option value="h264">h264</option>
      <option value="jpeg">jpeg</option>
      <option value="none">Do not record</option>
    </select>
  </div>
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Framerate</label>
    <input bind:value={output_settings.framerate} id="tf_folder" class="settings__input input--sec" type="number"/>
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={output_settings.crosshair}>
      <span class="slider round slider--sec"></span>
    </label>
    Display Crosshair
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={output_settings.hud}>
      <span class="slider round slider--sec"></span>
    </label>
    Display HUD
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={output_settings.text_chat}>
      <span class="slider round slider--sec"></span>
    </label>
    Enable Text Chat
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={output_settings.snd_fix}>
      <span class="slider round slider--sec"></span>
    </label>
    Attempt to fix sound issues.
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={output_settings.voice_chat}>
      <span class="slider round slider--sec"></span>
    </label>
    Enable Voice Chat
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={output_settings.lock}>
      <span class="slider round slider--sec"></span>
    </label>
    Prevent in-game settings from changing.
  </div>
</div>

<h2>Recording</h2>

<div class="setting">
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Commands</label>
    <input bind:value={recording_settings.commands} id="tf_folder" class="settings__input input--tert"/>
  </div>
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">End Commands</label>
    <input bind:value={recording_settings.end_commands} id="tf_folder" class="settings__input input--tert"/>
  </div>
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Delay before initial skip</label>
    <input bind:value={recording_settings.start_delay} id="tf_folder" class="settings__input input--tert" type="number"/>
  </div>
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Minimum ticks between clips</label>
    <input bind:value={recording_settings.minimum_ticks_between_clips} id="tf_folder" class="settings__input input--tert" type="number"/>
  </div>
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Ticks before bookmarks</label>
    <input bind:value={recording_settings.before_bookmark} id="tf_folder" class="settings__input input--tert" type="number"/>
  </div>
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Ticks after bookmarks</label>
    <input bind:value={recording_settings.after_bookmark} id="tf_folder" class="settings__input input--tert" type="number"/>
  </div>
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Ticks before killstreak per kill in streak</label>
    <input bind:value={recording_settings.before_killstreak_per_kill} id="tf_folder" class="settings__input input--tert" type="number"/>
  </div>
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Ticks after killstreak</label>
    <input bind:value={recording_settings.after_killstreak} id="tf_folder" class="settings__input input--tert" type="number"/>
  </div>
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Interval for rewind double taps</label>
    <input bind:value={recording_settings.interval_for_rewind_double_taps} id="tf_folder" class="settings__input input--tert" type="number"/>
  </div>
  <div class="settings__input-group">
    <label for="tf_folder" class="settings__label">Rewind amount upon double tap</label>
    <input bind:value={recording_settings.rewind_amount} id="tf_folder" class="settings__input input--tert" type="number"/>
  </div>
  <div class="settings__input-group">
    <label for="FOV" class="settings__label">FOV</label>
    <input bind:value={recording_settings.fov} id="FOV" class="settings__input input--tert" type="number"/>
  </div>
  <div class="settings__input-group">
    <label for="viewodel_fov" class="settings__label">Viewmodel FOV</label>
    <input bind:value={recording_settings.viewmodel_fov} id="viewodel_fov" class="settings__input input--tert" type="number"/>
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={recording_settings.record_continuous}>
      <span class="slider round slider--tert"></span>
    </label>
    Automatically record next demo when current demo completes
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={recording_settings.auto_close}>
      <span class="slider round slider--tert"></span>
    </label>
    Prevent idle hours
  </div>
  <div class="settings__switch">
    <label class="switch">
      <input type="checkbox" bind:checked={recording_settings.auto_suffix}>
      <span class="slider round slider--tert"></span>
    </label>
    Automatically label videos
  </div>
</div>

<div class="setting">
  <a href="/" class="cancel-btn">Cancel</a>
  <button on:click={saveSettings} class="Save">
    Save
  </button>
</div>

<style lang="scss">
  .setting {
    display: grid;
    grid-template-columns: 1fr 1fr;
    justify-content: space-between;
    gap: 1rem;
    margin: auto;
    margin-bottom: 2rem;
    width: 100%;
    max-width: 1000px;
    padding: 0rem 2rem;
  }

  .settings {
    &__input-group {
      display: flex;
      flex-direction: column;
      align-items: flex-start;
      width: 100%;

      & > input, & > select {
        width: 100%;
      }
    }

    &__switch {
      display: flex;
      align-items: center;
      gap: 1rem;
      width: 100%;
    }

    &__span {
      grid-column-start: 1;
      grid-column-end: span 2;
    }
  }
</style>
  