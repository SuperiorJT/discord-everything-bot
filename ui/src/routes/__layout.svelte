<script>
  import '../app.postcss'
  import SideBar from '../components/side-bar/SideBar.svelte';
  import SideNav from '../components/side-nav/SideNav.svelte';
  import { open } from '../components/side-bar/stores';
  import Header from '../components/header/header.svelte';
</script>

<style>
  .nav-link {
    @apply inline-flex h-full px-3 items-center text-sm uppercase font-bold hover:bg-gray-700 transition-all duration-150 border-b-2 border-gray-50 border-opacity-0;
  }

  .nav-link-active {
    @apply border-opacity-100;
  }

  .content-blur {
    @apply md:blur-sm lg:blur-0;
  }
</style>

<SideBar>
  <SideNav slot="sidenav" />
  <main slot="content">
    <Header />
    <div class="transition-all duration-300 ease-out filter blur-0"
      class:content-blur={$open}>
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

