<script lang="ts">
    import { onMount } from "svelte";
    import { Button } from "$lib/components/ui/button";
    import * as Card from "$lib/components/ui/card";
    import { api } from "$lib/api";
    import RefreshCw from "@lucide/svelte/icons/refresh-cw";
    import Loader2 from "@lucide/svelte/icons/loader-2";
    import Trash2 from "@lucide/svelte/icons/trash-2";

    interface Member {
        id: string;
        name: string;
        role: string;
    }

    let { hasEditPermission = false } = $props();

    let members = $state<Member[]>([]);
    let isLoading = $state(false);
    let fetchError = $state<string | null>(null);

    async function fetchMembers() {
        console.log("Fetching members from backend...");
        isLoading = true;
        fetchError = null;
        try {
            const data = await api.get<any>(
                "/rbac-demo/members?current_page=1&page_size=100",
            );
            console.log("Fetched members successfully:", data);
            const roles = ["Dev", "Designer", "QA"];
            if (data.results) {
                members = data.results.map((m: any) => ({
                    id: m.member_id,
                    name: `${m.first_name} ${m.last_name}`,
                    role: roles[Math.floor(Math.random() * roles.length)],
                }));
            } else {
                console.warn("No members found in response");
                members = [];
            }
        } catch (error) {
            fetchError = error instanceof Error ? error.message : String(error);
            console.error("Network error while fetching members:", error);
        } finally {
            isLoading = false;
        }
    }

    async function addMember() {
        console.log("Adding new member...");
        const firstNames = ["Charlie", "Diana", "Edward", "Fiona"];
        const lastNames = ["Smith", "Jones", "Brown", "Williams"];
        const roles = ["Dev", "Designer", "QA"];

        const firstName =
            firstNames[Math.floor(Math.random() * firstNames.length)];
        const lastName =
            lastNames[Math.floor(Math.random() * lastNames.length)];
        const role = roles[Math.floor(Math.random() * roles.length)];

        try {
            const member = await api.post<any>("/rbac-demo/members", {
                first_name: firstName,
                last_name: lastName,
            });
            console.log("Added member successfully:", member);
            members = [
                ...members,
                {
                    id: member.member_id,
                    name: `${member.first_name} ${member.last_name}`,
                    role: role,
                },
            ];
        } catch (error) {
            console.error("Network error while adding member:", error);
        }
    }

    async function deleteMember(id: string) {
        console.log(`Deleting member ${id}...`);
        try {
            await api.delete(`/rbac-demo/members/${id}`);
            console.log("Deleted member successfully");
            members = members.filter((m) => m.id !== id);
        } catch (error) {
            console.error("Network error while deleting member:", error);
        }
    }

    onMount(() => {
        fetchMembers();
    });
</script>

<Card.Root>
    <Card.Header class="pb-2">
        <Card.Title class="text-lg flex items-center justify-between">
            <span>👥 Team Members</span>
            <Button
                variant="ghost"
                size="icon"
                class="h-7 w-7"
                onclick={fetchMembers}
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
            {#if isLoading && members.length === 0}
                <div
                    class="flex items-center justify-center p-4 text-muted-foreground"
                >
                    <Loader2 class="h-4 w-4 animate-spin mr-2" />
                    Loading members...
                </div>
            {:else if fetchError}
                <div class="p-3 rounded bg-red-500/10 text-red-500 text-xs">
                    <p class="font-bold">Connection Error</p>
                    <p>{fetchError}</p>
                    <Button
                        size="sm"
                        variant="outline"
                        class="mt-2 h-7"
                        onclick={fetchMembers}
                    >
                        Retry
                    </Button>
                </div>
            {:else}
                {#each members as member (member.id)}
                    <div
                        class="bg-muted p-2 rounded text-sm flex justify-between items-center animate-in fade-in slide-in-from-left-2"
                    >
                        <span>
                            {member.name} ({member.role})
                        </span>
                        {#if hasEditPermission}
                            <Button
                                variant="ghost"
                                size="icon"
                                class="h-6 w-6 text-muted-foreground hover:text-destructive transition-colors"
                                onclick={() => deleteMember(member.id)}
                            >
                                <Trash2 class="h-3.5 w-3.5" />
                            </Button>
                        {/if}
                    </div>
                {/each}
            {/if}
        </div>
        {#if hasEditPermission}
            <div class="mt-4 pt-4 border-t flex justify-end">
                <Button size="sm" onclick={addMember}>➕ Add Member</Button>
            </div>
        {/if}
    </Card.Content>
</Card.Root>
