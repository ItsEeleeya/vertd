<script>
	import "../app.css";
	import "@fontsource/radio-canada-big/600.css";

	import Header from "$lib/components/Header.svelte";
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import { onMount } from "svelte";
	import { type } from "@tauri-apps/plugin-os";

	const { children } = $props();
	document.documentElement.setAttribute("platform", type());
	onMount(() => getCurrentWindow().show());
</script>

<div id="layout_container" class="w-screen h-screen bg-vert-graident">
	<Header />
	<div class="pt-13 h-full">
		{@render children?.()}
	</div>
</div>
