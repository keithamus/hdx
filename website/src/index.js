import { basicSetup } from "codemirror";
import { Decoration, EditorView, keymap } from "@codemirror/view";
import {
  Compartment,
  EditorSelection,
  EditorState,
  RangeSet,
  StateEffect,
  StateField,
} from "@codemirror/state";
import { css, cssLanguage } from "@codemirror/lang-css";
import { sass, sassLanguage } from "@codemirror/lang-sass";
import { rust, rustLanguage } from "@codemirror/lang-rust";
import { json, jsonLanguage } from "@codemirror/lang-json";
import { lintGutter, setDiagnostics } from "@codemirror/lint";
import {
  foldGutter,
  language,
  matchBrackets,
  syntaxTree,
} from "@codemirror/language";
import { csskitLight } from "./theme.js";

import { lex, minify, parse, parse_error_report } from "csskit_wasm";

function createHighlightEffect(view, { start, end }) {
  let range = EditorSelection.range(start, end);
  const highlighter = Decoration.mark({ class: "cm-highlight" });
  if (!start && !end) return;
  const addHighlight = StateEffect.define({
    map: ({ from, to }, change) => ({
      from: change.mapPos(from),
      to: change.mapPos(to),
    }),
  });
  const highlightField = StateField.define({
    create() {
      return Decoration.none;
    },
    update(highlights, tr) {
      highlights = RangeSet.empty;
      for (let e of tr.effects) {
        if (e.is(addHighlight)) {
          highlights = highlights.update({
            add: [highlighter.range(e.value.from, e.value.to)],
          });
        }
      }
      return highlights;
    },
    provide: (f) => EditorView.decorations.from(f),
  });
  const effects = [addHighlight.of(range)];
  if (!view.state.field(highlightField, false)) {
    effects.push(StateEffect.appendConfig.of([highlightField, csskitLight]));
  }
  view.dispatch({ effects });
}

class LoadCodeEvent extends Event {
  constructor(sourcetext) {
    super("load-code", { bubbles: true });
    this.sourcetext = sourcetext;
  }
}

class EditorChangeEvent extends Event {
  constructor(sourcetext) {
    super("change", { bubbles: true });
    this.sourcetext = sourcetext;
  }
}

class CsskitDiagnosticEvent extends Event {
  constructor(diagnostics) {
    super("diagnostic", { bubbles: true });
    this.diagnostics = diagnostics;
  }
}

const bufferToStream = (arrayBuffer) =>
  new ReadableStream({
    start(controller) {
      controller.enqueue(arrayBuffer);
      controller.close();
    },
  });
const streamToBuffer = async (stream) => {
  const reader = stream.getReader();
  const data = [];
  while (true) {
    const { done, value } = await reader.read();
    if (done) return new Uint8Array(data);
    if (result.value) {
      data.push(...value);
    }
  }
};

class CodeStorage {
  constructor(store) {
    this.store = store;
  }

  has() {
    try {
      return this.store.has();
    } catch (e) {
      console.error(e);
      return false;
    }
  }

  async get() {
    try {
      let raw = atob(this.store.get());
      let bytes = new Uint8Array(raw.length);
      for (let i = 0; i < raw.length; i += 1) bytes[i] = raw.charCodeAt(i);
      const reader = bufferToStream(bytes)
        .pipeThrough(new DecompressionStream("deflate-raw"))
        .getReader();
      let arr = [];
      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        for (let i of value) arr.push(String.fromCharCode(i));
      }
      return arr.join("");
    } catch (e) {
      console.error(e);
      return "";
    }
  }

  async set(code) {
    try {
      const reader = bufferToStream(new TextEncoder().encode(code))
        .pipeThrough(new CompressionStream("deflate-raw"))
        .getReader();
      const arr = [];
      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        for (let i of value) arr.push(i);
      }
      this.store.set(btoa(arr.map((s) => String.fromCharCode(s)).join("")));
    } catch (e) {
      console.error(e);
      return "";
    }
  }
}

const codeFromUrl = new CodeStorage({
  has() {
    return new URLSearchParams(window.location.search).has("code");
  },
  get() {
    return decodeURIComponent(
      new URLSearchParams(window.location.search).get("code"),
    );
  },
  set(code) {
    const url = new URL(window.location);
    url.searchParams.delete("example");
    if (code == "AwA=") {
      // empty
      url.searchParams.delete("code");
    } else {
      url.searchParams.set("code", code);
    }
    window.history.replaceState({ path: url.toString() }, null, url.toString());
  },
});

