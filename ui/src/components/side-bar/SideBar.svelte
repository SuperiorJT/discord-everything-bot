<script lang="typescript">
    import { open, expanded } from "./stores";
</script>

<style>
    .closed {
        @apply -translate-x-full lg:-translate-x-0;
    }
    .collapsed {
        @apply lg:w-20;
    }
</style>

<div class="flex min-h-screen">
    <div class="fixed top-0 bottom-0 left-0 z-40 w-full transition-all duration-300 ease-out transform lg:relative md:w-72"
        class:collapsed={!$expanded}
        class:closed={!$open}>
        <slot name="sidenav"/>
    </div>
    <div class="flex-1">
        <slot name="content"/>
    </div>
</div>