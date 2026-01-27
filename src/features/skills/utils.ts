export type SkillRecord = { name: string; path: string };
export type SkillsConfig = {
  enabled?: SkillRecord[];
  disabled?: SkillRecord[];
};

export function resolveEnabledSkills(
  skills: SkillRecord[],
  config?: SkillsConfig | null,
): Set<string> {
  const enabledEntries = config?.enabled ?? [];
  const disabledEntries = config?.disabled ?? [];
  const enabled = new Set<string>();

  if (enabledEntries.length > 0) {
    enabledEntries.forEach((entry) => {
      if (entry?.name && entry?.path) {
        enabled.add(`${entry.name}|${entry.path}`);
      }
    });
    return enabled;
  }

  if (disabledEntries.length > 0) {
    skills.forEach((skill) => {
      const key = `${skill.name}|${skill.path}`;
      const isDisabled = disabledEntries.some(
        (entry) => entry?.name === skill.name && entry?.path === skill.path,
      );
      if (!isDisabled) {
        enabled.add(key);
      }
    });
    return enabled;
  }

  skills.forEach((skill) => enabled.add(`${skill.name}|${skill.path}`));
  return enabled;
}
