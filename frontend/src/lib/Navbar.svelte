<script>
    let open = false;
</script>

<nav
    class="bg-white dark:bg-gray-900 border-b border-gray-200 dark:border-gray-700"
>
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16 items-center">
            <!-- Logo -->
            <div
                class="flex-shrink-0 flex items-center text-xl font-bold text-indigo-600 dark:text-indigo-400"
            >
                MySite
            </div>

            <!-- Desktop Menu -->
            <div class="hidden md:flex space-x-4">
                <a
                    href="/"
                    class="text-gray-700 dark:text-gray-300 hover:text-indigo-600"
                    >Home</a
                >
                <a
                    href="/about"
                    class="text-gray-700 dark:text-gray-300 hover:text-indigo-600"
                    >About</a
                >
                <a
                    href="/contact"
                    class="text-gray-700 dark:text-gray-300 hover:text-indigo-600"
                    >Contact</a
                >
            </div>

            <!-- Mobile Menu Button -->
            <button
                class="md:hidden text-gray-700 dark:text-gray-300 focus:outline-none"
                on:click={() => (open = !open)}
                aria-label="Toggle menu"
            >
                <svg
                    class="h-6 w-6"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d={open
                            ? "M6 18L18 6M6 6l12 12"
                            : "M4 6h16M4 12h16M4 18h16"}
                    />
                </svg>
            </button>
        </div>
    </div>

    <!-- Mobile Menu -->
    {#if open}
        <div class="md:hidden px-4 pb-4 space-y-2">
            <a
                href="/"
                class="block text-gray-700 dark:text-gray-300 hover:text-indigo-600"
                >Home</a
            >
            <a
                href="/about"
                class="block text-gray-700 dark:text-gray-300 hover:text-indigo-600"
                >About</a
            >
            <a
                href="/contact"
                class="block text-gray-700 dark:text-gray-300 hover:text-indigo-600"
                >Contact</a
            >
        </div>
    {/if}
</nav>
