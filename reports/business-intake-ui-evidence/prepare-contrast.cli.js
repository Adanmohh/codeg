async page => {
  await page.getByRole('button',{name:'Shared work',exact:true}).click();
  await page.getByRole('button',{name:/Shape the website launch brief/}).click();
  await page.getByRole('button',{name:'Edit task',exact:true}).click();
  await page.getByRole('textbox',{name:'Brief',exact:true}).fill('Synthetic pane draft retained after the contrast correction.');
  await page.getByRole('button',{name:'Show side by side',exact:true}).click();
  return {privateDraftOpened:true,saveEnabled:await page.getByRole('button',{name:'Save changes',exact:true}).isEnabled()};
}
