<script lang="ts">
	import {
		createInfiniteQuery,
		createMutation,
	} from "@tanstack/svelte-query";
	import { commands } from "../../bindings";
	import { onMount } from "svelte";
	import { createVirtualizer } from "../virtual/index.svelte";

	const LIMIT = 50;

	async function fetchTasks({
		pageParam = 1,
	}): Promise<{ items: any[]; nextCursor?: number }> {
		console.log("Fetching tasks, page:", pageParam);
		const result = await commands.getTasks(null, pageParam, LIMIT);
		if (result.status === "error") {
			console.error("Error fetching tasks:", result.error);
			throw new Error(result.error);
		}

		console.log(
			"Fetched tasks:",
			result.data.length,
			"Next cursor:",
			result.data.length === LIMIT ? pageParam + 1 : undefined,
		);

		return {
			items: result.data,
			nextCursor:
				result.data.length === LIMIT ? pageParam + 1 : undefined,
		};
	}

	// Infinite query state
	const tasksQuery = $state(
		createInfiniteQuery({
			queryKey: ["tasks"],
			queryFn: fetchTasks,
			initialPageParam: 1,
			getNextPageParam: (_lastGroup, groups) => groups.length,
		}),
	);

	// Flattened list of all tasks
	const allRows = $derived(() => {
		const data = $tasksQuery.data
			? $tasksQuery.data.pages.flatMap((page) => page.items)
			: [];
		console.log("allTasks derived, count:", data.length);
		return data;
	});

	// Virtualizer state
	let vlistRef: HTMLDivElement;

	const virtualizer = $state(
		createVirtualizer<HTMLDivElement, HTMLDivElement>({
			count: $tasksQuery.data?.pages.length ?? 0,
			getScrollElement: () => vlistRef,
			estimateSize: () => LIMIT,
			overscan: 10,
		}),
	);

	// Sync virtualizer count and trigger loading
	$effect(() => {
		// Use $allTasks to get the array value
		const total = allRows().length;
		const newCount = $tasksQuery.hasNextPage ? total + 1 : total;
		console.log(
			"Effect: total items:",
			total,
			"hasNextPage:",
			$tasksQuery.hasNextPage,
			"new virtualizer count:",
			newCount,
		);

		// Call methods on the virtualizer instance via $virtualizer
		// virtualizer.setOptions({ count: 0 });
		virtualizer.options.count = 10;

		const items = virtualizer.getVirtualItems();
		if (items.length === 0) {
			console.log("Effect: No virtual items yet.");
			return;
		}

		const last = items[items.length - 1];
		console.log(
			"Effect: Last virtual item index:",
			last?.index,
			"Current total:",
			total - 1,
		); // total -1 because total is 0-indexed length

		if (
			last &&
			last.index >= total - 1 && // Corrected: last.index can be total-1 when it's the last actual item
			$tasksQuery.hasNextPage &&
			!$tasksQuery.isFetchingNextPage
		) {
			console.log("Effect: Fetching next page...");
			$tasksQuery.fetchNextPage();
		}
	});

	// Mutation to add mock tasks
	const addMock = $state(
		createMutation({
			mutationFn: async (count: number) => {
				console.log("Adding mock tasks:", count);
				const result = await commands.addMockTasks(count);
				console.log("Mock tasks added result:", result);
				return result;
			},
			onSuccess: () => {
				console.log("Add mock success, refetching tasksQuery");
				$tasksQuery.refetch();
			},
			onError: (error) => {
				console.error("Error adding mock tasks:", error);
			},
		}),
	);

	onMount(() => {
		$addMock.mutate(100);
	});
</script>

<div class="size-full px-2 pb-4 flex flex-col gap-4 overflow-hidden">
	<div class="flex grow px-1 overflow-hidden rounded-2xl">
		<div class="size-full bg-neutral-50/5 rounded-2xl">
			<div
				bind:this={vlistRef}
				class="w-full h-full overflow-y-auto overscroll-contain rounded-2xl"
			>
				<!-- Ensure vlistRef is ready before rendering virtualizer items -->
				<div
					class="relative w-full"
					style="height: {virtualizer.getTotalSize()}px"
				>
					{#each virtualizer.getVirtualItems() as row (row.index)}
						<!-- Access task from $allTasks array using row.index -->
						{@const task = allRows()[row.index]}
						<div
							style="position:absolute; top:0; left:0; width:100%; height:50px; transform: translateY({row.start}px)"
							class="flex items-center gap-2 p-2 hover:bg-neutral-50/10 rounded-2xl"
							data-index={row.index}
						>
							{#if task}
								<div class="flex-1">
									<div class="flex gap-2">
										<code
											class="text-sm font-medium text-base-content/90"
											>({task.id})</code
										>
										<div
											class="text-sm font-medium text-base-content/50 truncate"
										>
											{task.inputPath}
										</div>
									</div>
									<div class="text-xs text-neutral-500">
										{task.kind}
									</div>
								</div>
							{:else}
								<div class="w-full text-center text-sm">
									{#if $tasksQuery.isFetchingNextPage || $tasksQuery.isLoading}Loading...
									{:else if $tasksQuery.hasNextPage}Loading
										more...
									{:else if allRows().length === 0 && !$tasksQuery.isLoading}No
										tasks found.
									{:else}No more tasks{/if}
								</div>
							{/if}
						</div>
					{/each}
				</div>
			</div>
		</div>
	</div>

	<div class="px-1 w-full flex justify-between items-center">
		<select class="select bg-neutral-50/10 max-w-xs">
			<option>.png</option>
			<option>.jpeg</option>
			<option>.webp</option>
			<option>.gif</option>
			<option>.hdr</option>
			<option>.raw</option>
		</select>
		<div class="flex gap-2">
			<button class="btn btn-neutral">Export all as .zip</button>
			<button class="btn btn-secondary">Convert all</button>
		</div>
	</div>
</div>
