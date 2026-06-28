<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { authStore } from '$lib/stores';
	import Icon from '@iconify/svelte';
	import Input from '../../lib/components/ui/input.svelte';
	import Button from '../../lib/components/ui/button.svelte';

	let username = '';
	let password = '';
	let error = '';

	$: loading = $authStore.loading;

	onMount(async () => {
		// Wait for auth to initialize before checking authentication status
		if (!$authStore.initialized) {
			await authStore.initialize();
		}

		// Redirect if already logged in
		if ($authStore.isAuthenticated) {
			goto('/packages');
		}
	});

	async function handleLogin() {
		if (!username || !password) {
			error = 'Please fill in all fields';
			return;
		}

		error = '';

		try {
			await authStore.login(username.trim(), password);
			goto('/packages');
		} catch (err) {
			error = err instanceof Error ? err.message : 'Login failed';
		}
	}

	function handleKeyPress(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			handleLogin();
		}
	}
</script>

<svelte:head>
	<title>Login - Spore</title>
</svelte:head>

<div class="mx-auto max-w-5xl">
	<div class="grid overflow-hidden rounded-xl border border-border bg-card lg:min-h-[560px] lg:grid-cols-2">
		<!-- Brand panel -->
		<div class="hidden flex-col bg-primary px-12 py-10 text-primary-foreground lg:flex">
			<a href="/" class="mb-auto flex items-center gap-3">
				<Icon icon="solar:box-bold-duotone" class="h-7 w-7 text-primary-foreground" />
				<span class="font-['Goga'] text-xl font-bold tracking-tight">Spore</span>
			</a>

			<div class="mb-auto">
				<h2 class="text-4xl font-bold leading-tight tracking-tight text-primary-foreground">
					Publish.<br />Share.<br />Reuse.
				</h2>
				<p class="mt-4 max-w-xs text-sm leading-relaxed text-primary-foreground/60">
					The modern package registry for your monorepo packages.
				</p>
			</div>

			<p class="text-xs text-primary-foreground/40">
				&copy; {new Date().getFullYear()} Spore by Facile.
			</p>
		</div>

		<!-- Form panel -->
		<div class="flex w-full flex-col items-center justify-center bg-background px-8 py-12">
			<div class="w-full max-w-sm">
				<div class="mb-8 flex items-center justify-center gap-3 lg:hidden">
					<Icon icon="solar:box-bold-duotone" class="h-8 w-8 text-foreground" />
					<span class="font-['Goga'] text-xl font-bold">Spore</span>
				</div>

				<div class="mb-8">
					<h1 class="text-2xl font-bold tracking-tight text-foreground">Welcome back</h1>
					<p class="mt-1.5 text-sm text-muted-foreground">Sign in to your Spore account.</p>
				</div>

				<div class="mb-6 flex gap-1 rounded-lg border border-border bg-muted p-1">
					<a
						href="/login"
						class="flex-1 rounded-md bg-background py-1.5 text-center text-sm font-medium text-foreground shadow-sm"
					>
						Log in
					</a>
					<a
						href="/register"
						class="flex-1 rounded-md py-1.5 text-center text-sm font-medium text-muted-foreground transition-colors hover:text-foreground"
					>
						Register
					</a>
				</div>

				{#if error}
					<div class="mb-4 rounded-md border border-destructive/20 bg-destructive/10 p-3 text-sm text-destructive">
						{error}
					</div>
				{/if}

				<form on:submit|preventDefault={handleLogin} class="space-y-4">
					<div class="space-y-1.5">
						<label for="username" class="text-sm font-medium leading-none">Username</label>
						<Input
							id="username"
							type="text"
							bind:value={username}
							on:keypress={handleKeyPress}
							placeholder="Enter your username"
							disabled={loading}
							required
						/>
					</div>

					<div class="space-y-1.5">
						<label for="password" class="text-sm font-medium leading-none">Password</label>
						<Input
							id="password"
							type="password"
							bind:value={password}
							on:keypress={handleKeyPress}
							placeholder="Enter your password"
							disabled={loading}
							required
						/>
					</div>

					<Button type="submit" disabled={loading} class="w-full">
						{#if loading}
							<Icon icon="solar:refresh-bold" class="mr-2 h-4 w-4 animate-spin" />
							Signing In...
						{:else}
							<Icon icon="solar:login-3-bold" class="mr-2 h-4 w-4" />
							Sign In
						{/if}
					</Button>
				</form>
			</div>
		</div>
	</div>
</div>
