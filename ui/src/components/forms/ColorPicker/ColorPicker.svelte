<script context="module" lang="ts">
  export const DEFAULT_COLOR = 'rgb(255, 255, 255)';
</script>

<script lang="ts">
  import { onMount, createEventDispatcher } from 'svelte';
  import { customPickerLoaded } from './stores';
  const dispatch = createEventDispatcher();
  const colors = [
    'rgb(255, 255, 255)',
    'rgb(96, 125, 138)',
    'rgb(255, 118, 115)',
    'rgb(255, 187, 92)',
    'rgb(255, 215, 78)',
    'rgb(109, 225, 148)',
    'rgb(99, 236, 219)',
    'rgb(90, 207, 245)',
    'rgb(112, 177, 255)',
    'rgb(176, 114, 255)'
  ];
  export let value = DEFAULT_COLOR;
  export let name;
  let pickerValue = '';
  let showPicker = false;

  function handleColorPickerSelect(e) {
    const rgb = e.target.rgb;
    pickerValue = `rgba(${rgb[0]}, ${rgb[1]}, ${rgb[2]}, ${rgb[3]})`;
    onChange(pickerValue);
  }

  function getButtonCircleMl(index: number) {
    if (index === 0 || index === 4) {
      return 0;
    }
    if (index === 1 || index === 3) {
      return 5;
    }
    if (index === 2) {
      return 7;
    }
    if (index === 5 || index === 7) {
      return -5;
    }
    if (index === 6) {
      return -7;
    }
  }

  function getButtonCircleMt(index: number) {
    if (index === 2 || index === 6) {
      return 0;
    }
    if (index === 3 || index === 5) {
      return 5;
    }
    if (index === 4) {
      return 7;
    }
    if (index === 1 || index === 7) {
      return -5;
    }
    if (index === 0) {
      return -7;
    }
  }

  const onChange = (val: string) => {
    console.log(val);
    value = val;
    dispatch('change', val);
  };

  onMount(async () => {
    if (!$customPickerLoaded) {
      await import('dino-color-picker');
      customPickerLoaded.update(() => true);
    }
    if (!colors.includes(value)) {
      pickerValue = value;
    }
  });
</script>

<div
  {name}
  class="flex items-center h-12 pl-2 bg-gray-200 rounded-md w-max dark:pl-0 dark:bg-transparent"
>
  {#each colors as color}
    <label
      class="relative w-5 h-5 mr-2 transition-all duration-200 origin-center transform rounded-full cursor-pointer"
      class:hover:scale-125={value !== color}
      style="background-color: {color};"
    >
      <div
        class="box-content absolute w-5 h-5 p-px border border-green-600 dark:border-green-400 rounded-full left-[-2px] top-[-2px] transition-all
            {value === color ? 'opacity-100 scale-100' : 'opacity-0 scale-150'}"
      />
      <input
        type="radio"
        class="invisible w-0 h-0"
        {name}
        value={color}
        on:change={e => onChange(e.target['value'])}
        checked={value === color}
      />
    </label>
  {/each}
  <button
    class="relative w-5 h-5 mr-2 transition-all duration-200 origin-center transform bg-gray-500 rounded-full cursor-pointer dark:bg-transparent"
    class:hover:scale-125={value !== pickerValue}
    class:hover:rotate-45={value !== pickerValue}
    on:click={() => (showPicker = !showPicker)}
  >
    <div
      class="box-content absolute w-5 h-5 p-px border border-green-600 dark:border-green-400 rounded-full left-[-2px] top-[-2px] transition-all
            {value === pickerValue ? 'opacity-100 scale-100' : 'opacity-0 scale-150'}"
    />
    {#each colors.slice(2) as color, i}
      <div
        class="absolute w-1 h-1 overflow-hidden transform -translate-x-1/2 -translate-y-1/2 rounded-full top-1/2 left-1/2"
        style="background-color: {color}; margin-left: {getButtonCircleMl(
          i
        )}px; margin-top: {getButtonCircleMt(i)}px;"
      />
    {/each}
  </button>
  {#if $customPickerLoaded && showPicker}
    <div class="relative z-10">
      <dino-color-picker
        class="absolute overflow-hidden bg-gray-800 rounded shadow-2xl"
        on:change={handleColorPickerSelect}
      />
    </div>
  {/if}
</div>
