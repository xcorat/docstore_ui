<script>
    import { Button } from "$lib/components/ui/button";
    import { Separator } from "$lib/components/ui/separator";
    import { page } from '$app/stores';
    
    // Navigation items
    const navItems = [
        { name: 'Home', href: '/', icon: '🏠' },
        { name: 'Documents', href: '/docs', icon: '📄' },
        { name: 'Clients', href: '/clients', icon: '👥' },
        { name: 'Tax Returns', href: '/returns', icon: '📊' },
        { name: 'Settings', href: '/settings', icon: '⚙️' }
    ];

    // Check if the current path matches the nav item
    $: currentPath = $page.url.pathname;
    /** @type {(href: string) => boolean} */
    $: isActive = (href) => {
        if (href === '/') {
            return currentPath === href;
        }
        return currentPath.startsWith(href);
    };
</script>

<aside class="hidden w-64 border-r bg-muted/40 lg:block">
    <div class="flex h-full flex-col">
        <div class="p-4">
            <h2 class="text-lg font-semibold">DocStore</h2>
        </div>
        <Separator />
        <nav class="flex-1 space-y-1 p-2">
            {#each navItems as item}
                <a 
                    href={item.href}
                    class="flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:bg-accent hover:text-accent-foreground {isActive(item.href) ? 'bg-accent text-accent-foreground' : 'text-muted-foreground'}"
                >
                    <span class="text-xl">{item.icon}</span>
                    {item.name}
                </a>
            {/each}
        </nav>
        <div class="p-4">
            <Button variant="outline" class="w-full">
                <span class="mr-2">🔄</span>
                Sync
            </Button>
        </div>
    </div>
</aside>