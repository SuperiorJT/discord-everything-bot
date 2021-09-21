<script lang="ts">
  import TextArea from './TextArea.svelte';
  import ColorPicker from './ColorPicker/ColorPicker.svelte';
  import ImageUpload from './ImageUpload.svelte';
  import { Field } from 'svelte-final-form';

  import { tooltip } from '$components/Tooltip.svelte';

  import { Table, Trash, X } from 'svelte-hero-icons';
  import Icon from 'svelte-hero-icons/Icon.svelte';
  import FieldArray from './FieldArray.svelte';

  export let name: string;

  let stripeColor = '#3B82F6';
</script>

<div class="block border-l-[6px] rounded pl-4" style="border-left-color: {stripeColor};">
  <Field name="{name}.color" let:input>
    <label for="{name}.color" class="opacity-60">Stripe Color</label>
    <ColorPicker
      name={input.name}
      value={input.value}
      on:change={e => {
        input.onChange(e.detail);
        stripeColor = e.detail;
      }}
    />
  </Field>
  <div class="flex flex-col mt-4 sm:flex-row">
    <div class="flex flex-col flex-grow max-w-lg">
      <div class="flex mb-2">
        <Field name="{name}.author.image" let:input>
          <ImageUpload
            name={input.name}
            on:change={e => input.onChange(e.detail)}
            value={input.value}
            circle
            label={'Author Image'}
            class="w-12 h-12 mr-5 mt-7"
          />
        </Field>
        <div class="flex flex-col flex-grow">
          <Field name="{name}.author.name" let:input>
            <label for="{name}.author" class="mb-2 text-sm opacity-60">Author</label>
            <input
              type="text"
              name={input.name}
              on:focus={input.onFocus}
              on:blur={input.onBlur}
              on:input={e => input.onChange(e.target['value'])}
              value={input.value}
              placeholder="Author name"
              class="mb-2 input-text"
            />
          </Field>
          <Field name="{name}.author.url" let:input>
            <input
              type="text"
              name={input.name}
              on:focus={input.onFocus}
              on:blur={input.onBlur}
              on:input={e => input.onChange(e.target['value'])}
              placeholder="Author url"
              class="mb-2 input-text"
            />
          </Field>
        </div>
      </div>
      <Field name="{name}.title" let:input>
        <label for="{name}.title" class="mb-2 text-sm opacity-60">Title</label>
        <input
          type="text"
          name={input.name}
          on:focus={input.onFocus}
          on:blur={input.onBlur}
          on:input={e => input.onChange(e.target['value'])}
          placeholder="Title Text"
          class="mb-2 input-text"
        />
      </Field>
      <Field name="{name}.url" let:input>
        <input
          type="text"
          name={input.name}
          on:focus={input.onFocus}
          on:blur={input.onBlur}
          on:input={e => input.onChange(e.target['value'])}
          placeholder="Title url"
          class="mb-4 input-text"
        />
      </Field>
      <Field name="{name}.description" let:input let:meta>
        <label for="{name}.description" class="mb-2 text-sm opacity-60">Description Template</label>
        <TextArea
          name={input.name}
          on:blur={input.onBlur}
          on:focus={input.onFocus}
          on:input={e => input.onChange(e.target['value'])}
          value={input.value}
          charLimit={2000}
          class="mb-4"
        />
      </Field>

      <label for="{name}.fields" class="mb-2 text-sm opacity-60">Additional Fields</label>
      <FieldArray name="{name}.fields" let:fields>
        {#each fields.names as name, i}
          <Field name="{name}.name" let:input>
            <div class="flex items-center mb-2">
              <input
                type="text"
                name={input.name}
                on:focus={input.onFocus}
                on:blur={input.onBlur}
                on:input={e => input.onChange(e.target['value'])}
                value={input.value}
                placeholder="Field name"
                class="input-text"
              />
              <button
                type="button"
                class="ml-2 transition-all ease-out rounded-md text-rose-500 bg-rose-500/10 hover:bg-rose-500/25 md:rounded-full disabled:text-white/20 disabled:bg-white/10 disabled:cursor-default"
                on:click={() => {
                  fields.mutators.remove(i);
                }}
              >
                <Icon src={Trash} class="flex-shrink-0 w-12 h-12 p-3 md:hidden" />
                <Icon src={X} class="flex-shrink-0 hidden w-8 h-8 p-1.5 md:inline-block" />
              </button>
            </div>
          </Field>
          <div class="flex items-center mb-4">
            <Field name="{name}.value" let:input>
              <input
                type="text"
                name={input.name}
                on:focus={input.onFocus}
                on:blur={input.onBlur}
                on:input={e => input.onChange(e.target['value'])}
                value={input.value}
                placeholder="Field value"
                class="input-text"
              />
            </Field>
            <Field name="{name}.inline" let:input>
              <button
                type="button"
                class="flex items-center justify-center ml-2"
                role="checkbox"
                aria-label="Inline"
                aria-checked={input.value}
                use:tooltip={{ message: 'Inline' }}
                on:click={() => input.onChange(!input.value)}
              >
                <Icon
                  src={Table}
                  solid
                  class="flex-shrink-0 w-12 h-12 p-3 md:w-8 md:h-8 md:p-1.5 transition-all ease-out rounded-md
                    {input.value ? 'text-blue-500/100' : 'text-white/40 hover:text-white/100'}"
                />
              </button>
            </Field>
          </div>
        {/each}
        <button
          type="button"
          on:click={() => fields.mutators.push({ name: '', value: '', inline: false })}
          class="self-start px-3 py-2 mb-4 text-base text-white transition-all bg-blue-500 rounded-md w-max hover:bg-blue-700"
        >
          + Add Field
        </button>
      </FieldArray>
      <Field name="{name}.image" let:input>
        <ImageUpload
          name={input.name}
          on:change={e => input.onChange(e.detail)}
          value={input.value}
          label={'Image'}
          class="w-48 h-48 mb-4"
        />
      </Field>
      <div class="flex items-end">
        <Field name="{name}.footer.image" let:input>
          <ImageUpload
            name={input.name}
            on:change={e => input.onChange(e.detail)}
            value={input.value}
            circle
            label={'Footer Image'}
            class="w-12 h-12 mr-5"
          />
        </Field>
        <Field name="{name}.footer.text" let:input>
          <div class="flex flex-col flex-grow">
            <label for="{name}.footer.text" class="mb-2 text-sm opacity-60">Footer</label>
            <input
              type="text"
              name={input.name}
              on:focus={input.onFocus}
              on:blur={input.onBlur}
              on:input={e => input.onChange(e.target['value'])}
              value={input.value}
              placeholder="Footer text"
              class="input-text"
            />
          </div>
        </Field>
      </div>
    </div>
    <Field name="{name}.thumbnail" let:input>
      <ImageUpload
        name={input.name}
        on:change={e => input.onChange(e.detail)}
        value={input.value}
        label={'Thumbnail'}
        class="order-first mb-4 ml-0 w-36 h-36 sm:order-last sm:ml-12"
      />
    </Field>
  </div>
</div>
