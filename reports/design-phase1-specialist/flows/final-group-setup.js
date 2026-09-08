// Browser-only response fixture for the accepted workspace sidebar group path.
// The server's three read-only list RPCs and all stored rows remain unchanged.
async (page) => {
  if (!page.url().startsWith("http://127.0.0.1:4327/")) throw Error("Fixture only")
  const counts = { list_folder_groups: 0, list_open_folder_details: 0, list_all_folder_details: 0 }
  for (const command of Object.keys(counts)) {
    await page.route(`http://127.0.0.1:4327/api/${command}`, async (route) => {
      const response = await route.fetch({ maxRedirects: 0, maxRetries: 0 })
      if (!response.ok()) throw Error("Read fixture unavailable")
      const original = await response.json()
      if (!Array.isArray(original)) throw Error("Unexpected list DTO")
      counts[command]++
      if (command === "list_folder_groups") {
        if (original.length !== 0) throw Error("Preserve existing groups")
        await route.fulfill({ response, json: [{ id: 9876, name: "Synthetic badge group", color: "inherit", sort_order: 1 }] })
      } else {
        const target = original.find((folder) => folder.id === 1)
        if (!target || target.group_id !== null) throw Error("Unexpected bound folder")
        await route.fulfill({ response, json: original.map((folder) => folder.id === 1 ? { ...folder, group_id: 9876 } : folder) })
      }
    })
  }
  await page.reload()
  const group = page.getByRole("button", { name: "Synthetic badge group 6 sessions running", exact: true })
  await group.waitFor()
  return { syntheticBrowserResponsesOnly: true, url: page.url(), counts, aria: await group.ariaSnapshot() }
}
