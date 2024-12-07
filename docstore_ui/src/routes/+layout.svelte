<script>
    import '../app.css';
	import Sidebar from '$lib/components/sidebar.svelte';

    import { Button } from "$lib/components/ui/button";
    import { Separator } from "$lib/components/ui/separator";
    
    // Navigation items
    const navItems = [
        { name: 'Documents', href: '/', icon: '📄' },
        { name: 'Clients', href: '/clients', icon: '👥' },
        { name: 'Tax Returns', href: '/returns', icon: '📊' },
        { name: 'Settings', href: '/settings', icon: '⚙️' }
    ];
    let { children } = $props();
</script>

<div class="flex h-screen">
    <Sidebar />

    <!-- Main Content -->
    <main class="flex-1 overflow-y-auto">
        {@render children()}
    </main>
</div>

<style>
    :global(html) {
        overflow: hidden;
    }
</style>
