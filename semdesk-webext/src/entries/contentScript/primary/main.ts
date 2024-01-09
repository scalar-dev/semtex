import renderContent from "../renderContent";
import "./style.css";
import * as browser from 'webextension-polyfill';

renderContent(
  import.meta.PLUGIN_WEB_EXT_CHUNK_CSS_PATHS,
  (_appRoot: HTMLElement) => {
    browser.runtime.sendMessage({
      content: { url: document.URL, html: document.body.innerHTML },
    });
  }
);
