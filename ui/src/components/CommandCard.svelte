<script lang="ts">
  import { Field } from 'svelte-final-form';
  import { ChevronUp } from 'svelte-hero-icons';
  import Icon from 'svelte-hero-icons/Icon.svelte';
  import { slide } from 'svelte/transition';

  export let label = 'Plugin command label';
  export let fieldName: string;
  let expanded = false;

  function toggleExpand(e: MouseEvent) {
    e.stopPropagation();
    expanded = !expanded;
  }

  function toggleEnabled(input: any) {
    if (!input.value && !expanded) {
      expanded = !expanded;
    }
    input.onChange(!input.value);
  }
</script>

<Field name={fieldName} let:input>
  <div
    class="block px-6 bg-gray-100 rounded shadow dark:bg-gray-800"
    class:hover:cursor-pointer={!expanded && input.value}
    on:click={e => (!expanded && input.value ? toggleExpand(e) : null)}
  >
    <div class="relative flex items-center justify-start h-16 overflow-hidden">
      <h3 class="flex-grow text-lg font-semibold">{label}</h3>
      <div class="" on:click|stopPropagation={() => toggleEnabled(input)}>
        {input.value ? 'enabled' : 'disabled'}
      </div>
      <div
        on:click={toggleExpand}
        class="p-1 ml-4 transition-all duration-300 ease-out transform scale-y-100 h-7 w-7"
        class:-mr-8={!input.value}
        class:-scale-y-100={!expanded}
        class:hover:cursor-pointer={expanded}
      >
        <Icon src={ChevronUp} />
      </div>
      <div
        class="absolute top-0 bottom-0 right-0 w-2 pointer-events-none bg-gradient-to-r from-transparent to-gray-100 dark:to-gray-800"
      />
    </div>
    {#if expanded && input.value}
      <div transition:slide|local={{ duration: 300 }} class="block pb-6">
        <slot />
      </div>
    {/if}
  </div>
</Field>
