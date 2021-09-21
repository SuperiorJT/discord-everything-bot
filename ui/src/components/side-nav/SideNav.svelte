<script>
  import Icon from 'svelte-hero-icons/Icon.svelte';
  import { ChevronLeft, Logout } from 'svelte-hero-icons';
  import { expanded, open } from '../side-bar/stores';
  import NavItem from './NavItem.svelte';
  import NavGroup from './NavGroup.svelte';

  function toggleExpand() {
    expanded.update(x => !x);
  }

  function toggleOpen() {
    open.update(x => !x);
  }
</script>

<nav
  class="h-full p-5 pt-6 bg-gray-100 border-r shadow-xl border-gray-50 dark:bg-gray-800 dark:border-gray-700 border-opacity-60"
>
  <!-- Nav Header -->
  <div class="flex items-center h-10 mb-6">
    <button
      on:click={toggleExpand}
      class="inline-flex flex-col items-center justify-center flex-shrink-0 w-0 h-full overflow-hidden transition-all duration-300 opacity-0 focus:outline-none lg:w-10 lg:opacity-60 hover:opacity-100"
    >
      <div class="w-5 h-5 transition-all duration-300" class:collapsed-icon={!$expanded}>
        <Icon src={Logout} solid class="transform rotate-180" />
      </div>
    </button>
    <div class:expanded-hidden={!$expanded}>
      <img src="https://via.placeholder.com/140x40?text=Brand+Logo" alt="" />
    </div>
    <div class="flex-auto" />
    <div
      class="w-10 h-full overflow-hidden transition-all delay-300 opacity-100 md:opacity-0 md:w-0 md:delay-0"
    >
      <button
        on:click={toggleOpen}
        class="inline-flex flex-col items-center justify-center flex-shrink-0 w-full h-full transition-all duration-300 focus:outline-none opacity-60 hover:opacity-100"
      >
        <div class="w-7 h-7">
          <Icon src={ChevronLeft} />
        </div>
      </button>
    </div>
  </div>

  <!-- Root Nav -->
  <NavItem label="Dashboard" link="/dashboard" />
  <NavItem label="Plugins" link="/plugins" />
  <NavItem label="Settings" link="/settings" />

  <!-- Server Management -->
  <NavGroup label="Server Management">
    <NavItem label="Welcome" link="/plugins/welcome-messages" />
    <NavItem label="Reaction Roles" link="/plugins/reaction-roles" />
  </NavGroup>
</nav>

<style lang="postcss">
  .collapsed-icon {
    transform: scaleX(-1);
  }
  .expanded-hidden {
    @apply lg:hidden;
  }
</style>
