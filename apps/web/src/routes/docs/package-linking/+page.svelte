<script lang="ts">
	import Icon from '@iconify/svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
</script>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-12">
		<h1 class="text-4xl font-bold tracking-tight mb-4" style="font-family: 'Goga', sans-serif;">
			Package Linking
		</h1>
		<p class="text-lg text-muted-foreground">
			Understanding how Spore links packages and manages dependencies in your monorepo
		</p>
	</div>

	<!-- Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">How Package Linking Works</h2>
		<p class="text-muted-foreground leading-relaxed mb-6">
			Spore's package linking system is designed to be simple, reliable, and production-friendly.
			Unlike traditional symlink-based approaches, Spore uses a copy-first strategy that works
			seamlessly across different environments and deployment scenarios.
		</p>

		<div class="bg-muted border border-border rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="lucide:info" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-foreground mb-2">Copy-First Approach</h3>
					<p class="text-sm text-muted-foreground">
						By default, Spore copies package contents to <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore_packages/</code> directories in your apps.
						This ensures compatibility with Docker, CI/CD systems, and deployment environments that don't
						support symlinks reliably.
					</p>
				</div>
			</div>
		</div>
	</section>

	<!-- Linking Modes -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Linking Modes</h2>

		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<!-- Copy Mode -->
			<div class="border rounded-lg p-6">
				<div class="flex items-center space-x-3 mb-4">
					<div class="w-12 h-12 bg-muted rounded-lg flex items-center justify-center">
						<Icon icon="lucide:copy" class="w-6 h-6 text-foreground" />
					</div>
					<div>
						<h3 class="font-semibold">Copy Mode (Default)</h3>
						<p class="text-sm text-muted-foreground">Production-ready package copying</p>
					</div>
				</div>

				<div class="space-y-3">
					<CodeBlock label="bash" copy="spore link"><span class="text-zinc-100">spore link</span></CodeBlock>

					<div class="text-sm space-y-2">
						<div class="flex items-start space-x-2">
							<Icon icon="lucide:check-circle" class="w-4 h-4 text-foreground mt-0.5 flex-shrink-0" />
							<span class="text-muted-foreground">Works in Docker containers</span>
						</div>
						<div class="flex items-start space-x-2">
							<Icon icon="lucide:check-circle" class="w-4 h-4 text-foreground mt-0.5 flex-shrink-0" />
							<span class="text-muted-foreground">Compatible with all CI/CD systems</span>
						</div>
						<div class="flex items-start space-x-2">
							<Icon icon="lucide:check-circle" class="w-4 h-4 text-foreground mt-0.5 flex-shrink-0" />
							<span class="text-muted-foreground">Can be committed to version control</span>
						</div>
						<div class="flex items-start space-x-2">
							<Icon icon="lucide:check-circle" class="w-4 h-4 text-foreground mt-0.5 flex-shrink-0" />
							<span class="text-muted-foreground">No platform-specific issues</span>
						</div>
					</div>
				</div>
			</div>

			<!-- Symlink Mode -->
			<div class="border rounded-lg p-6">
				<div class="flex items-center space-x-3 mb-4">
					<div class="w-12 h-12 bg-muted rounded-lg flex items-center justify-center">
						<Icon icon="lucide:link" class="w-6 h-6 text-foreground" />
					</div>
					<div>
						<h3 class="font-semibold">Symlink Mode</h3>
						<p class="text-sm text-muted-foreground">Development with live updates</p>
					</div>
				</div>

				<div class="space-y-3">
					<CodeBlock label="bash" copy="spore link --symlink"><span class="text-zinc-100">spore link --symlink</span></CodeBlock>

					<div class="text-sm space-y-2">
						<div class="flex items-start space-x-2">
							<Icon icon="lucide:check-circle" class="w-4 h-4 text-foreground mt-0.5 flex-shrink-0" />
							<span class="text-muted-foreground">Live updates during development</span>
						</div>
						<div class="flex items-start space-x-2">
							<Icon icon="lucide:check-circle" class="w-4 h-4 text-foreground mt-0.5 flex-shrink-0" />
							<span class="text-muted-foreground">Instant changes across apps</span>
						</div>
						<div class="flex items-start space-x-2">
							<Icon icon="lucide:x-circle" class="w-4 h-4 text-muted-foreground mt-0.5 flex-shrink-0" />
							<span class="text-muted-foreground">Docker compatibility issues</span>
						</div>
						<div class="flex items-start space-x-2">
							<Icon icon="lucide:x-circle" class="w-4 h-4 text-muted-foreground mt-0.5 flex-shrink-0" />
							<span class="text-muted-foreground">Platform-specific limitations</span>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- How Linking Works -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Step-by-Step Linking Process</h2>

		<div class="space-y-8">
			<!-- Step 1 -->
			<div>
				<div class="flex items-center space-x-3 mb-4">
					<div class="w-6 h-6 bg-muted rounded-full flex items-center justify-center">
						<span class="text-foreground font-bold text-xs">1</span>
					</div>
					<h3 class="text-lg font-semibold">Configuration Analysis</h3>
				</div>
				<div class="ml-9 space-y-3">
					<p class="text-muted-foreground">
						Spore reads your project configuration and identifies which packages each app depends on.
					</p>
					<CodeBlock label="spore.yml" copy={`apps:\n  frontend:\n    packages:\n      - types\n      - utils\n      - "@team/shared"`}><span class="text-zinc-600"># From spore.yml or app.yml</span>
