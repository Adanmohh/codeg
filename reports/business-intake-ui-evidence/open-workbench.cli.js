async page => {
  await page.getByRole('button',{name:'Shared work',exact:true}).click();
  await page.getByRole('button',{name:/Shape the website launch brief/}).click();
  await page.getByRole('button',{name:'Edit task',exact:true}).click();
  const brief=page.getByRole('textbox',{name:'Brief',exact:true});
  await brief.fill('Synthetic private draft retained across tabs, panes, locale and appearance.');
  await page.getByRole('button',{name:'Show side by side',exact:true}).click();
  await page.getByRole('separator',{name:'Resize work panes',exact:true}).waitFor();
  await page.screenshot({path:'reports/business-intake-ui-evidence/workbench-split-1280-light-before.png'});
  return await page.evaluate(()=>({
    overflow:document.documentElement.scrollWidth>innerWidth,
    panels:[...document.querySelectorAll('[role=tabpanel]')].filter(e=>getComputedStyle(e).visibility!=='hidden').map(e=>({label:document.getElementById(e.getAttribute('aria-labelledby'))?.innerText,width:e.getBoundingClientRect().width,scrollWidth:e.scrollWidth})),
    privateDraftPresent:document.querySelector('textarea')?.value.includes('Synthetic private draft retained'),
    privateStored:JSON.stringify(localStorage).includes('Synthetic private draft retained'),
  }));
}
