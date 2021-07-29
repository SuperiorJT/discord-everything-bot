<script lang="typescript">
    import FaBars from 'svelte-icons/fa/FaBars.svelte';
    import FaSearch from 'svelte-icons/fa/FaSearch.svelte'
    import { open, expanded } from '../side-bar/stores';

    function toggleSideNavOpen() {
        open.update(x => !x);
    }
</script>

<style>
    .expanded-spacing {
        @apply md:pl-[20.5rem] lg:pl-10;
    }
</style>

<header class="flex items-center justify-between lg:justify-start h-20 px-10 transition-all duration-300 ease-out bg-gray-800 lg:transition-none"
    class:expanded-spacing={$open}>
    <!-- Profile Menu -->
    <div class="w-16 lg:order-last">
        <img src="https://via.placeholder.com/58x30?text=Profile" alt="">
    </div>
    <!-- Brand Logo -->
    <div class="lg:hidden">
        <img src="https://via.placeholder.com/140x40?text=Brand+Logo" alt="">
    </div>
    <!-- SideNav Toggle -->
    <div class="flex lg:hidden justify-end w-16">
        <button on:click={toggleSideNavOpen}
            class="h-10 p-3 -mr-3 transition-all duration-300 focus:outline-none opacity-60 hover:opacity-100">
            <FaBars />
        </button>
    </div>
    <!-- Search -->
    <div class="hidden lg:flex items-center w-full max-w-lg">
        <div class="h-5 opacity-60 mr-3">
            <FaSearch />
        </div>
        <input type="text" placeholder="Search..." class="flex-grow bg-transparent border-0 focus:ring-0 placeholder-white placeholder-opacity-40 caret-white/40 text-white/60">
    </div>
    <!-- Flex Fill -->
    <div class="hidden lg:block flex-auto"></div>
</header>