async page => {
  const records = [
    {title: 'Shape the customer welcome · Synthetic', domain: 'marketing', priority: 'high', dueDate: '2026-10-01', assignee: 'Nabil · Synthetic member', reviewer: 'Samira · Synthetic manager', notes: 'Synthetic work. Draft a clear welcome for new customers. Deliver the message and the reason for its structure; do not send anything.'},
    {title: 'Check the homepage promise · Synthetic', domain: 'website', priority: 'normal', dueDate: '2026-12-31', assignee: 'Atlas · Synthetic agent · Agent', reviewer: '', notes: 'Synthetic work. Compare the existing homepage promise with the product experience. Record proposed changes for a human decision. No model or engine is started.'},
    {title: 'Summarize customer feedback · Synthetic', domain: 'feedback', priority: 'normal', dueDate: '', assignee: '', reviewer: '', notes: 'Synthetic work. Prepare a short summary of the feedback already available to the team. No new inbox or provider connection is implied.'},
    {title: 'Synthetic long title ' + 'CustomerContext'.repeat(13), domain: 'feedback', priority: 'low', dueDate: '2028-02-29', assignee: '', reviewer: '', notes: 'Synthetic layout proof. ' + 'UnbrokenCustomerReference'.repeat(20)},
  ];
  const created = [];
  await page.getByRole('heading', {name: 'My work', exact: true}).waitFor();
  for (const record of records) {
    await page.getByRole('button', {name: 'Create task', exact: true}).first().click();
    const form = page.getByRole('dialog', {name: 'Create task', exact: true});
    await form.getByLabel('Task title', {exact: true}).fill(record.title);
    await form.getByLabel('Brief', {exact: true}).fill(record.notes);
    await form.getByLabel('Area of work', {exact: true}).selectOption(record.domain);
    await form.getByLabel('Priority', {exact: true}).selectOption(record.priority);
    await form.getByLabel('Due date', {exact: true}).fill(record.dueDate);
    await form.getByLabel('Assigned to', {exact: true}).selectOption(record.assignee ? {label: record.assignee} : '');
    await form.getByLabel('Reviewer', {exact: true}).selectOption(record.reviewer ? {label: record.reviewer} : '');
    const responsePromise = page.waitForResponse(response => response.url().endsWith('/api/business/tasks/create'));
    await form.getByRole('button', {name: 'Create task', exact: true}).click();
    const response = await responsePromise;
    if (response.status() !== 200) throw Error('Actual task creation failed: ' + response.status());
    const {task} = await response.json();
    if (task.title !== record.title || task.dueDate !== (record.dueDate || null) || task.status !== 'todo') throw Error('Task/date roundtrip differs');
    created.push({id: task.id, title: task.title, dueDate: task.dueDate, revision: task.revision, ownerId: task.ownerId, assigneeId: task.assigneeId, reviewerId: task.reviewerId});
    const detail = page.getByRole('dialog', {name: record.title, exact: true});
    await detail.waitFor();
    await detail.getByRole('button', {name: 'Close', exact: true}).click();
  }
  await page.screenshot({path: 'reports/business-workspace-evidence/work-list-real-1280-light.png'});
  return {synthetic: true, actualUICreation: true, created, noProviderOrEngine: true};
}
