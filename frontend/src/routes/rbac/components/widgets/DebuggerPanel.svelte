<script lang="ts">
    interface Props {
        activeComponentCodes: string[];
        finalPermissions: string[];
    }

    let { activeComponentCodes, finalPermissions }: Props = $props();
</script>

<div class="space-y-6">
    <h2 class="text-xl font-bold flex items-center gap-2">
        <span class="bg-yellow-500/10 p-2 rounded-md text-yellow-500">🛠️</span>
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
