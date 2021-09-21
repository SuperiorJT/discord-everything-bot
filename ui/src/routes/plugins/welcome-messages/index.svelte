<script context="module" lang="ts">
  import WelcomeFactory from '$http/welcome';

  export async function load({ fetch }) {
    const url = `http://localhost:3030/${getActiveGuild()}/channels`;

    const res = await Promise.all([fetch(url), WelcomeFactory.fetchModuleData()]);

    let data = {
      props: {
        guildChannels: null,
        initialValues: null,
        errors: []
      }
    };
    const channelRes = res[0];
    const welcomeRes = res[1];

    if (channelRes.ok) {
      data.props.guildChannels = await channelRes.json();
    } else {
      data.props.errors.push({
        status: channelRes.status,
        error: new Error(`Could not load ${url}: ${JSON.stringify(channelRes)}`)
      });
    }

    if (welcomeRes.ok) {
      data.props.initialValues = await welcomeRes.json();
    } else {
      data.props.errors.push({
        status: welcomeRes.status,
        error: new Error(`Could not load welcome module data: ${JSON.stringify(welcomeRes)}`)
      });
    }

    return data;
  }
</script>

<script lang="ts">
  import CommandCard from '$components/CommandCard.svelte';
  import CommandCardSection from '$components/CommandCardSection.svelte';
  import TextArea from '$components/forms/TextArea.svelte';
  import { Form, Field } from 'svelte-final-form';
  import arrayMutators from 'final-form-arrays';
  import FieldCondition from '$components/forms/FieldCondition.svelte';
  import EmbedFieldGroup from '$components/forms/EmbedFieldGroup.svelte';
  import Snackbar from '$components/Snackbar.svelte';
  import { fly } from 'svelte/transition';
  import LoadingSpinner from '$components/LoadingSpinner.svelte';
  import type { IEmbed } from 'src/models/embed';
  import { getActiveGuild } from '$http/active-guild';

  export let guildChannels = [];
  let submitting = false;

  interface IFormData {
    id?: number;
    guildId?: string;
    enabled: boolean;
    join?: {
      enabled: boolean;
      messageType: string;
      channelId?: string;
      content?: string;
      embed?: IEmbed;
    };
    joinDm?: {
      enabled: boolean;
      messageType: string;
      channelId?: string;
      content?: string;
      embed?: IEmbed;
    };
    joinRoles?: {
      enabled: boolean;
    };
    leave?: {
      enabled: boolean;
    };
  }

  const onSubmit = async (values: IFormData) => {
    submitting = true;
    await uploadImages(values);
    const joinFields = values.join.embed.fields;
    const joinDmFields = values.joinDm.embed.fields;
    console.log(values.join.embed);
    const clone = JSON.parse(JSON.stringify(values));
    if (joinFields.length === 1 && !joinFields[0].name && !joinFields[0].value) {
      clone.join.embed.fields = [];
    }
    if (joinDmFields.length === 1 && !joinDmFields[0].name && !joinDmFields[0].value) {
      clone.joinDm.embed.fields = [];
    }
    const res = await WelcomeFactory.postModuleData(clone);

    if (res.ok) {
      initialValues = values;
    }
    submitting = false;
  };

  const uploadImages = async (values: IFormData) => {
    const fd = new FormData();
    let imagesFound = false;
    if (values?.join?.embed?.author?.image instanceof File) {
      fd.append('join.author.image', values.join.embed.author.image);
      imagesFound = true;
    }
    if (values?.join?.embed?.image instanceof File) {
      fd.append('join.image', values.join.embed.image);
      imagesFound = true;
    }
    if (values?.join?.embed?.thumbnail instanceof File) {
      fd.append('join.thumbnail', values.join.embed.thumbnail);
      imagesFound = true;
    }
    if (values?.join?.embed?.footer?.image instanceof File) {
      fd.append('join.footer.image', values.join.embed.footer.image);
      imagesFound = true;
    }
    if (imagesFound) {
      const res = await WelcomeFactory.uploadModuleImages(fd);
      if (res.ok) {
        const data = await res.json();
        console.log(data);
        values.join.embed.author.image =
          data['join.author.image'] || values.join.embed.author.image;
        values.join.embed.image = data['join.image'] || values.join.embed.image;
        values.join.embed.thumbnail = data['join.thumbnail'] || values.join.embed.thumbnail;
        values.join.embed.footer.image =
          data['join.footer.image'] || values.join.embed.footer.image;
      }
    }
  };

  const defaultValues: IFormData = {
    enabled: true,
    join: {
      enabled: false,
      messageType: 'text',
      content: 'Hey {member.name}, welcome to **{server.name}**!',
      embed: {
        color: '#3B82F6',
        description: 'Hey {member.name}, welcome to **{server.name}**!',
        author: {
          image: null,
          name: '',
          url: ''
        },
        title: '',
        url: '',
        image: null,
        thumbnail: null,
        footer: {
          text: '',
          image: null
        },
        fields: []
      }
    },
    joinDm: {
      enabled: false,
      messageType: 'text',
      content: 'Hey {member.name}, welcome to **{server.name}**!',
      embed: {
        color: '#3B82F6',
        description: 'Hey {member.name}, welcome to **{server.name}**!',
        author: {
          image: null,
          name: '',
          url: ''
        },
        title: '',
        url: '',
        image: null,
        thumbnail: null,
        footer: {
          text: '',
          image: null
        },
        fields: []
      }
    },
    joinRoles: {
      enabled: false
    },
    leave: {
      enabled: false
    }
  };
  const mutators = arrayMutators;

  export let initialValues: IFormData;

  if (!initialValues?.join) {
    initialValues = { ...defaultValues, ...initialValues };
  }