<span class="text-zinc-400">apps:</span>
  <span class="text-zinc-400">frontend:</span>
    <span class="text-zinc-400">packages:</span>
      - <span class="text-zinc-100">types</span>
      - <span class="text-zinc-100">utils</span>
      - <span class="text-zinc-100">"@team/shared"</span></CodeBlock>
				</div>
			</div>

			<!-- Step 2 -->
			<div>
				<div class="flex items-center space-x-3 mb-4">
					<div class="w-6 h-6 bg-muted rounded-full flex items-center justify-center">
						<span class="text-foreground font-bold text-xs">2</span>
					</div>
					<h3 class="text-lg font-semibold">Package Resolution</h3>
				</div>
				<div class="ml-9 space-y-3">
					<p class="text-muted-foreground">
						Local packages are found in the <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">packages/</code> directory. Remote packages are downloaded from registries.
					</p>
					<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
						<div class="border rounded p-3">
							<h4 class="font-medium mb-2">Local Packages</h4>
							<div class="text-sm text-muted-foreground">
								Found in: <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">packages/utils/</code><br>
								Source: <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">src/</code> directory
							</div>
						</div>
						<div class="border rounded p-3">
							<h4 class="font-medium mb-2">Remote Packages</h4>
							<div class="text-sm text-muted-foreground">
								Downloaded from: Spore registry<br>
								Cached locally for performance
							</div>
						</div>
					</div>
				</div>
			</div>

			<!-- Step 3 -->
			<div>
				<div class="flex items-center space-x-3 mb-4">
					<div class="w-6 h-6 bg-muted rounded-full flex items-center justify-center">
						<span class="text-foreground font-bold text-xs">3</span>
					</div>
					<h3 class="text-lg font-semibold">Directory Creation</h3>
				</div>
				<div class="ml-9 space-y-3">
					<p class="text-muted-foreground">
						The <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore_packages/</code> directory is created in each app that has package dependencies.
					</p>
					<CodeBlock label="tree">apps/frontend/