const codeFromLocal = new CodeStorage({
  has() {
    return localStorage.getItem("code") !== null;
  },
  get() {
    return localStorage.getItem("code");
  },
  set(code) {
    localStorage.setItem("code", code);
  },
});

class EditorHighlight extends Event {
  constructor({ start, end }) {
    super("editor-highlight", { bubbles: true });
    this.start = start;
    this.end = end;
  }
}

class EditorFocus extends Event {
  constructor({ start, end }) {
    super("editor-focus", { bubbles: true });
    this.start = start;
    this.end = end;
  }
}

class CsskitEditor extends HTMLElement {
  static define(tagName = "csskit-editor") {
    customElements.define(tagName, this);
  }

  get code() {
    return this.view.state.doc.toString();
  }

  shadowRoot = this.attachShadow({ mode: "open" });

  extensions = [
    basicSetup,
    EditorView.lineWrapping,
    csskitLight,
    css(),
    lintGutter(),
    EditorView.domEventHandlers({
      mouseover: (e) => this.handleEvent(e),
      click: (e) => this.handleEvent(e),
    }),
    EditorView.updateListener.of((view) => {
      if (view.startState.doc.toString() !== view.state.doc.toString()) {
        this.dispatchEvent(new EditorChangeEvent(this.code));
      }
    }),
  ];

  async connectedCallback() {
    let doc =
      await (codeFromUrl.has()
        ? codeFromUrl.get()
        : codeFromLocal.has()
        ? codeFromLocal.get()
        : this.textContent);
    this.view = new EditorView({
      extensions: this.extensions,
      parent: this.shadowRoot,
      root: this.shadowRoot,
      doc,
    });
    requestAnimationFrame(() =>
      this.dispatchEvent(new EditorChangeEvent(this.code))
    );
    this.ownerDocument.addEventListener("diagnostic", this);
    this.ownerDocument.addEventListener("viewer-highlight", this);
    this.ownerDocument.addEventListener("load-code", this);
    this.addEventListener("focusout", this);
    this.addEventListener("focusin", this);
  }

  handleEvent(event) {
    if (event.type == "diagnostic") {
      this.view.dispatch(setDiagnostics(this.view.state, event.diagnostics));
    } else if (event.type == "focusout") {
      codeFromUrl.set(event.target.code);
      codeFromLocal.set(event.target.code);
    } else if (event.type == "viewer-highlight") {
      createHighlightEffect(this.view, event);
    } else if (event.type == "mouseover") {
      const pos = this.view.posAtCoords(event);
      const tree = syntaxTree(this.view.state);
      let cursor = tree.cursorAt(pos);
      this.dispatchEvent(
        new EditorHighlight({ start: cursor.from, end: cursor.to }),
      );
    } else if (event.type == "click" || event.type === "focusin") {
      codeFromUrl.set(this.code);
      codeFromLocal.set(this.code);
      const tree = syntaxTree(this.view.state);
      let cursor;
      if (event.x && event.y) {
        const pos = this.view.posAtCoords(event);
        cursor = tree.cursorAt(pos);
      } else {
        cursor = tree.cursorAt(this.view.state.selection.anchor);
      }
      this.dispatchEvent(
        new EditorFocus({ start: cursor.from, end: cursor.to }),
      );
    } else if (event.type == "load-code") {
      this.view.dispatch(
        this.view.state.update({
          changes: {
            from: 0,
            to: this.view.state.doc.length,
            insert: event.sourcetext,
          },
        }),
      );
      this.dispatchEvent(new EditorChangeEvent(event.sourcetext));
    }
  }
}

class MetricEvent extends Event {
  constructor(name, value = performance.now(), unit = "ms") {
    super("metric", { bubbles: true });
    this.name = name;
    this.value = value;
    this.unit = unit;
  }
}

class ViewerHighlight extends Event {
  constructor(start, end) {
    super("viewer-highlight", { bubbles: true });
    this.start = start;
    this.end = end;
  }
}

class CsskitViewer extends HTMLElement {
  static define(tagName = "csskit-viewer") {
    customElements.define(tagName, this);
  }

  shadowRoot = this.attachShadow({ mode: "open" });

  compartment = new Compartment();

  extensions = [
    csskitLight,
    EditorView.editable.of(false),
    foldGutter(),
    EditorView.lineWrapping,
    this.compartment.of(css()),
    // Change language according to the current view
    EditorState.transactionExtender.of((tr) => {
      if (!tr.docChanged) return null;
      let format = this.format === "minify" ? css() : json();
      return {
        effects: this.compartment.reconfigure(format),
      };
    }),
    EditorView.domEventHandlers({ mouseover: (e) => this.handleEvent(e) }),
  ];

