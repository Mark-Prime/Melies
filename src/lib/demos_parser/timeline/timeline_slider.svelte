<script>
  // @ts-nocheck
  /** @type {{left: any, right: any}} */
  let { left, right } = $props();
</script>

<div class="settings__input-group">
  <div class="multi-range-slider">
    <input type="range" id="input-left" min="0" max="99" value={left} />
    <input type="range" id="input-right" min="1" max="100" value={right} />

    <div class="mr-slider">
      <div class="track"></div>
      <div
        class="range"
        style={`left: ${left}%; right: ${100 - right}%;`}
      ></div>
      <div class="left thumb" style={`left: ${left}%`}></div>
      <div class="right thumb" style={`left: ${right}%`}></div>
    </div>
  </div>
</div>

<style lang="scss">
  .mr-slider {
    position: relative;
    z-index: 1;
    height: 10px;
    margin: 0;
    width: 100%;

    & > .track {
      position: absolute;
      z-index: 1;
      left: 0;
      right: 0;
      top: 0;
      bottom: 0;
      border-radius: 5px;
      background: var(--tert-con-text);
    }

    & > .range {
      position: absolute;
      z-index: 2;
      left: 25%;
      right: 25%;
      top: 0;
      bottom: 0;
      border-radius: 5px;
      background: var(--tert);

      &:hover {
        background: var(--tert-text);
        opacity: 0.5;
      }
    }

    & > .thumb {
      position: absolute;
      z-index: 3;
      width: 10px;
      height: 10px;
      background: var(--tert-con);
      border-radius: 50%;
      box-shadow: 0 0 0 0 rgba(98, 0, 238, 0.1);
      transition: box-shadow 0.3s ease-in-out;

      &:hover {
        background: var(--tert-text);
      }
    }

    & > .thumb.left {
      left: 25%;
      // clip-path: polygon(50% 0, 0 0, 0 100%, 50% 100%);
    }

    & > .thumb.right {
      transform: translateX(-100%);
      // clip-path: polygon(50% 0, 100% 0, 100% 100%, 50% 100%);
    }
  }

  input[type="range"] {
    position: absolute;
    pointer-events: none;
    z-index: 2;
    height: 10px;
    width: 100%;
    opacity: 0;
  }

  input[type="range"]::-webkit-slider-thumb {
    pointer-events: all;
    width: 30px;
    height: 30px;
    border-radius: 0;
    border: 0 none;
    background-color: red;
    -webkit-appearance: none;
  }
</style>
