;async (page) =>
  page.evaluate(async () => {
    const { call, member, viewer, operator, organizationId, taskId } =
      window.taskFixture
    const checks = []
    function check(name, ok) {
      checks.push({ name, passed: Boolean(ok) })
      if (!ok) throw new Error(`Failed: ${name}`)
    }
    const current = await call("tasks/get", member.token, { taskId })
    check(
      "First browser observes second browser completion",
      current.status === 200 &&
        current.body.task.status === "done" &&
        current.body.task.revision === 5
    )
    check(
      "History has exactly five immutable events",
      current.body.activity.length === 5 &&
        current.body.deliverables.length === 1
    )
    check(
      "Calendar date is unchanged across contexts",
      current.body.task.dueDate === "2028-02-29"
    )
    check(
      "Terminal task rejects new contribution",
      (
        await call("tasks/note", member.token, {
          taskId,
          expectedRevision: 5,
          body: "Late write",
        })
      ).status === 403
    )
    check(
      "Human credential revocation succeeds",
      (
        await call("credentials/revoke", operator, {
          organizationId,
          credentialId: member.credential.id,
        })
      ).status === 200
    )
    check(
      "Revoked browser cannot read task",
      (await call("tasks/get", member.token, { taskId })).status === 401
    )
    const visible = await call("tasks/get", viewer.token, { taskId })
    check(
      "Independent viewer still reads exact shared result",
      visible.status === 200 &&
        visible.body.task.revision === 5 &&
        visible.body.activity.length === 5
    )
    check(
      "Rejections leave task history unchanged",
      JSON.stringify(visible.body.activity) ===
        JSON.stringify(current.body.activity)
    )
    check(
      "No local/session credentials persisted",
      localStorage.length === 0 && sessionStorage.length === 0
    )
    delete window.taskFixture
    return {
      kind: "cross-browser shared state and credential revocation",
      checks,
    }
  })
