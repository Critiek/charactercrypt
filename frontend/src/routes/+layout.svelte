<script lang="ts">
	import "../app.css";

	import Navbar from "$lib/nav/Navbar.svelte";

	let { children } = $props();
</script>

<Navbar/>

{@render children()}