<script lang="ts">
	import {
		createInfiniteQuery,
		createMutation,
	} from "@tanstack/svelte-query";
	import { commands } from "../../bindings";
	import { onMount } from "svelte";
	import { VList } from "virtua/svelte";

	const LIMIT = 50;
	const MAX_ITEMS = 500; // Maximum number of items to keep in memory
	const CLEANUP_THRESHOLD = 300; // When to start cleaning up old items

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

	// Infinite query state with data cleanup
	const tasksQuery = $state(
		createInfiniteQuery({
			queryKey: ["tasks"],
			queryFn: fetchTasks,
			initialPageParam: 1,
			getNextPageParam: (_lastGroup, groups) => groups.length,
			maxPages: Math.ceil(MAX_ITEMS / LIMIT), // Limit total pages
			select: (data) => {
				// Clean up old data when we exceed the threshold
				if (data.pages.length * LIMIT > CLEANUP_THRESHOLD) {
					// Keep only the most recent pages
					const pagesToKeep = Math.ceil(CLEANUP_THRESHOLD / LIMIT);
					return {
						...data,
						pages: data.pages.slice(-pagesToKeep),
					};
				}
				return data;
			},
		}),
	);

	// Flattened list of all tasks with memory optimization
	const allTasks = $derived(() => {
		const data =
			$tasksQuery.data?.pages.flatMap((page) => page.items) ?? [];
		// If we have too many items, keep only the most recent ones
		if (data.length > MAX_ITEMS) {
			return data.slice(-MAX_ITEMS);
		}
		return data;
	});

	// Virtual list state
	let vlistRef: HTMLDivElement;
	let virtualList: VList<any>;

	// Load more when reaching the end
	async function onScroll() {
		if (!virtualList) return;

		const endIndex = virtualList.findEndIndex();
		if (
			endIndex + 5 >= allTasks().length &&
			$tasksQuery.hasNextPage &&
			!$tasksQuery.isFetchingNextPage &&
			allTasks().length < MAX_ITEMS // Don't fetch more if we're at the limit
		) {
			await $tasksQuery.fetchNextPage();
		}
	}

	// Clean up old data periodically
	$effect(() => {
		const interval = setInterval(() => {
			if (allTasks().length > CLEANUP_THRESHOLD) {
				$tasksQuery.refetch();
			}
		}, 30000); // Check every 30 seconds

		return () => {
			clearInterval(interval);
		};
	});

	let addMock = $state(
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
				<VList
					bind:this={virtualList}
					class="w-full h-full"
					data={allTasks()}
					itemSize={50}
					overscan={5}
					onscroll={onScroll}
				>
					{#each allTasks() as task}
						<div
							class="flex items-center gap-2 p-2 hover:bg-neutral-50/10 rounded-2xl h-[50px]"
						>
							<div class="flex-1">
								<div class="flex gap-2">
									<code
										class="text-sm font-medium text-base-content/90"
									>
										({task.id})
									</code>
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
						</div>
					{/each}
					{#if $tasksQuery.isFetchingNextPage}
						<div class="flex justify-center p-2">
							<div
								class="loading loading-spinner loading-sm"
							></div>
						</div>
					{/if}
				</VList>
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
