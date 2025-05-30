<script lang="ts">
	import "../app.css";

	import Navbar from "$lib/Navbar.svelte";
	import SkeletonNavbar from "$lib/SkeletonNavbar.svelte";

	let { children } = $props();
</script>
{@render children()}
