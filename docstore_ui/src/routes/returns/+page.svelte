<script>
    import { listReturns } from '$lib/api/returns';
    import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "$lib/components/ui/table";
    import { Button } from "$lib/components/ui/button";
    import { onMount } from 'svelte';

    /** @type {import('$lib/api/returns').TaxReturn[]} */
    let returns = [];
    let loading = true;
    /** @type {string | null} */
    let error = null;

    async function loadReturns() {
        try {
            loading = true;
            returns = await listReturns();
            console.log('Loading returns...', returns);
        } catch (err) {
            error = String(err);
        } finally {
            loading = false;
        }
    }

    /** @param {number} amount */
    function formatCurrency(amount) {
        return new Intl.NumberFormat('en-US', {
            style: 'currency',
            currency: 'USD'
        }).format(amount);
    }

    onMount(() => {
        loadReturns();
    });
</script>

<div class="p-6">
    <div class="flex justify-between items-center mb-6">
        <h1 class="text-2xl font-semibold">Tax Returns</h1>
        <Button>
            <span class="mr-2">➕</span>
            Add Return
        </Button>
    </div>

    {#if loading}
        <div class="text-center py-8">Loading tax returns...</div>
    {:else if error}
        <div class="text-center py-8 text-red-500">{error}</div>
    {:else}
        <div class="border rounded-lg">
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead>Year</TableHead>
                        <TableHead>Filing Status</TableHead>
                        <TableHead>Tax Liability</TableHead>
                        <TableHead>Refund/Due</TableHead>
                        <TableHead class="text-right">Actions</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {#each returns as taxReturn}
                        <TableRow>
                            <TableCell>{taxReturn.tax_year}</TableCell>
                            <TableCell>{taxReturn.filing_status}</TableCell>
                            <TableCell>{formatCurrency(taxReturn.tax_liability)}</TableCell>
                            <TableCell>{formatCurrency(taxReturn.refund_or_amount_due)}</TableCell>
                            <TableCell class="text-right space-x-2">
                                <Button variant="ghost" size="sm">
                                    <span class="mr-2">📝</span>
                                    Edit
                                </Button>
                                <Button variant="ghost" size="sm">
                                    <span class="mr-2">📄</span>
                                    Request Files
                                </Button>
                            </TableCell>
                        </TableRow>
                    {:else}
                        <TableRow>
                            <TableCell colspan="5" class="text-center py-4">
                                No tax returns found. Add your first return to get started.
                            </TableCell>
                        </TableRow>
                    {/each}
                </TableBody>
            </Table>
        </div>
    {/if}
</div>