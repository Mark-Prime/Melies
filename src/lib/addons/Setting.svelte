<script>
  // @ts-nocheck
  import Input from "../Input.svelte";
  import Switch from "../Switch.svelte";
  export let setting;
  export let defaultTitle;
  export let depth = 0;

  let title = setting.title || defaultTitle;
</script>

{#if setting.type === "string"}
  <Input
    title={title}
    bind:value={setting.value}
    tooltip={setting.tooltip}
    color={['pri', 'sec', 'tert'][depth % 3]}
  />
{:else if setting.type === "int"}
  <Input
    title={title}
    bind:value={setting.value}
    tooltip={setting.tooltip}
    type="number"
    color={['pri', 'sec', 'tert'][depth % 3]}
  />
{:else if setting.type === "bool" || setting.type === "toggle"}
  <Switch
    title={title}
    bind:value={setting.value}
    tooltip={setting.tooltip}
    color={['pri', 'sec', 'tert'][depth % 3]}
  />
{/if}