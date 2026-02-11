<script lang="ts">
    import { type Component, COMPONENTS } from "./registry";
    import { Button } from "$lib/components/ui/button";
    import * as Card from "$lib/components/ui/card";
    import { Badge } from "$lib/components/ui/badge";

    let { activeComponentCodes = [] }: { activeComponentCodes: string[] } =
        $props();

    // Reactive: Compute active components object for easy lookup
    let activeComponentsMap = $derived(new Set(activeComponentCodes));

    // Reactive: Compute final API permissions (Union of all component permissions)
    let finalPermissions = $derived.by(() => {
        const perms = new Set<string>();
        activeComponentCodes.forEach((code) => {
            const comp = COMPONENTS.find((c) => c.code === code);
            if (comp) {
                comp.permissions.forEach((p) =>
                    perms.add(`${p.method} ${p.path}`),
                );
            }
        });
        return Array.from(perms).sort();
    });

    function hasComponent(code: string): boolean {
        return activeComponentsMap.has(code);
    }

    function scrollTo(id: string) {
        const el = document.getElementById(id);
        if (el) {
            el.scrollIntoView({ behavior: "smooth", block: "center" });
            // Add a brief highlight effect
            el.classList.add("ring-2", "ring-primary", "ring-offset-2");
            setTimeout(() => {
                el.classList.remove("ring-2", "ring-primary", "ring-offset-2");
            }, 2000);
        }
    }

    // --- Interactive State ---
    let members = $state([
        { id: 1, name: "Alice", role: "Admin" },
        { id: 2, name: "Bob", role: "Dev" },
    ]);

    let tasks = $state([
        { id: 101, title: "Frontend Design", status: "In Progress" },
        { id: 102, title: "Backend API", status: "Todo" },
    ]);

    function addMember() {
        const names = ["Charlie", "Diana", "Edward", "Fiona"];
        const roles = ["Dev", "Designer", "QA"];
        const newMember = {
            id: Date.now(),
            name: names[Math.floor(Math.random() * names.length)],
            role: roles[Math.floor(Math.random() * roles.length)],
        };
        members.push(newMember);
    }

    function archiveTask(id: number) {
        const index = tasks.findIndex((t) => t.id === id);
        if (index !== -1) {
            tasks.splice(index, 1);
        }
    }
</script>

