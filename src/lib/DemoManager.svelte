<script>
  import { run } from 'svelte/legacy';

  import { createEventDispatcher } from "svelte";
  import {
    faFileCircleMinus,
    faFileCirclePlus,
    faListCheck,
    faSquare,
    faSquareCheck,
    faTrash,
    faPen,
  } from "@fortawesome/free-solid-svg-icons";
  import Modal from "$lib/components/Modal.svelte";
  import Fa from "svelte-fa";
  import { invoke } from "@tauri-apps/api/core";
  import dayjs from "dayjs";
  import tickToTime from "$lib/composables/tickToTime.js";
  import { faCheck, faXmark } from "@fortawesome/free-solid-svg-icons";
  import Input from "$lib/components/Input.svelte";
  import Select from "$lib/components/Select.svelte";

  let settings = {};

  let enabled = $state(false);
  let renameModalEnabled = $state(false);
  let resp = $state({ loaded: false });
  let mapList = $state([]);
  let anySelected = $state(false);

  let renameNameInput = $state();
  let renameDemo = $state(null);
  let renameDefault = "{date}_{time}_{map}_{ticks}";

  let searchFilter = $state("");
  let mapFilter = $state("");
  let vdmFilter = $state(null);


  async function loadDemos() {
    settings = await invoke("load_settings");

    renameDefault = settings.demo_manager?.default_name || "{date}_{time}_{map}_{ticks}";

    try {
      resp = await invoke("load_demos");
      let collator = new Intl.Collator(undefined, {
        numeric: true,
        sensitivity: "base",
      });

      mapList = [...new Set(resp.demos.map((demo) => demo.header.map))].sort(
        (a, b) => collator.compare(a, b)
      );
    } catch (error) {
      alert(error);
    }
  }

  function toggle() {
    enabled = !enabled;
  }

  async function delete_vdm(fileName) {
    await invoke("delete_vdm", { fileName: fileName });

    loadDemos();
  }

  async function create_vdm(fileName) {
    await invoke("create_vdm", { fileName: fileName });

    loadDemos();
  }

  async function delete_demo(fileName, hasVdm) {
    let answer = !settings.demo_manager.confirm_deletion || await confirm(
      `Are you sure you want to delete this demo?${hasVdm ? `\nThis will also deleted the associated vdm.` : ``}`
    );

    if (!answer) {
      return;
    }

    await invoke("delete_demo", { fileName: fileName });

    loadDemos();
  }

  function filter(demo) {
    if (mapFilter && demo.header.map != mapFilter) {
      return false;
    }

    if (vdmFilter != null && vdmFilter != demo.hasVdm) {
      return false;
    }

    let search = searchFilter.toLowerCase();

    let isInName = demo.name.toLowerCase().includes(search);
    let isInNick = demo.header.nick.toLowerCase().includes(search);
    let isInServer = demo.header.server.toLowerCase().includes(search);
    let isInMap = demo.header.map.toLowerCase().includes(search);

    return isInName || isInNick || isInServer || isInMap;
  }

  function refreshList() {
    anySelected = checkSelected();
    resp = resp;
  }

  function checkSelected() {
    if (!resp.demos) {
      return false;
    }

    for (let demo of resp.demos) {
      if (demo.selected) {
        return true;
      }
    }

    return false;
  }

  function toggleSelected(demo) {
    demo.selected = !demo.selected;

    refreshList();
  }

  function selectAll(val = true) {
    for (let demo of resp.demos) {
      demo.selected = val;
    }

    refreshList();
  }

  function createVdms() {
    let selected = resp.demos.filter((demo) => demo.selected);

    for (let demo of selected) {
      if (demo.hasVdm) {
        continue;
      }

      create_vdm(demo.name);
    }

    refreshList();
  }

  function deleteVdms() {
    let selected = resp.demos.filter((demo) => demo.selected);

    for (let demo of selected) {
      if (!demo.hasVdm) {
        continue;
      }

      delete_vdm(demo.name);
    }

    refreshList();
  }

  async function deleteDemos() {
    let answer = !settings.demo_manager.confirm_deletion || await confirm(
      `Are you sure you want to delete these demos?\nThis will also deleted the associated vdms.`
    );

    if (!answer) {
      return;
    }

    let selected = resp.demos.filter((demo) => demo.selected);

    for (let demo of selected) {
      await invoke("delete_demo", { fileName: demo.name });
    }

    loadDemos();

    anySelected = false;
  }

  function openRenameModal(demo = null) {
    renameModalEnabled = true;

    renameDemo = demo;
    renameNameInput = renameDefault;

    console.log(demo);
  }

  function replaceInputName(demo) {
    let input = renameNameInput
      .replace("{nickname}", demo?.header?.nick)
      .replace("{ticks}", demo.header.ticks)
      .replace("{server}", demo?.header?.server)
      .replace("{map}", demo?.header?.map)
      .replace("{date}", dayjs.unix(demo?.metadata?.created?.secs_since_epoch).format("YYYY-MM-DD"))
      .replace("{time}", dayjs.unix(demo?.metadata?.created?.secs_since_epoch).format("HH-mm-ss"));

    return input;
  }

  async function renameFile(demo) {
    if (!renameNameInput) {
      return;
    }

    let demoPath = settings.tf_folder + demo.name;
    let newPath = demoPath.split("\\").slice(0, -1).join("\\") + "\\" + replaceInputName(demo) + ".dem";

    let payload = {
      oldPath: demoPath,
      newPath: newPath,
    };

    console.log({demo, payload});
    
    await invoke("rename_file", payload);

    if (demo.hasVdm) {
      let vdmPayload = {
        oldPath: demoPath.replace(".dem", ".vdm"),
        newPath: newPath.replace(".dem", ".vdm"),
      };
    
      await invoke("rename_file", vdmPayload);
    }

    renameModalEnabled = false;
  }

  function renameDemos() {
    let demos = [];

    if (renameDemo) {
      demos = [renameDemo];
    } else {
      demos = resp.demos.filter((demo) => demo.selected);
    }

    let promises = [];

    for (let demo of demos) {
      promises.push(renameFile(demo));
    } 

    Promise.all(promises).then(() => {
      loadDemos();
    });
  }
  run(() => {
    let filters = { map: mapFilter, search: searchFilter, vdm: vdmFilter };

    refreshList();
  });
