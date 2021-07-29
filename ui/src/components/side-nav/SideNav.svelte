<script lang="typescript">
    import FaChevronLeft from 'svelte-icons/fa/FaChevronLeft.svelte';
    import FaCaretLeft from 'svelte-icons/fa/FaCaretLeft.svelte';
    import { expanded, open } from "../side-bar/stores";
    import NavItem from './NavItem.svelte';
    import NavGroup from './NavGroup.svelte';

    function toggleExpand() {
        expanded.update(x => !x);
    }

    function toggleOpen() {
        open.update(x => !x);
    }
</script>

<style>
    .collapsed-icon {
        transform: scaleX(-1);
    }
    .expanded-hidden {
        @apply lg:hidden;
    }
</style>

<nav class="h-full p-5 pt-6 bg-gray-800 border-r border-gray-700 border-opacity-60">
    <!-- Nav Header -->
    <div class="flex items-center h-10 mb-6">
        <button on:click={toggleExpand} class="inline-flex flex-col items-center justify-center flex-shrink-0 w-0 h-full overflow-hidden transition-all duration-300 opacity-0 focus:outline-none lg:w-10 lg:opacity-60 hover:opacity-100">
            <div class="w-5 h-5 transition-all duration-300"
                class:collapsed-icon={!$expanded}>
                <FaCaretLeft />
            </div>
        </button>
        <div class:expanded-hidden={!$expanded}>
            <img src="https://via.placeholder.com/140x40?text=Brand+Logo" alt="">
        </div>
        <div class="flex-auto"></div>
        <div class="w-10 h-full overflow-hidden transition-all delay-300 opacity-100 md:opacity-0 md:w-0 md:delay-0">
            <button on:click={toggleOpen} class="inline-flex flex-col items-center justify-center flex-shrink-0 w-full h-full transition-all duration-300 focus:outline-none opacity-60 hover:opacity-100">
                <div class="w-5 h-5">
                    <FaChevronLeft />
                </div>
            </button>
        </div>
    </div>

    <!-- Root Nav -->
    <NavItem label="Dashboard" link="/dashboard"></NavItem>
    <NavItem label="Plugins" link="/plugins"></NavItem>
    <NavItem label="Settings" link="/settings"></NavItem>

    <!-- Server Management -->
    <NavGroup label="Server Management">
        <NavItem label="Welcome" link="/plugins/welcome-messages"></NavItem>
        <NavItem label="Reaction Roles" link="/plugins/reaction-roles"></NavItem>
    </NavGroup>
</nav>