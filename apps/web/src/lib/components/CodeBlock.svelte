<script lang="ts">
	export let label: string = '';
	export let copy: string | null = null;

	let copied = false;
	async function doCopy() {
		if (!copy) return;
		try {
			await navigator.clipboard.writeText(copy);
			copied = true;
			setTimeout(() => (copied = false), 1500);
		} catch (err) {
			console.error('Failed to copy to clipboard:', err);
		}
	}
</script>

<div class="overflow-hidden rounded-xl border border-white/10 bg-zinc-950 shadow-sm">
	<div class="flex items-center justify-between border-b border-white/10 px-4 py-2.5">
		<div class="flex items-center gap-2">
			<span class="flex gap-1.5" aria-hidden="true">
				<span class="h-2.5 w-2.5 rounded-full bg-white/15"></span>
				<span class="h-2.5 w-2.5 rounded-full bg-white/15"></span>
				<span class="h-2.5 w-2.5 rounded-full bg-white/15"></span>
			</span>
			{#if label}
				<span class="ml-1 font-mono text-xs text-zinc-500">{label}</span>
			{/if}
		</div>
		{#if copy}
			<button
				on:click={doCopy}
				class="text-zinc-500 transition-colors hover:text-zinc-200"
				title="Copy to clipboard"
				aria-label="Copy to clipboard"
			>
				{#if copied}
					<svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
					</svg>
				{:else}
					<svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
						/>
					</svg>
				{/if}
			</button>
		{/if}
	</div>
	<div class="overflow-x-auto p-4">
		<pre class="font-mono text-[13px] leading-relaxed text-zinc-200"><slot /></pre>
	</div>
</div>
