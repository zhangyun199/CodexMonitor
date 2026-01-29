type ToastPayload = {
  title: string;
  message?: string;
};

export function pushErrorToast({ title, message }: ToastPayload) {
  console.error(`[Toast] ${title}`, message ?? "");
}
