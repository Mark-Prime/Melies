<script>
  // @ts-nocheck
  import Input from "$lib/components/Input.svelte";
  import Switch from "$lib/components/Switch.svelte";
  /** @type {{setting: any, defaultTitle: any, depth?: number}} */
  let { setting = $bindable(), defaultTitle, depth = 0 } = $props();

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