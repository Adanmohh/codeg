"use client"

import { useCallback, useEffect, useRef, useState } from "react"
import { useLocale } from "next-intl"
import {
  ArrowRight,
  CircleCheck,
  LayoutGrid,
  List,
  ListTodo,
  LogOut,
  Menu,
  Plus,
  RefreshCw,
  Search,
  Users,
  X,
} from "lucide-react"
import { Input } from "@/components/ui/input"
import {
  Drawer,
  DrawerContent,
  DrawerDescription,
  DrawerTitle,
} from "@/components/ui/drawer"
import { cn } from "@/lib/utils"
import type { BusinessClient } from "@/lib/business/client"
import type { BusinessContext, Member } from "@/lib/business/identity"
import { useBusinessCopy } from "@/lib/business/copy"
import {
  BUSINESS_STATUSES,
  type BusinessDomain,
  type BusinessStatus,
} from "@/lib/business/presentation"
import {
  taskPreview,
  type ListInput,
  type TaskDetail,
  type TaskPage,
} from "@/lib/business/tasks"
import {
  Action,
  Brand,
  controlClass,
  ErrorNotice,
  Person,
  roleLabel,
} from "./ui"
import { BusinessPreferences } from "./preferences"
import { WorkList } from "./work-list"
import { People } from "./people"
import { CreateTask } from "./task-form"
import { TaskDetailDialog } from "./task-detail"

