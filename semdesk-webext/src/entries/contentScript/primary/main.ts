import { isProbablyReaderable } from "@mozilla/readability";
import renderContent from "../renderContent";
import "./style.css";
import * as browser from "webextension-polyfill";

renderContent(
  import.meta.PLUGIN_WEB_EXT_CHUNK_CSS_PATHS,
  (_appRoot: HTMLElement) => {
    if (isProbablyReaderable(document)) {
      browser.runtime.sendMessage({
        type: "store",
        content: {
          url: document.URL,
          html: document.body.innerHTML,
          title: document.title,
        },
      });
    } else {
      console.log("NOT READABLE");
    }
  }
);
