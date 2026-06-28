<script lang="ts">
	import { page } from '$app/stores';
	import Icon from '@iconify/svelte';

	const sections = [
		{
			title: 'Getting Started',
			icon: 'lucide:rocket',
			items: [
				{ title: 'Introduction', href: '/docs', exact: true },
				{ title: 'Installation', href: '/docs/installation' },
				{ title: 'Quick Start', href: '/docs/quick-start' },
				{ title: 'Authentication', href: '/docs/authentication' }
			]
		},
		{
			title: 'Core Concepts',
			icon: 'lucide:book-open',
			items: [
				{ title: 'Project Structure', href: '/docs/project-structure' },
				{ title: 'Configuration Files', href: '/docs/configuration' },
				{ title: 'Package Linking', href: '/docs/package-linking' },
				{ title: 'TypeScript Integration', href: '/docs/typescript' }
			]
		},
		{
			title: 'Package Management',
			icon: 'lucide:package',
			items: [
				{ title: 'Project Management', href: '/docs/project-management' },
				{ title: 'Package Development', href: '/docs/package-development' },
				{ title: 'Inter-Package Dependencies', href: '/docs/inter-package-dependencies' },
				{ title: 'Publishing', href: '/docs/publishing' }
			]
		},
		{
			title: 'Team Collaboration',
			icon: 'lucide:users',
			items: [
				{ title: 'Team Management', href: '/docs/teams' },
				{ title: 'Permissions', href: '/docs/permissions' },
				{ title: 'Workflow Best Practices', href: '/docs/workflows' }
			]
		},
		{
			title: 'Deployment',
			icon: 'lucide:server',
			items: [
				{ title: 'Self-Hosting', href: '/docs/self-hosting' },
				{ title: 'Docker Integration', href: '/docs/docker' },
				{ title: 'Production Deployment', href: '/docs/production' }
			]
		},
		{
			title: 'Advanced',
			icon: 'lucide:settings',
			items: [
				{ title: 'Package Aliases', href: '/docs/aliases' },
				{ title: 'Build Optimization', href: '/docs/build-optimization' },
				{ title: 'Troubleshooting', href: '/docs/troubleshooting' }
			]
		}
	];

	function isActive(href: string, exact = false): boolean {
		if (exact) {
			return $page.url.pathname === href;
		}
		return $page.url.pathname.startsWith(href) && $page.url.pathname !== '/docs';
	}

	// Get current page info for breadcrumbs
	$: currentPageInfo = (() => {
		for (const section of sections) {
			for (const item of section.items) {
				if (isActive(item.href, item.exact)) {
					return { section: section.title, page: item.title };
				}
			}
		}
		return { section: '', page: '' };
	})();

	let sidebarOpen = false;

	// Close sidebar on escape key
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape' && sidebarOpen) {
			sidebarOpen = false;
		}
	}
</script>

