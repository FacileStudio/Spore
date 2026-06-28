<script lang="ts">
	import { onMount } from 'svelte';
	import { packagesStore, authStore } from '$lib/stores';
	import { packagesApi } from '../lib/api';
	import { formatDownloadCount, formatLargeNumber } from '../lib/utils/format';
	import Icon from '@iconify/svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	const installCmd = 'curl -fsSL https://raw.githubusercontent.com/FacileStudio/Spore/main/install.sh | bash';

	$: packages = $packagesStore.packages;
	$: loading = $packagesStore.loading;
	$: initialized = $authStore.initialized;

	let stats = {
		totalPackages: 0,
		totalDownloads: 0,
		totalUsers: 0
	};
	let statsLoading = true;

	onMount(async () => {
		// Initialize auth first
		await authStore.initialize();

		// Load packages for home page and global stats in parallel
		try {
			const timeout = new Promise((_, reject) => {
				setTimeout(() => reject(new Error('Request timeout')), 3000);
			});

			try {
				const [, statsResult] = await Promise.all([
					Promise.race([packagesStore.fetchAll(), timeout]).catch(() => null),
					Promise.race([packagesApi.getGlobalStats(), timeout]).catch(() => null)
				]);

				if (statsResult) {
					stats = statsResult as any;
				}
			} catch (promiseError) {
				console.error('Promise handling error:', promiseError);
			}
			statsLoading = false;
		} catch (error) {
			console.error('Failed to fetch data:', error);
			stats = {
				totalPackages: 0,
				totalDownloads: 0,
				totalUsers: 0
			};
			statsLoading = false;
		}

		return () => {};
	});
</script>

<svelte:head>
	<title>Spore - Modern Package Registry for Monorepo Development</title>
	<meta name="description" content="The modern package registry for Spore monorepo packages. Publish, share, and manage packages for any programming language with teams. Fast publishing, team collaboration, and secure infrastructure." />
	<meta name="keywords" content="package registry, monorepo, package manager, team collaboration, private packages, multi-language, development tools, spore" />
	<meta name="author" content="Spore" />
	<meta name="viewport" content="width=device-width, initial-scale=1.0" />

	<!-- Open Graph / Facebook -->
	<meta property="og:type" content="website" />
	<meta property="og:url" content="https://spore.facile.studio/" />
	<meta property="og:title" content="Spore - Modern Package Registry for Monorepo Development" />
	<meta property="og:description" content="The modern package registry for Spore monorepo packages. Publish, share, and manage packages for any programming language with teams." />
	<meta property="og:image" content="https://spore.facile.studio/images/og/home.png" />
	<meta property="og:site_name" content="Spore" />
	<meta property="og:locale" content="en_US" />

	<!-- Twitter -->
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:url" content="https://spore.facile.studio/" />
	<meta name="twitter:title" content="Spore - Modern Package Registry for Monorepo Development" />
	<meta name="twitter:description" content="The modern package registry for Spore monorepo packages. Publish, share, and manage packages for any programming language with teams." />
	<meta name="twitter:image" content="https://spore.facile.studio/images/og/home.png" />

	<!-- Additional SEO -->
	<meta name="robots" content="index, follow" />
	<link rel="canonical" href="https://spore.facile.studio/" />
</svelte:head>

