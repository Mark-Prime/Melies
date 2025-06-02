<script>
  import { run } from 'svelte/legacy';

  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  /** @type {{color?: string, enabled: any, toggle: any, large?: boolean, tall?: boolean, small?: boolean, grow?: boolean, wide?: boolean, max_width?: any, header?: import('svelte').Snippet, children?: import('svelte').Snippet, footer?: import('svelte').Snippet}} */
  let {
    color = "pri",
    enabled,
    toggle,
    large = false,
    tall = false,
    small = false,
    grow = false,
    wide = false,
    min_width = null,
    min_height = null,
    max_width = null,
    max_height = null,
    width = null,
    height = null,
    header,
    children,
    footer,
    tabHeaders,
    tabs
  } = $props();

  let activeTab = $state(0);

  const tabClicked = (i) => dispatch("tabClicked", i);

  run(() => {
    activeTab = 0;

    if (enabled) {
      dispatch("open");
      return;
    }

    dispatch("close");
  });
</script>

{#if enabled}
  <div class="modal">
    <a class="modal__background" onclick={() => toggle()} href="/" aria-label="Close"> </a>
    <div class="modal__card-container">
      {#if tabHeaders}
        <div class="modal__tabs">
          {#each tabHeaders as tab, i}
            {#if typeof tab === "string" || tab?.enabled}
              <button
                class={`modal__tab btn--${['pri', 'sec', 'tert'][i % 3]}`}
                class:modal__tab--active={tab === tabHeaders[activeTab]}
                onclick={() => { activeTab = i; tabClicked(i) }}
              >
                {tab.title || tab}
              </button>
            {/if}
          {/each}
        </div>
      {/if}
      <div
        class="modal__card"
        class:modal__card--large={large}
        class:modal__card--small={small}
        class:modal__card--tall={tall}
        class:modal__card--grow={grow}
        class:modal__card--wide={wide}
        style={`--color: var(--${color});
          --color-con: var(--${color}-con);
          ${tabs && `width: 75vw`};
          ${min_width && `min-width: ${min_width}`};
          ${min_height && `min-height: ${min_height}`};
          ${max_width && `max-width: ${max_width}`};
          ${max_height && `max-height: ${max_height}`};
          ${width && `width: ${width}`};
          ${height && `height: ${height}`};
          ${tabs && `min-height: calc(${tabHeaders.length * 36}px + 1.5rem)`};`
        }
      >
        {#if header}
          <div class="modal__header">
            {@render header?.()}
          </div>
        {/if}
        {#if tabs}
          <div
            class="modal__content"
          >
            <h2>{tabHeaders[activeTab].title || tabHeaders[activeTab]}</h2>
            {@render tabs[activeTab]?.()}
          </div>
        {/if}
        {#if !tabs}
          <div class="modal__content">
            {@render children?.()}
          </div>
        {/if}
        {#if footer}
          <div class="modal__footer">
            {@render footer?.()}
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style lang="scss">
  .modal {
    position: fixed;
    width: 100%;
    height: 100%;
    left: 0;
    top: 0;
    z-index: 1000;
    display: flex;
    justify-content: center;
    align-items: center;
    overflow: hidden;

    &__header {
      position: sticky;
      top: 0;
      background-color: var(--bg);
      padding: 1rem;
      padding-bottom: 1px;
      z-index: 99;
    }

    &__footer {
      position: sticky;
      bottom: -2px;
      padding: 1rem;
      padding-top: 1px;
      background-color: var(--bg);
      z-index: 99999;
    }

    &__content {
      padding: 1rem;
      max-width: inherit;
    }

    &__tabs {
      position: absolute;
      width: fit-content;
      display: flex;
      flex-direction: column;
      align-items: flex-end;
      gap: 0.5rem;
      top: 1rem;
      right: 100%;
      z-index: 1001;
    }

    &__tab {
      position: relative;
      width: fit-content;
      white-space: nowrap;
      border-right: 0;
      border-radius: 8px 0 0 8px;
      transition: all 0.2s;
      padding-right: 0.25rem;

      font-size: x-small;

      &--active {
        padding-right: 2rem;
      }

      &:hover {
        cursor: pointer;
        padding-right: 3rem;
      }
    }

    &__card-container {
      position: relative;
      z-index: 1000;
    }

    &__card {
      height: fit-content;
      width: fit-content;
      width: 100%;
      max-width: min(calc(100vw - 2rem), 800px);
      max-height: min(calc(100vh - 2rem), 800px);
      background-color: var(--bg);
      border-radius: 8px;
      border: 1px solid var(--color-con);
      padding: 0;
      position: relative;
      z-index: 1000;
      overflow-y: auto;
      overflow-x: hidden;

      display: flex;
      flex-direction: column;
      justify-content: space-between;

      &--large {
        max-width: min(calc(100vw - 2rem), 1680px);
        max-height: min(calc(100vh - 2rem), 900px);
      }
      
      &--grow {
        max-width: min(calc(100vw - 2rem), 1680px);
        max-height: min(calc(100vh - 2rem), 900px);

        width: fit-content;
      }
      
      &--wide {
        max-width: min(calc(100vw - 2rem), 1680px);
        max-height: min(calc(100vh - 2rem), 900px);

        width: 100%;
      }

      &--small {
        max-width: min(calc(100vw - 2rem), 500px);
        max-height: min(calc(100vh - 2rem), 500px);
      }

      &--tall {
        height: 100%;
      }

      /* width */
      &::-webkit-scrollbar {
        width: 8px;
      }

      /* Track */
      &::-webkit-scrollbar-track {
        background: var(--color);
        border-radius: 0 8px 8px 0;
        overflow: hidden;
      }

      /* Handle */
      &::-webkit-scrollbar-thumb {
        background: var(--color-con);
        border-radius: 0 8px 8px 0;
      }
    }

    &__background {
      position: fixed;
      background-color: rgba(0, 0, 0, 0.6);
      width: 100%;
      height: 100%;
      left: 0;
      top: 0;
      z-index: 998;
      backdrop-filter: blur(5px);
    }
  }
</style>