<svelte:head>
	<title>Documentation - Spore</title>
	<meta name="description" content="Complete guide to using Spore CLI for monorepo package management, TypeScript integration, and team collaboration." />

	<!-- Essential Meta Tags -->
	<meta name="keywords" content="spore cli, monorepo, package manager, typescript, javascript, package registry, team collaboration, build tools" />
	<meta name="author" content="Spore" />
	<meta name="robots" content="index, follow" />
	<meta name="language" content="en" />
	<meta name="theme-color" content="#ffffff" />

	<!-- Open Graph Meta Tags -->
	<meta property="og:type" content="website" />
	<meta property="og:title" content="Spore CLI Documentation - Modern Monorepo Package Manager" />
	<meta property="og:description" content="Complete guide to using Spore CLI for monorepo package management, TypeScript integration, and team collaboration." />
	<meta property="og:image" content="/images/og/docs-main.png" />
	<meta property="og:image:width" content="1200" />
	<meta property="og:image:height" content="630" />
	<meta property="og:image:alt" content="Spore CLI Documentation - Modern Monorepo Package Manager" />
	<meta property="og:url" content="https://spore.klysium.com/docs" />
	<meta property="og:site_name" content="Spore" />
	<meta property="og:locale" content="en_US" />

	<!-- Twitter Card Meta Tags -->
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:site" content="@sporespace" />
	<meta name="twitter:creator" content="@sporespace" />
	<meta name="twitter:title" content="Spore CLI Documentation - Modern Monorepo Package Manager" />
	<meta name="twitter:description" content="Complete guide to using Spore CLI for monorepo package management, TypeScript integration, and team collaboration." />
	<meta name="twitter:image" content="/images/og/docs-main.png" />
	<meta name="twitter:image:alt" content="Spore CLI Documentation" />

	<!-- Additional SEO Meta Tags -->
	<link rel="canonical" href="https://spore.klysium.com/docs" />
	<meta name="format-detection" content="telephone=no" />
	<meta name="HandheldFriendly" content="true" />

	<!-- Structured Data -->
	<script type="application/ld+json">
	{
		"@context": "https://schema.org",
		"@type": "TechArticle",
		"headline": "Spore CLI Documentation",
		"description": "Complete guide to using Spore CLI for monorepo package management, TypeScript integration, and team collaboration.",
		"author": {
			"@type": "Organization",
			"name": "Spore"
		},
		"publisher": {
			"@type": "Organization",
			"name": "Spore",
			"logo": {
				"@type": "ImageObject",
				"url": "https://spore.klysium.com/images/logo.png"
			}
		},
		"datePublished": "2025-01-01",
		"dateModified": "2025-01-15",
		"mainEntityOfPage": {
			"@type": "WebPage",
			"@id": "https://spore.klysium.com/docs"
		},
		"image": "https://spore.klysium.com/images/og/docs-main.png",
		"articleSection": "Documentation",
		"keywords": ["monorepo", "package manager", "typescript", "cli tools", "build tools"]
	}
	</script>
</svelte:head>

<svelte:window on:keydown={handleKeydown} />