├── spore_packages/  <span class="text-zinc-600"># Auto-generated</span>
│   ├── types/
│   ├── utils/
│   └── @team_shared/  <span class="text-zinc-600"># Team packages</span>
└── src/</CodeBlock>
				</div>
			</div>

			<!-- Step 4 -->
			<div>
				<div class="flex items-center space-x-3 mb-4">
					<div class="w-6 h-6 bg-muted rounded-full flex items-center justify-center">
						<span class="text-foreground font-bold text-xs">4</span>
					</div>
					<h3 class="text-lg font-semibold">TypeScript Configuration</h3>
				</div>
				<div class="ml-9 space-y-3">
					<p class="text-muted-foreground">
						Your app's <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">tsconfig.json</code> is automatically updated with path mappings for clean imports.
					</p>
					<CodeBlock label="tsconfig.json"><span class="text-zinc-600">// tsconfig.json</span>
{'{'}
  <span class="text-zinc-400">"compilerOptions"</span>: {'{'}
    <span class="text-zinc-400">"baseUrl"</span>: <span class="text-zinc-100">"."</span>,
    <span class="text-zinc-400">"paths"</span>: {'{'}
      <span class="text-zinc-100">"@/types"</span>: [<span class="text-zinc-100">"./spore_packages/types"</span>],
      <span class="text-zinc-100">"@/utils"</span>: [<span class="text-zinc-100">"./spore_packages/utils"</span>],
      <span class="text-zinc-100">"@/shared"</span>: [<span class="text-zinc-100">"./spore_packages/@team_shared"</span>]
    {'}'}
  {'}'}
{'}'}</CodeBlock>
				</div>
			</div>
		</div>
	</section>

	<!-- TypeScript Aliases -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">TypeScript Aliases with Linking</h2>

		<div class="space-y-6">
			<p class="text-muted-foreground">
				Spore's linking system automatically configures TypeScript path mappings based on your chosen alias prefix,
				making package imports clean and consistent across your applications.
			</p>

			<div>
				<h3 class="text-lg font-semibold mb-3">Alias Configuration During Linking</h3>
				<p class="text-muted-foreground mb-4">
					When you run <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore link</code>, the system reads your alias configuration and updates TypeScript paths accordingly.
				</p>
				<CodeBlock label="spore.yml"><span class="text-zinc-600"># In spore.yml or app.yml</span>
<span class="text-zinc-400">apps:</span>
  <span class="text-zinc-400">frontend:</span>
    <span class="text-zinc-400">ts_alias:</span> <span class="text-zinc-100">"@"</span>        <span class="text-zinc-600"># Results in @/package-name</span>
    <span class="text-zinc-400">packages:</span> [<span class="text-zinc-100">ui-components</span>, <span class="text-zinc-100">shared-types</span>]
  <span class="text-zinc-400">api:</span>
    <span class="text-zinc-400">ts_alias:</span> <span class="text-zinc-100">"#"</span>        <span class="text-zinc-600"># Results in #/package-name</span>
    <span class="text-zinc-400">packages:</span> [<span class="text-zinc-100">database-models</span>, <span class="text-zinc-100">shared-types</span>]
  <span class="text-zinc-400">admin:</span>
    <span class="text-zinc-400">ts_alias:</span> <span class="text-zinc-100">"~"</span>        <span class="text-zinc-600"># Results in ~/package-name</span>
    <span class="text-zinc-400">packages:</span> [<span class="text-zinc-100">admin-components</span>, <span class="text-zinc-100">shared-types</span>]</CodeBlock>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Generated TypeScript Paths</h3>
				<p class="text-muted-foreground mb-4">
					The linking process creates path mappings in each app's <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">tsconfig.json</code> file.
				</p>
				<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
					<div>
						<h4 class="font-medium mb-2">Before Linking</h4>
						<CodeBlock label="tsconfig.json"><span class="text-zinc-600">// No path mappings</span>
{'{'}
  <span class="text-zinc-400">"compilerOptions"</span>: {'{'}
    <span class="text-zinc-400">"baseUrl"</span>: <span class="text-zinc-100">"."</span>
  {'}'}
{'}'}</CodeBlock>
					</div>
					<div>
						<h4 class="font-medium mb-2">After Linking</h4>
						<CodeBlock label="tsconfig.json"><span class="text-zinc-600">// Clean alias mappings</span>
{'{'}
  <span class="text-zinc-400">"compilerOptions"</span>: {'{'}
    <span class="text-zinc-400">"baseUrl"</span>: <span class="text-zinc-100">"."</span>,
    <span class="text-zinc-400">"paths"</span>: {'{'}
      <span class="text-zinc-100">"@/ui-components"</span>: [<span class="text-zinc-100">"./spore_packages/ui-components"</span>],
      <span class="text-zinc-100">"@/shared-types"</span>: [<span class="text-zinc-100">"./spore_packages/shared-types"</span>]
    {'}'}
  {'}'},
  <span class="text-zinc-400">"include"</span>: [<span class="text-zinc-100">"spore_packages/**/*"</span>]
{'}'}</CodeBlock>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Alias Resolution Process</h3>
				<p class="text-muted-foreground mb-4">
					Understanding how aliases are resolved during the linking process helps with troubleshooting.
				</p>

				<div class="space-y-4">
					<div class="border rounded-lg p-4">
						<div class="flex items-center space-x-3 mb-3">
							<div class="w-6 h-6 bg-muted rounded-full flex items-center justify-center">
								<span class="text-foreground font-bold text-xs">1</span>
							</div>
							<h4 class="font-semibold">Package Discovery</h4>
						</div>
						<div class="ml-9">
							<p class="text-sm text-muted-foreground mb-2">
								Spore scans your configuration to find which packages each app needs.
							</p>
							<CodeBlock label="output"><span class="text-zinc-600"># Found packages for frontend app:</span>
