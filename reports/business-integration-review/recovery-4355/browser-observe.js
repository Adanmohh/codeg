async page => {
  page.__reviewDtos = []
  page.__reviewDtoPending = []
  page.__reviewPhase = 'open-unprepared-withheld'
  page.on('response', response => {
    const path = response.url().split('http://127.0.0.1:4355')[1]?.split('?')[0]
    if (![
      '/api/business/intake/candidates/get',
      '/api/business/intake/sources/get',
      '/api/business/intake/imports/start',
      '/api/business/intake/imports/advance',
    ].includes(path)) return
    const phase = page.__reviewPhase
    const pending = (async () => {
      if (response.status() !== 200) {
        page.__reviewDtos.push({ phase, path, status: response.status() })
        return
      }
      const body = await response.json()
      const entry = { phase, path, status: 200 }
      if (path.endsWith('/candidates/get')) {
        const value = body.candidate
        Object.assign(entry, {
          id: value.id, revision: value.revision,
          sourceRevision: value.sourceRevision, disclosure: value.disclosure,
          hasPreparedDraft: value.hasPreparedDraft, draftNull: value.draft === null,
          requiresRebase: value.requiresRebase,
          newerNotesMatch: value.draft?.notes === 'Reviewer4355 newer exact private saved text',
        })
      } else if (path.endsWith('/sources/get')) {
        Object.assign(entry, {
          id: body.source.id, revision: body.source.revision,
          disclosure: body.disclosure, accessValidUntil: body.source.accessValidUntil,
          passageCount: body.passages.length,
        })
      } else {
        Object.assign(entry, { id: body.id, state: body.state, revision: body.revision })
      }
      page.__reviewDtos.push(entry)
    })().catch(() => page.__reviewDtos.push({ phase, path, evidenceReadError: true }))
    page.__reviewDtoPending.push(pending)
  })
  return { observerReady: true, now: new Date().toISOString() }
}