  get format() {
    const form = new FormData(
      document.getElementById(this.getAttribute("for-format")),
    );
    return form.get("format");
  }

  get code() {
    return document.getElementById(this.getAttribute("for-editor")).code;
  }

  handleEvent(event) {
    if (event.type === "change") {
      try {
        if (this.format === "lex") {
          const start = performance.now();
          const tokens = lex(this.code);
          this.dispatchEvent(
            new MetricEvent("parseend", performance.now() - start),
          );
          this.view.dispatch(
            this.view.state.update({
              changes: {
                from: 0,
                to: this.view.state.doc.length,
                insert: JSON.stringify(tokens, null, 2),
              },
            }),
          );
          this.dispatchEvent(new CsskitDiagnosticEvent([]));
        } else if (this.format === "errors") {
          const start = performance.now();
          const { diagnostics } = parse(this.code);
          this.dispatchEvent(new MetricEvent("parseend", performance.now() - start));
          const report = parse_error_report(this.code);
          this.view.dispatch(
            this.view.state.update({
              changes: {
                from: 0,
                to: this.view.state.doc.length,
                insert: report,
              },
            }),
          );
          this.dispatchEvent(new CsskitDiagnosticEvent(diagnostics));
        } else if (this.format === "minify") {
          const start = performance.now();
          const minified = minify(this.code);
          this.dispatchEvent(
            new MetricEvent("parseend", performance.now() - start),
          );
          this.dispatchEvent(
            new MetricEvent("minify", minified.length, "bytes"),
          );
          this.view.dispatch(
            this.view.state.update({
              changes: {
                from: 0,
                to: this.view.state.doc.length,
                insert: minified,
              },
            }),
          );
          this.dispatchEvent(new CsskitDiagnosticEvent([]));
        } else {
          const start = performance.now();
          const { ast, diagnostics } = parse(this.code);
          this.dispatchEvent(
            new MetricEvent("parseend", performance.now() - start),
          );
          this.view.dispatch(
            this.view.state.update({
              changes: {
                from: 0,
                to: this.view.state.doc.length,
                insert: JSON.stringify(ast, null, 2),
              },
            }),
          );
          this.dispatchEvent(new CsskitDiagnosticEvent(diagnostics));
        }
      } catch (e) {
        this.view.dispatch(
          this.view.state.update({
            changes: {
              from: 0,
              to: this.view.state.doc.length,
              insert: `${e.message}\n${e.stack}`,
            },
          }),
        );
        throw e;
      }
    } else if (event.type == "mouseover") {
      const pos = this.view.posAtCoords(event);
      const tree = syntaxTree(this.view.state);
      let cursor = tree.cursorAt(pos);
      let span = { from: 0, to: 0 };
      const textAt = ({ from, to }) =>
        this.view.state.doc.sliceString(from, to);
      // Go up and find the `type` key
      while (true) {
        let text = textAt(cursor);
        if (text == '"node"' || text === '"kind"') {
          cursor.prev();
          span.from = cursor.from;
          span.to =
            Number(matchBrackets(this.view.state, span.from, 1)?.end.to) || 0;
          break;
        }
        if (!cursor.prev()) break;
      }
      if (span.from === span.to) return;
      let json = {};
      try {
        json = JSON.parse(textAt(span));
      } catch {}
      let { start, end } = json;
      if (isFinite(start) && isFinite(end) && start < end) {
        this.dispatchEvent(new ViewerHighlight(start, end));
        if (span.from < span.to) {
          createHighlightEffect(this.view, { start: span.from, end: span.to });
        }
      }
    } else if (
      (event.type == "editor-highlight" || event.type == "editor-focus") &&
      event.target !== this
    ) {
      const tree = syntaxTree(this.view.state);
      let cursor = tree.cursor();
      let span = { from: 0, to: 0 };
      let start, end;
      const textAt = ({ from, to }) =>
        this.view.state.doc.sliceString(from, to);
      // Go up and find the `type` key
      while (cursor.next()) {
        if (textAt(cursor) == `"start"`) {
          cursor.next(); // number
          start = Number(textAt(cursor));
          cursor.next(); // full end line
          cursor.next(); // "end"
          cursor.next(); // number
          end = Number(textAt(cursor));
          if (end != event.end && start != event.start) {
            continue;
          }
          cursor.next(); // '}'
          span.to = cursor.to;
          span.from = Number(
            matchBrackets(this.view.state, cursor.to, -1)?.end
              .from,
          ) || 0;
          break;
        }
      }
      if (span.from < span.to) {
        createHighlightEffect(this.view, { start: span.from, end: span.to });
        if (isFinite(start) && isFinite(end) && start < end) {
          this.dispatchEvent(new ViewerHighlight(start, end));
        }
        if (event.type == "editor-focus") {
          this.view.dispatch({
            selection: { anchor: span.from, head: span.to },
            scrollIntoView: true,
          });
        }
      }
    }
  }