</script>

<button class="btn btn--sec" onclick={toggle}>
  <Fa icon={faListCheck} color={`var(--sec)`} />
  Demo Manager
</button>

<Modal color="sec" {toggle} {enabled} large tall on:open={loadDemos}>
  <h1>Demo Manager</h1>
  {#if resp.loaded}
    <div class="filters">
      <Input title="Search" color="sec" bind:value={searchFilter} />
      <Select title="Map" color="sec" bind:value={mapFilter}>
        <option value=""></option>
        {#each mapList as map}
          <option value={map}>{map}</option>
        {/each}
      </Select>
      <Select title="VDM Status" color="sec" bind:value={vdmFilter}>
        <option value={null}></option>
        <option value={true}>Has VDM</option>
        <option value={false}>Doesn't have VDM</option>
      </Select>
    </div>
    <div>
      <button onclick={selectAll}> Select All </button>
      <button disabled={!anySelected} onclick={() => selectAll(false)}>
        Deselect All
      </button>
      <button disabled={!anySelected} onclick={createVdms}>
        <Fa icon={faFileCirclePlus} color={`var(--pri)`} />
        Create VDMs
      </button>
      <button disabled={!anySelected} onclick={deleteVdms}>
        <Fa icon={faFileCircleMinus} color={`var(--pri)`} />
        Delete VDMs
      </button>
      <button disabled={!anySelected} onclick={deleteDemos}>
        <Fa icon={faTrash} color={`var(--pri)`} />
        Delete Demos
      </button>
      <button
        disabled={!anySelected}
        onclick={() => (openRenameModal())}
      >
        <Fa icon={faPen} color={`var(--pri)`} />
        Mass Rename
      </button>
    </div>
    <table>
      <thead>
        <tr>
          <th>Name</th>
          <th>Player Nickname</th>
          <th>Length</th>
          <th>Server</th>
          <th>Map</th>
          <th>Created Date</th>
          <th
            class="tooltip tooltip--left"
            data-tooltip={`Does the demo have a vdm?`}
            style="--kills: 0;"
          >
            VDM
          </th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each resp.demos.filter(filter) as demo}
          <tr class={"table_row " + (demo.hasVdm && "demo--hasvdm") + (demo.selected ? " demo--selected" : "")}>
            <td id={demo.name}>
              {demo.name}
            </td>
            <td>{demo.header.nick}</td>
            <td
              class="tooltip"
              data-tooltip={`${demo.header.ticks} ticks`}
              style="--kills: 0;"
            >
              {tickToTime(demo.header.ticks)}
            </td>
            <td>{demo.header.server}</td>
            <td>{demo.header.map}</td>
            <td>
              {dayjs
                .unix(demo.metadata.created.secs_since_epoch)
                .format("MMM DD, YYYY")}
            </td>
            <td class="table__has-vdm">
              {#if demo.hasVdm}
                <span
                  class="tooltip tooltip--left"
                  data-tooltip={`This demo has a VDM.`}
                  style="--kills: 0;"
                >
                  <Fa icon={faCheck} color={`var(--sec)`} />
                </span>
              {:else}
                <span
                  class="tooltip tooltip--left"
                  data-tooltip={`This demo does not have a VDM.`}
                  style="--kills: 0;"
                >
                  <Fa icon={faXmark} color={`var(--tert)`} />
                </span>
              {/if}
            </td>
            <td>
              <a
                name="#{demo.name}-select"
                class="icon checkbox tooltip tooltip--left"
                data-tooltip={`Select demo.`}
                onclick={() => toggleSelected(demo)}
                onkeydown={() => toggleSelected(demo)}
                tabindex="-1"
                role="button"
                href="/"
              >
                {#if demo.selected}
                  <Fa icon={faSquareCheck} color={`var(--pri)`} />
                {:else}
                  <Fa
                    icon={faSquare}
                    color={demo.hasVdm ? `var(--sec)` : `var(--tert)`}
                  />
                {/if}
              </a>
              <a
                name="#{demo.name}-rename"
                class="icon checkbox tooltip tooltip--left"
                data-tooltip={`Rename demo.`}
                onclick={() => openRenameModal(demo)}
                onkeydown={() => openRenameModal(demo)}
                tabindex="-1"
                role="button"
                href="/"
              >
                <Fa
                  icon={faPen}
                  color={demo.hasVdm ? `var(--sec)` : `var(--tert)`}
                />
              </a>
              {#if demo.hasVdm}
                <a
                  name="#{demo.name}-delete_vdm"
                  class="icon checkbox tooltip tooltip--left"
                  data-tooltip={`Delete VDM.`}
                  onclick={async () => await delete_vdm(demo.name)}
                  onkeydown={async () => await delete_vdm(demo.name)}
                  tabindex="-1"
                  role="button"
                  href="/"
                >
                  <Fa icon={faFileCircleMinus} color={`var(--err)`} />
                </a>
              {:else}
                <a
                  name="#{demo.name}-create_vdm"
                  class="icon checkbox tooltip tooltip--left"
                  data-tooltip={`Create blank VDM.`}
                  onclick={async () => await create_vdm(demo.name)}
                  onkeydown={async () => await create_vdm(demo.name)}
                  tabindex="-1"
                  role="button"
                  href="/"
                >
                  <Fa icon={faFileCirclePlus} color="var(--pri)" />
                </a>
              {/if}
              <a
                name="#{demo.name}-delete"
                class="icon checkbox tooltip tooltip--left"
                data-tooltip={`Delete this demo.`}
                style="--kills: 0;"
                onclick={async () => await delete_demo(demo.name, demo.hasVdm)}
                onkeydown={async () =>
                  await delete_demo(demo.name, demo.hasVdm)}
                tabindex="-1"
                role="button"
                href="/"
              >
                <Fa icon={faTrash} color={`var(--err)`} />
              </a>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
    <Modal
      color="tert"
      toggle={() => (renameModalEnabled = !renameModalEnabled)}
      enabled={renameModalEnabled}
    >
      <h4>{renameDemo ? `Rename ${renameDemo.name}` : `Mass Rename Demos`}</h4>
      <Input
        title="Rename Demo"
        type="text"
        color="tert"
        bind:value={renameNameInput}
      />
      <p>
        Metadata elements:
        <span
          class="tooltip"
          data-tooltip={`The player's nickname\nExample: ${renameDemo?.header?.nick || "JoseGonzales2007"}`}
          style="--kills: 1;"
        >
          {`{nickname}`},
        </span>
        <span
          class="tooltip"
          data-tooltip={`The number of ticks in the demo\nExample: ${renameDemo?.header?.ticks || 12345}`}
          style="--kills: 1;"
        >
          {`{ticks}`},
        </span>
        <span
          class="tooltip"
          data-tooltip={`The server the demo was played on\nExample: ${renameDemo?.header?.server || "skial.harvest.247"}`}
          style="--kills: 1;"
        >
          {`{server}`},
        </span>
        <span
          class="tooltip"
          data-tooltip={`The map the demo was played on\nExample: ${renameDemo?.header?.map || "koth_harvest_final"}`}
          style="--kills: 1;"
        >
          {`{map}`},
        </span>
        <span
          class="tooltip"
          data-tooltip={`The date the demo was created\nExample: ${dayjs.unix(renameDemo?.metadata?.created?.secs_since_epoch).format("YYYY-MM-DD") || "2022-01-01"}`}
          style="--kills: 1;"
        >
          {`{date}`},
        </span>
        <span
          class="tooltip"
          data-tooltip={`The time the demo was created\nExample: ${dayjs.unix(renameDemo?.metadata?.created?.secs_since_epoch).format("HH-mm-ss") || "03-10-35"}`}
          style="--kills: 1;"
        >
          {`{time}`}
        </span>
      </p>
      <div class="buttons">
        <button
          class="cancel-btn"
          onclick={() => (renameModalEnabled = !renameModalEnabled)}
        >
          Cancel
        </button>
        <button onclick={renameDemos} disabled={!renameNameInput}>
          {renameDemo ? `Rename` : `Mass Rename`}
        </button>
      </div>
    </Modal>
  {/if}
</Modal>

<style lang="scss">
  .buttons {
    width: 100%;
    display: flex;
    gap: 1rem;
    margin-top: 1rem;

    & > * {
      width: 100%;
    }
  }

  .filters {
    display: flex;
    gap: 1rem;
    padding: 0;
    margin: 0 0 1rem 0;
  }

  .icon {
    font-size: large;
    cursor: pointer;
  }

  .tooltip--left::before {
    left: auto;
    right: calc(100% - 2rem);
  }

  .tooltip--left::after {
    left: auto;
    right: calc(100% - 0.8rem);
  }

  .btn {
    display: flex;
    gap: 0.5rem;
  }

  table {
    border-collapse: separate;
    border-spacing: 0 2px;
    width: 100%;
  }

  th {
    text-align: left;
    white-space: nowrap;
  }

  .table_row {
    font-size: small;
    font-family: "Source Code Pro", monospace;
    color: var(--tert-con-text);
    padding: 0 1rem;
    margin: 2px 0;
    border-radius: 5px;

    transition: all 0.2s;

    background-color: var(--bg2);

    &.demo--hasvdm {
      & .tooltip:hover {
        color: var(--sec);
      }

      & > td {
        color: var(--sec-con-text);
        border-color: var(--sec-con);
      }
    }

    &:not(.demo--hasvdm){
      & .tooltip:hover {
        color: var(--tert);
      }
    }

    &.demo--selected > td {
      border-color: var(--pri);
    }

    &:hover {
      &:not(.demo--hasvdm) {
        filter: drop-shadow(0px 0px 3px var(--tert));
      }

      &.demo--hasvdm {
        filter: drop-shadow(0px 0px 3px var(--sec));
      }

      & > td {
        border-color: var(--tert-con-text);
      }

      &.demo--hasvdm > td {
        border-color: var(--sec-con-text);
      }
    }

    & > td {
      text-align: left;
      white-space: nowrap;
      border: 1px solid var(--tert-con);
      border-left-width: 0px;
      border-right-width: 0px;
      padding: 0.3rem 0.25rem;

      &:first-of-type {
        border-left-width: 1px;
        border-radius: 5px 0 0 5px;
        padding-left: 0.5rem;
      }

      &:last-of-type {
        border-right-width: 1px;
        border-radius: 0 5px 5px 0;
        padding-right: 0.5rem;
      }
    }
  }
</style>
