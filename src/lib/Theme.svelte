<script>
  import Collapse from "$lib/components/Collapse.svelte";
  import { invoke } from "@tauri-apps/api/core";
</script>

<div>
  <div class="setting">
    <div>
      <h4>Primary Colors</h4>
      <div class="colors">
        <div
          class="color_display tooltip"
          data-tooltip="pri"
          style="--color: var(--pri);"
        ></div>
        <div
          class="color_display tooltip"
          data-tooltip="pri_text"
          style="--color: var(--pri-text);"
        ></div>
        <div
          class="color_display tooltip"
          data-tooltip="pri_con"
          style="--color: var(--pri-con);"
        ></div>
        <div
          class="color_display tooltip"
          data-tooltip="pri_con_text"
          style="--color: var(--pri-con-text);"
        ></div>
      </div>
    </div>
    <div>
      <h4>Secondary Colors</h4>
      <div class="colors">
        <div
          class="color_display tooltip"
          data-tooltip="sec"
          style="--color: var(--sec);"
        ></div>
        <div
          class="color_display tooltip"
          data-tooltip="sec_text"
          style="--color: var(--sec-text);"
        ></div>
        <div
          class="color_display tooltip"
          data-tooltip="sec_con"
          style="--color: var(--sec-con);"
        ></div>
        <div
          class="color_display tooltip"
          data-tooltip="sec_con_text"
          style="--color: var(--sec-con-text);"
        ></div>
      </div>
    </div>
    <div>
      <h4>Tertiary Colors</h4>
      <div class="colors">
        <div
          class="color_display tooltip"
          data-tooltip="tert"
          style="--color: var(--tert);"
        ></div>
        <div
          class="color_display tooltip"
          data-tooltip="tert_text"
          style="--color: var(--tert-text);"
        ></div>
        <div
          class="color_display tooltip"
          data-tooltip="tert_con"
          style="--color: var(--tert-con);"
        ></div>
        <div
          class="color_display tooltip tooltip--left"
          data-tooltip="tert_con_text"
          style="--color: var(--tert-con-text);"
        ></div>
      </div>
    </div>
    <div>
      <h4>Error Colors</h4>
      <div class="colors">
        <div
          class="color_display tooltip"
          data-tooltip="err"
          style="--color: var(--err);"
        ></div>
        <div
          class="color_display tooltip"
          data-tooltip="err_text"
          style="--color: var(--err-text);"
        ></div>
        <div
          class="color_display tooltip"
          data-tooltip="err_con"
          style="--color: var(--err-con);"
        ></div>
        <div
          class="color_display tooltip"
          data-tooltip="err_con_text"
          style="--color: var(--err-con-text);"
        ></div>
      </div>
    </div>
    <div>
      <h4>Background Colors</h4>
      <div class="colors">
        <div
          class="color_display tooltip"
          data-tooltip="bg"
          style="--color: var(--bg);"
        ></div>
        <div
          class="color_display tooltip"
          data-tooltip="bg2"
          style="--color: var(--bg2);"
        ></div>
        <div
          class="color_display tooltip"
          data-tooltip="bg_text"
          style="--color: var(--bg-text);"
        ></div>
      </div>
    </div>
    <div>
      <h4>Team Colors</h4>
      <div class="colors">
        <div
          class="color_display tooltip"
          data-tooltip="red"
          style="--color: var(--red);"
        ></div>
        <div
          class="color_display tooltip"
          data-tooltip="blu"
          style="--color: var(--blu);"
        ></div>
      </div>
    </div>
  </div>
  <p>
    To install a new theme, click the button below and drag a theme.json file
    into this folder.
  </p>
  <button onclick={() => invoke("open_themes_folder")}>Install Theme</button>
</div>

<style>
  button {
    margin-bottom: 1rem;
  }

  .colors {
    display: flex;
    justify-content: space-evenly;
  }

  .color_display {
    width: 25px;
    height: 25px;
    border-radius: 50%;
    display: inline-block;
    margin: 0 0.5rem;
    display: relative;

    background-color: var(--color);
  }

  .setting {
    grid-template-columns: 1fr 1fr 1fr;
  }
</style>
