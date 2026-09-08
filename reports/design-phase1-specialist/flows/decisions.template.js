async (page) => {
  if (!page.url().startsWith("http://127.0.0.1:4327/")) throw Error("Owned fixture only")
  const capture = __CAPTURE__
  const scope = page.getByRole("region", { name: "Ops desk", exact: true })
  const cases = [], checks = []
  const stats = () => page.evaluate(async () => (await fetch("/api/ops_design_fixture_stats", { headers: { Authorization: "Bearer ops-design-synthetic-operator" } })).json())
  const before = await stats()
  const shot = async (name) => { await page.screenshot({ path: `reports/design-phase1-specialist/screenshots/${name}.png` }); cases.push({ name, raw: await capture(page) }) }
  for (const [id, phrase] of [[2,"it does not send again"],[3,"Provider acceptance does not confirm recipient delivery"],[4,"Reconcile the provider outcome first"],[5,"The provider did not accept this reply"],[6,"Its private reply content has been redacted"]]) {
    await page.setViewportSize({ width:1280,height:900 })
    await scope.getByRole("button",{name:new RegExp(`Reply review #${id} Task`)}).click()
    await scope.locator("article h1").waitFor()
    const copy = await scope.locator("article").innerText()
    if (!copy.includes(phrase) || copy.includes("No delivery receipt is implied")) throw Error(`Terminal copy mismatch ${id}`)
    if (await scope.getByRole("button",{name:/Approve and send|Deny proposal/}).count()) throw Error("Terminal dispatch available")
    await page.setViewportSize({ width:390,height:844 })
    await page.keyboard.press("Escape")
    await scope.locator("article h1").scrollIntoViewIfNeeded()
    await shot(`terminal-${id}-390-dark`)
    checks.push({ id, copy, offersAnotherSend:false })
  }
  await page.setViewportSize({ width:1280,height:900 })
  await scope.getByRole("button",{name:/Reply review #2 Task/}).click()
  await scope.getByRole("button",{name:"Finish recording receipt",exact:true}).click()
  await scope.getByRole("heading",{name:"Sent · provider accepted",exact:true}).waitFor()
  await shot("receipt-finished-1280-dark")
  if (await scope.getByRole("button",{name:"Finish recording receipt",exact:true}).count()) throw Error("Recording action remains")
  const after = await stats()
  if (after.providerRequests !== before.providerRequests) throw Error("Recording resent")
  checks.push({ BC14:true, providerBefore:before.providerRequests, providerAfter:after.providerRequests })
  return { syntheticOnly:true, checks, cases }
}
