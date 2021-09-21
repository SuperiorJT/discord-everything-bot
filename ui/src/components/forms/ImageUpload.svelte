<script context="module" lang="ts">
  const acceptableFormats = ['image/gif', 'image/jpeg', 'image/jpg', 'image/png'];
</script>

<script lang="ts">
  import { tooltip } from '$components/Tooltip.svelte';
  import { createEventDispatcher } from 'svelte';
  import { Photograph, Plus, Ban, Trash } from 'svelte-hero-icons';
  import Icon from 'svelte-hero-icons/Icon.svelte';
  export let circle = false;
  export let value: File | string;
  export let label = '';
  export let name: string;
  let customClass = '';
  export { customClass as class };

  const dispatch = createEventDispatcher();

  let input: HTMLInputElement;
  let button: HTMLButtonElement;

  let imageURL: string;

  let dragOver = false;
  let dragOverError = false;

  function handleFileInput() {
    if (input.files?.length > 0) {
      value = input.files[0];
      imageURL = URL.createObjectURL(value);
    } else {
      value = null;
    }
    dispatch('change', value);
  }

  function handleDragEnter(e: DragEvent) {
    if (e.dataTransfer.items.length !== 1) {
      dragOverError = true;
      return;
    }
    const item = e.dataTransfer.items[0];
    if (item.kind !== 'file' || !acceptableFormats.includes(item.type)) {
      dragOverError = true;
      return;
    }
    e.dataTransfer.dropEffect = 'copy';
    dragOver = true;
  }

  function handleDragEnd(e: DragEvent) {
    dragOver = false;
    dragOverError = false;
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    dragOver = false;
    dragOverError = false;

    if (e.dataTransfer.files.length !== 1) {
      return;
    }
    const file = e.dataTransfer.files[0];
    if (!file.type.startsWith('image')) {
      return;
    }
    value = file;
  }

  function removeCurrentValue() {
    value = null;
    dispatch('change', value);
  }

  function getImageURL() {
    if (!value) return;
    if (typeof value === 'string') {
      return value;
    } else if (value) {
      return URL.createObjectURL(value);
    }
  }

  $: imageURL = getImageURL();
</script>

<button
  type="button"
  use:tooltip={{ message: label }}
  aria-label={label}
  class="relative flex justify-center items-center {customClass} border border-dashed transition-all duration-300 ease-out
    {!value ? 'hover:border-blue-500 hover:bg-blue-500/5 hover:text-blue-500' : ''}
    {!dragOver && !dragOverError && !value
    ? 'border-black/60 dark:border-white/60 text-black/60 dark:text-white/60'
    : 'border-transparent dark:border-transparent text-transparent dark:transparent'}
    {circle ? 'rounded-full' : 'rounded'}
    {dragOver ? 'border-blue-500 bg-blue-500/5 text-blue-500 scale-125' : ''}
    {dragOverError ? 'border-rose-500 bg-red-600/5 text-rose-500' : ''}"
  on:dragenter={handleDragEnter}
  on:dragleave={handleDragEnd}
  on:dragover|preventDefault={() => {}}
  on:dragend={handleDragEnd}
  on:drop={handleDrop}
  on:click={() => input.click()}
  bind:this={button}
>
  {#if !dragOver && !dragOverError}
    <Icon src={Photograph} class="w-6 h-6 pointer-events-none" />
  {:else if dragOver}
    <Icon src={Plus} class="w-6 h-6 pointer-events-none" />
  {:else if dragOverError}
    <Icon src={Ban} class="w-6 h-6 pointer-events-none" />
  {/if}
  {#if value && !dragOver && !dragOverError}
    <button
      type="button"
      class="absolute inset-0 z-10 flex items-center justify-center w-full h-full opacity-0 text-rose-500/0 hover:text-rose-500/100 hover:opacity-100"
      on:click|stopPropagation|preventDefault={removeCurrentValue}
    >
      <Icon src={Trash} class="w-10 h-10 p-2 rounded-full bg-black/60" />
    </button>
    <img src={imageURL} alt="Preview" class="absolute h-full" />
  {/if}
  <input
    {name}
    type="file"
    bind:this={input}
    on:change={handleFileInput}
    class="hidden"
    accept={acceptableFormats.join(', ')}
  />
</button>

<style lang="postcss">
  button {
    transition-timing-function: cubic-bezier(0.175, 0.885, 0.32, 1.275);
  }
</style>
