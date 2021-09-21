<script lang="ts">
  import { scale } from 'svelte/transition';
  import { open, expanded } from '$components/side-bar/stores';

  let klass = '';
  export { klass as class };

  export let absolute = false;
  export let bottom = false;
  export let left = false;
  export let right = false;
  export let top = false;
  export let center = false;
  export let active = true;
  export let timeout: boolean | number = false;
  export let transition = scale;
  export let transitionParams = {};
  export let style = '';

  $: {
    if (active && typeof timeout === 'number' && timeout > 0) {
      setTimeout(() => {
        active = false;
      }, timeout);
    }
  }
</script>

<div
  class="flex inset-0 pointer-events-none transition-all transform duration-300 ease-out
    {$expanded ? 'lg:left-72' : 'lg:left-20'}
    {absolute ? 'absolute z-10' : 'fixed z-50'}
    {bottom ? '!items-end' : ''}
    {top ? '!items-start' : ''}
    {left ? '!justify-start' : ''}
    {right ? '!justify-end' : ''}
    {center ? 'justify-center items-center' : ''}"
>
  {#if active}
    <div
      transition:transition={transitionParams}
      on:introstart
      on:outrostart
      on:introend
      on:outroend
      class="pointer-events-auto {klass}"
      {style}
    >
      <slot />
    </div>
  {/if}
</div>
