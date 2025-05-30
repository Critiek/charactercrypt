<script lang="ts">
    import { Navigation } from "@skeletonlabs/skeleton-svelte";
    // Icons
    import IconMenu from "@lucide/svelte/icons/menu";
    import IconFolder from "@lucide/svelte/icons/folder";
    import IconImage from "@lucide/svelte/icons/image";
    import IconMusic from "@lucide/svelte/icons/music";
    import IconVideo from "@lucide/svelte/icons/video";
    import IconGames from "@lucide/svelte/icons/gamepad";
    import IconSettings from "@lucide/svelte/icons/settings";
    import IconArrowRight from "@lucide/svelte/icons/arrow-right";

    let isExpansed = $state(false);

    function toggleExpanded() {
        isExpansed = !isExpansed;
    }
</script>

<div
    class="card border-surface-100-900 grid h-[760px] w-full grid-cols-[auto_1fr] border-[1px]"
>
    <!-- Component -->
    <Navigation.Rail expanded={isExpansed}>
        {#snippet header()}
            <Navigation.Tile
                labelExpanded="Menu"
                onclick={toggleExpanded}
                title="Toggle Menu Width"><IconMenu /></Navigation.Tile
            >
        {/snippet}
        {#snippet tiles()}
            <Navigation.Tile labelExpanded="Browse Files" href="#/files">
                <IconFolder />
            </Navigation.Tile>
            <Navigation.Tile labelExpanded="Browse Images" href="#/images">
                <IconImage />
            </Navigation.Tile>
            <Navigation.Tile labelExpanded="Browse Music" href="#/music">
                <IconMusic />
            </Navigation.Tile>
            <Navigation.Tile labelExpanded="Browse Videos" href="#/videos">
                <IconVideo />
            </Navigation.Tile>
            <Navigation.Tile labelExpanded="Browse Games" href="/games">
                <IconGames />
            </Navigation.Tile>
        {/snippet}
        {#snippet footer()}
            <Navigation.Tile
                labelExpanded="Settings"
                href="/settings"
                title="Settings"><IconSettings /></Navigation.Tile
            >
        {/snippet}
    </Navigation.Rail>
    <!-- Content -->
    <div class="flex items-center justify-center">
        <div class="space-y-4">
            <div class="flex gap-4">
                <button type="button" class="btn preset-filled-primary-500"
                    >Button</button
                >
                <button type="button" class="btn preset-tonal-primary"
                    >Button</button
                >
                <button type="button" class="btn preset-outlined-primary-500"
                    >Button</button
                >
            </div>
            <div class="flex gap-4">
                <button type="button" class="btn preset-filled-secondary-500"
                    >Button</button
                >
                <button type="button" class="btn preset-tonal-secondary"
                    >Button</button
                >
                <button type="button" class="btn preset-outlined-secondary-500"
                    >Button</button
                >
            </div>
            <div class="flex gap-4">
                <button type="button" class="btn preset-filled-tertiary-500"
                    >Button</button
                >
                <button type="button" class="btn preset-tonal-tertiary"
                    >Button</button
                >
                <button type="button" class="btn preset-outlined-tertiary-500"
                    >Button</button
                >
            </div>
            <div class="flex gap-4">
                <button type="button" class="btn preset-filled-success-500"
                    >Button</button
                >
                <button type="button" class="btn preset-tonal-success"
                    >Button</button
                >
                <button type="button" class="btn preset-outlined-success-500"
                    >Button</button
                >
            </div>
            <div class="flex gap-4">
                <button type="button" class="btn preset-filled-warning-500"
                    >Button</button
                >
                <button type="button" class="btn preset-tonal-warning"
                    >Button</button
                >
                <button type="button" class="btn preset-outlined-warning-500"
                    >Button</button
                >
            </div>
            <div class="flex gap-4">
                <button type="button" class="btn preset-filled-error-500"
                    >Button</button
                >
                <button type="button" class="btn preset-tonal-error"
                    >Button</button
                >
                <button type="button" class="btn preset-outlined-error-500"
                    >Button</button
                >
            </div>
            <div class="flex gap-4">
                <button type="button" class="btn preset-filled-surface-500"
                    >Button</button
                >
                <button type="button" class="btn preset-tonal-surface"
                    >Button</button
                >
                <button type="button" class="btn preset-outlined-surface-500"
                    >Button</button
                >
            </div>
        </div>
    </div>
</div>