<span class="text-zinc-100">ui-components</span> <span class="text-zinc-600">(local)</span>
<span class="text-zinc-100">shared-types</span> <span class="text-zinc-600">(local)</span>
<span class="text-zinc-100">"@company/design-system"</span> <span class="text-zinc-600">(remote)</span></CodeBlock>
						</div>
					</div>

					<div class="border rounded-lg p-4">
						<div class="flex items-center space-x-3 mb-3">
							<div class="w-6 h-6 bg-muted rounded-full flex items-center justify-center">
								<span class="text-foreground font-bold text-xs">2</span>
							</div>
							<h4 class="font-semibold">Alias Mapping</h4>
						</div>
						<div class="ml-9">
							<p class="text-sm text-muted-foreground mb-2">
								Each package gets mapped to an alias path based on the app's ts_alias setting.
							</p>
							<CodeBlock label="output"><span class="text-zinc-600"># Generated mappings:</span>
<span class="text-zinc-100">"@/ui-components"</span> → <span class="text-zinc-400">./spore_packages/ui-components</span>
<span class="text-zinc-100">"@/shared-types"</span> → <span class="text-zinc-400">./spore_packages/shared-types</span>
<span class="text-zinc-100">"@/design-system"</span> → <span class="text-zinc-400">./spore_packages/@company_design-system</span></CodeBlock>
						</div>
					</div>

					<div class="border rounded-lg p-4">
						<div class="flex items-center space-x-3 mb-3">
							<div class="w-6 h-6 bg-muted rounded-full flex items-center justify-center">
								<span class="text-foreground font-bold text-xs">3</span>
							</div>
							<h4 class="font-semibold">TypeScript Update</h4>
						</div>
						<div class="ml-9">
							<p class="text-sm text-muted-foreground mb-2">
								The app's tsconfig.json is updated with the generated path mappings.
							</p>
							<CodeBlock label="tsconfig.json"><span class="text-zinc-600"># Updated tsconfig.json paths section</span>
<span class="text-zinc-400">"paths"</span>: {'{'}
  <span class="text-zinc-100">"@/*"</span>: [<span class="text-zinc-100">"./spore_packages/*"</span>]
{'}'}</CodeBlock>
						</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Different Package Types with Aliases</h3>
				<p class="text-muted-foreground mb-4">
					See how different types of packages are handled by the alias system during linking.
				</p>

				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-3 flex items-center">
							<Icon icon="lucide:home" class="w-4 h-4 mr-2 text-foreground" />
							Local Packages
						</h4>
						<div class="space-y-3">
							<CodeBlock label="spore.yml"><span class="text-zinc-600"># Configuration</span>
<span class="text-zinc-400">packages:</span>
  - <span class="text-zinc-100">user-management</span>
  - <span class="text-zinc-100">data-validation</span></CodeBlock>
							<CodeBlock label="typescript"><span class="text-zinc-600">// Generated imports</span>
import {'{ UserService }'} from '<span class="text-zinc-100">@/user-management</span>';
import {'{ validate }'} from '<span class="text-zinc-100">@/data-validation</span>';</CodeBlock>
						</div>
					</div>

					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-3 flex items-center">
							<Icon icon="lucide:users" class="w-4 h-4 mr-2 text-foreground" />
							Team Packages
						</h4>
						<div class="space-y-3">
							<CodeBlock label="spore.yml"><span class="text-zinc-600"># Configuration</span>
<span class="text-zinc-400">packages:</span>
  - <span class="text-zinc-100">"@myteam/shared-lib"</span>
  - <span class="text-zinc-100">"@myteam/ui-kit"</span></CodeBlock>
							<CodeBlock label="typescript"><span class="text-zinc-600">// Cleaned up imports</span>
