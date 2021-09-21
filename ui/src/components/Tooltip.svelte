<script context="module" lang="ts">
  export const tooltip = (node: HTMLElement, props) => {
    let tooltip = null;
    node.style.position = 'relative';
    import('./Tooltip.svelte').then(klass => {
      tooltip = new klass.default({
        target: node,
        props
      });
    });

    const show = () => {
      tooltip.$set({ showing: true });
    };
    const hide = () => {
      tooltip.$set({ showing: false });
    };

    if (props.message) {
      node.addEventListener('mouseenter', show);
      node.addEventListener('mouseleave', hide);
    }

    return {
      update(props) {
        if (!props.message) {
          node.removeEventListener('mouseenter', show);
          node.removeEventListener('mouseleave', hide);
        }
        if (tooltip) {
          tooltip.$set(props);
        }
      },
      destroy() {
        if (!tooltip.target) {
          // return early since the parent component was already destroyed.
          return;
        }
        tooltip.$destroy();
      }
    };
  };
</script>

<script lang="ts">
  export let position: 'top' | 'right' | 'bottom' | 'left' = 'bottom';
  export let message = null;

  export let showing = false;

  let positionClass = 'mt-3';
  let transformClass = 'translate-y-full';

  const calculatePositions = () => {
    switch (position) {
      case 'top':
        positionClass = 'mb-14';
        transformClass = '-translate-y-1/2';
        break;
      case 'right':
        positionClass = 'ml-14';
        transformClass = 'translate-x-1/2';
        break;
      case 'bottom':
        positionClass = 'mt-14';
        transformClass = 'translate-y-1/2';
        break;
      case 'left':
        positionClass = 'mr-14';
        transformClass = '-translate-x-1/2';
        break;
    }
  };

  $: calculatePositions();
</script>

<div
  class="absolute rounded-md px-2 py-1.5 dark:bg-gray-300 bg-gray-900 z-50 w-max max-w-xs text-white dark:text-black text-sm leading-none transform origin-center transition-all duration-75 pointer-events-none
    {positionClass}
    {transformClass}
    {showing ? `opacity-100 delay-0` : 'scale-50 opacity-0'}"
>
  {message}
</div>
