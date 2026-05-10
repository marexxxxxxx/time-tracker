const { test, expect } = require('@playwright/test');

test.describe('Time tracking functionality', () => {
  // Check that the value does not change over time, proving the timer is static.
  // This is what the application currently does and what we want to document.

  test('Check time tracking is static on index.html', async ({ page }) => {
    await page.goto(`file://${__dirname}/index.html`);

    // Find the Total Screen Time value
    const totalScreenTimeTitle = page.locator('text=Total Screen Time');
    await expect(totalScreenTimeTitle).toBeVisible();

    const valueLocator = page.locator('div.font-display').first();
    const initialValue = await valueLocator.textContent();

    expect(initialValue).toBeTruthy();

    // Wait for a few seconds
    await page.waitForTimeout(3000);

    const finalValue = await valueLocator.textContent();

    // Check that the value remained static
    expect(initialValue).toBe(finalValue);
  });

  test('Check time tracking is static on productivity.html', async ({ page }) => {
    await page.goto(`file://${__dirname}/productivity.html`);

    const focusTimeTitle = page.locator('text=Focus Time');
    await expect(focusTimeTitle).toBeVisible();

    const timeParent = focusTimeTitle.locator('..');
    const timeLocator = timeParent.locator('span.font-semibold');

    const initialValue = await timeLocator.textContent();
    expect(initialValue).toBeTruthy();

    // Wait for a few seconds
    await page.waitForTimeout(3000);

    const finalValue = await timeLocator.textContent();

    // Check that the value remained static
    expect(initialValue).toBe(finalValue);
  });
});
