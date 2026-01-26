import { chromium, Browser, BrowserContext, Page } from "playwright";
import * as readline from "readline";

type Session = {
  id: string;
  browser?: Browser;
  context: BrowserContext;
  page: Page;
};

const sessions = new Map<string, Session>();

function jsonResponse(id: string, result?: unknown, error?: string) {
  const payload = error
    ? { id, error: { message: error } }
    : { id, result: result ?? null };
  process.stdout.write(JSON.stringify(payload) + "\n");
}

async function handleCreate(params: any) {
  const headless = params?.headless !== false;
  const viewport = params?.viewport ?? { width: 1280, height: 720 };
  const userDataDir = params?.userDataDir as string | undefined;
  let context: BrowserContext;
  let browser: Browser | undefined;

  if (userDataDir) {
    context = await chromium.launchPersistentContext(userDataDir, {
      headless,
      viewport,
    });
  } else {
    browser = await chromium.launch({ headless });
    context = await browser.newContext({ viewport });
  }

  const page = await context.newPage();
  if (params?.startUrl) {
    await page.goto(params.startUrl, { waitUntil: "domcontentloaded" });
  }

  const sessionId = `b-${Date.now()}-${Math.floor(Math.random() * 10000)}`;
  sessions.set(sessionId, { id: sessionId, browser, context, page });
  return { sessionId };
}

async function getSession(sessionId: string): Promise<Session> {
  const session = sessions.get(sessionId);
  if (!session) {
    throw new Error("Invalid sessionId");
  }
  return session;
}

async function handleNavigate(params: any) {
  const session = await getSession(params.sessionId);
  const waitUntil = params.waitUntil ?? "load";
  const timeout = params.timeoutMs ?? 30000;
  await session.page.goto(params.url, { waitUntil, timeout });
  return { ok: true };
}

async function handleScreenshot(params: any) {
  const session = await getSession(params.sessionId);
  const fullPage = params.fullPage ?? true;
  const buffer = await session.page.screenshot({
    fullPage,
    type: "png",
  });
  const url = session.page.url();
  const title = await session.page.title();
  const viewport = session.page.viewportSize();
  return {
    base64Png: buffer.toString("base64"),
    url,
    title,
    width: viewport?.width ?? null,
    height: viewport?.height ?? null,
  };
}

async function handleClick(params: any) {
  const session = await getSession(params.sessionId);
  if (params.selector) {
    await session.page.click(params.selector);
  } else if (typeof params.x === "number" && typeof params.y === "number") {
    await session.page.mouse.click(params.x, params.y);
  } else {
    throw new Error("Missing selector or x/y");
  }
  return { ok: true };
}

async function handleType(params: any) {
  const session = await getSession(params.sessionId);
  const selector = params.selector;
  if (!selector) throw new Error("Missing selector");
  if (params.clearFirst) {
    await session.page.fill(selector, "");
  }
  await session.page.type(selector, params.text ?? "");
  return { ok: true };
}

async function handlePress(params: any) {
  const session = await getSession(params.sessionId);
  await session.page.keyboard.press(params.key ?? "Enter");
  return { ok: true };
}

async function handleEvaluate(params: any) {
  const session = await getSession(params.sessionId);
  const result = await session.page.evaluate(params.js ?? "");
  return { result };
}

async function handleSnapshot(params: any) {
  const screenshot = await handleScreenshot(params);
  const session = await getSession(params.sessionId);
  const elements = await session.page.$$eval(
    "a,button,input,textarea,select",
    (nodes) =>
      nodes.slice(0, 50).map((node) => {
        const el = node as HTMLElement;
        return {
          tag: el.tagName.toLowerCase(),
          text: el.innerText?.trim() ?? "",
          name: (el as HTMLInputElement).name ?? "",
          id: el.id ?? "",
          href: (el as HTMLAnchorElement).href ?? "",
        };
      })
  );
  return { ...screenshot, elements };
}

async function handleClose(params: any) {
  const session = await getSession(params.sessionId);
  await session.context.close();
  await session.browser?.close();
  sessions.delete(params.sessionId);
  return { ok: true };
}

const rl = readline.createInterface({ input: process.stdin });
rl.on("line", async (line) => {
  const trimmed = line.trim();
  if (!trimmed) return;
  let message: any;
  try {
    message = JSON.parse(trimmed);
  } catch {
    return;
  }
  const id = message.id;
  const method = message.method;
  const params = message.params ?? {};
  if (!id || !method) return;

  try {
    let result: any = null;
    switch (method) {
      case "browser.create":
        result = await handleCreate(params);
        break;
      case "browser.list":
        result = { sessions: Array.from(sessions.keys()) };
        break;
      case "browser.close":
        result = await handleClose(params);
        break;
      case "browser.navigate":
        result = await handleNavigate(params);
        break;
      case "browser.screenshot":
        result = await handleScreenshot(params);
        break;
      case "browser.click":
        result = await handleClick(params);
        break;
      case "browser.type":
        result = await handleType(params);
        break;
      case "browser.press":
        result = await handlePress(params);
        break;
      case "browser.evaluate":
        result = await handleEvaluate(params);
        break;
      case "browser.snapshot":
        result = await handleSnapshot(params);
        break;
      default:
        jsonResponse(id, null, "Unknown method");
        return;
    }
    jsonResponse(id, result);
  } catch (err: any) {
    jsonResponse(id, null, err?.message ?? "Worker error");
  }
});
