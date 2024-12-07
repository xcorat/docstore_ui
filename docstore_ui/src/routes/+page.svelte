<script>
    import { Button } from "$lib/components/ui/button";
    import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "$lib/components/ui/card";
    import { Separator } from "$lib/components/ui/separator";

    function handleAccept() {
        // Handle accept action here
        console.log('Accepted');
    }
</script>

<!-- Main Content -->
<main class="flex-1 overflow-y-auto">
    <div class="container mx-auto p-6">
        <Card class="w-full max-w-4xl mx-auto">
            <CardHeader>
                <CardTitle>Welcome to DocStore</CardTitle>
                <CardDescription>Your secure document management solution</CardDescription>
            </CardHeader>
            <CardContent>
                <div class="grid gap-6">
                    <div class="flex items-center justify-between">
                        <div>
                            <h3 class="text-lg font-medium">Recent Documents</h3>
                            <p class="text-sm text-muted-foreground">
                                Your most recently accessed documents will appear here
                            </p>
                        </div>
                        <Button>
                            <span class="mr-2">📤</span>
                            Upload
                        </Button>
                    </div>
                    <Separator />
                    <div class="text-center text-muted-foreground py-8">
                        No documents yet. Start by uploading your first document.
                    </div>
                    <div class="flex items-center justify-center">
                        <Button on:click={handleAccept} class="w-full">Accept</Button>
                    </div>
                </div>
            </CardContent>
        </Card>
    </div>
</main>
<style>
</style>
