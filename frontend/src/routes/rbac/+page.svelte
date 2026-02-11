<script lang="ts">
    import { COMPONENTS, ROLES, type Component } from "./components/registry";
    import Simulator from "./components/simulator.svelte";
    import { Button } from "$lib/components/ui/button";
    import * as Card from "$lib/components/ui/card";
    import { Label } from "$lib/components/ui/label";
    import { Checkbox } from "$lib/components/ui/checkbox";
    import { Badge } from "$lib/components/ui/badge";

    // State
    let selectedRoleKey = $state("ADMIN");
    let activeComponentCodes = $state([...ROLES.ADMIN.components]);

    // Derived
    let selectedRole = $derived(ROLES[selectedRoleKey as keyof typeof ROLES]);

    function selectRole(key: string) {
        selectedRoleKey = key;
        // Reset components to the role's default when switching roles
        activeComponentCodes = [...ROLES[key as keyof typeof ROLES].components];
    }

    function toggleComponent(code: string) {
        if (activeComponentCodes.includes(code)) {
            activeComponentCodes = activeComponentCodes.filter(
                (c) => c !== code,
            );
        } else {
            activeComponentCodes = [...activeComponentCodes, code];
        }
    }
</script>

<div class="container mx-auto py-8 h-[calc(100vh-4rem)] flex flex-col">
    <div class="mb-6 flex items-center justify-between">
        <div>
            <h1 class="text-3xl font-bold tracking-tight">
                Component-First RBAC Demo
            </h1>
            <p class="text-muted-foreground mt-1">
                Visualizing how "UI Components" map to "API Permissions".
            </p>
        </div>
        <div class="flex gap-2">
            <Button variant="outline" href="/">Back to Home</Button>
            <Button href="https://github.com/rshke/Stitch-up" target="_blank"
                >View Source</Button
            >
        </div>
    </div>

    <div class="grid grid-cols-12 gap-8 flex-1 min-h-0">
        <!-- LEFT PANEL: Configuration -->
        <div
            class="col-span-12 md:col-span-4 lg:col-span-3 flex flex-col gap-6 overflow-y-auto pr-2"
        >
            <!-- 1. Role Selector -->
            <Card.Root>
                <Card.Header class="pb-3">
                    <Card.Title>1. Select Role</Card.Title>
                    <Card.Description>Choose a preset persona</Card.Description>
                </Card.Header>
                <Card.Content class="grid gap-2">
                    {#each Object.entries(ROLES) as [key, role]}
                        <Button
                            variant={selectedRoleKey === key
                                ? "default"
                                : "outline"}
                            class="justify-start"
                            onclick={() => selectRole(key)}
                        >
                            {role.name}
                        </Button>
                    {/each}
                </Card.Content>
            </Card.Root>

            <!-- 2. Component Toggles -->
            <Card.Root class="flex-1 flex flex-col min-h-0">
                <Card.Header class="pb-3">
                    <Card.Title>2. Authorized Components</Card.Title>
                    <Card.Description>
                        Granting <strong>UI capabilities</strong> automatically grants
                        necessary API permissions.
                    </Card.Description>
                </Card.Header>
                <Card.Content class="flex-1 overflow-y-auto space-y-4">
                    {#each COMPONENTS as comp}
                        <div
                            class="flex items-start space-x-3 p-3 rounded-md border transition-colors hover:bg-accent/50 {activeComponentCodes.includes(
                                comp.code,
                            )
                                ? 'bg-accent/20 border-primary/20'
                                : ''}"
                        >
                            <Checkbox
                                id={comp.code}
                                checked={activeComponentCodes.includes(
                                    comp.code,
                                )}
                                onCheckedChange={() =>
                                    toggleComponent(comp.code)}
                            />
                            <div class="grid gap-1.5 leading-none">
                                <Label
                                    for={comp.code}
                                    class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 cursor-pointer"
                                >
                                    {comp.name}
                                </Label>
                                <p class="text-xs text-muted-foreground">
                                    {comp.description}
                                </p>
                                <div class="pt-1 flex flex-wrap gap-1">
                                    {#each comp.permissions as perm}
                                        <Badge
                                            variant="outline"
                                            class="text-[10px] px-1 py-0 h-4 bg-background"
                                        >
                                            {perm.method}
                                        </Badge>
                                    {/each}
                                </div>
                            </div>
                        </div>
                    {/each}
                </Card.Content>
            </Card.Root>
        </div>

        <!-- RIGHT PANEL: Simulation -->
        <div
            class="col-span-12 md:col-span-8 lg:col-span-9 h-full min-h-0 rounded-xl border bg-zinc-50/50 dark:bg-zinc-900/20 p-6 overflow-y-auto"
        >
            <Simulator {activeComponentCodes} />
        </div>
    </div>
</div>