{#if !initialized}
	<!-- Loading state while checking authentication -->
	<div class="flex min-h-[50vh] items-center justify-center">
		<div class="text-center">
			<div class="mx-auto mb-4 h-8 w-8 animate-spin rounded-full border-b-2 border-foreground"></div>
			<p class="text-muted-foreground">Loading...</p>
		</div>
	</div>
{:else}
	<!-- Hero -->
	<section class="relative overflow-hidden py-16 md:py-24">
		<div
			class="absolute -inset-[500px] rotate-12"
			style="background-image: radial-gradient(circle, #9ca3af 2px, transparent 2px);
				background-size: 24px 24px;
				mask-image: radial-gradient(ellipse 80% 60% at center, black 20%, transparent 80%);
				-webkit-mask-image: radial-gradient(ellipse 80% 60% at center, black 20%, transparent 80%);
				opacity: 0.12;"
		></div>
		<div class="relative z-10 mx-auto max-w-3xl text-center">
			<span class="inline-flex items-center gap-2 rounded-full border border-border bg-background/70 px-3 py-1 text-xs font-medium text-muted-foreground backdrop-blur">
				<span class="h-1.5 w-1.5 rounded-full bg-foreground"></span>
				Monorepo package manager for any language
			</span>
			<h1 class="mt-6 text-4xl font-bold tracking-tight md:text-6xl" style="font-family: 'Goga', sans-serif;">
				Ship packages<br />without the friction.
			</h1>
			<p class="mx-auto mt-6 max-w-xl text-lg text-muted-foreground">
				Spore is a fast, private package registry for monorepos. Publish, link, and
				share packages across apps and teams in any language.
			</p>
			<div class="mt-8 flex flex-col items-center justify-center gap-3 sm:flex-row">
				<a
					href="/packages"
					class="inline-flex items-center gap-2 rounded-lg bg-primary px-6 py-3 font-medium text-primary-foreground transition-colors hover:bg-primary/90"
				>
					<Icon icon="solar:box-bold-duotone" class="h-5 w-5" />
					Browse packages
				</a>
				<a
					href="/docs"
					class="inline-flex items-center gap-2 rounded-lg border border-border px-6 py-3 font-medium transition-colors hover:bg-accent hover:text-accent-foreground"
				>
					<Icon icon="solar:book-bold-duotone" class="h-5 w-5" />
					Read the docs
				</a>
			</div>
			<div class="mx-auto mt-10 max-w-2xl text-left">
				<CodeBlock label="bash" copy={installCmd}>
<span class="text-zinc-600">$ </span><span class="text-zinc-100">curl -fsSL https://raw.githubusercontent.com/FacileStudio/Spore/main/install.sh | bash</span></CodeBlock>
			</div>
		</div>
	</section>

	<!-- Stats -->
	<section class="grid grid-cols-3 gap-6 border-y border-border py-10">
		{#each [{ label: 'Packages', value: stats.totalPackages }, { label: 'Downloads', value: stats.totalDownloads }, { label: 'Developers', value: stats.totalUsers }] as stat}
			<div class="text-center">
				{#if statsLoading}
					<div class="mx-auto mb-2 h-9 w-16 animate-pulse rounded bg-muted"></div>
				{:else}
					<div class="mb-1 text-3xl font-bold tracking-tight md:text-4xl" style="font-family: 'Goga', sans-serif;">
						{formatLargeNumber(stat.value)}
					</div>
				{/if}
				<div class="text-sm text-muted-foreground">{stat.label}</div>
			</div>
		{/each}
	</section>

	<!-- Recent Packages -->
	<section class="py-16">
		<div class="mb-8 flex items-end justify-between">
			<div>
				<h2 class="text-2xl font-bold md:text-3xl" style="font-family: 'Goga', sans-serif;">Recent packages</h2>
				<p class="mt-1 text-muted-foreground">Freshly published to the registry.</p>
			</div>
			<a href="/packages" class="shrink-0 text-sm font-medium text-muted-foreground transition-colors hover:text-foreground">
				View all →
			</a>
		</div>

		{#if loading}
			<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
				{#each Array(6) as _item, index}
					<div class="animate-pulse rounded-xl border border-border p-6" key={index}>
						<div class="mb-2 h-4 w-3/4 rounded bg-muted"></div>
						<div class="mb-4 h-3 w-1/2 rounded bg-muted"></div>
						<div class="h-3 w-full rounded bg-muted"></div>
					</div>
				{/each}
			</div>
		{:else if packages.length === 0}
			<div class="rounded-xl border border-dashed border-border py-16 text-center">
				<Icon icon="solar:box-bold-duotone" class="mx-auto mb-4 h-14 w-14 text-muted-foreground" />
				<h3 class="text-lg font-semibold">No packages yet</h3>
				<p class="mb-5 mt-1 text-muted-foreground">Be the first to publish a package to Spore.</p>
				<a
					href="/login"
					class="inline-flex items-center gap-2 rounded-lg bg-primary px-5 py-2.5 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90"
				>
					<Icon icon="solar:rocket-2-bold-duotone" class="h-4 w-4" />
					Get started
				</a>
			</div>
		{:else}
			<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
				{#each packages.slice(0, 6) as pkg}
					<a
						href="/packages/{pkg.name}"
						class="group block rounded-xl border border-border p-6 transition-all hover:border-foreground/30 hover:shadow-sm"
					>
						<div class="mb-3 flex items-start justify-between">
							<div class="min-w-0">
								<h3 class="truncate font-semibold leading-none">{pkg.name}</h3>
								<p class="mt-1.5 text-sm text-muted-foreground">v{pkg.version}</p>
							</div>
							{#if pkg.team}
								<span class="shrink-0 rounded-full border border-border px-2 py-0.5 text-xs text-muted-foreground">
									{pkg.team.name}
								</span>
							{/if}
						</div>
						{#if pkg.description}
							<p class="mb-4 line-clamp-2 text-sm text-muted-foreground">{pkg.description}</p>
						{/if}
						<div class="flex items-center justify-between text-xs text-muted-foreground">
							<span>by {pkg.owner.username}</span>
							<span>{formatDownloadCount(pkg.totalDownloadsCount || pkg.downloadsCount)} downloads</span>
						</div>
					</a>
				{/each}
			</div>
		{/if}
	</section>

	<!-- How It Works -->
	<section class="border-t border-border py-16">
		<div class="mb-12 text-center">
			<h2 class="text-2xl font-bold md:text-3xl" style="font-family: 'Goga', sans-serif;">How it works</h2>
			<p class="mt-2 text-muted-foreground">From zero to a linked monorepo in two commands.</p>
		</div>

		<div class="mx-auto grid max-w-6xl grid-cols-1 gap-10 lg:grid-cols-2">
			<!-- Step 1 -->
			<div class="space-y-4">
				<div class="flex items-center gap-3">
					<span class="flex h-8 w-8 items-center justify-center rounded-full bg-primary text-sm font-semibold text-primary-foreground">1</span>
					<h3 class="text-lg font-semibold">Initialize your project</h3>
				</div>
				<CodeBlock label="Terminal">
<span class="text-zinc-600">$ </span><span class="text-zinc-100">spore init MyProject</span>
<span class="text-zinc-400">✅ Initialized new Spore project: MyProject</span>
<span class="text-zinc-400">📁 Created directories: packages/, apps/</span>
<span class="text-zinc-500">💡 Use 'spore init:package &lt;name&gt;' to create packages</span></CodeBlock>
				<CodeBlock label="spore.yml">
<span class="text-zinc-400">name:</span> <span class="text-zinc-100">MyProject</span>
<span class="text-zinc-400">description:</span> <span class="text-zinc-100">A modern TypeScript monorepo</span>
<span class="text-zinc-400">apps:</span>
  <span class="text-zinc-400">frontend:</span>
    <span class="text-zinc-400">tsAlias:</span> <span class="text-zinc-100">"#"</span>
    <span class="text-zinc-400">packages:</span> <span class="text-zinc-100">[types, utils]</span>
<span class="text-zinc-400">scripts:</span>
  <span class="text-zinc-400">setup:</span> <span class="text-zinc-100">"npm install"</span>
  <span class="text-zinc-400">test-all:</span> <span class="text-zinc-100">"spore run test --all"</span></CodeBlock>
			</div>

			<!-- Step 2 -->
			<div class="space-y-4">
				<div class="flex items-center gap-3">
					<span class="flex h-8 w-8 items-center justify-center rounded-full bg-primary text-sm font-semibold text-primary-foreground">2</span>
					<h3 class="text-lg font-semibold">Create packages</h3>
				</div>
				<CodeBlock label="Terminal">
<span class="text-zinc-600">$ </span><span class="text-zinc-100">spore init:package utils --team myteam</span>
<span class="text-zinc-400">📦 Initialized new package: utils</span></CodeBlock>
				<CodeBlock label="packages/utils/package.yml">
<span class="text-zinc-400">name:</span> <span class="text-zinc-100">utils</span>
<span class="text-zinc-400">team:</span> <span class="text-zinc-100">myteam</span>
<span class="text-zinc-400">version:</span> <span class="text-zinc-100">1.0.0</span>
<span class="text-zinc-400">description:</span> <span class="text-zinc-100">Shared utility functions</span>
<span class="text-zinc-400">tags:</span> <span class="text-zinc-100">[utilities, helpers]</span>
<span class="text-zinc-400">scripts:</span>
  <span class="text-zinc-400">build:</span> <span class="text-zinc-100">"tsc && rollup -c"</span>
  <span class="text-zinc-400">test:</span> <span class="text-zinc-100">"vitest run"</span></CodeBlock>
			</div>
		</div>
	</section>

	<!-- Essential Commands -->
	<section class="border-t border-border py-16">
		<div class="mb-12 text-center">
			<h2 class="text-2xl font-bold md:text-3xl" style="font-family: 'Goga', sans-serif;">Essential commands</h2>
			<p class="mt-2 text-muted-foreground">The everyday toolkit.</p>
		</div>

		<div class="mx-auto grid max-w-5xl grid-cols-1 gap-5 md:grid-cols-2">
			{#each [
				{ icon: 'solar:link-bold-duotone', name: 'Link packages', cmd: 'spore link', out: ['🔗 Successfully copied all packages and', '   updated TypeScript configurations'] },
				{ icon: 'solar:rocket-2-bold-duotone', name: 'Publish package', cmd: 'spore publish --team myteam', out: ['📦 Successfully published utils v1.0.0', '   Team: myteam'] },
				{ icon: 'solar:play-bold-duotone', name: 'Run scripts', cmd: 'spore run dev', out: ['🚀 Running app script \'dev\'...', '📝 Command: vite dev --port 3000'] },
				{ icon: 'solar:hammer-bold-duotone', name: 'Build apps', cmd: 'spore build', out: ['🔨 Building all apps...', '✅ Successfully built: 2/2 apps'] }
			] as c}
				<div class="rounded-xl border border-border p-5">
					<div class="mb-3 flex items-center gap-2">
						<Icon icon={c.icon} class="h-5 w-5 text-foreground" />
						<span class="font-semibold">{c.name}</span>
					</div>
					<CodeBlock label="Terminal">
<span class="text-zinc-600">$ </span><span class="text-zinc-100">{c.cmd}</span>
{#each c.out as line}<span class="text-zinc-400">{line}</span>
{/each}</CodeBlock>
				</div>
			{/each}
		</div>
	</section>

	<!-- Why Spore -->
	<section class="border-t border-border py-16">
		<div class="mb-12 text-center">
			<h2 class="text-2xl font-bold md:text-3xl" style="font-family: 'Goga', sans-serif;">Why Spore</h2>
		</div>
		<div class="mx-auto grid max-w-5xl grid-cols-1 gap-5 md:grid-cols-3">
			{#each [
				{ icon: 'solar:rocket-2-bold-duotone', title: 'Fast publishing', body: 'Publish packages instantly with the Spore CLI. Simple, secure, and integrated with your workflow.' },
				{ icon: 'solar:users-group-two-rounded-bold-duotone', title: 'Team collaboration', body: 'Create teams, manage permissions, and collaborate on packages with role-based access control.' },
				{ icon: 'solar:shield-check-bold-duotone', title: 'Secure & reliable', body: 'Built with security first. JWT authentication, checksums, and secure file storage.' }
			] as f}
				<div class="rounded-xl border border-border p-6">
					<div class="mb-4 flex h-11 w-11 items-center justify-center rounded-lg border border-border">
						<Icon icon={f.icon} class="h-6 w-6 text-foreground" />
					</div>
					<h3 class="mb-2 text-lg font-semibold">{f.title}</h3>
					<p class="text-sm text-muted-foreground">{f.body}</p>
				</div>
			{/each}
		</div>
	</section>

	<!-- Final CTA -->
	<section class="border-t border-border py-16">
		<div class="mx-auto max-w-2xl rounded-2xl border border-border bg-card p-8 text-center md:p-12">
			<h2 class="text-2xl font-bold md:text-3xl" style="font-family: 'Goga', sans-serif;">Ready to get started?</h2>
			<p class="mx-auto mt-3 max-w-md text-muted-foreground">
				Install the Spore CLI and start building your monorepo today.
			</p>
			<div class="mx-auto mt-6 max-w-xl text-left">
				<CodeBlock label="bash" copy={installCmd}>
<span class="text-zinc-600">$ </span><span class="text-zinc-100">curl -fsSL https://raw.githubusercontent.com/FacileStudio/Spore/main/install.sh | bash</span></CodeBlock>
			</div>
			<div class="mt-6 flex flex-col justify-center gap-3 sm:flex-row">
				<a
					href="/register"
					class="inline-flex items-center justify-center gap-2 rounded-lg bg-primary px-6 py-3 font-medium text-primary-foreground transition-colors hover:bg-primary/90"
				>
					<Icon icon="solar:rocket-2-bold-duotone" class="h-4 w-4" />
					Create account
				</a>
				<a
					href="/docs"
					class="inline-flex items-center justify-center gap-2 rounded-lg border border-border px-6 py-3 font-medium transition-colors hover:bg-accent hover:text-accent-foreground"
				>
					<Icon icon="solar:book-bold-duotone" class="h-4 w-4" />
					Read the docs
				</a>
			</div>
		</div>
	</section>
{/if}
