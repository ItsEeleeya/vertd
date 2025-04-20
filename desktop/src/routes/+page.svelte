<script lang="ts">
	import DropArea from "$lib/components/DropArea.svelte";
	import clsx from "clsx";
	import {
		AudioLines,
		BookText,
		Film,
		Image,
		LucideChevronDown,
	} from "lucide-svelte";
	import { onMount } from "svelte";
	import { blur, slide } from "svelte/transition";

	let showFormats = $state(true);

	onMount(() => {
		const onKeyDown = (e: KeyboardEvent) => {
			if (e.code === "KeyS") {
				showFormats = !showFormats;
			}
		};
		window.addEventListener("keydown", onKeyDown);
		return () => window.removeEventListener("keydown", onKeyDown);
	});

	const status: {
		[key: string]: {
			ready: "yes" | "soon";
			formats: string;
			icon: typeof Image;
		};
	} = $derived({
		Images: {
			ready: "soon",
			formats: "",
			icon: Image,
		},
		Audio: {
			ready: "soon",
			formats: "",
			icon: AudioLines,
		},
		Documents: {
			ready: "soon",
			formats: "",
			icon: BookText,
		},
		Video: {
			ready: "yes",
			formats: "",
			icon: Film,
		},
	});
</script>

<main data-tauri-drag-region class="h-full flex flex-col *:px-3 *:w-full">
	<div data-tauri-drag-region class="grow min-h-1/3 pt-1 pb-3">
		<DropArea />
	</div>
	<div data-tauri-drag-region>
		<div data-tauri-drag-region class="w-full flex items-center pb-3">
			{#if showFormats}
				<h2
					transition:blur={{ duration: 300 }}
					class="text-center opacity-85 text-xl pointer-events-none px-1"
				>
					VERT supports…
				</h2>
			{/if}
			<button
				type="button"
				class="btn btn-sm btn-ghost btn-square btn-transparent text-sm ml-auto opacity-80 transition-all duration-150 gap-1"
				onclick={() => (showFormats = !showFormats)}
				aria-expanded={showFormats}
				title={showFormats
					? "Hide supported formats"
					: "Show supported formats"}
			>
				<div class="duration-500" class:rotate-180={!showFormats}>
					<LucideChevronDown size="14" />
				</div>
			</button>
		</div>
		{#if showFormats}
			<div
				data-tauri-drag-region
				class="flex gap-3 flex-wrap justify-center items-stretch max-w-2xl pb-3"
				transition:slide={{ duration: 200 }}
			>
				{#each Object.entries(status) as [key, s]}
					{@const Icon = s.icon}
					<div
						class="bg-neutral-50/8 rounded-2xl p-3 shadow-lg flex flex-col flex-1 min-h-28 min-w-50 max-w-full"
					>
						<div
							class="flex items-center justify-start gap-2 text-xl"
						>
							<div
								class={clsx(
									"p-2.5 rounded-full text-black",
									{
										"bg-vert-blue": key === "Images",
										"bg-vert-purple": key === "Audio",
										"bg-vert-green": key === "Documents",
										"bg-vert-red": key === "Video",
									},
								)}
							>
								<Icon size="18" />
							</div>
							<span>{key}</span>
						</div>

						<div
							class=" flex flex-col text-start justify-start mt-2 font-normal text-sm"
						>
							{#if s.ready === "yes"}
								<p>Available</p>
								<p>
									<b>Supported formats:</b>
									{s.formats}
								</p>
							{:else}
								<p>Coming soon to desktop</p>
							{/if}
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</main>
