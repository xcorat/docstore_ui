<script>
    import Dropzone from "svelte-file-dropzone";
    import { page } from '$app/stores';

    /** @type {{ accepted: File[], rejected: any[] }} */
    let files = {
        accepted: [],
        rejected: []
    };

    let uploadStatus = '';
    let isUploading = false;

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
            
            if (response.ok) {
                uploadStatus = result.message;
                files.accepted = [];
            } else {
                throw new Error(result.message || 'Upload failed');
            }
        } catch (error) {
            console.error(error);
            uploadStatus = `Error: ${error.message?? 'Unknown error'}`;
        } finally {
            isUploading = false;
        }
    }

    $: clientId = $page.params.client_id;
</script>

<div class="container mx-auto flex flex-col items-center justify-start min-h-screen p-4">
    <h1 class="text-3xl font-bold mb-8">Welcome, {clientId}!</h1>
    
    {files.accepted.length}
    <div class="w-full max-w-xl">
        <div class="upload-container">
            <Dropzone
                accept={[".pdf", ".txt", ".md"]}
                multiple={true}
                on:drop={handleFilesSelect}
                class="p-16 border-2 border-dashed rounded-lg hover:border-primary transition-colors duration-200"
            >
                <div class="text-center">
                    <p class="text-lg mb-2">Drop files here or click to select</p>
                    <p class="text-sm text-muted-foreground">Supported files: PDF, TXT, MD</p>
                </div>
            </Dropzone>

            {#if files.accepted.length > 0}
                <div class="files-list">
                    <h3 class="text-lg font-semibold mb-2">Files to upload:</h3>
                    <ul>
                        {#each files.accepted as file}
                            <li>{file.name} ({Math.round(file.size / 1024)} KB)</li>
                        {/each}
                    </ul>
                    <button 
                        on:click={handleUpload} 
                        disabled={isUploading}
                        class="upload-button"
                    >
                        {isUploading ? 'Uploading...' : 'Upload Files'}
                    </button>
                </div>
            {/if}

            {#if uploadStatus}
                <div class="status-message">
                    {uploadStatus}
                </div>
            {/if}

            {#if files.rejected.length > 0}
                <div class="mt-4 p-4 bg-destructive/10 rounded">
                    <h3 class="text-lg font-semibold text-destructive mb-2">Rejected Files:</h3>
                    <ul class="space-y-1">
                        {#each files.rejected as rejection}
                            <li class="text-sm text-destructive">
                                {rejection.file.name} - {rejection.errors[0].message}
                            </li>
                        {/each}
                    </ul>
                </div>
            {/if}
        </div>
    </div>
</div>

<style>
    .upload-container {
        max-width: 600px;
        margin: 2rem auto;
    }

    .files-list {
        margin-top: 1rem;
        padding: 1rem;
        background: #f5f5f5;
        border-radius: 4px;
    }

    .files-list ul {
        list-style: none;
        padding: 0;
    }

    .files-list li {
        padding: 0.5rem 0;
        border-bottom: 1px solid #eee;
    }

    .upload-button {
        margin-top: 1rem;
        padding: 0.5rem 1rem;
        background: #4CAF50;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
    }

    .upload-button:disabled {
        background: #cccccc;
        cursor: not-allowed;
    }

    .status-message {
        margin-top: 1rem;
        padding: 1rem;
        border-radius: 4px;
        background: #e3f2fd;
    }
</style>
