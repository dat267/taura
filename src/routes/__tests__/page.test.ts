import { describe, it, expect } from "vitest";

describe("Home page", () => {
  it("should export prerender config", async () => {
    // Verify the layout doesn't SSR, which is correct for Tauri
    const layout = await import("../+layout");
    expect(layout.ssr).toBe(false);
    expect(layout.prerender).toBe(true);
  });
});
