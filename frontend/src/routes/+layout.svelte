<script lang="ts">
	import "../app.css";

	import Navbar from "$lib/layout/Navbar.svelte";
	import Footer from "$lib/layout/Footer.svelte";

	let { children } = $props();
</script>

<Navbar/>

<div class="max-w-7xl mb-4 mx-auto px-4 sm:px-6 lg:px-8">
{@render children()}
</div>

<!-- <Footer/> -->