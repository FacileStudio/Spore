<script lang="ts">
	import Icon from '@iconify/svelte';

	let copyText = '';
	let showCopied = false;

	async function copyToClipboard(text: string) {
		try {
			await navigator.clipboard.writeText(text);
			copyText = text;
			showCopied = true;
			setTimeout(() => {
				showCopied = false;
			}, 2000);
		} catch (err) {
			console.error('Failed to copy text: ', err);
		}
	}
</script>

<svelte:head>
	<title>Publishing Packages - Spore CLI Documentation</title>
	<meta name="description" content="Learn how to publish packages to Spore registry, including version management, authentication, CI/CD workflows, and automated GitHub Actions." />
	<meta property="og:title" content="Publishing Packages - Spore CLI" />
	<meta property="og:description" content="Learn how to publish packages to Spore registry, including version management, authentication, CI/CD workflows, and automated GitHub Actions." />
	<meta property="og:image" content="/images/og/publishing.png" />
	<meta property="og:url" content="https://spore.facile.studio/docs/publishing" />
	<meta name="twitter:title" content="Publishing Packages - Spore CLI" />
	<meta name="twitter:description" content="Learn how to publish packages to Spore registry, including version management, authentication, CI/CD workflows, and automated GitHub Actions." />
	<meta name="twitter:image" content="/images/og/publishing.png" />
	<link rel="canonical" href="https://spore.facile.studio/docs/publishing" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Goga', 'Satoshi', sans-serif;">
			Publishing Packages
		</h1>
		<p class="text-lg text-muted-foreground">
			Share your packages with the world through Spore registry and team collaboration
		</p>
	</div>

	<!-- Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Overview</h2>
		<p class="text-muted-foreground leading-relaxed mb-6">
			Spore registry allows you to publish and share packages with your team or the public. 
			Whether you're building internal company libraries or contributing to open source, 
			Spore provides a seamless publishing experience with version management and access controls.
		</p>
		
		<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-muted rounded-lg flex items-center justify-center mb-4">
					<Icon icon="lucide:upload-bold" class="w-6 h-6 text-foreground" />
				</div>
				<h3 class="font-semibold mb-2">Easy Publishing</h3>
				<p class="text-sm text-muted-foreground">
					One-command publishing with automatic validation and version management.
				</p>
			</div>
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-muted rounded-lg flex items-center justify-center mb-4">
					<Icon icon="lucide:users-group-two-rounded-bold" class="w-6 h-6 text-foreground" />
				</div>
				<h3 class="font-semibold mb-2">Team Collaboration</h3>
				<p class="text-sm text-muted-foreground">
					Organize packages by teams with granular access controls and permissions.
				</p>
			</div>
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-muted rounded-lg flex items-center justify-center mb-4">
					<Icon icon="lucide:shield-check-bold" class="w-6 h-6 text-foreground" />
				</div>
				<h3 class="font-semibold mb-2">Version Management</h3>
				<p class="text-sm text-muted-foreground">
					Automatic semantic versioning with changelog generation and rollback support.
				</p>
			</div>
		</div>
	</section>

	<!-- Authentication -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Authentication</h2>
		
		<div class="space-y-8">
			<div>
				<h3 class="text-lg font-semibold mb-4">Getting Started</h3>
				<p class="text-muted-foreground mb-4">
					Before publishing packages, you need to authenticate with Spore registry.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Login to Spore</h4>
						<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-sm p-4 rounded-lg relative group">
							<code>spore auth</code>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('spore auth')}
							>
								{#if showCopied && copyText === 'spore auth'}
									<Icon icon="lucide:check-circle" class="w-4 h-4 text-zinc-100" />
								{:else}
									<Icon icon="lucide:copy" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<p class="text-sm text-muted-foreground mt-2">
							Opens browser for OAuth authentication or prompts for credentials.
						</p>
					</div>

					<div>
						<h4 class="font-medium mb-2">Verify Authentication</h4>
						<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-sm p-4 rounded-lg relative group">
							<code>spore auth</code>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('spore auth')}
							>
								{#if showCopied && copyText === 'spore auth'}
									<Icon icon="lucide:check-circle" class="w-4 h-4 text-zinc-100" />
								{:else}
									<Icon icon="lucide:copy" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<p class="text-sm text-muted-foreground mt-2">
							Shows your current authenticated user and associated teams.
						</p>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-4">Environment Configuration</h3>
				<p class="text-muted-foreground mb-4">
					Configure your registry URL and authentication settings for different environments.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Set Registry URL</h4>
						<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-sm p-4 rounded-lg relative group">
							<code>spore config set registry https://spore.mycompany.com</code>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('spore config set registry https://spore.mycompany.com')}
							>
								{#if showCopied && copyText.includes('mycompany')}
									<Icon icon="lucide:check-circle" class="w-4 h-4 text-zinc-100" />
								{:else}
									<Icon icon="lucide:copy" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Environment Variables</h4>
						<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-gray-400"># Set in your environment or CI/CD</span>
<span class="text-zinc-400">export</span> <span class="text-zinc-100">SPORE_TOKEN</span>=<span class="text-zinc-100">"your-auth-token"</span>
<span class="text-zinc-400">export</span> <span class="text-zinc-100">SPORE_SPACE_URL</span>=<span class="text-zinc-100">"https://spore.mycompany.com"</span></code></pre>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Publishing Process -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Publishing Process</h2>
		
		<div class="space-y-8">
			<div>
				<h3 class="text-lg font-semibold mb-4">Basic Publishing</h3>
				<p class="text-muted-foreground mb-4">
					Publish packages from their directory with automatic validation and build verification.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Publish Current Package</h4>
						<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-sm p-4 rounded-lg relative group">
							<code>cd packages/utils && spore publish</code>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('cd packages/utils && spore publish')}
							>
								{#if showCopied && copyText.includes('packages/utils')}
									<Icon icon="lucide:check-circle" class="w-4 h-4 text-zinc-100" />
								{:else}
									<Icon icon="lucide:copy" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<p class="text-sm text-muted-foreground mt-2">
							Publishes the package in the current directory to Spore registry.
						</p>
					</div>

					<div>
						<h4 class="font-medium mb-2">Pre-publication Checklist</h4>
						<div class="bg-muted border border-border rounded-lg p-6">
							<h5 class="font-semibold text-foreground mb-3">Spore automatically verifies:</h5>
							<ul class="text-sm text-foreground space-y-1">
								<li>✓ Package configuration is valid</li>
								<li>✓ Version follows semantic versioning</li>
								<li>✓ Build completes successfully</li>
								<li>✓ Tests pass (if test script exists)</li>
								<li>✓ No uncommitted changes (git)</li>
								<li>✓ Authentication and permissions</li>
							</ul>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Example Publishing Flow</h4>
						<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-xs p-4 rounded-lg">
							<pre><code><span class="text-zinc-100">$ cd packages/utils && spore publish</span>

<span class="text-zinc-400">🔍 Validating package configuration...</span>
<span class="text-zinc-100">✓ package.yml is valid</span>
<span class="text-zinc-100">✓ Version 1.2.0 follows semver</span>

<span class="text-zinc-400">🏗️ Building package...</span>
<span class="text-zinc-100">✓ TypeScript compilation successful</span>
<span class="text-zinc-100">✓ All tests pass</span>

<span class="text-zinc-400">📦 Publishing to Spore...</span>
<span class="text-zinc-100">✓ Package uploaded successfully</span>
<span class="text-zinc-100">✓ Registry updated</span>

<span class="text-zinc-100">🎉 Successfully published utils@1.2.0</span>
<span class="text-gray-400">   View at: https://spore.space/packages/utils</span></code></pre>
						</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-4">Publishing Options</h3>
				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Dry Run</h4>
						<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-sm p-4 rounded-lg relative group">
							<code>spore publish --dry-run</code>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('spore publish --dry-run')}
							>
								{#if showCopied && copyText === 'spore publish --dry-run'}
									<Icon icon="lucide:check-circle" class="w-4 h-4 text-zinc-100" />
								{:else}
									<Icon icon="lucide:copy" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<p class="text-sm text-muted-foreground mt-2">
							Validate and build without actually publishing to the registry.
						</p>
					</div>

					<div>
						<h4 class="font-medium mb-2">Force Publish</h4>
						<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-sm p-4 rounded-lg relative group">
							<code>spore publish --force</code>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('spore publish --force')}
							>
								{#if showCopied && copyText === 'spore publish --force'}
									<Icon icon="lucide:check-circle" class="w-4 h-4 text-zinc-100" />
								{:else}
									<Icon icon="lucide:copy" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<p class="text-sm text-muted-foreground mt-2">
							Skip some validation checks (use with caution in emergency situations).
						</p>
					</div>

					<div>
						<h4 class="font-medium mb-2">Tag Releases</h4>
						<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-sm p-4 rounded-lg relative group">
							<code>spore publish --tag beta</code>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('spore publish --tag beta')}
							>
								{#if showCopied && copyText.includes('--tag beta')}
									<Icon icon="lucide:check-circle" class="w-4 h-4 text-zinc-100" />
								{:else}
									<Icon icon="lucide:copy" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<p class="text-sm text-muted-foreground mt-2">
							Publish with a distribution tag (latest, beta, alpha, next).
						</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Team Publishing -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Team Publishing</h2>
		
		<div class="space-y-8">
			<div>
				<h3 class="text-lg font-semibold mb-4">Team-scoped Packages</h3>
				<p class="text-muted-foreground mb-4">
					Organize packages under team namespaces for better organization and access control.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Configure Team Namespace</h4>
						<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-sm p-4 rounded-lg relative group">
							<pre><code><span class="text-gray-400"># packages/shared-lib/package.yml</span>
<span class="text-zinc-400">name:</span> <span class="text-zinc-100">shared-lib</span>
<span class="text-zinc-400">team:</span> <span class="text-zinc-100">myteam</span>           <span class="text-gray-400"># Published as @myteam/shared-lib</span>
<span class="text-zinc-400">version:</span> <span class="text-zinc-100">1.0.0</span></code></pre>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard(`# packages/shared-lib/package.yml
name: shared-lib
team: myteam           # Published as @myteam/shared-lib
version: 1.0.0`)}
							>
								{#if showCopied && copyText.includes('shared-lib')}
									<Icon icon="lucide:check-circle" class="w-4 h-4 text-zinc-100" />
								{:else}
									<Icon icon="lucide:copy" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Publishing Team Package</h4>
						<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-sm p-4 rounded-lg relative group">
							<code>cd packages/shared-lib && spore publish</code>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('cd packages/shared-lib && spore publish')}
							>
								{#if showCopied && copyText.includes('shared-lib')}
									<Icon icon="lucide:check-circle" class="w-4 h-4 text-zinc-100" />
								{:else}
									<Icon icon="lucide:copy" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<p class="text-sm text-muted-foreground mt-2">
							Publishes to registry as <code>@myteam/shared-lib</code> with team access controls.
						</p>
					</div>

					<div>
						<h4 class="font-medium mb-2">Using Team Packages</h4>
						<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-gray-400"># In your app configuration</span>
<span class="text-zinc-400">packages:</span>
  - <span class="text-zinc-100">types</span>                    <span class="text-gray-400"># Local package</span>
  - <span class="text-zinc-100">"@myteam/shared-lib"</span>     <span class="text-gray-400"># Team package from registry</span></code></pre>
						</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-4">Access Control</h3>
				<p class="text-muted-foreground mb-4">
					Team packages inherit access permissions from team membership and roles.
				</p>

				<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
					<div class="border rounded-lg p-6">
						<h4 class="font-semibold mb-3 text-foreground">Team Members</h4>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• View team packages</li>
							<li>• Install team packages</li>
							<li>• View package metadata</li>
							<li>• Access documentation</li>
						</ul>
					</div>

					<div class="border rounded-lg p-6">
						<h4 class="font-semibold mb-3 text-foreground">Team Admins</h4>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• All member permissions</li>
							<li>• Publish new versions</li>
							<li>• Manage package settings</li>
							<li>• View download analytics</li>
						</ul>
					</div>

					<div class="border rounded-lg p-6">
						<h4 class="font-semibold mb-3 text-foreground">Team Owners</h4>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• All admin permissions</li>
							<li>• Delete packages</li>
							<li>• Transfer ownership</li>
							<li>• Manage team members</li>
						</ul>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Version Management -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Version Management</h2>
		
		<div class="space-y-8">
			<div>
				<h3 class="text-lg font-semibold mb-4">Semantic Versioning</h3>
				<p class="text-muted-foreground mb-4">
					Spore enforces semantic versioning to ensure predictable updates and compatibility.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Version Bumping</h4>
						<div class="grid grid-cols-1 md:grid-cols-3 gap-3">
							<div class="border rounded-lg p-4">
								<h5 class="font-medium mb-2">Patch Release</h5>
								<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-sm p-3 rounded mb-2">
									<code>spore version patch</code>
								</div>
								<p class="text-xs text-muted-foreground">Bug fixes, no breaking changes</p>
							</div>
							<div class="border rounded-lg p-4">
								<h5 class="font-medium mb-2">Minor Release</h5>
								<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-sm p-3 rounded mb-2">
									<code>spore version minor</code>
								</div>
								<p class="text-xs text-muted-foreground">New features, backward compatible</p>
							</div>
							<div class="border rounded-lg p-4">
								<h5 class="font-medium mb-2">Major Release</h5>
								<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-sm p-3 rounded mb-2">
									<code>spore version major</code>
								</div>
								<p class="text-xs text-muted-foreground">Breaking changes</p>
							</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Pre-release Versions</h4>
						<div class="space-y-3">
							<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-sm p-3 rounded">
								<code>spore version 2.0.0-beta.1</code>
							</div>
							<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-sm p-3 rounded">
								<code>spore version prerelease --preid alpha</code>
							</div>
						</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-4">Release Management</h3>
				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Complete Release Workflow</h4>
						<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-sm p-4 rounded-lg relative group">
							<div class="space-y-1">
								<div><code># 1. Update version</code></div>
								<div><code>spore version minor</code></div>
								<div><code># 2. Build and test</code></div>
								<div><code>spore run build && spore run test</code></div>
								<div><code># 3. Publish to registry</code></div>
								<div><code>spore publish</code></div>
								<div><code># 4. Create git tag</code></div>
								<div><code>git tag v1.3.0 && git push --tags</code></div>
							</div>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard(`# 1. Update version
spore version minor
# 2. Build and test
spore run build && spore run test
# 3. Publish to registry
spore publish
# 4. Create git tag
git tag v1.3.0 && git push --tags`)}
							>
								{#if showCopied && copyText.includes('Update version')}
									<Icon icon="lucide:check-circle" class="w-4 h-4 text-zinc-100" />
								{:else}
									<Icon icon="lucide:copy" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Automated Releases</h4>
						<p class="text-sm text-muted-foreground mb-2">
							Set up automated releases with CI/CD:
						</p>
						<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-sm p-4 rounded-lg">
							<pre><code>{@html `<span class="text-gray-400"># .github/workflows/release.yml</span>
<span class="text-zinc-400">name:</span> <span class="text-zinc-100">Release</span>
<span class="text-zinc-400">on:</span>
  <span class="text-zinc-400">push:</span>
    <span class="text-zinc-400">tags:</span> [<span class="text-zinc-100">'v*'</span>]

<span class="text-zinc-400">jobs:</span>
  <span class="text-zinc-400">publish:</span>
    <span class="text-zinc-400">runs-on:</span> <span class="text-zinc-100">ubuntu-latest</span>
    <span class="text-zinc-400">steps:</span>
      - <span class="text-zinc-400">run:</span> <span class="text-zinc-100">spore login --token \${{ secrets.SPORE_TOKEN }}</span>
      - <span class="text-zinc-400">run:</span> <span class="text-zinc-100">spore publish</span>`}</code></pre>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Package Discovery -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Package Discovery</h2>
		
		<div class="space-y-8">
			<div>
				<h3 class="text-lg font-semibold mb-4">Package Metadata</h3>
				<p class="text-muted-foreground mb-4">
					Optimize your package for discoverability with rich metadata and documentation.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Package Tags</h4>
						<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-gray-400"># package.yml</span>
<span class="text-zinc-400">tags:</span>
  - <span class="text-zinc-100">utilities</span>
  - <span class="text-zinc-100">typescript</span>
  - <span class="text-zinc-100">react</span>
  - <span class="text-zinc-100">ui-components</span>
  - <span class="text-zinc-100">design-system</span></code></pre>
						</div>
						<p class="text-sm text-muted-foreground mt-2">
							Tags help users discover your package in the Spore web interface.
						</p>
					</div>

					<div>
						<h4 class="font-medium mb-2">Rich Description</h4>
						<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-zinc-400">description:</span> <span class="text-zinc-100">"React UI components following Material Design principles. Includes buttons, forms, navigation, and data display components with full TypeScript support and customizable themes."</span></code></pre>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Documentation</h4>
						<div class="bg-zinc-950 text-zinc-200 border border-white/10 font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-gray-400"># package.yml</span>
<span class="text-zinc-400">documentation:</span>
  <span class="text-zinc-400">homepage:</span> <span class="text-zinc-100">"https://ui.mycompany.com"</span>
  <span class="text-zinc-400">repository:</span> <span class="text-zinc-100">"https://github.com/mycompany/ui-components"</span>
  <span class="text-zinc-400">issues:</span> <span class="text-zinc-100">"https://github.com/mycompany/ui-components/issues"</span></code></pre>
						</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-4">README Best Practices</h3>
				<div class="space-y-4">
					<div class="border rounded-lg p-6">
						<h4 class="font-semibold mb-3">Essential Sections</h4>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• Clear package description and purpose</li>
							<li>• Installation instructions</li>
							<li>• Basic usage examples</li>
							<li>• API documentation or links</li>
							<li>• Contributing guidelines</li>
							<li>• License information</li>
						</ul>
					</div>

					<div class="border rounded-lg p-6">
						<h4 class="font-semibold mb-3">Code Examples</h4>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• Include practical, copy-paste examples</li>
							<li>• Show import statements</li>
							<li>• Demonstrate common use cases</li>
							<li>• Include TypeScript types</li>
							<li>• Show configuration options</li>
						</ul>
					</div>
				</div>
			</div>
		</div>
	</section>

	

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<a href="/docs/teams" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:users-group-two-rounded-bold" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Team Management</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Learn how to create and manage teams for collaborative package development.
				</p>
			</a>

			<a href="/docs/permissions" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:shield-check-bold" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Permissions</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Understand access controls, roles, and security for package publishing.
				</p>
			</a>

			<a href="/docs/package-development" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:box-bold" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Package Development</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Go deeper into package development workflows and best practices.
				</p>
			</a>

			<a href="/docs/self-hosting" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:server-bold" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Self-Hosting</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Set up your own private Spore registry for internal package management.
				</p>
			</a>
		</div>
	</section>
</div>