import {'{ ApiClient }'} from '<span class="text-zinc-100">@/shared-lib</span>';
import {'{ Button }'} from '<span class="text-zinc-100">@/ui-kit</span>';</CodeBlock>
						</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Linking Mode Impact on Aliases</h3>
				<p class="text-muted-foreground mb-4">
					Both copy and symlink modes generate the same alias mappings, but with different underlying implementations.
				</p>

				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2 text-foreground">Copy Mode</h4>
						<div class="space-y-2 text-sm">
							<p class="text-muted-foreground">Packages are copied to spore_packages/</p>
							<CodeBlock label="bash" copy="spore link"><span class="text-zinc-100">spore link</span></CodeBlock>
							<div class="text-xs text-muted-foreground">
								→ Stable imports<br>
								→ Production ready<br>
								→ Version locked
							</div>
						</div>
					</div>

					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2 text-foreground">Symlink Mode</h4>
						<div class="space-y-2 text-sm">
							<p class="text-muted-foreground">Packages are symlinked to spore_packages/</p>
							<CodeBlock label="bash" copy="spore link --symlink"><span class="text-zinc-100">spore link --symlink</span></CodeBlock>
							<div class="text-xs text-muted-foreground">
								→ Live updates<br>
								→ Development mode<br>
								→ Hot reloading
							</div>
						</div>
					</div>
				</div>

				<div class="mt-4 bg-muted border border-border rounded-lg p-4">
					<div class="flex items-start space-x-3">
						<Icon icon="lucide:info" class="w-5 h-5 text-foreground mt-0.5 flex-shrink-0" />
						<div class="text-sm">
							<h4 class="font-medium text-foreground mb-1">Aliases Work the Same</h4>
							<p class="text-muted-foreground">
								Regardless of linking mode, your import statements remain identical. The alias system
								abstracts away the underlying file structure, providing consistency across development and production.
							</p>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Advanced Alias Linking Scenarios -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Advanced Alias Scenarios</h2>

		<div class="space-y-8">
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-4">Mixed Package Sources</h3>
				<p class="text-muted-foreground mb-4">
					When apps use packages from multiple sources, aliases provide a unified import interface.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Complex Configuration</h4>
						<CodeBlock label="spore.yml"><span class="text-zinc-400">apps:</span>
  <span class="text-zinc-400">web-app:</span>
    <span class="text-zinc-400">ts_alias:</span> <span class="text-zinc-100">"@"</span>
    <span class="text-zinc-400">packages:</span>
      <span class="text-zinc-600"># Local packages</span>
      - <span class="text-zinc-100">shared-types</span>
      - <span class="text-zinc-100">ui-components</span>
      <span class="text-zinc-600"># Team packages</span>
      - <span class="text-zinc-100">"@company/design-system"</span>
      - <span class="text-zinc-100">"@team/analytics"</span>
      <span class="text-zinc-600"># External packages</span>
      - <span class="text-zinc-100">"@external/charts"</span></CodeBlock>
					</div>

					<div>
						<h4 class="font-medium mb-2">Unified Import Experience</h4>
						<CodeBlock label="typescript"><span class="text-zinc-600">// All packages use consistent alias imports</span>
