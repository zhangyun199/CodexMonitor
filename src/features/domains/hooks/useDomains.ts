import { useCallback, useEffect, useMemo, useState } from "react";
import type { Domain } from "../../../types";
import {
  listDomains,
  createDomain as createDomainService,
  updateDomain as updateDomainService,
  deleteDomain as deleteDomainService,
} from "../../../services/tauri";

export function useDomains() {
  const [domains, setDomains] = useState<Domain[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await listDomains();
      setDomains(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const createDomain = useCallback(async (domain: Domain) => {
    const created = await createDomainService(domain);
    setDomains((prev) => [...prev, created]);
    return created;
  }, []);

  const updateDomain = useCallback(async (domain: Domain) => {
    const updated = await updateDomainService(domain);
    setDomains((prev) =>
      prev.map((entry) => (entry.id === updated.id ? updated : entry)),
    );
    return updated;
  }, []);

  const deleteDomain = useCallback(async (domainId: string) => {
    await deleteDomainService(domainId);
    setDomains((prev) => prev.filter((entry) => entry.id !== domainId));
  }, []);

  const domainsById = useMemo(
    () =>
      domains.reduce<Record<string, Domain>>((acc, domain) => {
        acc[domain.id] = domain;
        return acc;
      }, {}),
    [domains],
  );

  return {
    domains,
    domainsById,
    loading,
    error,
    refresh,
    createDomain,
    updateDomain,
    deleteDomain,
  };
}
