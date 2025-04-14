<script lang="ts">
	import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
	import { onDestroy, onMount } from "svelte";

	const webview = getCurrentWebviewWindow();

	let unlistenDragDrop: () => void | undefined;
	onMount(async () => {
		unlistenDragDrop = await webview.onDragDropEvent((event) => {
			if (event.payload.type === "over") {
				console.log("User hovering", event.payload.position);
			} else if (event.payload.type === "drop") {
				console.log("User dropped", event.payload.paths);
			} else {
				console.log("File drop cancelled");
			}
		});
	});
	onDestroy(() => unlistenDragDrop?.());
</script>

<div
	class="size-full rounded-lg border-3 border-dashed border-neutral-50/20 bg-[var(--bg)]/60 flex flex-col items-center justify-center gap-2 cursor-pointer"
>
	<div
		class="rounded-full bg-accent size-10 flex items-center justify-center text-black text-4xl"
	>
		+
	</div>
	Drop or click to add
</div>