<span class="text-zinc-300">import</span> {'{ '}<span class="text-zinc-100">User</span> {'} '}<span class="text-zinc-300">from</span> <span class="text-zinc-100">'@/shared-types'</span>;        <span class="text-zinc-600">// Local</span>
<span class="text-zinc-300">import</span> {'{ '}<span class="text-zinc-100">Button</span> {'} '}<span class="text-zinc-300">from</span> <span class="text-zinc-100">'@/ui-components'</span>;     <span class="text-zinc-600">// Local</span>
<span class="text-zinc-300">import</span> {'{ '}<span class="text-zinc-100">Theme</span> {'} '}<span class="text-zinc-300">from</span> <span class="text-zinc-100">'@/design-system'</span>;     <span class="text-zinc-600">// Team</span>
<span class="text-zinc-300">import</span> {'{ '}<span class="text-zinc-100">track</span> {'} '}<span class="text-zinc-300">from</span> <span class="text-zinc-100">'@/analytics'</span>;        <span class="text-zinc-600">// Team</span>
<span class="text-zinc-300">import</span> {'{ '}<span class="text-zinc-100">LineChart</span> {'} '}<span class="text-zinc-300">from</span> <span class="text-zinc-100">'@/charts'</span>;          <span class="text-zinc-600">// External</span></CodeBlock>
					</div>

					<div class="bg-muted border border-border rounded-lg p-4">
						<div class="flex items-start space-x-3">
							<Icon icon="lucide:lightbulb" class="w-5 h-5 text-foreground mt-0.5 flex-shrink-0" />
							<div class="text-sm">
								<h4 class="font-medium text-foreground mb-1">Package Source Transparency</h4>
								<p class="text-muted-foreground">
									Developers don't need to remember whether a package is local, from a team registry, or external.
									The alias system provides a consistent interface regardless of the package source.
								</p>
							</div>
						</div>
					</div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-4">Alias Switching Between Environments</h3>
				<p class="text-muted-foreground mb-4">
					Different environments may require different linking strategies while maintaining the same alias imports.
				</p>

				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div class="space-y-4">
						<h4 class="font-semibold">Development Environment</h4>
						<div class="border rounded p-4">
							<div class="mb-3">
								<CodeBlock label="bash" copy="spore link --symlink"><span class="text-zinc-100">spore link --symlink</span></CodeBlock>
							</div>
							<ul class="text-xs text-muted-foreground space-y-1">
								<li>• Live package updates</li>
								<li>• Fast iteration cycle</li>
								<li>• Hot module replacement</li>
								<li>• Real-time debugging</li>
							</ul>
						</div>
					</div>

					<div class="space-y-4">
						<h4 class="font-semibold">Production Environment</h4>
						<div class="border rounded p-4">
							<div class="mb-3">
								<CodeBlock label="bash" copy="spore link"><span class="text-zinc-100">spore link</span></CodeBlock>
							</div>
							<ul class="text-xs text-muted-foreground space-y-1">
								<li>• Stable package versions</li>
								<li>• Docker compatibility</li>
								<li>• Predictable builds</li>
								<li>• Version control friendly</li>
							</ul>
						</div>
					</div>
				</div>

				<div class="mt-4 bg-muted border border-border rounded-lg p-4">
					<div class="flex items-start space-x-3">
						<Icon icon="lucide:check-circle" class="w-5 h-5 text-foreground mt-0.5 flex-shrink-0" />
						<div class="text-sm">
							<h4 class="font-medium text-foreground mb-1">Same Code, Different Linking</h4>
							<p class="text-muted-foreground">
								Your application code remains unchanged between development and production.
								Only the linking strategy changes, while aliases provide consistent import paths.
							</p>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Package Types -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Package Types & Handling</h2>

		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<!-- Local Packages -->
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-4 flex items-center">
					<Icon icon="lucide:home-bold" class="w-5 h-5 mr-2 text-foreground" />
					Local Packages
				</h3>
				<div class="space-y-3">
					<p class="text-sm text-muted-foreground">
						Packages stored in your <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">packages/</code> directory
					</p>
					<CodeBlock label="spore.yml"><span class="text-zinc-400">packages:</span>
  - <span class="text-zinc-100">utils</span>
  - <span class="text-zinc-100">types</span>
  - <span class="text-zinc-100">ui-components</span></CodeBlock>
					<ul class="text-sm text-muted-foreground space-y-1">
						<li>• Source copied from <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">packages/&lt;name&gt;/src/</code></li>
						<li>• Always uses latest local version</li>
						<li>• No version constraints</li>
						<li>• Instant availability</li>
					</ul>
				</div>
			</div>

			<!-- Remote Packages -->
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-4 flex items-center">
					<Icon icon="lucide:cloud-bold" class="w-5 h-5 mr-2 text-foreground" />
					Remote Packages
				</h3>
				<div class="space-y-3">
					<p class="text-sm text-muted-foreground">
						Packages downloaded from Spore registry
					</p>
					<CodeBlock label="spore.yml"><span class="text-zinc-400">packages:</span>
  - <span class="text-zinc-100">"@team/shared-lib"</span>
  - <span class="text-zinc-100">"@company/design-system"</span>
  - <span class="text-zinc-100">"@external/utility"</span></CodeBlock>
					<ul class="text-sm text-muted-foreground space-y-1">
						<li>• Downloaded and cached locally</li>
						<li>• Version management support</li>
						<li>• Team namespace support</li>
						<li>• Authentication required for private packages</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Workflow -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Development Workflow</h2>

		<div class="space-y-6">
			<p class="text-muted-foreground">
				Here's the typical workflow for developing with linked packages in a Spore monorepo.
			</p>

			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<!-- Development Mode -->
				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:code" class="w-5 h-5 mr-2 text-foreground" />
						Development Mode
					</h3>
					<div class="space-y-3">
						<CodeBlock label="bash" copy="spore link --symlink"><span class="text-zinc-100">spore link --symlink</span></CodeBlock>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• Use for active development</li>
							<li>• Changes immediately reflect in apps</li>
							<li>• Hot reload works across packages</li>
							<li>• Perfect for debugging</li>
						</ul>
					</div>
				</div>

				<!-- Production Mode -->
				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:rocket-bold" class="w-5 h-5 mr-2 text-foreground" />
						Production Mode
					</h3>
					<div class="space-y-3">
						<CodeBlock label="bash" copy="spore link"><span class="text-zinc-100">spore link</span></CodeBlock>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• Use for builds and deployment</li>
							<li>• Works in any environment</li>
							<li>• Docker compatible</li>
							<li>• Stable and reliable</li>
						</ul>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Pre-commit Hook -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Automatic Updates</h2>

		<div class="space-y-6">
			<div class="bg-muted border border-border rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="lucide:refresh-bold" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
					<div class="flex-1">
						<h3 class="font-semibold text-foreground mb-2">Pre-commit Hook</h3>
						<p class="text-sm text-muted-foreground mb-3">
							Spore automatically includes a pre-commit hook that updates copied packages
							before each commit, ensuring your linked packages are always current.
						</p>
						<CodeBlock label="bash" copy="spore link --update-copied"><span class="text-zinc-600"># Automatically runs before each commit</span>
