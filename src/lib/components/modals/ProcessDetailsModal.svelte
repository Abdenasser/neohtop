<script lang="ts">
  import { tick } from "svelte";
  import type { Process } from "$lib/types";
  import { formatBytes, formatDate } from "$lib/utils";
  import ProcessIcon from "../process/ProcessIcon.svelte";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import { Progress } from "$lib/components/ui/progress/index.js";
  import * as Sheet from "$lib/components/ui/sheet/index.js";
  import * as Card from "$lib/components/ui/card/index.js";
  import * as Table from "$lib/components/ui/table/index.js";
  import * as Collapsible from "$lib/components/ui/collapsible/index.js";
  import {
    Cpu,
    MemoryStick,
    Terminal,
    GitBranch,
    ChevronDown,
    ArrowUpRight,
  } from "@lucide/svelte";
  export let show = false;
  export let process: Process | null = null;
  export let onClose: () => void;
  export let processes: Process[] = [];
  export let onShowDetails: (process: Process) => void;
  $: children = process ? processes.filter((p) => p.ppid === process.pid) : [];
  $: parent = process
    ? processes.find((p) => p.pid === process.ppid)
    : undefined;
  let detail: HTMLDivElement | null = null;
  let inspectedIdentity = "";
  $: if (process && show) {
    const identity = `${process.pid}:${process.start_time}`;
    if (identity !== inspectedIdentity) {
      inspectedIdentity = identity;
      void tick().then(() => {
        detail?.scrollTo({ top: 0 });
        detail?.querySelector<HTMLElement>("[data-slot=sheet-title]")?.focus();
      });
    }
  }
</script>

