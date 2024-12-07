<script>
    import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "$lib/components/ui/card";
</script>

<div class="container mx-auto p-4 space-y-4">
    <Card>
        <CardHeader>
            <CardTitle>Settings</CardTitle>
            <CardDescription>Configure your DocStore settings</CardDescription>
        </CardHeader>
        <CardContent>
            <div class="space-y-4">
                <div>
                    <h3 class="text-lg font-medium">Storage Settings</h3>
                    <p class="text-sm text-muted-foreground">Configure where your documents are stored</p>
                    <div class="mt-2">
                        <div class="flex items-center gap-2">
                            <span class="font-mono text-sm">Root Path:</span>
                            <span class="text-muted-foreground">/path/to/storage</span>
                        </div>
                    </div>
                </div>

                <div>
                    <h3 class="text-lg font-medium">Database Settings</h3>
                    <p class="text-sm text-muted-foreground">Database connection information</p>
                    <div class="mt-2">
                        <div class="flex items-center gap-2">
                            <span class="font-mono text-sm">Database:</span>
                            <span class="text-muted-foreground">SQLite</span>
                        </div>
                    </div>
                </div>
            </div>
        </CardContent>
    </Card>
</div>