type View = "mine" | "shared" | "review" | "people"
export function BusinessWorkspace({
  client,
  context,
  onContext,
  disconnect,
}: {
  client: BusinessClient
  context: BusinessContext
  onContext: (context: BusinessContext) => void
  disconnect: () => void
}) {
  const copy = useBusinessCopy()
  const locale = useLocale()
  const actor = context.member!
  const organizationId = context.organization!.id
  const [view, setView] = useState<View>("mine")
  const [mode, setMode] = useState<"list" | "board">("list")
  const [domain, setDomain] = useState<BusinessDomain | "">("")
  const [status, setStatus] = useState<BusinessStatus | "">("")
  const [search, setSearch] = useState("")
  const [query, setQuery] = useState("")
  const [archived, setArchived] = useState(false)
  const [page, setPage] = useState<TaskPage | null>(null)
  const [members, setMembers] = useState<Member[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<unknown>(null)
  const [navOpen, setNavOpen] = useState(false)
  const [creating, setCreating] = useState(false)
  const [selected, setSelected] = useState<{
    id: string
    initial?: TaskDetail
  } | null>(null)
  const [refreshKey, setRefreshKey] = useState(0)
  const generation = useRef(0)
  const menuRef = useRef<HTMLButtonElement>(null)
  const contextCallback = useRef(onContext)
  useEffect(() => {
    contextCallback.current = onContext
  }, [onContext])
  useEffect(() => {
    const wide = window.matchMedia("(min-width: 1024px)")
    const resize = () => {
      if (wide.matches) setNavOpen(false)
    }
    wide.addEventListener("change", resize)
    return () => wide.removeEventListener("change", resize)
  }, [])
  // Revalidate membership on focus and periodically without legacy WebSockets.
  // A changed member revision remounts the private workspace at the session boundary.
  useEffect(() => {
    let cancelled = false
    const check = () => {
      if (document.visibilityState === "hidden") return
      void client
        .identity("context", {})
        .then((value) => {
          if (!cancelled) contextCallback.current(value)
        })
        .catch((caught) => {
          if (!cancelled) setError(caught)
        })
    }
    window.addEventListener("focus", check)
    const timer = window.setInterval(check, 30000)
    return () => {
      cancelled = true
      window.removeEventListener("focus", check)
      window.clearInterval(timer)
    }
  }, [client])
  useEffect(() => {
    const version = ++generation.current
    setLoading(true)
    setError(null)
    setPage(null)
    const input: ListInput = {
      view: view === "mine" ? "mine" : "shared",
      ...(domain && { domain }),
      ...(view === "review"
        ? { status: "review" as const }
        : status
          ? { status }
          : {}),
      ...(query && { query }),
      archived,
    }
    void Promise.all([
      view === "people" ? Promise.resolve(null) : client.tasks("list", input),
      client.identity("members/list", { organizationId }),
    ])
      .then(([tasks, directory]) => {
        if (generation.current !== version) return
        setPage(tasks)
        setMembers(directory)
      })
      .catch((caught) => {
        if (generation.current === version) setError(caught)
      })
      .finally(() => {
        if (generation.current === version) setLoading(false)
      })
    return () => {
      generation.current = version + 1
    }
  }, [
    client,
    organizationId,
    view,
    domain,
    status,
    query,
    archived,
    refreshKey,
  ])
  const reload = useCallback(async () => {
    try {
      contextCallback.current(await client.identity("context", {}))
      setRefreshKey((value) => value + 1)
    } catch (caught) {
      setError(caught)
    }
  }, [client])
  async function more() {
    if (!page || loading) return
    const version = generation.current
    setLoading(true)
    try {
      const next = await client.tasks("list", {
        view: view === "mine" ? "mine" : "shared",
        ...(domain && { domain }),
        ...(view === "review"
          ? { status: "review" as const }
          : status
            ? { status }
            : {}),
        ...(query && { query }),
        archived,
        page: page.page + 1,
      })
      if (version !== generation.current) return
      setPage({
        ...next,
        tasks: [
          ...page.tasks,
          ...next.tasks.filter(
            (item) => !page.tasks.some((old) => old.id === item.id)
          ),
        ],
      })
    } catch (caught) {
      if (version === generation.current) setError(caught)
    } finally {
      if (version === generation.current) setLoading(false)
    }
  }
  const navItems = [
    { id: "mine", title: copy.myWork, icon: ListTodo },
    { id: "shared", title: copy.sharedWork, icon: Users },
    { id: "review", title: copy.review, icon: CircleCheck },
    { id: "people", title: copy.team, icon: Users },
  ] as const
  const title = navItems.find((item) => item.id === view)!.title
  const hint =
    view === "mine"
      ? copy.myWorkHint
      : view === "review"
        ? copy.reviewHint
        : copy.sharedWorkHint
  function navigate(next: View) {
    setView(next)
    setStatus("")
    setArchived(false)
    setNavOpen(false)
  }
  const navigation = (
    <>
      <div className="flex items-center justify-between gap-3">
        <Brand />
        <Action
          variant="ghost"
          size="icon"
          className="size-11 p-0 lg:hidden"
          onClick={() => setNavOpen(false)}
          aria-label={copy.close}
        >
          <X aria-hidden="true" />
        </Action>
      </div>
      <div className="mt-9 mb-3 min-w-0 px-2">
        <p className="text-muted-foreground text-xs">{copy.workspace}</p>
        <p className="mt-1 truncate text-sm font-semibold">
          <bdi>{context.organization!.name}</bdi>
        </p>
      </div>
      <nav className="space-y-1" aria-label={copy.workspace}>
        {navItems.map(({ id, title: label, icon: Icon }) => (
          <button
            type="button"
            key={id}
            onClick={() => navigate(id)}
            aria-current={view === id ? "page" : undefined}
            className={cn(
              "focus-visible:ring-ring flex min-h-12 w-full items-center gap-3 rounded-xl px-3 py-3 text-start text-sm font-medium outline-none focus-visible:ring-2",
              view === id
                ? "bg-primary/10 text-primary"
                : "text-sidebar-foreground hover:bg-sidebar-accent"
            )}
          >
            <Icon className="size-4 shrink-0" aria-hidden="true" />
            {label}
          </button>
        ))}
      </nav>
      <div className="mt-auto space-y-5 pt-10">
        {context.capabilities.legacyOperator && (
          <a
            href={client.native ? "/workspace" : `${client.label}/workspace`}
            className="focus-visible:ring-ring text-muted-foreground hover:text-foreground flex min-h-11 items-center gap-2 rounded-lg px-2 text-sm outline-none focus-visible:ring-2"
          >
            {copy.openEngineering}
            <ArrowRight
              className="ms-auto size-4 shrink-0 rtl:rotate-180"
              aria-hidden="true"
            />
          </a>
        )}
        <BusinessPreferences />
        <div className="border-border border-t pt-4">
          <Person
            person={{ name: actor.displayName, kind: actor.kind }}
            fallback=""
          />
          <p className="text-muted-foreground mt-2 text-xs">
            {roleLabel(actor.role, copy)} ·{" "}
            {client.native ? copy.localLabel : copy.connected}
          </p>
          <Action
            variant="ghost"
            className="mt-2 w-full justify-start px-0"
            onClick={disconnect}
          >
            <LogOut aria-hidden="true" />
            {copy.disconnect}
          </Action>
        </div>
      </div>
    </>
  )
  return (
    <div className="bg-background flex h-dvh min-w-0 overflow-hidden">
      <aside className="bg-sidebar border-border hidden w-[248px] shrink-0 flex-col overflow-y-auto border-e p-5 lg:flex">
        {navigation}
      </aside>
      <div className="flex min-w-0 flex-1 flex-col">
        <header className="bg-background border-border flex shrink-0 items-center justify-between gap-3 border-b px-4 py-3 lg:hidden">
          <Action
            ref={menuRef}
            variant="ghost"
            size="icon"
            className="size-11 p-0"
            onClick={() => setNavOpen(true)}
            aria-label={copy.menu}
          >
            <Menu aria-hidden="true" />
          </Action>
          <span className="min-w-0 truncate text-sm font-semibold">
            <bdi>{context.organization!.name}</bdi>
          </span>
          <Brand compact />
        </header>
        <main
          id="business-main"
          tabIndex={-1}
          className="focus-visible:ring-ring min-w-0 flex-1 overflow-x-hidden overflow-y-auto px-5 py-7 outline-none focus-visible:ring-2 focus-visible:ring-inset sm:px-8 lg:px-10 lg:py-10"
        >
          <div className="mx-auto max-w-6xl">
            {view === "people" ? (
              <People
                client={client}
                context={context}
                members={members}
                reload={reload}
              />
            ) : (
              <>
                <header className="mb-8 flex flex-wrap items-end justify-between gap-5">
                  <div className="min-w-0">
                    <p className="text-primary mb-3 text-sm font-semibold">
                      {copy.workspace}
                    </p>
                    <h1 className="text-3xl font-semibold tracking-tight sm:text-4xl">
                      {title}
                    </h1>
                    <p className="text-muted-foreground mt-3 max-w-xl text-sm leading-relaxed">
                      {hint}
                    </p>
                  </div>
                  {page?.canCreate && (
                    <Action onClick={() => setCreating(true)}>
                      <Plus aria-hidden="true" />
                      {copy.createTask}
                    </Action>
                  )}
                </header>
                {actor.role === "viewer" && (
                  <p className="bg-muted/40 mb-6 rounded-xl border p-4 text-sm">
                    {copy.readOnlyHint}
                  </p>
                )}
                <div className="mb-5 space-y-3">
                  <form
                    className="flex gap-2"
                    onSubmit={(event) => {
                      event.preventDefault()
                      setQuery(search.trim())
                    }}
                  >
                    <div className="relative min-w-0 flex-1">
                      <Search
                        className="text-muted-foreground pointer-events-none absolute start-3 top-3.5 size-4"
                        aria-hidden="true"
                      />
                      <Input
                        className="min-h-11 rounded-xl ps-10"
                        value={search}
                        onChange={(event) => setSearch(event.target.value)}
                        aria-label={copy.search}
                        placeholder={copy.search}
                        maxLength={240}
                      />
                      <button type="submit" className="sr-only">
                        {copy.search}
                      </button>
                    </div>
                    <Action
                      variant="outline"
                      onClick={() => void reload()}
                      aria-label={copy.refresh}
                      disabled={loading}
                      size="icon"
                      className="size-11 p-0"
                    >
                      <RefreshCw aria-hidden="true" />
                    </Action>
                  </form>
                  <div className="flex flex-wrap items-center gap-2">
                    <select
                      className={
                        controlClass +
                        " !w-auto max-w-full flex-1 basis-40 sm:flex-none sm:basis-auto"
                      }
                      aria-label={copy.domain}
                      value={domain}
                      onChange={(event) =>
                        setDomain(event.target.value as BusinessDomain | "")
                      }
                    >
                      <option value="">{copy.allDomains}</option>
                      {actor.domains.map((item) => (
                        <option key={item} value={item}>
                          {copy[item]}
                        </option>
                      ))}
                    </select>
                    {view !== "review" && (
                      <select
                        className={
                          controlClass +
                          " !w-auto max-w-full flex-1 basis-40 sm:flex-none sm:basis-auto"
                        }
                        aria-label={copy.status}
                        value={status}
                        onChange={(event) =>
                          setStatus(event.target.value as BusinessStatus | "")
                        }
                      >
                        <option value="">{copy.allStatuses}</option>
                        {BUSINESS_STATUSES.map((item) => (
                          <option key={item} value={item}>
                            {copy[item]}
                          </option>
                        ))}
                      </select>
                    )}
                    <label className="text-muted-foreground flex min-h-11 items-center gap-2 px-2 text-xs">
                      <input
                        type="checkbox"
                        checked={archived}
                        onChange={(event) => setArchived(event.target.checked)}
                        className="accent-primary size-4"
                      />
                      {copy.archived}
                    </label>
                    <div
                      className="bg-muted/50 ms-auto flex shrink-0 gap-1 rounded-xl p-1"
                      aria-label={copy.workspace}
                    >
                      <Action
                        variant={mode === "list" ? "outline" : "ghost"}
                        className="px-3"
                        onClick={() => setMode("list")}
                        aria-pressed={mode === "list"}
                      >
                        <List aria-hidden="true" />
                        {copy.list}
                      </Action>
                      <Action
                        variant={mode === "board" ? "outline" : "ghost"}
                        className="px-3"
                        onClick={() => setMode("board")}
                        aria-pressed={mode === "board"}
                      >
                        <LayoutGrid aria-hidden="true" />
                        {copy.board}
                      </Action>
                    </div>
                  </div>
                </div>
                {page && (
                  <p className="text-muted-foreground mb-3 text-xs tabular-nums">
                    {page.tasks.length} {copy.loadedTasks}
                  </p>
                )}
                {page && page.tasks.length > 0 && (
                  <WorkList
                    tasks={page.tasks.map((task) =>
                      taskPreview(task, members, copy.memberUnavailable)
                    )}
                    mode={mode}
                    onOpen={(id) => setSelected({ id })}
                  />
                )}
                {page && page.tasks.length === 0 && (
                  <section className="border-border bg-card rounded-2xl border px-6 py-12 sm:p-12">
                    <div className="bg-primary/10 text-primary mb-5 flex size-12 items-center justify-center rounded-2xl">
                      <ListTodo className="size-6" aria-hidden="true" />
                    </div>
                    <h2 className="text-xl font-semibold tracking-tight">
                      {query || domain || status || archived
                        ? copy.noResults
                        : view === "mine"
                          ? copy.emptyMyTitle
                          : view === "review"
                            ? copy.emptyReviewTitle
                            : copy.emptyTitle}
                    </h2>
                    <p className="text-muted-foreground mt-3 max-w-md text-sm leading-relaxed">
                      {query || domain || status || archived
                        ? copy.noResultsHint
                        : view === "mine"
                          ? copy.emptyMyHint
                          : view === "review"
                            ? copy.emptyReviewHint
                            : copy.emptyHint}
                    </p>
                    <div className="mt-6 flex flex-wrap gap-3">
                      {query || domain || status || archived ? (
                        <Action
                          variant="outline"
                          onClick={() => {
                            setQuery("")
                            setSearch("")
                            setDomain("")
                            setStatus("")
                            setArchived(false)
                          }}
                        >
                          {copy.clearFilters}
                        </Action>
                      ) : view === "mine" ? (
                        <Action
                          variant="outline"
                          onClick={() => navigate("shared")}
                        >
                          {copy.sharedWork}
                          <ArrowRight
                            className="rtl:rotate-180"
                            aria-hidden="true"
                          />
                        </Action>
                      ) : page.canCreate && view === "shared" ? (
                        <Action onClick={() => setCreating(true)}>
                          <Plus aria-hidden="true" />
                          {copy.createTask}
                        </Action>
                      ) : null}
                    </div>
                  </section>
                )}
                {page?.hasMore && (
                  <div className="mt-5 flex justify-center">
                    <Action
                      variant="outline"
                      disabled={loading}
                      onClick={() => void more()}
                    >
                      {copy.more}
                    </Action>
                  </div>
                )}
              </>
            )}
            {loading && (
              <p role="status" className="text-muted-foreground py-6 text-sm">
                {copy.loading}
              </p>
            )}
            {error != null && (
              <div className="mt-5">
                <ErrorNotice error={error}>
                  <Action
                    variant="outline"
                    disabled={loading}
                    onClick={() => void reload()}
                  >
                    {copy.retry}
                  </Action>
                </ErrorNotice>
              </div>
            )}
          </div>
        </main>
      </div>
      <Drawer
        open={navOpen}
        onOpenChange={setNavOpen}
        modal
        disablePointerDismissal={false}
        swipeDirection={locale === "ar" ? "right" : "left"}
      >
        <DrawerContent
          showCloseButton={false}
          finalFocus={menuRef}
          className="flex w-[min(320px,calc(100%-1rem))] flex-col overflow-y-auto rounded-2xl bg-sidebar p-5"
        >
          <DrawerTitle className="sr-only">{copy.workspace}</DrawerTitle>
          <DrawerDescription className="sr-only">{copy.menu}</DrawerDescription>
          {navigation}
        </DrawerContent>
      </Drawer>
      {creating && (
        <CreateTask
          client={client}
          actor={actor}
          members={members}
          onClose={() => setCreating(false)}
          onCreated={(detail) => {
            setCreating(false)
            setSelected({ id: detail.task.id, initial: detail })
            void reload()
          }}
        />
      )}
      {selected && (
        <TaskDetailDialog
          key={selected.id}
          taskId={selected.id}
          initial={selected.initial}
          client={client}
          actor={actor}
          members={members}
          legacyOperator={context.capabilities.legacyOperator}
          onClose={() => setSelected(null)}
          onChanged={() => void reload()}
        />
      )}
    </div>
  )
}