<div class="min-h-screen bg-card">
	<!-- Mobile sidebar overlay -->
	{#if sidebarOpen}
		<div class="fixed inset-0 z-40 lg:hidden">
			<div
				class="fixed inset-0 bg-black/30"
				role="button"
				tabindex="0"
				on:click={() => sidebarOpen = false}
				on:keydown={(e) => { if (e.key === 'Escape') sidebarOpen = false; }}
				aria-label="Close sidebar"
			></div>
		</div>
	{/if}

	<!-- Sidebar -->
	<div class="fixed inset-y-0 left-0 z-50 w-80 bg-card border-r border-border transform {sidebarOpen ? 'translate-x-0' : '-translate-x-full'} transition-transform duration-200 ease-out lg:translate-x-0">
		<div class="flex h-full flex-col">
			<!-- Header -->
			<div class="flex h-20 items-center justify-between px-8 border-b border-border">
				<div class="flex items-center space-x-4">
					<a href="/docs" class="flex items-center space-x-3 group">
						<div class="w-8 h-8 bg-muted rounded-lg flex items-center justify-center group-hover:bg-gray-200 transition-colors duration-200">
							<Icon icon="lucide:book-open" class="w-4 h-4 text-muted-foreground" />
						</div>
						<h1 class="text-xl font-semibold text-foreground group-hover:text-foreground transition-colors duration-200" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
							Spore Docs
						</h1>
					</a>
				</div>
				<div class="flex items-center space-x-3">
					<a href="/" class="flex items-center text-sm text-muted-foreground hover:text-foreground transition-colors duration-200 px-3 py-1.5 rounded-md hover:bg-muted">
						<Icon icon="lucide:arrow-left" class="w-3 h-3 mr-1.5" />
						Back to Spore
					</a>
					<button
						on:click={() => sidebarOpen = false}
						class="lg:hidden p-2 rounded-lg hover:bg-muted text-muted-foreground hover:text-foreground transition-colors duration-200"
					>
						<Icon icon="lucide:x" class="w-5 h-5" />
					</button>
				</div>
			</div>

			<!-- Navigation -->
			<nav class="flex-1 overflow-y-auto py-6 px-4">
				<div class="space-y-6">
					{#each sections as section}
						<div>
							<div class="flex items-center space-x-2 mb-3 px-2">
								<Icon icon={section.icon} class="w-4 h-4 text-muted-foreground" />
								<h3 class="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
									{section.title}
								</h3>
							</div>
							<ul class="space-y-1">
								{#each section.items as item}
									<li>
										<a
											href={item.href}
											class="block px-3 py-2 text-sm rounded-md transition-colors {isActive(item.href, item.exact)
												? 'bg-blue-50 text-blue-700 font-medium'
												: 'text-muted-foreground hover:text-foreground hover:bg-muted'}"
											on:click={() => sidebarOpen = false}
										>
											<div class="flex items-center">
												<div class="w-1.5 h-1.5 {isActive(item.href, item.exact) ? 'bg-blue-600' : 'bg-gray-400'} rounded-full mr-3"></div>
												{item.title}
											</div>
										</a>
									</li>
								{/each}
							</ul>
						</div>
					{/each}
				</div>
			</nav>

			<!-- Footer -->
			<div class="border-t border-border p-6 bg-muted">
				<div class="flex items-center justify-between text-xs">
					<div class="flex items-center space-x-2">
						<div class="w-2 h-2 bg-green-500 rounded-full"></div>
						<span class="text-muted-foreground font-medium">Spore v1.0.0</span>
					</div>
					<a href="https://github.com/saravenpi/spore" class="flex items-center space-x-2 text-muted-foreground hover:text-foreground transition-colors duration-200 px-3 py-1.5 rounded-md hover:bg-muted group">
						<Icon icon="lucide:github" class="w-4 h-4" />
						<span class="font-medium">GitHub</span>
					</a>
				</div>
			</div>
		</div>
	</div>

	<!-- Main content -->
	<div class="lg:pl-80">
		<!-- Top bar for mobile -->
		<div class="sticky top-0 z-30 lg:hidden bg-card border-b border-border">
			<div class="h-16 flex items-center justify-between px-4">
				<button
					on:click={() => sidebarOpen = true}
					class="p-2.5 rounded-lg hover:bg-muted transition-colors duration-200"
					aria-label="Open navigation menu"
				>
					<Icon icon="lucide:menu" class="w-5 h-5 text-muted-foreground" />
				</button>
				<div class="flex items-center space-x-2">
					<div class="w-6 h-6 bg-muted rounded-md flex items-center justify-center">
						<Icon icon="lucide:book-open" class="w-3 h-3 text-muted-foreground" />
					</div>
					<div class="flex flex-col items-center">
						<h1 class="text-lg font-semibold text-foreground" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
							Spore Docs
						</h1>
						{#if currentPageInfo.section && $page.url.pathname !== '/docs'}
							<div class="text-xs text-muted-foreground font-medium">
								{currentPageInfo.section}
							</div>
						{/if}
					</div>
				</div>
				<div class="w-9"></div> <!-- Spacer for centering -->
			</div>
			
			{#if currentPageInfo.page && $page.url.pathname !== '/docs'}
				<div class="px-4 pb-3 border-t border-border">
					<div class="flex items-center space-x-1 text-sm mt-2">
						<a href="/docs" class="text-muted-foreground hover:text-foreground transition-colors font-medium">Docs</a>
						<Icon icon="lucide:chevron-right" class="w-3 h-3 text-gray-400" />
						<span class="text-foreground font-semibold">{currentPageInfo.page}</span>
					</div>
				</div>
			{/if}
		</div>

		<!-- Page content -->
		<main class="min-h-screen bg-card">
			<slot />
		</main>
	</div>
</div>
