<script lang="ts">
    import { fade } from "svelte/transition";
    import { onMount } from "svelte";

    import {
        Files,
        AlignJustify,
        X,
        CircleUserRound,
        PencilRuler,
    } from "@lucide/svelte";

    let open = false;

    onMount(() => {
        function handleClickOutsideNav() {
            open = false;
        }

        document.addEventListener("click", handleClickOutsideNav);
    })
</script>

<nav>
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16 items-center">
            <!-- Logo -->
            <a
                class="flex-shrink-0 flex items-center text-4xl font-bold text-purple-700 dark:text-purple-400"
                href="/"
            >
                CharacterCrypt
            </a>

            <!-- Desktop Menu -->
            <div class="hidden md:flex space-x-6">
                <a
                    href="/characters"
                    class="text-2xl hover:text-purple-700 dark:hover:text-purple-400"
                    >Characters<Files class="ml-2 inline-flex text-purple-700 dark:text-purple-400" /></a
                >
                <a href="/characters" class="text-2xl hover:text-purple-700 dark:hover:text-purple-400"
                    >Editor<PencilRuler class="ml-2 inline-flex text-purple-700 dark:text-purple-400" /></a
                >
                <a href="/characters" class="text-2xl hover:text-purple-700 dark:hover:text-purple-400"
                    >User<CircleUserRound class="ml-2 inline-flex text-purple-700 dark:text-purple-400" /></a
                >
            </div>

            <!-- Mobile Menu Button -->
            <button
                class="md:hidden focus:outline-none"
                on:click={() => (open = !open)}
                aria-label="Toggle menu"
            >
                {#if !open}
                    <AlignJustify />
                {:else if open}
                    <X />
                {/if}
            </button>
        </div>
    </div>

    <!-- Mobile Menu -->
    {#if open}
        <!-- <div class="relative"> -->
            <div transition:fade class="absolute bg-neutral-200/90 dark:bg-neutral-900/90 text-left right-0 md:hidden px-5 pb-5 pt-5 space-y-2">
                <a href="/characters" class="flex items-center justify-between text-2xl hover:text-purple-700 dark:hover:text-purple-400"
                    >Characters<Files class="ml-2 inline-flex text-purple-700 dark:text-purple-400" /></a
                >
                <a href="/characters" class="flex items-center justify-between text-2xl hover:text-purple-700 dark:hover:text-purple-400"
                    >Editor<PencilRuler class="ml-2 inline-flex text-purple-700 dark:text-purple-400" /></a
                >
                <a href="/characters" class="flex items-center justify-between text-2xl hover:text-purple-700 dark:hover:text-purple-400"
                    >User<CircleUserRound class="ml-2 inline-flex text-purple-700 dark:text-purple-400" /></a
                >
            </div>
        <!-- </div> -->
    {/if}
</nav>
