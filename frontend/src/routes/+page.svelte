<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
</script>

<div class="container mx-auto py-10">
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <!-- Access Control Card -->
        <Card.Root class="hover:shadow-lg transition-shadow duration-300">
            <Card.Header>
                <Card.Title>Access Control</Card.Title>
                <Card.Description
                    >Manage user permissions and roles efficiently.</Card.Description
                >
            </Card.Header>
            <Card.Content>
                <div
                    class="aspect-video w-full overflow-hidden rounded-md flex items-center justify-center bg-black/50"
                >
                    <img
                        src="/access-control.png"
                        alt="Access Control"
                        class="h-full w-full object-cover transition-transform duration-300 hover:scale-105"
                    />
                </div>
            </Card.Content>
            <Card.Footer class="flex justify-end">
                <Button variant="secondary" href="/rbac">View Project</Button>
            </Card.Footer>
        </Card.Root>

        <!-- Placeholder for future cards -->
        <!-- <Card.Root class="opacity-50">
             <Card.Content class="flex items-center justify-center h-64">
                 <p class="text-muted-foreground">More coming soon...</p>
             </Card.Content>
        </Card.Root> -->
    </div>
</div>
