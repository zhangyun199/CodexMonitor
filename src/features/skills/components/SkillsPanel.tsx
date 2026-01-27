import { useCallback, useEffect, useMemo, useState } from "react";
import Download from "lucide-react/dist/esm/icons/download";
import RefreshCcw from "lucide-react/dist/esm/icons/refresh-ccw";
import type { SkillValidationResult } from "../../../types";
import {
  getSkillsList,
  skillsConfigRead,
  skillsConfigWrite,
  skillsInstallFromGit,
  skillsValidate,
} from "../../../services/tauri";
import { resolveEnabledSkills, type SkillsConfig } from "../utils";

export type SkillsPanelProps = {
  workspaceId: string | null;
};

export function SkillsPanel({ workspaceId }: SkillsPanelProps) {
  const [results, setResults] = useState<SkillValidationResult[]>([]);
  const [skills, setSkills] = useState<
    { name: string; path: string; description?: string }[]
  >([]);
  const [enabledSkills, setEnabledSkills] = useState<Set<string>>(new Set());
  const [installUrl, setInstallUrl] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    if (!workspaceId) return;
    setLoading(true);
    setError(null);
    try {
      const data = (await skillsValidate(workspaceId)) as SkillValidationResult[];
      setResults(data ?? []);
      const response = await getSkillsList(workspaceId);
      const rawBuckets = response?.result?.data ?? response?.data ?? [];
      const rawSkills =
        response?.result?.skills ??
        response?.skills ??
        (Array.isArray(rawBuckets)
          ? rawBuckets.flatMap((bucket: any) => bucket?.skills ?? [])
          : []);
      const parsed = rawSkills.map((item: any) => ({
        name: String(item.name ?? ""),
        path: String(item.path ?? ""),
        description: item.description ? String(item.description) : undefined,
      }));
      const normalized = parsed.filter((item: any) => item.name);
      setSkills(normalized);
      const config = (await skillsConfigRead(workspaceId)) as SkillsConfig | null;
      setEnabledSkills(resolveEnabledSkills(normalized, config));
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, [workspaceId]);

  const install = useCallback(async () => {
    if (!workspaceId || !installUrl.trim()) return;
    setLoading(true);
    setError(null);
    try {
      await skillsInstallFromGit(installUrl.trim(), "workspace", workspaceId);
      setInstallUrl("");
      await refresh();
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, [installUrl, refresh, workspaceId]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const persistConfig = useCallback(
    async (nextEnabled: Set<string>) => {
      if (!workspaceId) return;
      const enabled = skills
        .filter((skill) => nextEnabled.has(`${skill.name}|${skill.path}`))
        .map((skill) => ({ name: skill.name, path: skill.path }));
      const disabled = skills
        .filter((skill) => !nextEnabled.has(`${skill.name}|${skill.path}`))
        .map((skill) => ({ name: skill.name, path: skill.path }));
      await skillsConfigWrite(workspaceId, {
        enabled,
        disabled,
      });
    },
    [skills, workspaceId],
  );

  const skillRows = useMemo(
    () =>
      skills.map((skill) => {
        const key = `${skill.name}|${skill.path}`;
        const enabled = enabledSkills.has(key);
        return (
          <label key={key} className="memory-panel-row">
            <input
              type="checkbox"
              checked={enabled}
              onChange={(event) => {
                const next = new Set(enabledSkills);
                if (event.target.checked) {
                  next.add(key);
                } else {
                  next.delete(key);
                }
                setEnabledSkills(next);
                void persistConfig(next);
              }}
            />
            <div>
              <div className="memory-panel-result-title">{skill.name}</div>
              {skill.description && (
                <div className="memory-panel-result-subtitle">{skill.description}</div>
              )}
            </div>
          </label>
        );
      }),
    [enabledSkills, persistConfig, skills],
  );

  return (
    <div className="memory-panel">
      <div className="memory-panel-header">
        <div className="memory-panel-title">Skills</div>
        <div className="memory-panel-actions">
          <button
            type="button"
            className="ghost icon-button"
            onClick={() => void refresh()}
            title="Refresh skills"
          >
            <RefreshCcw aria-hidden />
          </button>
        </div>
      </div>

      <div className="memory-panel-form">
        <label className="memory-panel-label">Install from Git</label>
        <div className="memory-panel-row">
          <input
            className="memory-panel-input"
            value={installUrl}
            onChange={(event) => setInstallUrl(event.target.value)}
            placeholder="https://github.com/user/skill-repo"
          />
          <button type="button" className="ghost" onClick={() => void install()}>
            <Download aria-hidden />
            Install
          </button>
        </div>
      </div>

      {error && <div className="memory-panel-error">{error}</div>}
      {loading && <div className="memory-panel-status">Loading…</div>}

      <div className="memory-panel-results">
        {results.length === 0 && !loading && (
          <div className="memory-panel-status">No skill issues detected.</div>
        )}
        {results.map((skill) => (
          <div key={skill.name} className="memory-panel-result">
            <div className="memory-panel-result-title">{skill.name}</div>
            {skill.description && (
              <div className="memory-panel-result-subtitle">{skill.description}</div>
            )}
            {skill.issues.length > 0 && (
              <ul className="memory-panel-tags">
                {skill.issues.map((issue) => (
                  <li key={issue} className="memory-panel-tag">
                    ⚠️ {issue}
                  </li>
                ))}
              </ul>
            )}
          </div>
        ))}
      </div>

      <div className="memory-panel-results">
        {skills.length === 0 && !loading && (
          <div className="memory-panel-status">No skills loaded.</div>
        )}
        {skillRows}
      </div>
    </div>
  );
}
