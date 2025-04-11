<template>
  <q-page class="">
    <q-splitter
      v-model="splitPercent"
      style="height: 90vh"
      @update:model-value="updatePanel()"
      :limits="[0, Infinity]"
    >
      <template v-slot:before>
        <div class="q-pa-md">
          <div id="editorRef" ref="editorRef" style="height: 80vh"></div>
        </div>
      </template>

      <template v-slot:after>
        <div class="q-pa-md">
          <ConstraintsVisualization :data="vis" />
        </div>
      </template>
    </q-splitter>
  </q-page>
</template>

<script setup lang="ts">
import { Ref, ref } from 'vue';
import * as monaco from 'monaco-editor';
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
import { language, theme } from 'src/services/PlonkScriptLanguage';
import init, { try_run } from '../transpiler';
import { convertMockProverOutputToObject } from 'src/services/MockProverTranslator';
import { MockProverData } from 'src/services/ConstraintSystem';
import ConstraintsVisualization from '../components/ConstraintsVisualization.vue';
self.MonacoEnvironment = {
  getWorker() {
    return new editorWorker();
  },
};

monaco.languages.register({ id: 'plonkscript' });
monaco.languages.setMonarchTokensProvider('plonkscript', language);
monaco.editor.defineTheme('plonkscript', theme);
// monaco.editor.setModelMarkers();

const splitPercent = ref(50);
const editorRef = ref(null);

const code = `
# k: 4
# in1: 1
# in2: 1

gate add(a, b, c, s) {
    s <| a + b - c;
}

region first_row(a, b, c, s, in1, in2) {
    a[0] <== in1;
    b[0] <== in2;
    c[0] <== a[0] + b[0];
    s[0] <-- enable;

    [b[0], c[0]]
}

region next_row(a, b, c, s, last_b, last_c) {
    a[0] <== last_b;
    b[0] <== last_c;
    c[0] <== a[0] + b[0];
    s[0] <-- enable;

    c[0]
}

let N = 10;

public input in1;
public input in2;
public output out;

private advice a;
private advice b;
private advice c;

public selector s;

add(a, b, c, s);
let fr = first_row(a, b, c, s, in1, in2);
let last_b = fr[0];
let last_c = fr[1];
for i in 1..N {
    let c = next_row(a, b, c, s, last_b, last_c);
    last_b = last_c;
    last_c = c;
}

out <== last_c;

// Press Ctrl+Enter to execute the code
`;
const vis: Ref<MockProverData | undefined> = ref(undefined);

let editor: monaco.editor.IStandaloneCodeEditor | undefined = undefined;

setTimeout(() => {
  if (editorRef.value) {
    // console.log('combine');
    editor = monaco.editor.create(editorRef.value as HTMLElement, {
      value: code,
      language: 'plonkscript',
      theme: 'plonkscript',
      minimap: {
        enabled: false,
      },
    });

    editor.addAction({
      id: 'runCode',

      label: 'RunCode',

      keybindings: [
        monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyR,
        monaco.KeyMod.WinCtrl | monaco.KeyCode.Enter,
        monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter,
      ],

      contextMenuGroupId: 'debug',

      contextMenuOrder: 1.5,

      run: function (ed) {
        const code = ed.getValue();
        const result = try_run({ 
          code, 
          modules: {} 
        });
        vis.value = convertMockProverOutputToObject(result);
      },
    });
  }
}, 300);

init();

function updatePanel() {
  if (!editor) return;
  editor.layout();
}
</script>
