<script lang="ts">
  import { slide } from 'svelte/transition';
  import Icon from 'svelte-hero-icons/Icon.svelte';
  import { ChevronUp } from 'svelte-hero-icons';
  import { expanded } from '../side-bar/stores';
  export let label = 'group label';
  let open = true;
</script>

{#if $expanded}
  <div>
    <button
      on:click={() => (open = !open)}
      class="flex items-center justify-between w-full h-10 px-2 focus:outline-none"
    >
      <div class="text-xs font-bold uppercase truncate">{label}</div>
      <div class="h-4 transition-all duration-300" class:icon-open={open} class:icon-closed={!open}>
        <Icon src={ChevronUp} />
      </div>
    </button>

    <div class="overflow-hidden">
      {#if open}
        <div transition:slide|local={{ duration: 300 }}>
          <slot />
        </div>
      {/if}
    </div>
  </div>
{:else}
  <div class="flex items-center justify-center h-10">
    <div class="w-full h-px bg-white bg-opacity-20" />
  </div>
  <slot />
{/if}

<style lang="postcss">
  .icon-closed {
    transform: scaleY(-1);
  }
  .icon-open {
    transform: scaleY(1);
  }
</style>
