<script lang="ts">
  import { Menu, Search } from 'svelte-hero-icons';
  import Icon from 'svelte-hero-icons/Icon.svelte';
  import { open, expanded } from '../side-bar/stores';

  function toggleSideNavOpen() {
    open.update(x => !x);
  }
</script>

<header
  class="flex items-center justify-between h-20 px-10 transition-all duration-300 ease-out bg-gray-100 lg:justify-start dark:bg-gray-800 lg:transition-none"
  class:expanded-spacing={$open}
>
  <!-- Profile Menu -->
  <div class="w-16 lg:order-last">
    <img src="https://via.placeholder.com/58x30?text=Profile" alt="" />
  </div>
  <!-- Brand Logo -->
  <div class="lg:hidden">
    <img src="https://via.placeholder.com/140x40?text=Brand+Logo" alt="" />
  </div>
  <!-- SideNav Toggle -->
  <div class="flex justify-end w-16 lg:hidden">
    <button
      on:click={toggleSideNavOpen}
      class="h-10 p-2 -mr-2 transition-all duration-300 focus:outline-none opacity-60 hover:opacity-100"
    >
      <Icon src={Menu} />
    </button>
  </div>
  <!-- Search -->
  <div class="items-center hidden w-full max-w-lg lg:flex">
    <div class="h-6 mr-2 opacity-60">
      <Icon src={Search} />
    </div>
    <input
      type="text"
      placeholder="Search..."
      class="flex-grow bg-transparent border-0 placeholder-black/60 dark:placeholder-white/60 focus:ring-0 placeholder-opacity-40 caret-black/60 dark:caret-white/60 text-black/60 dark:text-white/60"
    />
  </div>
  <!-- Flex Fill -->
  <div class="flex-auto hidden lg:block" />
</header>

<style lang="postcss">
  .expanded-spacing {
    @apply md:pl-[20.5rem] lg:pl-10;
  }
</style>
