<script lang="ts">
	import { type as ostype } from "@tauri-apps/plugin-os";
	import Logo from "$lib/assets/Logo.svg?component";
	import { LucideEllipsis } from "lucide-svelte";
	import { Menu, MenuItem } from "@tauri-apps/api/menu";
	import { LogicalPosition } from "@tauri-apps/api/dpi";

	async function showMenu(event: MouseEvent) {
		const menu = await Menu.new({
			id: "appmenu",
			items: [
				await MenuItem.new({
					id: "open_settings",
					text: "Settings",
				}),
				await MenuItem.new({
					id: "open_about",
					text: "About"
				})
			],
		});

		const buttonElement = event.target as HTMLElement;
		const rect = buttonElement.getBoundingClientRect();
		console.log(`${JSON.stringify(rect)}`);

		menu.popup(new LogicalPosition(rect.left, rect.bottom + 5));
	}
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
		class="flex-1 h-3 max-w-md flex justify-center items-center text-neutral-50/80 px-2"
	>
		<Logo
			aria-hidden="true"
			class="w-10 fill-current pointer-events-none"
		/>
	</div>

	<div
		data-tauri-drag-region
		class="flex-none min-w-40 flex items-center justify-end p-2 h-6 gap-2"
	>
		<button
			type="button"
			onclick={showMenu}
			class="btn btn-square btn-xs btn-ghost btn-transparent"
		>
			<LucideEllipsis
				size="18"
				aria-hidden="true"
				class="pointer-events-none"
			/>
		</button>
	</div>
</header>
