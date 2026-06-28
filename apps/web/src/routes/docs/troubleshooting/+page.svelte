<script lang="ts">
	import Icon from '@iconify/svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
</script>

<svelte:head>
	<title>Troubleshooting - Spore CLI Documentation</title>
	<meta name="description" content="Common issues and solutions for Spore CLI, package linking, authentication, builds, and publishing. Debug commands and support resources." />
	<meta property="og:title" content="Troubleshooting - Spore CLI" />
	<meta property="og:description" content="Common issues and solutions for Spore CLI, package linking, authentication, builds, and publishing. Debug commands and support resources." />
	<meta property="og:url" content="https://spore.facile.studio/docs/troubleshooting" />
	<meta name="twitter:title" content="Troubleshooting - Spore CLI" />
	<meta name="twitter:description" content="Common issues and solutions for Spore CLI, package linking, authentication, builds, and publishing. Debug commands and support resources." />
	<link rel="canonical" href="https://spore.facile.studio/docs/troubleshooting" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Goga', 'Satoshi', sans-serif;">
			Troubleshooting
		</h1>
		<p class="text-lg text-muted-foreground">
			Common issues and solutions for Spore CLI and monorepo management
		</p>
	</div>

	<!-- Installation Issues -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Installation Issues</h2>

		<div class="space-y-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:danger-triangle-bold" class="w-5 h-5 mr-2 text-muted-foreground" />
					Command not found
				</h3>
				<div class="space-y-3 text-sm">
					<p class="text-muted-foreground">If you get "command not found" after installation:</p>
					<ul class="space-y-1 ml-4 text-muted-foreground">
						<li>• Check if <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">/usr/local/bin</code> is in your PATH</li>
						<li>• Try restarting your terminal</li>
						<li>• On macOS: <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">sudo chown -R $(whoami) /usr/local/bin</code></li>
						<li>• Manually add to PATH: <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">export PATH=$PATH:/usr/local/bin</code></li>
					</ul>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:lock-bold" class="w-5 h-5 mr-2 text-muted-foreground" />
					Permission denied
				</h3>
				<div class="space-y-3 text-sm">
					<p class="text-muted-foreground">If you get permission errors:</p>
					<ul class="space-y-1 ml-4 text-muted-foreground">
						<li>• Make sure the binary is executable: <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">chmod +x spore</code></li>
						<li>• On macOS, allow the app in System Preferences > Security</li>
						<li>• Use <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">sudo</code> for system-wide installation</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Linking Issues -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Package Linking Issues</h2>

		<div class="space-y-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:x-circle" class="w-5 h-5 mr-2 text-muted-foreground" />
					Package not found
				</h3>
				<div class="space-y-3 text-sm">
					<p class="text-muted-foreground">When packages can't be found:</p>
					<ul class="space-y-1 ml-4 text-muted-foreground">
						<li>• Check package name spelling in configuration</li>
						<li>• Verify package exists in <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">packages/</code> directory</li>
						<li>• For remote packages, check authentication</li>
						<li>• Run <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore status</code> to see available packages</li>
					</ul>
					<div class="mt-3">
						<CodeBlock label="bash" copy="spore link --verbose"><code class="whitespace-nowrap block text-zinc-100">spore link --verbose</code></CodeBlock>
					</div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:code" class="w-5 h-5 mr-2 text-muted-foreground" />
					TypeScript import errors
				</h3>
				<div class="space-y-3 text-sm">
					<p class="text-muted-foreground">When imports don't work:</p>
					<ul class="space-y-1 ml-4 text-muted-foreground">
						<li>• Verify tsAlias configuration in app.yml or spore.yml</li>
						<li>• Check if tsconfig.json was updated correctly</li>
						<li>• Run <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore link --force</code> to refresh all links</li>
						<li>• Restart your IDE/TypeScript service</li>
					</ul>
					<div class="mt-3">
						<CodeBlock label="bash" copy="spore link --force"><code class="whitespace-nowrap block text-zinc-100">spore link --force</code></CodeBlock>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Authentication Issues -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Authentication Issues</h2>

		<div class="space-y-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:shield-warning-bold" class="w-5 h-5 mr-2 text-muted-foreground" />
					Authentication failed
				</h3>
				<div class="space-y-3 text-sm">
					<p class="text-muted-foreground">When authentication fails:</p>
					<ul class="space-y-1 ml-4 text-muted-foreground">
						<li>• Check that your token is correct and not expired</li>
						<li>• Ensure there are no extra spaces in the token</li>
						<li>• Verify the environment variable is set correctly</li>
						<li>• Test with <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore auth</code> command</li>
					</ul>
					<div class="mt-3">
						<CodeBlock label="bash" copy="spore auth"><code class="whitespace-nowrap block text-zinc-100">spore auth</code></CodeBlock>
					</div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:wifi-router-bold" class="w-5 h-5 mr-2 text-muted-foreground" />
					Connection failed
				</h3>
				<div class="space-y-3 text-sm">
					<p class="text-muted-foreground">When can't connect to registry:</p>
					<ul class="space-y-1 ml-4 text-muted-foreground">
						<li>• Check your internet connection</li>
						<li>• Verify the SPORE_SPACE_URL is correct</li>
						<li>• Check if you're behind a corporate firewall</li>
						<li>• Try with <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">--verbose</code> flag for more details</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Build Issues -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Build Issues</h2>

		<div class="space-y-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:bug-bold" class="w-5 h-5 mr-2 text-muted-foreground" />
					Build failures
				</h3>
				<div class="space-y-3 text-sm">
					<p class="text-muted-foreground">When builds fail:</p>
					<ul class="space-y-1 ml-4 text-muted-foreground">
						<li>• Check build script configuration in package.yml or app.yml</li>
						<li>• Verify all dependencies are installed</li>
						<li>• Run build command manually to see detailed error</li>
						<li>• Check for missing TypeScript types</li>
					</ul>
					<div class="mt-3">
						<CodeBlock label="bash" copy="spore run build --verbose"><code class="whitespace-nowrap block text-zinc-100">spore run build --verbose</code></CodeBlock>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Publishing Issues -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Publishing Issues</h2>

		<div class="space-y-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:upload-bold" class="w-5 h-5 mr-2 text-muted-foreground" />
					Publish failed
				</h3>
				<div class="space-y-3 text-sm">
					<p class="text-muted-foreground">When publishing fails:</p>
					<ul class="space-y-1 ml-4 text-muted-foreground">
						<li>• Check if you have publish permissions for the package</li>
						<li>• Verify version number follows semantic versioning</li>
						<li>• Ensure package name is unique (or team namespaced)</li>
						<li>• Check if version already exists</li>
						<li>• Use <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">--dry-run</code> to test before publishing</li>
					</ul>
					<div class="mt-3">
						<CodeBlock label="bash" copy="spore publish --dry-run"><code class="whitespace-nowrap block text-zinc-100">spore publish --dry-run</code></CodeBlock>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Debug Commands -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Debug Commands</h2>

		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:adhesive-plaster-bold-duotone" class="w-5 h-5 mr-2 text-muted-foreground" />
					Diagnostic Commands
				</h3>
				<div class="space-y-3 text-sm">
					<div>
						<div class="font-medium"><code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore status</code></div>
						<div class="text-muted-foreground">Show project and linking status</div>
					</div>
					<div>
						<div class="font-medium"><code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore --version</code></div>
						<div class="text-muted-foreground">Show CLI version</div>
					</div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:document-text-bold" class="w-5 h-5 mr-2 text-muted-foreground" />
					Verbose Logging
				</h3>
				<div class="space-y-3 text-sm">
					<div>
						<div class="font-medium"><code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore link --verbose</code></div>
						<div class="text-muted-foreground">Detailed linking output</div>
					</div>
					<div>
						<div class="font-medium"><code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore publish --verbose</code></div>
						<div class="text-muted-foreground">Detailed publishing output</div>
					</div>
					<div>
						<div class="font-medium"><code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore run build --verbose</code></div>
						<div class="text-muted-foreground">Detailed build output via scripts</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Dependency Issues -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Dependency Issues</h2>

		<div class="space-y-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:package-x" class="w-5 h-5 mr-2 text-muted-foreground" />
					Circular dependency detected
				</h3>
				<div class="space-y-3 text-sm">
					<p class="text-muted-foreground">When packages depend on each other in a cycle:</p>
					<ul class="space-y-1 ml-4 text-muted-foreground">
						<li>• Identify the circular path using <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore deps tree</code></li>
						<li>• Extract shared logic into a separate package</li>
						<li>• Use dependency injection or interfaces to break cycles</li>
						<li>• Consider merging packages if they're tightly coupled</li>
					</ul>
					<div class="mt-3">
						<CodeBlock label="bash" copy="spore deps check --verbose"><code class="whitespace-nowrap block text-zinc-100">spore deps check --verbose</code></CodeBlock>
					</div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:alert-triangle" class="w-5 h-5 mr-2 text-muted-foreground" />
					Version conflicts
				</h3>
				<div class="space-y-3 text-sm">
					<p class="text-muted-foreground">When multiple packages require incompatible versions:</p>
					<ul class="space-y-1 ml-4 text-muted-foreground">
						<li>• Use <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore deps sync</code> to find and resolve conflicts</li>
						<li>• Update version constraints to be more flexible</li>
						<li>• Consider using <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">peerDependencies</code> for shared libraries</li>
						<li>• Pin to specific versions if needed for stability</li>
					</ul>
					<div class="mt-3">
						<CodeBlock label="bash" copy="spore deps sync"><code class="whitespace-nowrap block text-zinc-100">spore deps sync</code></CodeBlock>
					</div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:search-x" class="w-5 h-5 mr-2 text-muted-foreground" />
					Package not found
				</h3>
				<div class="space-y-3 text-sm">
					<p class="text-muted-foreground">When a dependency cannot be resolved:</p>
					<ul class="space-y-1 ml-4 text-muted-foreground">
						<li>• Check package name spelling and case sensitivity</li>
						<li>• Ensure package exists in local packages/ directory</li>
						<li>• Verify remote packages are published to the registry</li>
						<li>• Check authentication for private packages</li>
						<li>• Use <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore search</code> to find available packages</li>
					</ul>
					<div class="mt-3">
						<CodeBlock label="bash" copy={'spore search "package-name"'}><code class="whitespace-nowrap block text-zinc-100">spore search "package-name"</code></CodeBlock>
					</div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:code-2" class="w-5 h-5 mr-2 text-muted-foreground" />
					TypeScript import errors
				</h3>
				<div class="space-y-3 text-sm">
					<p class="text-muted-foreground">When TypeScript can't find package imports:</p>
					<ul class="space-y-1 ml-4 text-muted-foreground">
						<li>• Run <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore link --force</code> to refresh path mappings</li>
						<li>• Check if package has been built with <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore run build</code></li>
						<li>• Verify TypeScript alias configuration in app config</li>
						<li>• Ensure package exports correct types from index.ts</li>
						<li>• Restart TypeScript language server in your editor</li>
					</ul>
					<div class="mt-3">
						<CodeBlock label="bash" copy="spore link --force && spore run build"><code class="whitespace-nowrap block text-zinc-100">spore link --force && spore run build</code></CodeBlock>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Getting Help -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Getting Help</h2>

		<div class="space-y-6">
			<div class="bg-muted border border-border rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="lucide:question-circle-bold" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
					<div>
						<h3 class="font-semibold text-foreground mb-2">Still Need Help?</h3>
						<p class="text-sm text-muted-foreground mb-4">
							If you're still experiencing issues, here are additional resources:
						</p>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• Check the <a href="https://github.com/FacileStudio/Spore/issues" class="underline">GitHub issues</a></li>
							<li>• Join our community discussions</li>
							<li>• Review the complete documentation</li>
							<li>• Contact support team</li>
							<li>• Visit the <a href="/docs/inter-package-dependencies" class="underline">Inter-Package Dependencies guide</a></li>
						</ul>
					</div>
				</div>
			</div>
		</div>
	</section>
</div>
