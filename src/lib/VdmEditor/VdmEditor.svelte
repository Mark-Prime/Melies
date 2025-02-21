<script>
  import { invoke } from "@tauri-apps/api/core";
  import { faPenFancy } from "@fortawesome/free-solid-svg-icons";
  import Modal from "$lib/components/Modal.svelte";
  import Fa from "svelte-fa";
  import VdmAction from "./VdmAction.svelte";
  import ActionEdit from "./ActionEdit.svelte";
  import AddAction from "./AddAction.svelte";
  import dayjs from "dayjs";

  let enabled = $state(false);
  let resp = $state({});
  let vdm = $state([]);
  let vdmName = $state("");

  let selectedAction = $state(0);

  function toggle() {
    enabled = !enabled;

    resp = {};
    vdm = [];
    vdmName = "";
    selectedAction = 0;
  }

  async function loadVdms() {
    resp = await invoke("load_vdms");
  }

  async function loadVdm(name) {
    vdmName = name;
    vdm = await invoke("load_vdm", { name });
    $inspect(vdm);
  }

  async function saveVdm(name) {
    resp = await invoke("save_vdm", { name, vdm });
    $inspect(resp);
    toggle();
  }

  function deleteAction(index) {
    vdm.splice(index, 1);

    vdm = vdm;

    if (selectedAction > vdm.length - 1) {
      selectedAction = vdm.length - 1;
    }
  }

  function select(index) {
    selectedAction = index;
  }

  function updateAction(index, newAction) {
    vdm[index] = newAction.detail;
  }

  function addAction(newAction) {
    selectedAction = vdm.length;

    vdm = [...vdm, ...newAction.detail];
  }
</script>

<button class="btn" onclick={toggle}>
  <Fa icon={faPenFancy} color={`var(--pri)`} />
  VDM Editor
</button>

