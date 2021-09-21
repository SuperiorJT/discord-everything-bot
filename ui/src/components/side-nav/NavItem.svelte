<script lang="ts">
  import { page } from '$app/stores';
  import { isActivePath } from '$utils/active-path';
  import { open } from '$components/side-bar/stores';
  import Icon from 'svelte-hero-icons/Icon.svelte';
  import { DotsCircleHorizontal } from 'svelte-hero-icons';

  export let label = 'Navigation Link Title';
  export let link = '';

  function closeSideNav() {
    open.update(() => false);
  }
</script>

<a
  href={link}
  class="relative flex items-center h-10 px-2 overflow-hidden transition-all duration-200 bg-black bg-opacity-0 rounded dark:bg-opacity-0 dark:bg-white opacity-60 hover:opacity-100"
  class:active={isActivePath($page.path, link)}
>
  <div class="absolute inset-0 lg:hidden" on:click={closeSideNav} />
  <div class="flex-none w-6 h-6 text-black dark:text-white icon">
    <slot name="icon">
      <Icon src={DotsCircleHorizontal} class="w-full h-full" />
    </slot>
  </div>
  <div class="ml-4 text-sm text-black truncate dark:text-white label">
    {label}
  </div>
  <div class="flex-auto" />
</a>

<style lang="postcss">
  .active {
    @apply bg-opacity-5 opacity-100;
  }
  .active .icon {
    @apply text-blue-500 text-opacity-100;
  }
  .active .label {
    @apply text-opacity-100;
  }
</style>