</script>

<Form {onSubmit} {mutators} {initialValues} let:form let:state>
  <form on:submit|preventDefault={form.submit} class="pb-16">
    {#if state.dirty}
      <Snackbar
        transition={fly}
        transitionParams={{ y: 300 }}
        bottom
        center
        class="flex w-full md:w-[calc(100%-5rem)] justify-between items-center rounded-t-sm md:rounded shadow-2xl p-4 md:mb-6 bg-gray-900"
      >
        <div>Changes detected! Please save or cancel.</div>
        <div class="space-x-2 text-sm">
          <button
            type="button"
            class="align-middle w-16 px-3 py-2 text-white transition-all bg-gray-700 rounded-md hover:bg-gray-600"
            on:click={form.reset()}>Cancel</button
          >
          <button
            class="inline-flex justify-center align-middle w-16 px-3 py-2 text-white transition-all bg-blue-500 rounded-md {submitting
              ? 'opacity-50 cursor-default'
              : 'hover:bg-blue-600'}"
            aria-disabled={submitting}
          >
            {#if submitting}
              <LoadingSpinner />
            {:else}
              Save
            {/if}
          </button>
        </div>
      </Snackbar>
    {/if}
    <div class="flex justify-between">
      <h1 class="text-xl font-bold">Welcome</h1>
      <div>switch</div>
    </div>
    <p class="mt-4 mb-6 text-sm opacity-60">Give new users a warm welcome!</p>
    <CommandCard label="Send a message when a user joins the server" fieldName="join.enabled">
      <CommandCardSection>
        <Field name="join.channelId" let:input>
          <label for="join.channelId" class="block mb-2 text-sm opacity-60"
            >Welcome Message Channel</label
          >
          <select
            name={input.name}
            on:blur={input.onBlur}
            on:focus={input.onFocus}
            on:change={e => input.onChange(e.target['value'])}
            class="input-text {!input.value ? 'text-white/40' : 'text-white'}"
          >
            {#if !input.value}
              <option value="" disabled selected>Please select a channel...</option>
            {/if}
            {#each guildChannels as channel}
              {#if channel.type === 0}
                <option value={channel.id} selected={input.value === channel.id}
                  >{channel.name}</option
                >
              {/if}
            {/each}
          </select>
        </Field>
      </CommandCardSection>
      <CommandCardSection>
        <Field name="join.messageType" let:input>
          <label class="mr-4">
            <input
              type="radio"
              on:change={e => input.onChange(e.target['value'])}
              name={input.name}
              value="text"
              checked={input.value === 'text'}
            />
            Text message
          </label>
          <label>
            <input
              type="radio"
              on:change={e => input.onChange(e.target['value'])}
              name={input.name}
              value="embed"
              checked={input.value === 'embed'}
            />
            Embed message
          </label>
        </Field>

        <div class="mt-4">
          <FieldCondition when="join.messageType" is="text">
            <Field name="join.content" let:input let:meta>
              <TextArea
                name={input.name}
                on:blur={input.onBlur}
                on:focus={input.onFocus}
                on:input={e => input.onChange(e.target['value'])}
                value={input.value}
                charLimit={2000}
                class="max-w-lg"
              />
            </Field>
          </FieldCondition>
          <FieldCondition when="join.messageType" is="embed">
            <EmbedFieldGroup name="join.embed" />
          </FieldCondition>
        </div>
      </CommandCardSection>
    </CommandCard>
  </form>
  <!-- <pre class="text-xs">
        {JSON.stringify(state.values, null, 2)}
    </pre> -->
</Form>
