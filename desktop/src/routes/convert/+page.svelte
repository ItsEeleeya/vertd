<script lang="ts">
	// TODO: Update TanStack Query and Virtual once they are updated to support Runes.
	import {
		createInfiniteQuery,
		createMutation,
	} from "@tanstack/svelte-query";
	import { createVirtualizer } from "@tanstack/svelte-virtual";
	import { commands } from "../../bindings";
	import { onMount } from "svelte";

	const LIMIT = 5;
	async function fetchTasks({
		pageParam = 0,
	}): Promise<{ rows: any[]; nextCursor?: number }> {
		console.log("Fetching tasks, page:", pageParam);
		const result = await commands.getTasks(null, pageParam, LIMIT);
		console.log(JSON.stringify(result));
		if (result.status === "error") {
			console.error("Error fetching tasks:", result.error);
			throw new Error(result.error);
		}

		const hasMore = result.data.length === LIMIT;
		console.log(
			"Fetched tasks:",
			result.data.length,
			"Next cursor:",
			hasMore ? pageParam + LIMIT : undefined,
		);

		return {
			rows: result.data,
			nextCursor: hasMore ? pageParam + LIMIT : undefined,
		};
	}

	const query = createInfiniteQuery({
		queryKey: ["projects"],
		queryFn: ({ pageParam }) => fetchTasks({ pageParam }),
		initialPageParam: 0,
		getNextPageParam: (lastPage) => lastPage.nextCursor,
	});

	$: allRows =
		($query.data && $query.data.pages.flatMap((page) => page.rows)) || [];

	let virtualListEl: HTMLDivElement;
	$: virtualizer = createVirtualizer<HTMLDivElement, HTMLDivElement>({
		count: 0,
		getScrollElement: () => virtualListEl,
		estimateSize: () => 100,
		overscan: 20,
	});

	$: {
		$virtualizer.setOptions({
			count: $query.hasNextPage ? allRows.length + 1 : allRows.length,
		});

		const [lastItem] = [...$virtualizer.getVirtualItems()].reverse();

		if (
			lastItem &&
			lastItem.index > allRows.length - 1 &&
			$query.hasNextPage &&
			!$query.isFetchingNextPage
		) {
			$query.fetchNextPage();
		}
	}

	async function fetchServerPage(
		limit: number,
		offset: number = 0,
	): Promise<{ rows: string[]; nextOffset: number }> {
		const rows = new Array(limit)
			.fill(0)
			.map((_e, i) => `Async loaded row #${i + (offset - 1) * limit}`);

		await new Promise((r) => setTimeout(r, 500));
		return { rows, nextOffset: offset + 1 };
	}

	$: addMock = createMutation({
		mutationFn: async (count: number) => {
			console.log("Adding mock tasks:", count);
			const result = await commands.addMockTasks(count);
			console.log("Mock tasks added result:", result);
			return result;
		},
		onSuccess: () => {
			console.log("Add mock success, refetching tasksQuery");
			$query.refetch();
		},
		onError: (error) => {
			console.error("Error adding mock tasks:", error);
		},
	});

	onMount(() => {
		$addMock.mutate(20);
	});
</script>

<main>
	<div>
		<button onclick={commands.printAll} class="btn btn-sm btn-accent m-4"
			>Print all
		</button>
	</div>
	{#if $query.isLoading}
		Loading...
	{:else if $query.isError}
		<span>Error: {$query.error.message}</span>
	{:else if $query.isSuccess}
		<div
			class="border border-base-content/20 p-2 mx-1 h-100 rounded-md overflow-auto overscroll"
			bind:this={virtualListEl}
		>
			<div
				class="relative w-full"
				style="height:{$virtualizer.getTotalSize()}px;"
			>
				{#each $virtualizer.getVirtualItems() as row (row.index)}
					{@const task = allRows[row.index]}
					<div
						class:list-item-even={row.index % 2 === 0}
						class:list-item-odd={row.index % 2 === 1}
						style="position: absolute; top: 0; left: 0; width: 100%; height: {row.size}px; transform: translateY({row.start}px);"
					>
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
					</div>
				{/each}
			</div>
		</div>
	{/if}
	{#if $query.isFetching && !$query.isFetchingNextPage}
		<p>Background updating...</p>
	{/if}
</main>
