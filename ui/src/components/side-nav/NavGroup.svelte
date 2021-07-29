<script>
    import { slide } from 'svelte/transition';
    import FaAngleUp from 'svelte-icons/fa/FaAngleUp.svelte'
    import { expanded } from '../side-bar/stores';
    export let label = "group label"
    let open = true;
</script>

<style>
    .icon-closed {
        transform: scaleY(-1);
    }
    .icon-open {
        transform: scaleY(1);
    }
</style>


{#if $expanded}
<div>
    <button on:click={() => open = !open}
        class="flex items-center justify-between w-full h-10 px-2 focus:outline-none">
        <div class="text-xs font-bold uppercase truncate">{label}</div>
        <div class="h-4 transition-all duration-300"
            class:icon-open={open}
            class:icon-closed={!open}>
            <FaAngleUp />
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
    <div class="w-full h-px bg-white bg-opacity-20"></div>
</div>
<slot />
{/if}