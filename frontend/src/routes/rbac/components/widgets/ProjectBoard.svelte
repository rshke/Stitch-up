<script lang="ts">
    import { onMount } from "svelte";
    import * as Card from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
    import { api } from "$lib/api";
    import RefreshCw from "@lucide/svelte/icons/refresh-cw";
    import Loader2 from "@lucide/svelte/icons/loader-2";

    interface Project {
        project_id: string;
        name: string;
        description: string;
    }

    let { canArchive = false, hasEditPermission = false } = $props();

    let projects = $state<Project[]>([]);
    let isLoading = $state(false);
    let fetchError = $state<string | null>(null);

    async function fetchProjects() {
        console.log("Fetching projects from backend...");
        isLoading = true;
        fetchError = null;
        try {
            const data = await api.get<any>(
                "/rbac-demo/projects?current_page=1&page_size=100",
            );
            console.log("Fetched projects successfully:", data);
            if (data.results) {
                projects = data.results;
            } else {
                console.warn("No projects found in response");
                projects = [];
            }
        } catch (error) {
            fetchError = error instanceof Error ? error.message : String(error);
            console.error("Network error while fetching projects:", error);
        } finally {
            isLoading = false;
        }
    }

    async function addProject() {
        console.log("Adding new project...");
        const names = ["Apollo", "Beagle", "Curiosity", "Dragon"];
        const descs = [
            "System core",
            "Mobile app",
            "Data pipeline",
            "UI redesign",
        ];

        const name = `${names[Math.floor(Math.random() * names.length)]} ${Math.floor(Math.random() * 1000)}`;
        const description = descs[Math.floor(Math.random() * descs.length)];

        try {
            const project = await api.post<Project>("/rbac-demo/projects", {
                name,
                description,
            });
            console.log("Created project successfully:", project);
            projects = [...projects, project];
        } catch (error) {
            console.error("Failed to add project:", error);
        }
    }

    async function archiveProject(id: string) {
        console.log(`Archiving project ${id}...`);
        try {
            await api.delete(`/rbac-demo/projects/${id}`);
            console.log("Archived project successfully");
            projects = projects.filter((p) => p.project_id !== id);
        } catch (error) {
            console.error("Network error while archiving project:", error);
        }
    }

    onMount(() => {
        fetchProjects();
    });
</script>

<Card.Root>
    <Card.Header class="pb-2">
        <Card.Title class="text-lg flex items-center justify-between">
            <span>🚀 Project Alpha</span>
            <Button
                variant="ghost"
                size="icon"
                class="h-7 w-7"
                onclick={fetchProjects}
                disabled={isLoading}
            >
                <RefreshCw
                    class="h-3.5 w-3.5 {isLoading ? 'animate-spin' : ''}"
                />
            </Button>
        </Card.Title>
    </Card.Header>
    <Card.Content>
        <div class="space-y-2">
            {#if isLoading && projects.length === 0}
                <div
                    class="flex items-center justify-center p-4 text-muted-foreground"
                >
                    <Loader2 class="h-4 w-4 animate-spin mr-2" />
                    Loading projects...
                </div>
            {:else if fetchError}
                <div class="p-3 rounded bg-red-500/10 text-red-500 text-xs">
                    <p class="font-bold">Connection Error</p>
                    <p>{fetchError}</p>
                    <Button
                        size="sm"
                        variant="outline"
                        class="mt-2 h-7"
                        onclick={fetchProjects}
                    >
                        Retry
                    </Button>
                </div>
            {:else}
                {#each projects as project (project.project_id)}
                    <div
                        class="flex items-center gap-2 animate-in fade-in slide-in-from-top-2 p-2 bg-muted/30 rounded"
                    >
                        <div class="flex flex-col">
                            <span class="text-sm font-medium"
                                >{project.name}</span
                            >
                            {#if project.description}
                                <span
                                    class="text-xs text-muted-foreground truncate max-w-[150px]"
                                >
                                    {project.description}
                                </span>
                            {/if}
                        </div>
                        {#if canArchive}
                            <button
                                class="ml-auto text-xs text-muted-foreground hover:text-foreground transition-colors text-red-400 hover:text-red-500"
                                onclick={() =>
                                    archiveProject(project.project_id)}
                            >
                                Remove
                            </button>
                        {/if}
                    </div>
                {/each}
                {#if projects.length === 0}
                    <p class="text-sm text-muted-foreground italic">
                        No active projects.
                    </p>
                {/if}
            {/if}
        </div>
        {#if hasEditPermission}
            <div class="mt-4 pt-4 border-t flex justify-end">
                <Button size="sm" onclick={addProject}>➕ Add Project</Button>
            </div>
        {/if}
    </Card.Content>
</Card.Root>
