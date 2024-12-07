<script>
    import { listClients } from '$lib/api/clients';
    import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "$lib/components/ui/table";
    import { Button } from "$lib/components/ui/button";
    import { onMount } from 'svelte';

    /** @type {import('$lib/api/types').Client[]} */
    let clients = [];
    let loading = true;
    /** @type {string | null} */
    let error = null;

    async function loadClients() {
        try {
            loading = true;
            clients = await listClients();
        } catch (err) {
            error = String(err) || 'Unknown error';
        } finally {
            loading = false;
        }
    }

    onMount(() => {
        loadClients();
    });
</script>

<div class="p-6">
    <div class="flex justify-between items-center mb-6">
        <h1 class="text-2xl font-semibold">Clients</h1>
        <Button>
            <span class="mr-2">➕</span>
            Add Client
        </Button>
    </div>

    {#if loading}
        <div class="text-center py-8">Loading clients...</div>
    {:else if error}
        <div class="text-center py-8 text-red-500">{error}</div>
    {:else}
        <div class="border rounded-lg">
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead>Name</TableHead>
                        <TableHead>Email</TableHead>
                        <TableHead>Phone</TableHead>
                        <TableHead>Address</TableHead>
                        <TableHead class="text-right">Actions</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {#each clients as client}
                        <TableRow>
                            <TableCell>
                                {client.first_name} {client.last_name}
                            </TableCell>
                            <TableCell>{client.email}</TableCell>
                            <TableCell>{client.phone_number}</TableCell>
                            <TableCell>{client.address}</TableCell>
                            <TableCell class="text-right">
                                <Button variant="ghost" size="sm">
                                    <span class="mr-2">📝</span>
                                    Edit
                                </Button>
                                <Button variant="ghost" size="sm">
                                    <span class="mr-2">📊</span>
                                    Returns
                                </Button>
                            </TableCell>
                        </TableRow>
                    {:else}
                        <TableRow>
                            <TableCell colspan="5" class="text-center py-4">
                                No clients found. Add your first client to get started.
                            </TableCell>
                        </TableRow>
                    {/each}
                </TableBody>
            </Table>
        </div>
    {/if}
</div>