<script>
    import { Button } from "$lib/components/ui/button";
    import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "$lib/components/ui/card";

    function handleAccept() {
        // Handle accept action here
        console.log('Accepted');
    }
</script>

<div class="container mx-auto flex items-center justify-center min-h-screen p-4">
    <Card class="w-[350px]">
        <CardHeader>
            <CardTitle>Welcome to DocStore</CardTitle>
            <CardDescription>Your secure document management solution</CardDescription>
        </CardHeader>
        <CardContent>
            <p class="text-muted-foreground">
                Store, manage, and access your documents securely. Click accept to continue.
            </p>
        </CardContent>
        <CardFooter>
            <Button on:click={handleAccept} class="w-full">Accept</Button>
        </CardFooter>
    </Card>
</div>
