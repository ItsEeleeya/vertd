<script>
	import Header from "$lib/components/Header.svelte";
</script>

<div class="w-screen h-screen bg-vert-graident">
	<Header />
	<div class="pt-11 h-full">
		<slot />
	</div>
</div>
