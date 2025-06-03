<script>
    let open = false;
    import {
        Files,
        AlignJustify,
        X,
        CircleUserRound,
        PencilRuler,
    } from "@lucide/svelte";
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
                    >Characters<Files class="ml-2 inline-flex" /></a
                >
                <a href="/characters" class="text-2xl hover:text-purple-700 dark:hover:text-purple-400"
                    >Editor<PencilRuler class="ml-2 inline-flex" /></a
                >
                <a href="/characters" class="text-2xl hover:text-purple-700 dark:hover:text-purple-400"
                    >User<CircleUserRound class="ml-2 inline-flex" /></a
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
        <div class="absolute text-right md:hidden px-4 pb-4 space-y-2">
            <a href="/characters" class="block text-2xl hover:text-purple-400"
                >Characters<Files class="ml-2 inline-flex" /></a
            >
            <a href="/characters" class="block text-2xl hover:text-purple-400"
                >Editor<PencilRuler class="ml-2 inline-flex" /></a
            >
            <a href="/characters" class="block text-2xl hover:text-purple-400"
                >User<CircleUserRound class="ml-2 inline-flex" /></a
            >
        </div>
    {/if}
</nav>
