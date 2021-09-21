<script lang="ts">
  export let value = '';
  export let charLimit = Infinity;
  export let name: string;
  let focused = false;
  let klass = '';
  export { klass as class };

  $: hasCharLimit = charLimit !== Infinity;
</script>

<div
  class="relative block w-full pt-2 pl-4 pr-10 transition-all bg-gray-200 rounded-md dark:bg-gray-900 ring-1 ease hover:ring-blue-500 {klass}"
  class:pb-8={hasCharLimit}
  class:pb-2={!hasCharLimit}
  class:ring-lightBlue-400={focused}
  class:ring-transparent={!focused}
>
  <textarea
    {name}
    {value}
    on:focus={() => (focused = true)}
    on:blur={() => (focused = false)}
    on:change
    on:input
    on:focus
    on:blur
    rows="4"
    class="w-full p-0 text-base bg-transparent border-0 resize-none focus:ring-0"
  />

  {#if hasCharLimit}
    <div class="absolute text-sm bottom-2 opacity-60 right-4">{value.length} / {charLimit}</div>
  {/if}
</div>
