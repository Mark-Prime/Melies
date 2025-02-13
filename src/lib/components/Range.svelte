<script>
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();



  /** @type {{value: any, title?: any, key?: any, min?: number, max?: number, color?: string}} */
  let {
    value = $bindable(),
    title = null,
    key = title.toLowerCase().replace(/ /g, "_") || "range",
    min = 0,
    max = 255,
    color = "pri"
  } = $props();

  const change = () => dispatch("change", value);
</script>

<div class="input__range">
  <div>
    <label for={key}>{title}</label>
    <p>{value}</p>
  </div>
  <input
    type="range"
    id={key}
    bind:value
    onchange={change}
    {min}
    {max}
    style={`--color: var(--${color}); --color-con: var(--${color}-con);`}
  />
</div>

<style lang="scss">
  input {
    font-size: 1.5rem;
    width: 100%;
    color: var(--color-con);
    appearance: none;
    -webkit-appearance: none;
    outline: none;
    border: none;
    border-radius: 15px;
    padding: 0rem;

    --thumb-height: 1rem;
    --track-height: 0.125em;
    --track-color: var(--color);
    --brightness-hover: 180%;
    --brightness-down: 80%;
    --clip-edges: 0.125em;

    overflow: hidden;

    &:active {
      cursor: grabbing;
    }

    &:disabled {
      filter: grayscale(1);
      opacity: 0.3;
      cursor: not-allowed;
    }

    &::-webkit-slider-thumb {
      -webkit-appearance: none;
      width: 15px;
      height: 15px;
      border-radius: 50%;
      // background: var(--color-con);
      cursor: pointer;

      &:active {
        cursor: grabbing;
      }
    }
  }

  /* === WebKit specific styles === */
  input[type="range"],
  input[type="range"]::-webkit-slider-runnable-track,
  input[type="range"]::-webkit-slider-thumb {
    -webkit-appearance: none;
    transition: all ease 100ms;
    height: var(--thumb-height);
  }

  input[type="range"]::-webkit-slider-runnable-track,
  input[type="range"]::-webkit-slider-thumb {
    position: relative;
  }

  input[type="range"]::-webkit-slider-thumb {
    --thumb-radius: calc((var(--thumb-height) * 0.5) - 1px);
    --clip-top: calc((var(--thumb-height) - var(--track-height)) * 0.5 - 0.5px);
    --clip-bottom: calc(var(--thumb-height) - var(--clip-top));
    --clip-further: calc(100% + 1px);
    --box-fill: calc(-100vmax - var(--thumb-width, var(--thumb-height))) 0 0
      100vmax currentColor;

    width: var(--thumb-width, var(--thumb-height));
    background: linear-gradient(currentColor 0 0) scroll no-repeat left center /
      50% calc(var(--track-height) + 1px);
    background-color: currentColor;
    box-shadow: var(--box-fill);
    border-radius: var(--thumb-width, var(--thumb-height));

    filter: brightness(100%);
    clip-path: polygon(
      100% -1px,
      var(--clip-edges) -1px,
      0 var(--clip-top),
      -100vmax var(--clip-top),
      -100vmax var(--clip-bottom),
      0 var(--clip-bottom),
      var(--clip-edges) 100%,
      var(--clip-further) var(--clip-further)
    );
  }

  input[type="range"]:hover::-webkit-slider-thumb {
    filter: brightness(var(--brightness-hover));
    cursor: grab;
  }

  input[type="range"]:active::-webkit-slider-thumb {
    filter: brightness(var(--brightness-down));
    cursor: grabbing;
  }

  input[type="range"]::-webkit-slider-runnable-track {
    background: linear-gradient(var(--track-color) 0 0) scroll no-repeat center /
      100% calc(var(--track-height) + 1px);
  }

  input[type="range"]:disabled::-webkit-slider-thumb {
    cursor: not-allowed;
  }

  /* === Firefox specific styles === */
  input[type="range"],
  input[type="range"]::-moz-range-track,
  input[type="range"]::-moz-range-thumb {
    appearance: none;
    transition: all ease 100ms;
    height: var(--thumb-height);
  }

  input[type="range"]::-moz-range-track,
  input[type="range"]::-moz-range-thumb,
  input[type="range"]::-moz-range-progress {
    background: #fff0;
  }

  input[type="range"]::-moz-range-thumb {
    background: currentColor;
    border: 0;
    width: var(--thumb-width, var(--thumb-height));
    border-radius: var(--thumb-width, var(--thumb-height));
    cursor: grab;
  }

  input[type="range"]:active::-moz-range-thumb {
    cursor: grabbing;
  }

  input[type="range"]::-moz-range-track {
    width: 100%;
    background: var(--track-color);
  }

  input[type="range"]::-moz-range-progress {
    appearance: none;
    background: currentColor;
    transition-delay: 30ms;
  }

  input[type="range"]::-moz-range-track,
  input[type="range"]::-moz-range-progress {
    height: calc(var(--track-height) + 1px);
    border-radius: var(--track-height);
  }

  input[type="range"]::-moz-range-thumb,
  input[type="range"]::-moz-range-progress {
    filter: brightness(100%);
  }

  input[type="range"]:hover::-moz-range-thumb,
  input[type="range"]:hover::-moz-range-progress {
    filter: brightness(var(--brightness-hover));
  }

  input[type="range"]:active::-moz-range-thumb,
  input[type="range"]:active::-moz-range-progress {
    filter: brightness(var(--brightness-down));
  }

  input[type="range"]:disabled::-moz-range-thumb {
    cursor: not-allowed;
  }

  .input__range {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;

    & > div {
      display: flex;
      justify-content: space-between;
      gap: 0.5rem;
      width: 100%;
    }
  }

  p {
    margin: 0;
  }
</style>
