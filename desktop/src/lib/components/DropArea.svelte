<script lang="ts">
	import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
	import { LucidePlus } from "lucide-svelte";
	import { onMount } from "svelte";
	import { blur, scale, slide } from "svelte/transition";

	// let {
	// 	dragEnter = () => {},
	// 	dragHovering: dragOver = () => {},
	// 	dragDrop = () => {},
	// 	dropLeave: dropOver = () => {},
	// }: {
	// 	dragEnter: () => void;
	// 	dragHovering: () => void;
	// 	dragDrop: () => void;
	// 	dropLeave: () => void;
	// } = $props();

	let dragState = $state<"enter" | "over" | "drop" | "leave">("leave");
	const webview = getCurrentWebviewWindow();
	onMount(() => {
		const unlistenDragDrop = webview.onDragDropEvent((event) => {
			dragState = event.payload.type;

			if (event.payload.type === "over") {
				console.log("User hovering", event.payload.position);
			} else if (event.payload.type === "drop") {
				console.log("User dropped", event.payload.paths);
			} else {
				console.log(
					"File drop cancelled " + JSON.stringify(event.payload),
				);
			}
		});

		return () => unlistenDragDrop.then((unlisten) => unlisten());
	});
</script>

<div
	class="size-full rounded-2xl border-2 border-dashed border-transparent hover:border-neutral-50/10 duration-200 hover:bg-vert-purple/5 flex flex-col items-center justify-center gap-2 cursor-pointer"
>
	{#key dragState}
		<div
			class="rounded-full bg-secondary size-10 flex items-center justify-center text-black text-4xl transition-all duration-1000"
			class:animate-bounce={dragState === "over"}
			in:scale={{ start: 0.5, duration: 100, delay: 50 }}
		>
			<LucidePlus size="24" />
		</div>
	{/key}

	{#if dragState === "leave"}
		<p in:blur>Click to add or Drag and Drop</p>
	{:else if dragState === "over"}
		<p in:blur class="animate-bounce">Release to add</p>
	{:else}{/if}
</div>
