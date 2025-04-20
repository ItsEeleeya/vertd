<script>
	import "../app.css";
	import "@fontsource/radio-canada-big/600.css";

	import Header from "$lib/components/Header.svelte";
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import { onMount } from "svelte";
	import { type } from "@tauri-apps/plugin-os";

	document.documentElement.setAttribute("platform", type());

	onMount(() => getCurrentWindow().show());
</script>

<div id="layout_container" class="w-screen h-screen bg-vert-graident hidden">
	<Header />
	<div class="pt-11 h-full">
		<slot />
	</div>
</div>
