import { expect, test } from "@playwright/test";

// Smoke tests for the gallery page. The first group needs no chain access; the
// second reads from the gallery and NFT contracts deployed on testnet.

test.describe("page", () => {
  test("home page mounts with the gallery title", async ({ page }) => {
    const response = await page.goto("/");
    expect(response?.ok()).toBeTruthy();
    await expect(
      page.getByRole("heading", { name: "Stellar Squares", level: 1 }),
    ).toBeVisible();
  });

  test("shows all 20 squares", async ({ page }) => {
    await page.goto("/");
    await expect(page.getByRole("heading", { level: 2 })).toHaveCount(20);
    await expect(
      page.getByRole("heading", { name: "Square #0", exact: true }),
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { name: "Square #19", exact: true }),
    ).toBeVisible();
  });

  test("Connect opens the wallet modal", async ({ page }) => {
    await page.goto("/");
    await page.getByRole("button", { name: "Connect", exact: true }).click();
    await expect(page.getByText("Connect Wallet")).toBeVisible();
  });
});

test.describe("contracts on testnet", () => {
  test("every square shows an owner and a token URI", async ({ page }) => {
    await page.goto("/");
    // Owners are read from the NFT contract; "Loading..." and the error text
    // both fail this match.
    await expect(
      page.getByText(/^Owned by (the Gallery Contract|G[A-Z2-7]{55})$/),
    ).toHaveCount(20, { timeout: 30_000 });
    await expect(page.getByRole("link", { name: "Token URI" })).toHaveCount(20);
  });
});
