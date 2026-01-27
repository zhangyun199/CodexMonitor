import { describe, expect, it } from "vitest";
import { resolveEnabledSkills } from "./utils";

describe("resolveEnabledSkills", () => {
  const skills = [
    { name: "one", path: "/a" },
    { name: "two", path: "/b" },
  ];

  it("defaults to all enabled when no config", () => {
    const enabled = resolveEnabledSkills(skills, null);
    expect(enabled.has("one|/a")).toBe(true);
    expect(enabled.has("two|/b")).toBe(true);
  });

  it("respects enabled list", () => {
    const enabled = resolveEnabledSkills(skills, {
      enabled: [{ name: "one", path: "/a" }],
    });
    expect(enabled.has("one|/a")).toBe(true);
    expect(enabled.has("two|/b")).toBe(false);
  });

  it("respects disabled list", () => {
    const enabled = resolveEnabledSkills(skills, {
      disabled: [{ name: "two", path: "/b" }],
    });
    expect(enabled.has("one|/a")).toBe(true);
    expect(enabled.has("two|/b")).toBe(false);
  });
});
