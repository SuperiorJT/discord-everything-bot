<script>
    export let href: string;
    export let title = "Plugin Title";
    export let desc = "Power up your automation with cool commands and automations!";
</script>

<li class="h-32 transition-all duration-300 bg-gray-500 rounded shadow-md bg-opacity-30 hover:bg-opacity-50 hover:shadow-lg">
    <a href="{href}" class="flex h-full p-5 overflow-hidden">
        <div class="flex-shrink-0 w-16 h-16 mt-1 mr-4">
            <slot name="image">
                <img src="https://image.flaticon.com/icons/png/128/1829/1829586.png" alt="A placeholder icon">
            </slot>
        </div>
        <div class="flex-grow">
            <h3 class="mb-2 text-xl font-bold ">{title}</h3>
            <p class="text-sm leading-tight text-gray-300 opacity-80">{desc}</p>
        </div>
    </a>
</li>