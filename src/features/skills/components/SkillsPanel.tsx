import { useCallback, useEffect, useState } from "react";
import Download from "lucide-react/dist/esm/icons/download";
import RefreshCcw from "lucide-react/dist/esm/icons/refresh-ccw";
import type { SkillValidationResult } from "../../../types";
import { skillsInstallFromGit, skillsValidate } from "../../../services/tauri";

export type SkillsPanelProps = {
  workspaceId: string | null;
};

export function SkillsPanel({ workspaceId }: SkillsPanelProps) {
  const [results, setResults] = useState<SkillValidationResult[]>([]);
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
    </div>
  );
}
