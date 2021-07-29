<script lang="typescript">
    import { page } from "$app/stores";
    import { isActivePath } from "../../util/active-path";
    import { open } from '../side-bar/stores';
    import FaRegCircle from 'svelte-icons/fa/FaRegCircle.svelte'

    export let label = "Navigation Link Title";
    export let link = "";

    function closeSideNav() {
        open.update(() => false);
    }
</script>

<style>
    .active {
        @apply bg-opacity-5 opacity-100;
    }
    .active .icon {
        @apply text-lightBlue-400 text-opacity-100;
    }
    .active .label {
        @apply text-opacity-100;
    }
</style>

<a href="{link}" class="relative flex items-center h-10 px-2 overflow-hidden transition-all duration-200 bg-white bg-opacity-0 rounded opacity-60 hover:opacity-100"
    class:active={isActivePath($page.path, link)}>
    <div class="absolute inset-0 lg:hidden" on:click={closeSideNav}></div>
    <div class="flex-none w-6 h-6 text-white icon">
        <slot name="icon">
            <FaRegCircle />
        </slot>
    </div>
    <div class="ml-4 text-sm text-white truncate label">
        {label}
    </div>
    <div class="flex-auto"></div>
</a>