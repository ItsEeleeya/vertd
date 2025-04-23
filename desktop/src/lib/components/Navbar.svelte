<script lang="ts">
	import { beforeNavigate } from "$app/navigation";
	import { page } from "$app/state";
	import Logo from "$lib/assets/Logo.svg?component";
	import { goingLeft, effects } from "$lib/store/index.svelte";
	import { LucideRefreshCw, LucideSettings } from "lucide-svelte";
	import { quintOut } from "svelte/easing";
	import { fade } from "svelte/transition";
	import Panel from "./Panel.svelte";

	const items: {
		name: string;
		path: string;
		icon: any;
		iconOnly?: boolean;
		badge?: boolean;
	}[] = [
		{
			name: "Vert",
			path: "/",
			icon: Logo,
			iconOnly: true,
		},
		{
			name: "Convert",
			path: "/convert",
			icon: LucideRefreshCw,
		},
		{
			name: "Settings",
			path: "/settings",
			icon: LucideSettings,
		},
	];

	let links = $state<HTMLAnchorElement[]>([]);
	let container = $state<HTMLDivElement>();
	let containerRect = $derived(container?.getBoundingClientRect());
	$effect(() => {
		$inspect(containerRect);
	});

	const linkRects = $derived(links.map((l) => l.getBoundingClientRect()));

	const selectedIndex = $derived(
		items.findIndex((i) => i.path === page.url.pathname),
	);

	const isSecretPage = $derived(selectedIndex === -1);

	beforeNavigate((e) => {
		const oldIndex = items.findIndex(
			(i) => i.path === (e.from?.url.pathname || ""),
		);
		const newIndex = items.findIndex(
			(i) => i.path === (e.to?.url.pathname || ""),
		);
		if (newIndex < oldIndex) {
			goingLeft.set(true);
		} else {
			goingLeft.set(false);
		}
	});

	const duration = 300;
</script>

{#snippet link(item: (typeof items)[0], index: number)}
	{@const Icon = item.icon}
	{@const activeLink = selectedIndex === index}
	<a
		bind:this={links[index]}
		href={item.path}
		aria-label={item.name}
		class="h-7 relative z-10 rounded-xl flex flex-1 items-center justify-center overflow-hidden cursor-default"
		draggable={false}
	>
		<div class="grid grid-rows-1 grid-cols-1">
			{#key item.name}
				<div
					class="w-full row-start-1 col-start-1 h-full flex items-center justify-center gap-2"
					in:fade={{
						duration: 350,
						easing: quintOut,
					}}
					out:fade={{
						duration: 350,
						easing: quintOut,
					}}
				>
					<div class="relative">
						<div class="*:transition-colors *:duration-200">
							{#if item.iconOnly}
								<Icon
									class="h-2.5 stroke-transparent {activeLink
										? 'fill-secondary-content'
										: 'fill-base-content'}"
								/>
							{:else}
								<Icon
									class="size-4 {activeLink
										? 'text-secondary-content'
										: 'text-base-content'}"
								/>
							{/if}
						</div>
						{#if item.badge}
							<div
								class="absolute overflow-hidden grid grid-rows-1 grid-cols-1 -top-0 -right-2 size-2 rounded-full bg-secondary mx-1"
								transition:fade={{
									duration: 350,
									easing: quintOut,
								}}
							></div>
						{/if}
					</div>
					{#if !item.iconOnly}
						<p
							class="text-sm flex font-body transition-all duration-200"
							class:text-secondary-content={activeLink}
						>
							{item.name}
						</p>
					{/if}
				</div>
			{/key}
		</div>
	</a>
{/snippet}

<div bind:this={container}>
	<Panel
		class="w-90 h-9 rounded-2xl px-1 flex items-center justify-center gap-1 relative bg-neutral-50/5 shadow-md shadow-neutral/5"
	>
		{@const linkRect = linkRects.at(selectedIndex) || linkRects[0]}
		{#if linkRect}
			<div
				class="absolute bg-secondary/80 rounded-xl"
				style="width: {linkRect.width}px; height: {linkRect.height}px; top: {linkRect.top -
					(containerRect?.top || 0)}px; left: {linkRect.left -
					(containerRect?.left || 0)}px; opacity: {isSecretPage
					? 0
					: 1}; {$effects
					? `transition: left var(--transition) ${duration}ms, top var(--transition) ${duration}ms, opacity var(--transition) ${duration}ms;`
					: ''}"
			></div>
		{/if}
		{#each items as item, i (item.path)}
			{@render link(item, i)}
		{/each}
		<div class="w-0.5 bg-separator h-full hidden md:flex"></div>
	</Panel>
</div>
