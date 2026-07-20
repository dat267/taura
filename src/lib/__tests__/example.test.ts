import { describe, it, expect } from "vitest";

describe("example", () => {
  it("should pass", () => {
    expect(1 + 1).toBe(2);
  });

  it("should handle strings", () => {
    const greeting = "Hello, World!";
    expect(greeting).toContain("World");
  });
});
