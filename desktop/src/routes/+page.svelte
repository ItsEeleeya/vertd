<script lang="ts">
</script>

<main class="h-full flex flex-col *:p-3 *:w-full">
	<div class="h-full">
		<div
			class="size-full rounded-lg border-3 border-dashed border-neutral-50/20 bg-[var(--bg)]/60 flex flex-col items-center justify-center gap-2"
		>
			<div
				class="rounded-full bg-accent size-10 flex items-center justify-center text-black text-4xl"
			>
				+
			</div>
			Drop or click to add
		</div>
	</div>
	<div class="h-18 flex items-center justify-between gap-4">
		<div class="flex gap-2 items-center justify-center">
			<p>Set all to</p>
			<select name="export-mode" class="btn max-w-25 text-ellipsis">
				<option value="mkv">.mkv</option>
				<option value="mp4">.mp4</option>
				<option value="webm">.webmeeee</option>
				<option value="avi">.avi</option>
			</select>
		</div>
		<div class="flex gap-2">
			<button type="button" class="btn">X</button>
			<button type="button" class="btn">Export all as .zip</button>
			<button type="button" class="btn highlight">Convert all</button>
		</div>
	</div>
</main>
