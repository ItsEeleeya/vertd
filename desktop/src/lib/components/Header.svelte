<script lang="ts">
	import { type as ostype } from "@tauri-apps/plugin-os";
	import Logo from "$lib/assets/Logo.svg?component";
</script>

<header
	data-tauri-drag-region
	class="fixed top-0 h-11 w-full flex items-center justify-between transition-colors duration-200"
>
	<div
		data-tauri-drag-region
		class="flex-none min-w-40 flex items-center h-7 *:h-full"
	>
		{#if ostype() === "macos"}
			<div data-tauri-drag-region class="w-20"></div>
		{/if}

		<!-- <button
			type="button"
			class="btn btn-primary btn-xs btn-simple py-3 text-primary"
		>
			{"<"} Back
		</button> -->
	</div>

	<div
		data-tauri-drag-region
		class="flex-1 h-3 max-w-md flex justify-center items-center text-primary px-2"
	>
		<Logo
			aria-hidden="true"
			class="w-10 fill-current pointer-events-none"
		/>
	</div>

	<div
		data-tauri-drag-region
		class="flex-none min-w-40 flex items-center justify-end p-2 h-8 gap-2"
	>
		…
	</div>
</header>
