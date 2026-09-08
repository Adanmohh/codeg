async page => {
  const fixture = await page.evaluate(async () => (await fetch('/__business_fixture')).json());
  const after = fixture.export.endsWith('/review-export');
  const phase = after ? 'after' : 'before';
  const cases = [];
  const preferences = await page.context().newPage();
  await preferences.goto('http://127.0.0.1:4346/business.html');
  for (const locale of (after ? ['en', 'ar'] : ['en'])) {
    await preferences.getByLabel(/^(Language|اللغة)$/).selectOption(locale);
    const searchName = locale === 'en' ? 'Find a task' : 'البحث عن مهمة';
    const refreshName = locale === 'en' ? 'Refresh' : 'تحديث';
    for (const theme of (after ? ['light', 'dark'] : ['light'])) {
      await preferences.getByLabel(/^(Appearance|المظهر)$/).selectOption(theme);
      await page.waitForFunction(({locale, theme}) => document.documentElement.dir === (locale === 'ar' ? 'rtl' : 'ltr') && document.documentElement.classList.contains('dark') === (theme === 'dark'), {locale, theme});
      for (const width of (after ? [390, 768, 1280] : [1280])) {
        await page.setViewportSize({width, height: 900});
        await page.bringToFront();
        const search = page.getByRole('textbox', {name: searchName, exact: true});
        await search.waitFor();
        await page.waitForFunction(() => !document.querySelector('#business-main form button[type="button"]').disabled);
        if (await search.inputValue()) throw Error('Placeholder sample requires an empty field');
        await page.getByRole('heading', {level: 1}).click();
        const measured = await search.evaluate(el => {
          const canvas = document.createElement('canvas'); canvas.width = 1; canvas.height = 1;
          const cx = canvas.getContext('2d', {willReadFrequently: true});
          const rgba = value => {cx.clearRect(0, 0, 1, 1); cx.fillStyle = value; cx.fillRect(0, 0, 1, 1); return [...cx.getImageData(0, 0, 1, 1).data];};
          const layers = [];
          for (let node = el; node; node = node.parentElement) {
            const style = getComputedStyle(node);
            layers.push({tag: node.tagName, background: rgba(style.backgroundColor), opacity: style.opacity, image: style.backgroundImage});
          }
          let bg = [255, 255, 255];
          for (const layer of [...layers].reverse()) bg = bg.map((v, i) => Math.round(layer.background[i] * (layer.background[3] / 255) + v * (1 - layer.background[3] / 255)));
          const placeholder = getComputedStyle(el, '::placeholder');
          const fg = rgba(placeholder.color);
          fg[3] = fg[3] / 255 * Number(placeholder.opacity);
          const rect = el.getBoundingClientRect();
          return {sample: {text: el.placeholder, fg: `rgba(${fg.join(', ')})`, bg: `rgb(${bg.join(', ')})`, sizePx: parseFloat(placeholder.fontSize), bold: parseInt(placeholder.fontWeight, 10) >= 700}, cssColor: placeholder.color, placeholderOpacity: placeholder.opacity, disabled: el.disabled, layers, input: {width: rect.width, height: rect.height}, viewport: innerWidth, scrollWidth: document.documentElement.scrollWidth, direction: document.documentElement.dir};
        });
        if (measured.disabled || measured.layers.some(layer => layer.opacity !== '1' || layer.image !== 'none')) throw Error('Unexpected opacity/image/disabled sample; do not waive it');
        await search.click();
        await page.keyboard.press('Tab');
        await page.waitForTimeout(500);
        const focus = await page.evaluate(() => {
          const el = document.activeElement; const style = getComputedStyle(el); const rect = el.getBoundingClientRect();
          return {name: el.getAttribute('aria-label') || el.textContent.trim(), width: rect.width, height: rect.height, clipPath: style.clipPath, outlineStyle: style.outlineStyle, outlineWidth: style.outlineWidth, shadow: style.boxShadow, focusVisible: el.matches(':focus-visible')};
        });
        if (after && (focus.name !== refreshName || focus.width < 44 || focus.height < 44 || focus.clipPath !== 'none' || !focus.focusVisible || focus.shadow === 'none')) throw Error('Search Tab has no visible refresh focus');
        if (!after && (focus.width !== 1 || focus.height !== 1 || focus.clipPath !== 'inset(50%)')) throw Error('Before hidden-focus finding did not reproduce');
        if (measured.scrollWidth > width) throw Error('Search layout overflow');
        await page.screenshot({path: `reports/business-workspace-evidence/search-${phase}-${width}-${theme}-${locale}.png`});
        await search.fill('Worker conflict clarity · Synthetic');
        let responsePromise = page.waitForResponse(response => response.url().endsWith('/api/business/tasks/list'));
        await search.press('Enter');
        const response = await responsePromise;
        const result = await response.json();
        if (response.status() !== 200 || result.page !== 0 || result.tasks.length !== 1 || result.tasks[0].id !== 'ec9d106d-fa40-4e26-b2dc-612cedd587ad') throw Error('Enter did not return the real filtered worker task');
        await search.fill('');
        responsePromise = page.waitForResponse(response => response.url().endsWith('/api/business/tasks/list'));
        await search.press('Enter');
        if ((await responsePromise).status() !== 200) throw Error('Clear search failed');
        cases.push({locale, theme, width, ...measured, focus, enter: {status: 200, page: result.page, count: result.tasks.length}});
      }
    }
  }
  await preferences.getByLabel(/^(Language|اللغة)$/).selectOption('en');
  await preferences.getByLabel(/^(Appearance|المظهر)$/).selectOption('light');
  await preferences.close();
  await page.bringToFront();
  await page.setViewportSize({width: 1280, height: 900});
  return {synthetic: true, phase, export: fixture.export, backendPort: fixture.backendPort, cases, providerOrTaskWrites: 0};
}
