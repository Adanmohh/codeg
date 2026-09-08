// Adapted protected fixture orchestration: Codeg identity browser-api.js at
// d9882b68b268b516b1d1a24031862a022a797899, Apache-2.0. No response mocking.
;async (page) => {
  const origin = "http://127.0.0.1:4342"
  const blocked = []
  await page.route("**/*", (route) => {
    if (route.request().url().startsWith(`${origin}/`)) return route.continue()
    blocked.push("off-origin")
    return route.abort()
  })
  await page.goto(`${origin}/__business_task_fixture`)
  const result = await page.evaluate(async () => {
    const operator = "business-tasks-synthetic-operator"
    const checks = []
    function check(name, ok) {
      checks.push({ name, passed: Boolean(ok) })
      if (!ok) throw new Error(`Failed: ${name}`)
    }
    async function call(path, token, input) {
      const r = await fetch(`/api/business/${path}`, {
        method: "POST",
        credentials: "omit",
        redirect: "error",
        headers: {
          "content-type": "application/json",
          ...(token ? { authorization: `Bearer ${token}` } : {}),
        },
        body: JSON.stringify({ input }),
      })
      return {
        status: r.status,
        body: await r.json(),
        cache: r.headers.get("cache-control"),
      }
    }
    check(
      "Anonymous tasks denied",
      (await call("tasks/list", null, {})).status === 401
    )
    const boot = await call("bootstrap", operator, {
      organizationName: "Synthetic shared task team",
      ownerName: "Synthetic operator",
    })
    check(
      "Actual operator bootstraps",
      boot.status === 200 && boot.body.operator
    )
    const organizationId = boot.body.organization.id
    async function human(name, role, domains) {
      const member = await call("members/create", operator, {
        organizationId,
        displayName: name,
        kind: "human",
        role,
        domains,
      })
      check(`Creates ${name}`, member.status === 200)
      const credential = await call("credentials/issue", operator, {
        organizationId,
        memberId: member.body.id,
        label: "Synthetic browser only",
      })
      check(`Issues ${name}`, credential.status === 200)
      return {
        member: member.body,
        credential: credential.body.credential,
        token: credential.body.token,
      }
    }
    const member = await human("Browser task author", "member", ["feedback"])
    const viewer = await human("Browser task viewer", "viewer", ["feedback"])
    const outside = await human("Browser marketing member", "member", [
      "marketing",
    ])
    const created = await call("tasks/create", member.token, {
      title: "Synthetic customer follow-up",
      domain: "feedback",
      notes: "Public relationship context only",
      dueDate: "2028-02-29",
    })
    check(
      "Human task needs no executor",
      created.status === 200 &&
        created.body.execution === null &&
        created.body.task.assigneeId === null
    )
    check(
      "Creator and default owner derive from credential",
      created.body.task.ownerId === member.member.id &&
        created.body.task.creatorId === member.member.id &&
        created.body.activity[0].actor.id === member.member.id
    )
    check(
      "Calendar deadline roundtrips unchanged",
      created.body.task.dueDate === "2028-02-29"
    )
    check(
      "Default reviewer remains explicit null",
      created.body.task.reviewerId === null &&
        created.body.task.status === "todo" &&
        created.body.task.revision === 1
    )
    check("Task response is no-store", created.cache === "no-store")
    const taskId = created.body.task.id
    const get = (token) => call("tasks/get", token, { taskId })
    check("Viewer reads shared task", (await get(viewer.token)).status === 200)
    check(
      "Different domain cannot read task",
      (await get(outside.token)).status === 404
    )
    check(
      "Viewer cannot add work",
      (
        await call("tasks/note", viewer.token, {
          taskId,
          expectedRevision: 1,
          body: "Denied",
        })
      ).status === 403
    )
    check(
      "Caller actor cannot be supplied",
      (
        await call("tasks/create", member.token, {
          title: "Forged",
          domain: "feedback",
          actor: boot.body.member.id,
        })
      ).status === 400
    )
    check(
      "Caller organization cannot be supplied",
      (
        await call("tasks/get", member.token, {
          taskId,
          organizationId: crypto.randomUUID(),
        })
      ).status === 400
    )
    for (const dueDate of [
      "2027-02-29",
      "2028-02-29T00:00:00Z",
      "0000-01-01",
    ]) {
      check(
        `Rejects non-calendar deadline ${dueDate}`,
        (
          await call("tasks/create", member.token, {
            title: "Bad date",
            domain: "feedback",
            dueDate,
          })
        ).status === 400
      )
    }
    const owned = await call("tasks/list", member.token, {
      view: "mine",
      query: "Synthetic customer",
    })
    check(
      "Mine filter returns actual human ownership",
      owned.status === 200 &&
        owned.body.tasks.length === 1 &&
        owned.body.tasks[0].id === taskId
    )
    check(
      "Literal filter does not expand SQL wildcard",
      (await call("tasks/list", member.token, { query: "%" })).body.tasks
        .length === 0
    )
    const progress = await call("tasks/progress", member.token, {
      taskId,
      expectedRevision: 1,
      status: "in_progress",
    })
    check(
      "Human-only work progresses",
      progress.status === 200 && progress.body.task.status === "in_progress"
    )
    const notes = await Promise.all(
      ["First update", "Competing update"].map((body) =>
        call("tasks/note", member.token, { taskId, expectedRevision: 2, body })
      )
    )
    check(
      "Concurrent HTTP writes have one winner and one409",
      notes.filter((r) => r.status === 200).length === 1 &&
        notes.filter((r) => r.status === 409).length === 1
    )
    check(
      "Conflict has recovery key",
      notes.find((r) => r.status === 409).body.i18n_key ===
        "business.revisionConflict"
    )
    const after = await get(member.token)
    check(
      "Race commits one revision and one activity",
      after.body.task.revision === 3 && after.body.activity.length === 3
    )
    const body = "Exact human deliverable\nContact next week; no message sent."
    const submitted = await call("tasks/submit", member.token, {
      taskId,
      expectedRevision: 3,
      body,
    })
    check(
      "Human submits without agent or folder",
      submitted.status === 200 &&
        submitted.body.task.status === "review" &&
        submitted.body.execution === null
    )
    check(
      "Review retains exact payload and author",
      submitted.body.deliverables[0].body === body &&
        submitted.body.deliverables[0].author.id === member.member.id
    )
    check(
      "Member cannot bypass review with done",
      (
        await call("tasks/progress", member.token, {
          taskId,
          expectedRevision: 4,
          status: "done",
        })
      ).status === 400
    )
    check(
      "Member cannot accept own review without Review grant",
      (
        await call("tasks/review", member.token, {
          taskId,
          expectedRevision: 4,
          decision: "accept",
        })
      ).status === 403
    )
    check(
      "Member cannot entrust a legacy run",
      (
        await call("tasks/entrust-execution", member.token, {
          taskId,
          expectedRevision: 4,
          workTaskId: 1,
        })
      ).status === 403
    )
    check(
      "No proof or private lineage in detail",
      !/delegation_json|authority_id|credential_id|connection_id|token_hash/.test(
        JSON.stringify(submitted.body)
      )
    )
    check(
      "Browser stores no credentials",
      localStorage.length === 0 && sessionStorage.length === 0
    )
    // Only this synthetic in-memory page retains the token for the final read.
    // It is never emitted, saved to storage or included in a report/screenshot.
    window.taskFixture = {
      call,
      member,
      viewer,
      operator,
      organizationId,
      taskId,
    }
    return {
      organizationId,
      taskId,
      memberId: member.member.id,
      status: "review",
      revision: 4,
      checks,
    }
  })
  return {
    kind: "actual Chromium fetch against protected Rust API; test landing only",
    mockedResponses: false,
    blockedOrigins: blocked,
    ...result,
  }
}
