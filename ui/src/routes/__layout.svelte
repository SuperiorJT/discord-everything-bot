<script lang="ts">
  import '../app.postcss';
  import SideBar from '$components/side-bar/SideBar.svelte';
  import SideNav from '$components/side-nav/SideNav.svelte';
  import { open } from '$components/side-bar/stores';
  import Header from '$components/header/header.svelte';
</script>

<SideBar>
  <SideNav slot="sidenav" />
  <main slot="content" class="flex flex-col h-full">
    <Header />
    <div class="flex-grow relative">
      <div
        class="absolute z-[60] inset-0 transition-all duration-300 ease-out backdrop-filter backdrop-blur-none pointer-events-none"
        class:content-blur={$open}
      />
      <slot />
    </div>
  </main>
</SideBar>

<!-- <div class="flex">
  <nav></nav>
</div>

<nav class="flex items-center justify-between h-16 px-4 bg-gray-800">
  <div class="text-2xl font-bold -mt-1.5 w-60">Bingus Bot</div>
  <div class="flex h-full space-x-2">
    <a href="/dashboard" class="nav-link" class:nav-link-active={isActivePath($page.path, '/dashboard')}>Dashboard</a>
    <a href="/plugins" class="nav-link" class:nav-link-active={isActivePath($page.path, '/plugins')}>Plugins</a>
    <a href="/settings" class="nav-link" class:nav-link-active={isActivePath($page.path, '/settings')}>Settings</a>
  </div>
  <div class="w-60"></div>
</nav>


<main>
  <slot />
</main>

<footer class="block p-4" /> -->
<style lang="postcss">
  @supports (backdrop-filter: blur(4px)) {
    .content-blur {
      @apply md:backdrop-blur-sm lg:backdrop-blur-none md:pointer-events-auto lg:pointer-events-none;
    }
  }

  @supports not (backdrop-filter: blur(4px)) {
    .content-blur {
      @apply md:bg-black/80 lg:bg-transparent md:pointer-events-auto lg:pointer-events-none;
    }
  }
</style>
