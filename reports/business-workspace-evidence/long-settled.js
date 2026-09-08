async page => {
  const dialog = page.getByRole('dialog');
  await page.waitForFunction(() => { const node = document.querySelector('[role=dialog]'); return node && getComputedStyle(node).opacity === '1' && getComputedStyle(node).transform === 'none'; });
  const measured = await dialog.evaluate(el => ({viewport: innerWidth, pageWidth: document.documentElement.scrollWidth, dialogWidth: el.clientWidth, contentWidth: el.scrollWidth, opacity: getComputedStyle(el).opacity, transform: getComputedStyle(el).transform}));
  await page.screenshot({path: 'reports/business-workspace-evidence/long-detail-before-390.png'});
  await dialog.getByRole('button', {name: 'Close', exact: true}).click();
  await page.setViewportSize({width: 1280, height: 900});
  return {synthetic: true, uiSource: 'c1a35955', measured, probeCorrection: 'OverlayScrollbars has two dormant scroll-driven handles with running state and null currentTime; settling uses actual dialog opacity and transform.'};
}
