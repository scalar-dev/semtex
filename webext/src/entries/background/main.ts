import { Readability } from "@mozilla/readability";
import browser from "webextension-polyfill";

const ingestUrl = "http://localhost:8080/ingest";

interface Entry {
  title: string;
  content: string;
  source: {
    name: string;
    url: string;
  };
}

class Mutex {
  _locking: Promise<unknown>;
  _locked: boolean;

  constructor() {
    this._locking = Promise.resolve();
    this._locked = false;
  }

  isLocked() {
    return this._locked;
  }

  lock() {
    this._locked = true;
    let unlockNext: (value?: unknown) => any;
    let willLock = new Promise((resolve) => (unlockNext = resolve));
    willLock.then(() => (this._locked = false));
    let willUnlock = this._locking.then(() => unlockNext);
    this._locking = this._locking.then(() => willLock);
    return willUnlock;
  }
}

const storageMutex = new Mutex();

setInterval(async () => {
  let unlock = await storageMutex.lock();

  try {
    const toSync: Entry[] = (await browser.storage.local.get({ toSync: [] }))["toSync"];

    if (toSync.length > 0) {
      await fetch(ingestUrl, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ items: toSync }),
      });

      await browser.storage.local.set({ toSync: [] });
    }
  } finally {
    unlock();
  }
}, 10_000);

browser.runtime.onMessage.addListener(async (message) => {
  if (message.type == "store") {
    const htmlDoc = document.implementation.createHTMLDocument();
    htmlDoc.open();
    htmlDoc.write(message.content.html);
    htmlDoc.close();

    const rdb = new Readability(htmlDoc).parse();

    if (rdb) {
      const unlock = await storageMutex.lock();

      try {
        const toSync: Entry[] = (
          await browser.storage.local.get({ toSync: [] })
        )["toSync"];

        toSync.push({
          title: rdb.title || message.content.title,
          content: rdb.textContent,
          source: {
            name: "browser",
            url: message.content.url,
          },
        });

        await browser.storage.local.set({ toSync });
      } finally {
        unlock();
      }
      console.log("DONE");
    }
  }
});
