<script>
    import { listClients, listClientFiles } from "$lib/api/clients";
    import { createUrl } from "$lib/api/config";
    import * as Select from "$lib/components/ui/select";
    import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "$lib/components/ui/card";
    import { onMount } from "svelte";
    import { ApiError } from "$lib/api/types";

    /** @type {import('$lib/api/types').Client[]} */
    let clients = [];
    /** @type {import('$lib/api/clients').ClientFile[]} */
    let files = [];
    /** @type {string[]} */
    let fnames = [];
    /** @type {string | undefined} */
    let selectedClientId = undefined;
    let loading = true;
    /** @type {string | null} */
    let error = null;

    onMount(async () => {
        try {
            clients = await listClients();
        } catch (err) {
            error = String(err);
        } finally {
            loading = false;
        }
    });

    /** 
     * Handles the selection of a client and fetches their documents
     * @param {{value: string}} client
     */
    async function handleClientSelect(client) {
        console.log('handleClientSelect', client);
        selectedClientId =  client.value;
        loading = true;
        error = null;
        
        try {
            fnames = await listClientFiles( client.value);
            console.log('files', fnames);
        } catch (err) {
            if (err instanceof ApiError && err.status === 404) {
                // Not an error, just no files found
            } else {
                error = String(err);
            }
            fnames = [];
        } finally {
            loading = false;
        }
    }
</script>

<div class="container mx-auto p-4 space-y-4">
    <Card>
        <CardHeader>
            <CardTitle>Client Documents</CardTitle>
            <CardDescription>Select a client to view their documents</CardDescription>
        </CardHeader>
        <CardContent>
            <Select.Root onSelectedChange={handleClientSelect}>
                <Select.Trigger class="w-full">
                    <Select.Value placeholder="Select a client" />
                </Select.Trigger>
                <Select.Content>
                    {#each clients as client}
                        <Select.Item value={client.client_id.toString()}>{client.first_name} {client.last_name}</Select.Item>
                    {/each}
                </Select.Content>
            </Select.Root>
        </CardContent>
    </Card>
    {#if loading}
        <div class="text-center">Loading...</div>
    {:else if error}
        <div class="text-red-500">{error}</div>
    {:else if selectedClientId && fnames.length === 0}
        <div class="text-center">No documents found for this client</div>
    {:else if selectedClientId}
        <Card>
            <CardHeader>
                <CardTitle>Documents</CardTitle>
                <CardDescription>
                    {fnames.length} document{fnames.length === 1 ? '' : 's'} found
                </CardDescription>
            </CardHeader>
            <CardContent>
                <div class="space-y-2">
                    {#each fnames as fname}
                        <div class="flex items-center justify-between p-2 hover:bg-muted rounded-lg">
                            <div class="flex items-center gap-2">
                                <span class="text-xl">📄</span>
                                <span>{fname}</span>
                            </div>
                            <a href='{createUrl(`/files/${selectedClientId}/${fname}`)}' class="text-blue-500 hover:underline" target="_blank">View</a>
                        </div>
                    {/each}
                </div>
            </CardContent>
        </Card>
    {/if}
</div>