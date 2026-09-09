// Apache-2.0 Codeg: personal sign-in flow from
// owner-personal-session.cli.js@1974d97f44b69db96051ace1c68717cc8c27b6e9.
// Installed CLI0.1.18 run-code executes in its Node process; no token in code,
// CLI arguments, returned evidence, persisted state or a filled-form snapshot.
async page => {
  const node = page.constructor.constructor('return process')();
  const fs = node.getBuiltinModule('node:fs');
  const credentials = JSON.parse(fs.readFileSync('/Users/mohamedadan/projects/_worktrees/ops-desk/tickets/.docs/business-intake-fixtures/unified-YQmRz8/ui-credentials.json', 'utf8'));
  const token = credentials.sessions.owner.token;
  if (!token || !page.url().startsWith('http://127.0.0.1:4355/business')) throw Error('Wrong synthetic session');
  const metadata = await page.evaluate(async () => (await fetch('/__business_intake_fixture')).json());
  if (!metadata.synthetic || metadata.backendPort !== 4351 || metadata.workspaceBackendPort !== 4351 || !metadata.export.endsWith('recovery-60d600db0')) throw Error('Wrong unified export');
  await page.setViewportSize({width:1280,height:900});
  await page.emulateMedia({colorScheme:'light'});
  await page.getByRole('combobox', {name:'Language',exact:true}).selectOption('en');
  await page.getByLabel('Workspace address', {exact:true}).fill('http://127.0.0.1:4355');
  try {
    await page.getByLabel('Personal access token', {exact:true}).fill(token);
    await page.getByRole('button', {name:'Connect',exact:true}).click();
    await page.getByRole('heading', {name:'My work',exact:true}).waitFor();
  } catch {
    throw Error('Synthetic sign-in did not reach workspace; no form snapshot captured');
  }
  if (await page.locator('input[type=password]').count()) throw Error('Credential form remains mounted');
  const evidence = await page.evaluate(secret => ({
    personalSession:true,
    legacyTokenPresent:!!localStorage.getItem('codeg_token'),
    credentialStored:Object.values(localStorage).some(value=>value.includes(secret)) || Object.values(sessionStorage).some(value=>value.includes(secret)),
    nativeChrome:!!document.querySelector('[data-business-native-chrome]'),
  }), token);
  if(evidence.credentialStored)throw Error('Credential persisted');
  return {...evidence,legacyEntry:await page.getByRole('link',{name:'Open engineering workspace',exact:true}).count()};
}
