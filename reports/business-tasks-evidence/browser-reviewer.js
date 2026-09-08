// Independent CLI browser context, same authoritative synthetic server DB.
;async (page) => {
  const origin = "http://127.0.0.1:4342"
  await page.route("**/*", (route) =>
    route.request().url().startsWith(`${origin}/`)
      ? route.continue()
      : route.abort()
  )
  await page.goto(`${origin}/__business_task_fixture`)
  return page.evaluate(async () => {
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
          authorization: `Bearer ${token}`,
        },
        body: JSON.stringify({ input }),
      })
      return { status: r.status, body: await r.json() }
    }
    const context = await call("context", operator, {})
    const organizationId = context.body.organization.id
    const reviewer = await call("members/create", operator, {
      organizationId,
      displayName: "Independent human reviewer",
      kind: "human",
      role: "manager",
      domains: ["feedback"],
    })
    const issued = await call("credentials/issue", operator, {
      organizationId,
      memberId: reviewer.body.id,
      label: "Synthetic second browser",
    })
    const token = issued.body.token
    const list = await call("tasks/list", token, {
      status: "review",
      query: "Synthetic customer follow-up",
    })
    check(
      "Independent reviewer sees actual shared review",
      list.status === 200 && list.body.tasks.length === 1
    )
    const task = list.body.tasks[0]
    const before = await call("tasks/get", token, { taskId: task.id })
    check(
      "Reviewer differs from creator",
      task.creatorId !== reviewer.body.id && task.reviewerId === null
    )
    check(
      "Full current deliverable is readable",
      before.body.deliverables[0].body ===
        "Exact human deliverable\nContact next week; no message sent."
    )
    check(
      "Current reviewer capability authorizes acceptance",
      task.capabilities.review
    )
    const stale = await call("tasks/review", token, {
      taskId: task.id,
      expectedRevision: task.revision - 1,
      decision: "accept",
    })
    check("Stale human acceptance is rejected", stale.status === 409)
    const accepted = await call("tasks/review", token, {
      taskId: task.id,
      expectedRevision: task.revision,
      decision: "accept",
      comment: "Reviewed this exact public deliverable",
    })
    check(
      "Permitted human completes task",
      accepted.status === 200 &&
        accepted.body.task.status === "done" &&
        accepted.body.task.revision === 5
    )
    check(
      "Review attribution is individual manager credential",
      accepted.body.activity[4].actor.id === reviewer.body.id &&
        accepted.body.activity[4].payload.reviewedRevision === 4
    )
    check(
      "Deadline and exact deliverable survive review",
      accepted.body.task.dueDate === "2028-02-29" &&
        accepted.body.deliverables[0].body === before.body.deliverables[0].body
    )
    check(
      "No agent/engineering execution was required",
      accepted.body.execution === null
    )
    check(
      "Duplicate acceptance cannot create another event",
      (
        await call("tasks/review", token, {
          taskId: task.id,
          expectedRevision: 4,
          decision: "accept",
        })
      ).status === 409
    )
    check(
      "Reviewer cannot entrust arbitrary engineering run",
      (
        await call("tasks/entrust-execution", token, {
          taskId: task.id,
          expectedRevision: 5,
          workTaskId: 1,
        })
      ).status === 403
    )
    check(
      "No credential browser persistence",
      localStorage.length === 0 && sessionStorage.length === 0
    )
    return {
      kind: "independent browser; real human review via protected API",
      taskId: task.id,
      reviewerId: reviewer.body.id,
      checks,
    }
  })
}
