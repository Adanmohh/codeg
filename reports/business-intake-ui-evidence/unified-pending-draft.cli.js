async page => {
  await page.getByRole('tab',{name:'Sources',exact:true}).click();
  const sources=page.getByRole('tabpanel',{name:'Sources',exact:true});
  await sources.getByRole('heading',{name:'ui: meeting-follow-up',exact:true}).waitFor();
  const candidates=sources.getByRole('region',{name:'Private drafts & decisions',exact:true});
  await candidates.getByRole('button',{name:/^Pending 1 Pending/}).click();
  const draft=sources.getByRole('region',{name:'Private task draft',exact:true});
  await draft.getByLabel('Task title',{exact:true}).fill('Synthetic UI pending source review');
  await draft.getByLabel('Brief',{exact:true}).fill('UI private draft kept through a task-source navigation request. This is not accepted shared text.');
  await draft.getByLabel('Due date',{exact:true}).fill('2027-01-02');
  await draft.getByLabel('Owner mentioned in source (optional)',{exact:true}).fill('Synthetic human clarification');
  await draft.getByLabel('Deadline mentioned in source (optional)',{exact:true}).fill('Calendar suggestion only');
  return {unsavedTitle:await draft.getByLabel('Task title',{exact:true}).inputValue(),dueDate:await draft.getByLabel('Due date',{exact:true}).inputValue(),saved:false};
}