<span class="text-zinc-100">spore link --update-copied</span></CodeBlock>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Manual Updates</h3>
				<p class="text-muted-foreground mb-3">
					You can also manually update copied packages when needed:
				</p>
				<CodeBlock label="bash" copy="spore link --force"><span class="text-zinc-100">spore link --force</span></CodeBlock>
				<p class="text-sm text-muted-foreground mt-2">
					Forces re-linking of all packages, useful when switching between modes or after package updates.
				</p>
			</div>
		</div>
	</section>

	<!-- Troubleshooting -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Troubleshooting</h2>

		<div class="space-y-6">
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:danger-triangle-bold" class="w-5 h-5 mr-2 text-foreground" />
						Common Issues
					</h3>
					<div class="space-y-3 text-sm">
						<div>
							<div class="font-medium">Package not found</div>
							<div class="text-muted-foreground">Check package name in configuration</div>
						</div>
						<div>
							<div class="font-medium">TypeScript errors</div>
							<div class="text-muted-foreground">Run <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore link --force</code> to refresh</div>
						</div>
						<div>
							<div class="font-medium">Import paths not working</div>
							<div class="text-muted-foreground">Verify tsAlias configuration</div>
						</div>
					</div>
				</div>

				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:tools-bold" class="w-5 h-5 mr-2 text-foreground" />
						Debug Commands
					</h3>
					<div class="space-y-3 text-sm">
						<div>
							<div class="font-medium"><code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore status</code></div>
							<div class="text-muted-foreground">Show project and linking status</div>
						</div>
						<div>
							<div class="font-medium"><code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore link --verbose</code></div>
							<div class="text-muted-foreground">Detailed linking output</div>
						</div>
						<div>
							<div class="font-medium"><code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore link --dry-run</code></div>
							<div class="text-muted-foreground">Preview changes without applying</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<a href="/docs/typescript" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:code-2" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">TypeScript Integration</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Deep dive into TypeScript configuration and advanced features.
				</p>
			</a>

			<a href="/docs/project-management" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:terminal" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">CLI Commands</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Explore all linking commands and their options in detail.
				</p>
			</a>
		</div>
	</section>
</div>