<div class="grid grid-cols-1 lg:grid-cols-3 gap-6 h-full">
    <!-- Simulated UI (Left 2 cols) -->
    <div class="lg:col-span-2 space-y-6">
        <h2 class="text-xl font-bold flex items-center gap-2">
            <span class="bg-primary/10 p-2 rounded-md text-primary">🖥️</span>
            Live Dashboard Preview
        </h2>

        <div
            class="border rounded-lg p-6 bg-card min-h-[400px] shadow-sm relative overflow-hidden"
        >
            <!-- Simulated Navigation Bar -->
            <div class="flex items-center gap-4 border-b pb-4 mb-6">
                <div class="w-8 h-8 bg-primary/20 rounded-full"></div>
                <div class="font-bold">Team Workspace</div>

                <div class="ml-auto flex gap-2">
                    {#if hasComponent("comp_member_list")}
                        <Button
                            variant="ghost"
                            size="sm"
                            onclick={() => scrollTo("widget-members")}
                            >Members</Button
                        >
                    {/if}
                    {#if hasComponent("comp_board_view")}
                        <Button
                            variant="ghost"
                            size="sm"
                            onclick={() => scrollTo("widget-projects")}
                            >Projects</Button
                        >
                    {/if}
                    {#if hasComponent("comp_audit_log")}
                        <Button
                            variant="ghost"
                            size="sm"
                            class="text-red-400 hover:text-red-300"
                            onclick={() => scrollTo("widget-audit")}
                            >Audit Logs</Button
                        >
                    {/if}
                </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <!-- Widget: Member List -->
                {#if hasComponent("comp_member_list")}
                    <div
                        id="widget-members"
                        class="transition-all duration-500 rounded-lg"
                    >
                        <Card.Root>
                            <Card.Header class="pb-2">
                                <Card.Title class="text-lg"
                                    >👥 Team Members</Card.Title
                                >
                            </Card.Header>
                            <Card.Content>
                                <div class="space-y-2">
                                    {#each members as member (member.id)}
                                        <div
                                            class="bg-muted p-2 rounded text-sm flex justify-between animate-in fade-in slide-in-from-left-2"
                                        >
                                            <span
                                                >{member.name} ({member.role})</span
                                            >
                                        </div>
                                    {/each}
                                </div>
                                {#if hasComponent("comp_member_edit")}
                                    <div
                                        class="mt-4 pt-4 border-t flex justify-end"
                                    >
                                        <Button size="sm" onclick={addMember}
                                            >➕ Add Member</Button
                                        >
                                    </div>
                                {/if}
                            </Card.Content>
                        </Card.Root>
                    </div>
                {/if}

                <!-- Widget: Project Board -->
                {#if hasComponent("comp_board_view")}
                    <div
                        id="widget-projects"
                        class="transition-all duration-500 rounded-lg"
                    >
                        <Card.Root>
                            <Card.Header class="pb-2">
                                <Card.Title class="text-lg"
                                    >🚀 Project Alpha</Card.Title
                                >
                            </Card.Header>
                            <Card.Content>
                                <div class="space-y-2">
                                    {#each tasks as task (task.id)}
                                        <div
                                            class="flex items-center gap-2 animate-in fade-in slide-in-from-top-2"
                                        >
                                            <div
                                                class="w-2 h-2 rounded-full {task.status ===
                                                'In Progress'
                                                    ? 'bg-green-500'
                                                    : 'bg-yellow-500'}"
                                            ></div>
                                            <span class="text-sm"
                                                >{task.title}</span
                                            >
                                            {#if hasComponent("comp_task_archive")}
                                                <button
                                                    class="ml-auto text-xs text-muted-foreground hover:text-foreground transition-colors"
                                                    onclick={() =>
                                                        archiveTask(task.id)}
                                                    >Archive</button
                                                >
                                            {/if}
                                        </div>
                                    {/each}
                                    {#if tasks.length === 0}
                                        <p
                                            class="text-sm text-muted-foreground italic"
                                        >
                                            No active tasks.
                                        </p>
                                    {/if}
                                </div>
                            </Card.Content>
                        </Card.Root>
                    </div>
                {/if}

                <!-- Widget: Audit Log -->
                {#if hasComponent("comp_audit_log")}
                    <div
                        id="widget-audit"
                        class="md:col-span-2 transition-all duration-500 rounded-lg"
                    >
                        <Card.Root class="border-red-500/20 bg-red-500/5">
                            <Card.Header class="pb-2">
                                <Card.Title class="text-lg text-red-500"
                                    >🔒 Sensitive Audit Logs</Card.Title
                                >
                                <Card.Description
                                    >Only authorized auditors can see this.</Card.Description
                                >
                            </Card.Header>
                            <Card.Content>
                                <div class="font-mono text-xs space-y-1">
                                    <p>
                                        [2023-10-27 10:00] Alice deleted Project
                                        X
                                    </p>
                                    <p>
                                        [2023-10-27 10:05] Bob accessed Secret
                                        Key
                                    </p>
                                </div>
                            </Card.Content>
                        </Card.Root>
                    </div>
                {/if}
            </div>

            {#if activeComponentCodes.length === 0}
                <div
                    class="absolute inset-0 flex items-center justify-center text-muted-foreground bg-background/50 backdrop-blur-sm"
                >
                    Select a component to preview UI
                </div>
            {/if}
        </div>
    </div>

    <!-- Debugger (Right col) -->
    <div class="space-y-6">
        <h2 class="text-xl font-bold flex items-center gap-2">
            <span class="bg-yellow-500/10 p-2 rounded-md text-yellow-500"
                >🛠️</span
            >
            Backend State
        </h2>

        <div
            class="border rounded-lg p-4 bg-muted/50 font-mono text-xs min-h-[400px]"
        >
            <div class="mb-4">
                <div class="font-bold text-muted-foreground mb-2">
                    AUTHORIZED COMPONENTS:
                </div>
                {#if activeComponentCodes.length > 0}
                    <ul class="space-y-1">
                        {#each activeComponentCodes as code}
                            <li class="text-green-500">✓ {code}</li>
                        {/each}
                    </ul>
                {:else}
                    <span class="text-muted-foreground opacity-50">None</span>
                {/if}
            </div>

            <div class="border-t border-border/50 my-4"></div>

            <div>
                <div class="font-bold text-muted-foreground mb-2">
                    CALCULATED API SCOPES:
                </div>
                {#if finalPermissions.length > 0}
                    <ul class="space-y-1">
                        {#each finalPermissions as perm}
                            <li class="text-blue-400">{perm}</li>
                        {/each}
                    </ul>
                {:else}
                    <span class="text-muted-foreground opacity-50"
                        >No API access</span
                    >
                {/if}

                <div
                    class="mt-4 p-2 bg-yellow-500/10 rounded text-yellow-600 dark:text-yellow-400"
                >
                    ℹ️ Note how permissions merge. If multiple components need <code
                        >GET /api/members</code
                    >, it appears once. It only disappears when ALL components
                    needing it are removed.
                </div>
            </div>
        </div>
    </div>
</div>
