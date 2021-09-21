<script lang="ts">
  import { getContext, onDestroy } from 'svelte';
  import { FORM } from 'svelte-final-form/src/Form.svelte';
  import { fieldSubscriptionItems } from 'final-form';
  import type { FormApi, Unsubscribe } from 'final-form';

  export let name: string,
    subscription = getFieldSubscriptionItems(),
    validate = undefined;

  let meta = {};
  let fields: {
    name?: string;
    value?: any[];
    names?: string[];
    length?: number;
    mutators?: Record<string, (...args: any[]) => any>;
  } = {};
  let unsubscribe: Unsubscribe;

  const form = getContext(FORM) as FormApi;

  if (process.env.NODE_ENV !== 'production' && !form) {
    throw new Error(
      'Could not find svelte-final-form context value. Please ensure that your Field is inside the Form component.'
    );
  }

  $: {
    unsubscribe && unsubscribe();

    unsubscribe = form.registerField(
      name,
      fieldState => {
        const { value, ...fieldMeta } = fieldState;
        meta = fieldMeta;
        fields = {
          name,
          value,
          names: (value as any[]).map((_, i) => `${name}[${i}]`),
          length: fieldMeta.length || 0,
          mutators: Object.keys(form.mutators).reduce((res, key) => {
            res[key] = (...args: any[]) => form.mutators[key](name, ...args);
            return res;
          }, {})
        };
      },
      { ...subscription, length: true },
      {
        getValidator: () => validate
      }
    );
  }

  onDestroy(() => {
    unsubscribe && unsubscribe();
  });

  function getFieldSubscriptionItems() {
    return fieldSubscriptionItems.reduce((result, key) => {
      result[key] = true;
      return result;
    }, {});
  }
</script>

<slot {fields} {meta} />
