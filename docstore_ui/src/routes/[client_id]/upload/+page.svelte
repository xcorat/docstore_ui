<script>
    import Dropzone from "svelte-file-dropzone";
    import { page } from '$app/stores';
    import { Button } from "$lib/components/ui/button";
    import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "$lib/components/ui/card";
    import { getClient } from "$lib/api/clients";
    import { onMount } from "svelte";

    /** @type {{ accepted: File[], rejected: any[] }} */
    let files = {
        accepted: [],
        rejected: []
    };

    let uploadStatus = '';
    let isUploading = false;
    /** @type {import('$lib/api/types').Client | null} */
    let client = null;

    onMount(async () => {
        try {
            client = await getClient($page.params.client_id);
        } catch (error) {
            console.error('Failed to load client:', error);
        }
    });

    /**
     * @param {CustomEvent<{acceptedFiles: File[], fileRejections: any[]}>} e
     */
    async function handleFilesSelect(e) {
        const { acceptedFiles, fileRejections } = e.detail;
        files = {
            accepted: [...files.accepted, ...acceptedFiles],
            rejected: [...files.rejected, ...fileRejections]
        };
    }

    async function handleUpload() {
        if (files.accepted.length === 0) {
            uploadStatus = 'No files selected';
            return;
        }

        isUploading = true;
        uploadStatus = 'Uploading files...';
        try {
            const formData = new FormData();
            files.accepted.forEach(file => {
                formData.append('files', file, file.name);
            });
            const response = await fetch(`/api/files/upload/${$page.params.client_id}`, {
                method: 'POST',
                body: formData
            })

            const result = await response.json();
            console.log(result);
            
            if (response.ok) {
                uploadStatus = `Successfully uploaded ${result.files.length} files`;
                files.accepted = [];
            } else {
                throw new Error(result.message || 'Upload failed');
            }
        } catch (error) {
            console.error(error);
            uploadStatus = `Error: ${String(error)}`;
        } finally {
            isUploading = false;
        }
    }

    $: clientId = $page.params.client_id;
</script>

<div class="container mx-auto p-4 space-y-4">
    <Card>
        <CardHeader>
            <CardTitle>Upload Documents</CardTitle>
            <CardDescription>
                {#if client}
                    Upload documents for {client.first_name} {client.last_name}
                {:else}
                    Upload client documents
                {/if}
            </CardDescription>
        </CardHeader>
        <CardContent>
            <div class="space-y-4">
                <Dropzone
                    accept={[".pdf", ".txt", ".md"]}
                    multiple={true}
                    on:drop={handleFilesSelect}
                    class="p-16 border-2 border-dashed rounded-lg hover:border-primary transition-colors duration-200 bg-muted/50"
                >
                    <div class="text-center space-y-2">
                        <div class="text-4xl mb-4">📄</div>
                        <p class="text-lg font-medium">Drop files here or click to select</p>
                        <p class="text-sm text-muted-foreground">Supported files: PDF, TXT, MD</p>
                    </div>
                </Dropzone>

                {#if files.accepted.length > 0}
                    <div class="space-y-4">
                        <h3 class="text-lg font-medium">Files to upload</h3>
                        <div class="space-y-2">
                            {#each files.accepted as file}
                                <div class="flex items-center justify-between p-2 bg-muted rounded-lg">
                                    <div class="flex items-center gap-2">
                                        <span class="text-xl">📄</span>
                                        <span>{file.name}</span>
                                        <span class="text-sm text-muted-foreground">({Math.round(file.size / 1024)} KB)</span>
                                    </div>
                                </div>
                            {/each}
                        </div>
                        <div class="flex justify-end">
                            <Button 
                                on:click={handleUpload} 
                                disabled={isUploading}
                            >
                                {#if isUploading}
                                    <span class="mr-2">⏳</span> Uploading...
                                {:else}
                                    <span class="mr-2">📤</span> Upload Files
                                {/if}
                            </Button>
                        </div>
                    </div>
                {/if}

                {#if uploadStatus}
                    <div class={`p-4 rounded-lg ${uploadStatus.includes('Error') ? 'bg-destructive/10 text-destructive' : 'bg-primary/10 text-primary'}`}>
                        {uploadStatus}
                    </div>
                {/if}
            </div>
        </CardContent>
    </Card>
</div>

<style>
    :global(.dropzone) {
        border: 2px dashed hsl(var(--muted-foreground));
        transition: all 0.2s ease;
    }
    
    :global(.dropzone:hover),
    :global(.dropzone.active) {
        border-color: hsl(var(--primary));
        background-color: hsl(var(--muted) / 0.7);
    }
</style>
