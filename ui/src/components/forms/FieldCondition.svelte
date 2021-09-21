<script lang="ts">
  import { Field } from 'svelte-final-form';
  export let when: string, is: any;
</script>

<Field name={when} subscription={{ value: true }} let:input>
  {#if input.value === is}
    <slot />
  {/if}
</Field>
