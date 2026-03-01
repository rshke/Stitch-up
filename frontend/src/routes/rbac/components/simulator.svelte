<script lang="ts">
    import { type Component, COMPONENTS } from "./registry";
    import { Button } from "$lib/components/ui/button";
    import MemberWidget from "./widgets/MemberWidget.svelte";
    import ProjectBoard from "./widgets/ProjectBoard.svelte";
    import DebuggerPanel from "./widgets/DebuggerPanel.svelte";

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
                        >
                            Members
                        </Button>
                    {/if}
                    {#if hasComponent("comp_board_view")}
                        <Button
                            variant="ghost"
                            size="sm"
                            onclick={() => scrollTo("widget-projects")}
                        >
                            Projects
                        </Button>
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
                        <MemberWidget
                            hasEditPermission={hasComponent("comp_member_edit")}
                        />
                    </div>
                {/if}

                <!-- Widget: Project Board -->
                {#if hasComponent("comp_board_view")}
                    <div
                        id="widget-projects"
                        class="transition-all duration-500 rounded-lg"
                    >
                        <ProjectBoard
                            canArchive={hasComponent("comp_task_archive")}
                            hasEditPermission={hasComponent(
                                "comp_project_edit",
                            )}
                        />
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
    <DebuggerPanel {activeComponentCodes} {finalPermissions} />
</div>
