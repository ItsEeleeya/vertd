<script>
	import "../app.css";
	import "@fontsource/radio-canada-big/600.css";

	import Header from "$lib/components/Header.svelte";
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import { onMount } from "svelte";
	import { type } from "@tauri-apps/plugin-os";
	import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query";
	// import { QueryDevtools } from "@tanstack/svelte-query-devtools";

	const { children } = $props();
	document.documentElement.setAttribute("platform", type());
	onMount(() => getCurrentWindow().show());

	const queryClient = new QueryClient({
		defaultOptions: {
			queries: {
				enabled: true,
			},
		},
	});
</script>

<div id="layout_container" class="w-screen h-screen bg-vert-graident">
	<Header />
	<QueryClientProvider client={queryClient}>
		<div class="pt-13 h-full">
			{@render children?.()}
		</div>
		<!-- <QueryDevtools /> -->
	</QueryClientProvider>
</div>
