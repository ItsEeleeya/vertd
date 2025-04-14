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
	import { blur, slide } from "svelte/transition";

	let supportedFormatsVisible = $state(true);

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
	<div data-tauri-drag-region class="pb-3">
		<div data-tauri-drag-region class="w-full flex items-center">
			{#if supportedFormatsVisible}
				<h2
					transition:blur={{ duration: 300 }}
					class="text-center opacity-85 text-xl pointer-events-none p-1"
				>
					Supported formats
				</h2>
			{/if}
			<button
				type="button"
				class="btn text-sm ml-auto opacity-80 transition-all duration-150 min-w-52 gap-2 items-stretch"
				onclick={() =>
					(supportedFormatsVisible = !supportedFormatsVisible)}
			>
				<LucideChevronDown
					size="14"
					class={clsx(
						"transition-all duration-300",
						supportedFormatsVisible ? "" : "rotate-180",
					)}
				/>
				{#key supportedFormatsVisible}
					<span in:blur={{ duration: 300 }}>
						{#if supportedFormatsVisible}
							<span in:blur>Hide supported formats</span>
						{:else}
							<span in:blur>Show supported formats </span>
						{/if}
					</span>
				{/key}
			</button>
		</div>
		{#if supportedFormatsVisible}
			<div
				data-tauri-drag-region
				class="pt-3 flex gap-3 flex-wrap justify-center items-stretch max-w-2xl mx-auto transition-all duration-1000 ease-in-out hidden:opacity-0 opacity-100"
				transition:slide={{ duration: 300 }}
			>
				{#each Object.entries(status) as [key, s]}
					{@const Icon = s.icon}
					<div
						class="file-category-card flex-1 min-h-28 min-w-50 max-w-full"
					>
						<div class="file-category-card-inner">
							<div
								class={clsx("icon-container", {
									"bg-accent-blue": key === "Images",
									"bg-accent-purple": key === "Audio",
									"bg-accent-green": key === "Documents",
									"bg-accent-red": key === "Video",
								})}
							>
								<Icon size="18" />
							</div>
							<span>{key}</span>
						</div>

						<div class="file-category-card-content mt-2">
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

<style>
	@reference "tailwindcss";
	@reference "../app.css";

	.formats-container {
		transition: all 0.3s ease-in-out;
		overflow: hidden;
		margin-top: 0.75rem;
		max-height: 500px;
		opacity: 1;
	}

	.formats-container.hidden {
		opacity: 0;
		max-height: 0;
		margin-top: 0;
	}

	.file-category-card {
		@apply bg-neutral-50/8 rounded-2xl p-3 shadow-lg flex flex-col;
	}

	.file-category-card p {
		@apply font-normal text-start text-sm;
	}

	.file-category-card-inner {
		@apply flex items-center justify-start gap-2 text-xl;
	}

	.file-category-card-content {
		@apply flex flex-col text-center justify-start;
	}

	.icon-container {
		@apply p-2.5 rounded-full text-on-accent;
	}
</style>
