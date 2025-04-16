<script>
	import Header from "$lib/components/Header.svelte";
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import { onMount } from "svelte";

	onMount(() => getCurrentWindow().show());
</script>

<div class="w-screen h-screen bg-vert-graident">
	<Header />
	<div class="pt-11 h-full">
		<slot />
	</div>
</div>
