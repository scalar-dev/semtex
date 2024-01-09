import { Readability } from "@mozilla/readability";
import browser from "webextension-polyfill";

browser.runtime.onMessage.addListener((message) => {
  const htmlDoc = document.implementation.createHTMLDocument();
  htmlDoc.open();
  htmlDoc.write(message.content.html);
  htmlDoc.close();

  const rdb = new Readability(htmlDoc).parse();

  if (rdb) {
    fetch("http://localhost:8080/ingest", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        title: rdb.title,
        content: rdb.textContent,
        source: {
          name: "browser",
          url: message.content.url,
        },
      }),
    });
  }
});