<Sheet.Root
  open={show}
  onOpenChange={(open) => {
    if (!open) onClose();
  }}
  ><Sheet.Content
    bind:ref={detail}
    class="gap-6 overflow-y-auto p-6 *:shrink-0 data-[side=right]:w-full data-[side=right]:sm:max-w-3xl"
    ><Sheet.Header class="p-0"
      ><div class="flex items-start gap-3 pr-8">
        <div
          class="flex size-12 shrink-0 items-center justify-center rounded-xl border border-border bg-muted"
        >
          <ProcessIcon
            pid={process?.pid}
            startTime={process?.start_time}
            size={28}
          />
        </div>
        <div class="min-w-0">
          <Sheet.Title tabindex={-1} class="break-all text-xl outline-none"
            >{process?.name || "Process no longer available"}</Sheet.Title
          ><Sheet.Description
            >{process
              ? `PID ${process.pid} · ${process.status}`
              : "This process is no longer in the latest snapshot. Close this panel or choose another process."}</Sheet.Description
          >
        </div>
      </div></Sheet.Header
    >{#if process}<div class="grid grid-cols-2 gap-4">
        <Card.Root size="sm" class="gap-2"
          ><Card.Header
            ><Card.Description class="flex items-center gap-2"
              ><Cpu class="size-4" />CPU usage</Card.Description
            ></Card.Header
          ><Card.Content class="flex-1 justify-between gap-3"
            ><p class="text-2xl font-semibold tabular-nums">
              {process.cpu_usage.toFixed(1)}%
            </p>
            <Progress
              value={Math.min(100, Math.max(0, process.cpu_usage))}
              aria-label="Process CPU usage"
              style={`--primary:var(--${process.cpu_usage > 80 ? "red" : process.cpu_usage > 50 ? "yellow" : "blue"})`}
            /></Card.Content
          ></Card.Root
        ><Card.Root size="sm" class="gap-2"
          ><Card.Header
            ><Card.Description class="flex items-center gap-2"
              ><MemoryStick class="size-4" />Resident memory</Card.Description
            ></Card.Header
          ><Card.Content class="flex-1 justify-between gap-3"
            ><p class="text-2xl font-semibold tabular-nums">
              {formatBytes(process.memory_usage)}
            </p>
            <p class="text-xs text-muted-foreground">
              Virtual: {formatBytes(process.virtual_memory)}
            </p></Card.Content
          ></Card.Root
        >
      </div>
      <Card.Root size="sm"
        ><Card.Header
          ><Card.Title class="text-sm">Process information</Card.Title
          ></Card.Header
        ><Card.Content class="grid grid-cols-2 gap-x-6 gap-y-4 text-sm"
          >{#each [["Name", process.name], ["User", process.user], ["Session ID", process.session_id ?? "Unavailable"], ["Threads", process.threads ?? "Unavailable"], ["Started", formatDate(process.start_time)], ["Run time", `${Math.floor(process.run_time / 3600)}h ${Math.floor((process.run_time % 3600) / 60)}m ${process.run_time % 60}s`]] as [label, value]}<div
              class="min-w-0"
            >
              <p class="text-xs text-muted-foreground">{label}</p>
              <p class="mt-1 break-all" data-selectable>{value}</p>
            </div>{/each}
          <div>
            <p class="text-xs text-muted-foreground">Parent process</p>
            {#if parent}<Button
                variant="link"
                size="sm"
                class="mt-1 h-auto whitespace-normal p-0 text-left text-xs"
                onclick={() => parent && onShowDetails(parent)}
                >{parent.name} · {process.ppid}<ArrowUpRight /></Button
              >{:else}<p class="mt-1 text-xs" data-selectable>
                {process.ppid} · not in this snapshot
              </p>{/if}
          </div>
          <div>
            <p class="text-xs text-muted-foreground">Disk I/O</p>
            <p class="mt-1 text-xs" data-selectable>
              Read {formatBytes(process.disk_usage[0])} · Written {formatBytes(
                process.disk_usage[1],
              )}
            </p>
          </div></Card.Content
        ></Card.Root
      ><Card.Root size="sm"
        ><Card.Header
          ><Card.Title class="flex items-center gap-2 text-sm"
            ><Terminal class="size-4" />Command & root</Card.Title
          ></Card.Header
        ><Card.Content class="gap-4"
          ><div>
            <p class="mb-2 text-xs text-muted-foreground">Command</p>
            <pre
              class="max-h-44 overflow-auto whitespace-pre-wrap break-all rounded-md border border-border bg-background p-3 font-mono text-xs"
              data-selectable>{process.command || "Unavailable"}</pre>
          </div>
          <div>
            <p class="text-xs text-muted-foreground">Root</p>
            <p class="mt-1 break-all font-mono text-xs" data-selectable>
              {process.root || "Unavailable"}
            </p>
          </div></Card.Content
        ></Card.Root
      ><Card.Root size="sm"
        ><Card.Header
          ><Card.Title class="flex items-center gap-2 text-sm"
            ><GitBranch class="size-4" />Child processes<Badge
              variant="secondary">{children.length}</Badge
            ></Card.Title
          ></Card.Header
        ><Card.Content
          >{#if children.length}<Table.Root
              ><Table.Header
                ><Table.Row
                  ><Table.Head>Name</Table.Head><Table.Head>PID</Table.Head
                  ><Table.Head class="text-right">CPU</Table.Head><Table.Head
                    class="text-right">Memory</Table.Head
                  ></Table.Row
                ></Table.Header
              ><Table.Body
                >{#each children as child}<Table.Row
                    ><Table.Cell
                      ><Button
                        variant="link"
                        size="sm"
                        class="h-auto max-w-64 justify-start whitespace-normal p-0 text-left text-xs"
                        onclick={() => onShowDetails(child)}
                        >{child.name}<ArrowUpRight class="size-3" /></Button
                      ></Table.Cell
                    ><Table.Cell class="font-mono text-xs"
                      >{child.pid}</Table.Cell
                    ><Table.Cell class="text-right font-mono text-xs"
                      >{child.cpu_usage.toFixed(1)}%</Table.Cell
                    ><Table.Cell class="text-right font-mono text-xs"
                      >{formatBytes(child.memory_usage)}</Table.Cell
                    ></Table.Row
                  >{/each}</Table.Body
              ></Table.Root
            >{:else}<p class="text-sm text-muted-foreground">
              No child process in the current snapshot.
            </p>{/if}</Card.Content
        ></Card.Root
      ><Collapsible.Root
        ><Collapsible.Trigger
          class="flex w-full items-center justify-between gap-3 rounded-md border border-border bg-card p-4 text-sm font-medium focus-visible:outline-2 focus-visible:outline-ring"
          >Environment variables ({process.environ.length})<ChevronDown
            class="size-4"
          /></Collapsible.Trigger
        ><Collapsible.Content class="pt-3"
          ><p class="mb-3 text-xs text-muted-foreground">
            Values reported by the operating system for this process.
          </p>
          <div
            class="max-h-72 overflow-auto rounded-md border border-border bg-card p-4"
            data-selectable
          >
            {#each process.environ as env}<p
                class="mb-2 whitespace-pre-wrap break-all font-mono text-xs last:mb-0"
              >
                {env}
              </p>{:else}<p class="text-sm text-muted-foreground">
                No accessible environment variables reported.
              </p>{/each}
          </div></Collapsible.Content
        ></Collapsible.Root
      >
      <p class="text-xs text-muted-foreground">
        Readings follow the process snapshot. Virtual address space is not
        physical RAM; CPU can exceed 100% across cores.
      </p>{/if}</Sheet.Content
  ></Sheet.Root
>
