<script lang="ts">
    import { fade } from "svelte/transition";
    import {
        Files,
        AlignJustify,
        X,
        CircleUserRound,
        PencilRuler,
    } from "@lucide/svelte";
    import { clickOutside } from "./clickOutside";

    let open: boolean = false;

    // Combined nav items
    const navItems = [
        { name: "Characters", href: "/characters", icon: Files },
        { name: "Editor", href: "/characters", icon: PencilRuler },
        { name: "User", href: "/characters", icon: CircleUserRound },
    ];
</script>

<nav class="font-mono border-b-2 border-purple-700 dark:border-purple-400 mb-12">
    <div class="max-w-7xl mb-4 mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16 items-center">
            <!-- Logo -->
            <a
                class="flex-shrink-0 flex items-center text-4xl font-bold text-purple-700 dark:text-purple-400 p-2 border-dotted border-2 border-purple-700 dark:border-purple-400"
                href="/"
            >
                CharacterCrypt
            </a>

            <!-- Combined Menu -->
            <div class="hidden md:flex space-x-6">
                {#each navItems as { name, href, icon }}
                    <a
                        {href}
                        class="border-b-2 text-2xl hover:text-purple-700 dark:hover:text-purple-400 flex items-center"
                    >
                        {name}
                        <svelte:component
                            this={icon}
                            class="inline-flex ml-2 text-purple-700 dark:text-purple-400"
                        />
                    </a>
                {/each}
            </div>

            <!-- Mobile Menu Button -->
            <button
                class="md:hidden focus:outline-none"
                onclick={() => (open = !open)}
                aria-label="Toggle menu"
            >
                {#if !open}
                    <AlignJustify />
                {:else}
                    <X />
                {/if}
            </button>
        </div>
    </div>

    <!-- Mobile Menu -->
    {#if open}
        <div
            use:clickOutside
            onoutclick={() => (open = false)}
            transition:fade
            class="absolute border-2 border-purple-700 dark:border-purple-400 bg-neutral-200 dark:bg-neutral-900 text-left right-0 md:hidden px-5 pb-5 pt-5 space-y-2"
        >
            {#each navItems as { name, href, icon }}
                <a
                    {href}
                    class="flex items-center justify-between text-2xl hover:text-purple-700 dark:hover:text-purple-400"
                >
                    {name}
                    <svelte:component
                        this={icon}
                        class="ml-2 inline-flex text-purple-700 dark:text-purple-400"
                    />
                </a>
            {/each}
        </div>
    {/if}
</nav>