<Modal color="pri" {toggle} {enabled} tall grow on:open={loadVdms}>
  {#snippet footer()}
    <div >
      {#if resp.loaded && resp.vdms && !vdmName}
        <div class="buttons">
          <button class="btn btn--cancel" onclick={toggle}>Cancel</button>
          <button class="btn" disabled>New VDM</button>
        </div>
      {:else if vdm}
        <div class="buttons">
          <button class="btn btn--cancel" onclick={toggle}>Cancel</button>
          <button class="btn" onclick={() => saveVdm(vdmName)}>Save</button>
        </div>
      {/if}
    </div>
  {/snippet}
  {#if resp.loaded && resp.vdms && !vdmName}
    <h1>Load VDM</h1>
    {#each resp.vdms as vdm}
      <div class={"demo"}>
        <p>{vdm.name}</p>
        <p>{dayjs.unix(vdm.metadata.created.secs_since_epoch).format('MMM DD, YYYY')}</p>
        <div class="add_demo">
          <button class="btn" onclick={() => loadVdm(vdm.name)}> Edit </button>
        </div>
      </div>
    {/each}
  {:else if vdm}
    <div class="edit-vdm">
      <h1>{vdmName}</h1>
      <div class="actions">
        <div class="actions--display">
          {#each vdm as action, i}
            <VdmAction
              {action}
              index={i}
              selected={i === selectedAction}
              on:select={() => select(i)}
              on:update={(newAction) => updateAction(i, newAction)}
            />
          {/each}
          <AddAction on:add={addAction} />
        </div>
        <div>
          <ActionEdit
            action={vdm[selectedAction]}
            on:change={(newAction) => updateAction(selectedAction, newAction)}
            on:delete={() => deleteAction(selectedAction)}
          />
        </div>
      </div>
    </div>
  {:else}
    <div class="loading">
      <div class="loadingio-spinner-dual-ball-gstkvx2ybq5">
        <div class="ldio-h6cxzkuee3g">
          <div></div>
          <div></div>
          <div></div>
        </div>
      </div>
      <h4>Loading VDMs...</h4>
    </div>
  {/if}
</Modal>

<style lang="scss">
  h1 {
    margin-top: 0.5rem;
  }

  .edit-vdm {
    display: grid;
    grid-template-columns: 1fr;
    grid-template-rows: min-content 1fr min-content;
    height: 100%;
    max-width: 770px;
  }

  .actions {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.5rem;

    &--display {
      display: flex;
      flex-direction: column;
      gap: 0.2rem;

      max-height: calc(770px - 7rem);
      overflow-y: auto;

      padding-right: 0.25rem;

      /* width */
      &::-webkit-scrollbar {
        width: 8px;
      }

      /* Track */
      &::-webkit-scrollbar-track {
        background: var(--color);
        border-radius: 8px;
        overflow: hidden;
      }

      /* Handle */
      &::-webkit-scrollbar-thumb {
        background: var(--color-con);
        border-radius: 8px;
      }
    }

    & div:last-child {
      padding-left: 0.25rem;
    }
  }

  .buttons {
    display: flex;
    gap: 1rem;
    justify-content: center;
    margin-top: 1rem;
  }

  .btn {
    display: flex;
    gap: 0.5rem;
  }

  .demo {
    font-size: small;
    padding: 0.3rem 0.5rem;
    margin: 2px 0;
    font-family: "Source Code Pro", monospace;
    color: var(--pri-con-text);
    border: 1px solid var(--pri-con);
    border-radius: 5px;

    display: grid;
    grid-template-columns: 2fr 1fr 3rem;
    white-space: nowrap;

    transition: all 0.2s;

    height: fit-content;

    & p {
      text-align: left;
      padding: 0;
      margin: 0;
    }
  }

  .add_demo {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 1px;
    padding: 0;
    margin: 0;

    & > button {
      font-size: 12px;
      padding: 0.3rem 0.7rem;
      margin: 0;
      // height: 100%;
      border-radius: 5px;
      width: fit-content;
    }
  }

  .loading {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    position: relative;

    & h4 {
      position: absolute;
      bottom: 1rem;
    }
  }

  @keyframes ldio-h6cxzkuee3g-o {
    0% {
      opacity: 1;
      transform: translate(0, 0);
    }
    49.99% {
      opacity: 1;
      transform: translate(100px, 0);
    }
    50% {
      opacity: 0;
      transform: translate(100px, 0);
    }
    100% {
      opacity: 0;
      transform: translate(0, 0);
    }
  }
  @keyframes ldio-h6cxzkuee3g {
    0% {
      transform: translate(0, 0);
    }
    50% {
      transform: translate(100px, 0);
    }
    100% {
      transform: translate(0, 0);
    }
  }
  .ldio-h6cxzkuee3g div {
    position: absolute;
    width: 60px;
    height: 60px;
    border-radius: 50%;
    top: 40px;
    left: 20px;
  }
  .ldio-h6cxzkuee3g div:nth-child(1) {
    background: var(--pri-con);
    animation: ldio-h6cxzkuee3g 0.6097560975609756s linear infinite;
    animation-delay: -0.3048780487804878s;
  }
  .ldio-h6cxzkuee3g div:nth-child(2) {
    background: var(--sec-con);
    animation: ldio-h6cxzkuee3g 0.6097560975609756s linear infinite;
    animation-delay: 0s;
  }
  .ldio-h6cxzkuee3g div:nth-child(3) {
    background: var(--pri-con);
    animation: ldio-h6cxzkuee3g-o 0.6097560975609756s linear infinite;
    animation-delay: -0.3048780487804878s;
  }
  .loadingio-spinner-dual-ball-gstkvx2ybq5 {
    width: 200px;
    height: 200px;
    display: inline-block;
    overflow: hidden;
    background: transparent;
  }
  .ldio-h6cxzkuee3g {
    width: 100%;
    height: 100%;
    position: relative;
    transform: translateZ(0) scale(1);
    backface-visibility: hidden;
    transform-origin: 0 0; /* see note above */
  }
  .ldio-h6cxzkuee3g div {
    box-sizing: content-box;
  }
</style>
