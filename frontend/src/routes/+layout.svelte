<script lang="ts">
	import "./layout.css";
	import favicon from "$lib/assets/favicon.svg";
	import { Button } from "$lib/components/ui/button";

	let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<div
	class="min-h-screen bg-background text-foreground font-sans overflow-x-hidden"
>
	<!-- Header -->
	<header
		class="sticky top-0 z-50 w-full border-b border-border/50 bg-background/80 backdrop-blur-md"
	>
		<div
			class="container flex h-14 max-w-screen-2xl items-center mx-auto px-4"
		>
			<div class="mr-4 flex">
				<a class="mr-6 flex items-center space-x-2" href="/">
					<span class="font-bold text-lg tracking-tight"
						>Stitch-up</span
					>
				</a>
			</div>
			<div
				class="flex flex-1 items-center justify-between space-x-2 md:justify-end"
			>
				<div class="w-full flex-1 md:w-auto md:flex-none">
					<!-- Search or other nav items can go here -->
				</div>
				<nav class="flex items-center space-x-1">
					<Button variant="ghost" size="sm" class="h-8 w-8 px-0">
						<span class="sr-only">GitHub</span>
						<!-- Icon placeholder -->
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="24"
							height="24"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
							class="lucide lucide-github"
							><path
								d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"
							/><path d="M9 18c-4.51 2-5-2-7-2" /></svg
						>
					</Button>
				</nav>
			</div>
		</div>
	</header>

	<!-- Main Content -->
	<main class="relative z-10 flex-1">
		{@render children()}
	</main>
</div>
