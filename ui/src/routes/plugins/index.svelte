<script>
    import PluginTile from "../../components/PluginTile.svelte";
    import { page } from "$app/stores";
    import { assets } from "$app/paths";
</script>

<style>

</style>

<ol class="grid grid-cols-1 gap-6 px-8 mx-auto my-6 lg:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 2xl:max-w-[1920px]">
    <PluginTile href="{$page.path}/welcome-messages" title="Welcome" desc="Give new users a warm welcome!">
        <img slot="image" src="{assets}/plugin-icons/welcome-messages.png" alt="💌">
    </PluginTile>
    <PluginTile href="{$page.path}/reaction-roles" title="Reaction Roles" desc="Let your members get roles by reacting to a message">
        <img slot="image" src="{assets}/plugin-icons/reaction-roles.png" alt="😊">
    </PluginTile>
</ol>