  connectedCallback() {
    this.ownerDocument.addEventListener("change", this);
    this.ownerDocument.addEventListener("diagnostic", this);
    this.ownerDocument.addEventListener("editor-highlight", this);
    this.ownerDocument.addEventListener("editor-focus", this);
    this.shadowRoot.addEventListener("mouseover", this);
    this.view = new EditorView({
      extensions: this.extensions,
      parent: this.shadowRoot,
      root: this.shadowRoot,
      doc: this.textContent,
    });
  }
}

class MetricObserver extends HTMLElement {
  static define(tagName = "metric-observer") {
    customElements.define(tagName, this);
  }

  shadowRoot = this.attachShadow({ mode: "open" });

  connectedCallback() {
    this.shadowRoot.innerHTML =
      `<slot></slot><span part=value></span><span part=unit></span>`;
    this.ownerDocument.addEventListener("metric", (e) => {
      if (e.name !== this.getAttribute("name")) return;
      if (Math.floor(e.value) == e.value) {
        this.shadowRoot.querySelector(
          "[part=value]",
        ).textContent = `${e.value}`;
      } else {
        this.shadowRoot.querySelector(
          "[part=value]",
        ).textContent = `${e.value.toFixed(2)}`;
      }
      this.shadowRoot.querySelector("[part=unit]").textContent = `${e.unit}`;
    });
  }
}

class ErrorDiagnosticCount extends HTMLElement {
  static define(tagName = "error-diagnostic-count") {
    customElements.define(tagName, this);
  }

  shadowRoot = this.attachShadow({ mode: "open" });

  connectedCallback() {
    this.ownerDocument.addEventListener("diagnostic", (e) => {
      this.setAttribute("count", e.diagnostics.length);
      this.shadowRoot.textContent = e.diagnostics.length;
    });
  }
}

class ViewOptions extends HTMLElement {
  static define(tagName = "view-options") {
    customElements.define(tagName, this);
  }

  connectedCallback() {
    this.addEventListener("change", this);
    this.setValueFromUrl();
  }

  handleEvent(e) {
    if (event.target.matches("input[type=radio][name=format]")) {
      const url = new URL(window.location);
      url.searchParams.set("format", event.target.value);
      window.history.replaceState(
        { path: url.toString() },
        null,
        url.toString(),
      );
    }
  }

  setValueFromUrl() {
    const url = new URL(window.location);
    if (!url.searchParams.has("format")) return;
    const format = url.searchParams.get("format");
    for (
      const radio of this.querySelectorAll(
        "input[type=radio][name=format]",
      )
    ) {
      radio.checked = format == radio.value;
    }
  }
}

class ExamplePicker extends HTMLElement {
  static define(tagName = "example-picker") {
    customElements.define(tagName, this);
  }

  connectedCallback() {
    this.addEventListener("change", this);
    this.setValueFromUrl();
  }

  get example() {
    const form = new FormData(this.querySelector("form"));
    return form.get("example");
  }

  handleEvent(e) {
    const url = new URL(window.location);
    url.searchParams.set("example", this.example);
    url.searchParams.delete("code");
    window.history.replaceState({ path: url.toString() }, null, url.toString());
    this.loadExample(this.example);
  }

  setValueFromUrl() {
    const url = new URL(window.location);
    if (!url.searchParams.has("example")) return;
    this.loadExample(url.searchParams.get("example"));
  }

  async loadExample(example) {
    if (example == "") {
      return this.dispatchEvent(new LoadCodeEvent(""));
    }
    const res = await fetch(new URL(example, window.location.href));
    if (res.ok) {
      this.dispatchEvent(new LoadCodeEvent(await res.text()));
    }
  }
}

class WasmLoader extends HTMLElement {
  static define(tagName = "wasm-loader") {
    customElements.define(tagName, this);
  }

  async connectedCallback() {
    MetricObserver.define();
    CsskitViewer.define();
    CsskitEditor.define();
    ViewOptions.define();
    ErrorDiagnosticCount.define();
    ExamplePicker.define();
    this.remove();
  }
}
WasmLoader.define();
